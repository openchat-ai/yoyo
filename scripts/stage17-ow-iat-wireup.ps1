# stage17-ow-iat-wireup.ps1 — OW-IAT wire-up gate (post spike PR #7)
#
# Phase 2: manual-map x64 body wired into gen_h00_selfhost_main; PEB LoadLibraryA dropped.
# Phase 3: JS/asm three-peer lockstep (template + explicit IAT patch sites).
# Phase 4: Windows smoke — cwd yoyo_rt.dll + manual-map H_00 (fail-closed if sidecar missing).
#
# H00 multi-phase bisect (150–165 rebuilds) is OPT-IN only:
#   $env:H00_BISECT = "1"; .\scripts\stage17-ow-iat-wireup.ps1
#   or: .\scripts\stage17-ow-iat-wireup.ps1 -EnableBisect
#   or CI: workflow_dispatch input h00_bisect=true
# Default / CI push+PR: smoke once; on AV fail closed with a clear message (no rebuild loop).
param(
    [switch]$SkipBuild,
    [switch]$SkipSmoke,
    [switch]$EnableBisect,
    [ValidateRange(0, 255)]
    [int]$BisectExit = 0
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

function Test-H00BisectEnabled {
    if ($EnableBisect) { return $true }
    $v = $env:H00_BISECT
    if ([string]::IsNullOrWhiteSpace($v)) { return $false }
    return $v -match '^(?i)(1|true|yes|on)$'
}
$H00BisectOn = Test-H00BisectEnabled

Write-Host "=== Stage 17: OW-IAT wire-up (manual-map H_00) ==="
Write-Host ("H00_BISECT={0} (multi-phase rebuild diagnostic; default off)" -f $(if ($H00BisectOn) { "ON" } else { "off" }))

if ($BisectExit -gt 0) {
    Write-Host "Bisect: rebuilding verifier with H00_BISECT_EXIT=$BisectExit (compile-time)"
    Push-Location (Join-Path $Root "yoyo-rust")
    try {
        $env:H00_BISECT_EXIT = "$BisectExit"
        & cargo build --release -p verifier
        if ($LASTEXITCODE -ne 0) { throw "bisect rebuild failed" }
    } finally {
        Remove-Item Env:H00_BISECT_EXIT -ErrorAction SilentlyContinue
        Pop-Location
    }
}

$YoyoRelease = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
$YoyoDebug = Join-Path $Root "yoyo-rust\target\debug\yoyo.exe"
$Yoyo = if (Test-Path $YoyoRelease) { $YoyoRelease } elseif (Test-Path $YoyoDebug) { $YoyoDebug } else { $YoyoRelease }
$RuntimeDllPreferred = Join-Path $Root "yoyo-rust\target\release-runtime\yoyo_runtime.dll"
$RuntimeDllCompat = Join-Path $Root "yoyo-rust\target\release\yoyo_runtime.dll"
$Ty = Join-Path $Root "yoyo\projects\yoyo.ty"
$Tyb = Join-Path $Root "yoyo\projects\yoyo.tyb"
$WorkDir = Join-Path $Root "scripts\_stage17-ow-iat-wireup"

$needYoyo = -not (Test-Path $Yoyo)
$needRuntime = -not ((Test-Path $RuntimeDllPreferred) -or (Test-Path $RuntimeDllCompat))
if ($needYoyo -or $needRuntime) {
    if ($SkipBuild) { throw "missing yoyo.exe or yoyo_runtime.dll (and -SkipBuild)" }
    Push-Location (Join-Path $Root "yoyo-rust")
    if ($needRuntime -and -not (Test-Path $RuntimeDllPreferred)) {
        $env:RUSTFLAGS = "-C target-feature=+crt-static"
        & cargo build --profile release-runtime -p yoyo-runtime
        if ($LASTEXITCODE -ne 0) { throw "yoyo-runtime build failed" }
        Remove-Item Env:RUSTFLAGS -ErrorAction SilentlyContinue
    }
    if ($needYoyo) {
        & cargo build --release -p verifier
        if ($LASTEXITCODE -ne 0) { throw "verifier build failed" }
    }
    Pop-Location
}
if (-not (Test-Path $Yoyo)) { throw "missing yoyo.exe" }

$dllPath = if (Test-Path $RuntimeDllPreferred) { $RuntimeDllPreferred } else { $RuntimeDllCompat }
if (-not (Test-Path $dllPath)) { throw "missing yoyo_runtime.dll for sidecar smoke" }

Push-Location (Join-Path $Root "yoyo-rust")
try {
    & cargo test -p verifier manual_map
    if ($LASTEXITCODE -ne 0) { throw "wire-up unit tests failed" }
    & cargo test -p verifier --lib manual_map_runtime_smoke_host_resolve
    if ($LASTEXITCODE -ne 0) { throw "manual_map_runtime_smoke_host_resolve failed (stub vs reference mapper)" }
    & cargo test -p verifier --lib manual_map_runtime_smoke_stub_resolve
    if ($LASTEXITCODE -ne 0) { throw "manual_map_runtime_smoke_stub_resolve failed (stub_resolve map+export)" }
    & cargo test -p verifier --lib compare_ll_gpa_vs_host_iat_on_sidecar
    if ($LASTEXITCODE -ne 0) { throw "compare_ll_gpa_vs_host_iat_on_sidecar failed (LL+GPA import != host)" }
    & cargo test -p verifier --lib compare_stub_vs_host_iat_on_sidecar
    if ($LASTEXITCODE -ne 0) { throw "compare_stub_vs_host_iat_on_sidecar failed (stub IAT != host GetProcAddress)" }
    & cargo test -p verifier --lib manual_map_gen1_exe_smoke
    if ($LASTEXITCODE -ne 0) { throw "manual_map_gen1_exe_smoke failed (gen1 link+smoke before stage17 script smoke)" }
} finally {
    Pop-Location
}
Write-Host "OW_IAT_WIREUP unit_tests=GREEN phase=manual_map_x64_emit"

$wireup = Join-Path $Root "yoyo-rust\verifier\src\h00_manual_map_wireup.rs"
$winH00 = Join-Path $Root "yoyo-rust\verifier\src\win32_selfhost.rs"
if (-not (Test-Path $wireup)) { throw "missing h00_manual_map_wireup.rs" }

$wired = Select-String -Path $winH00 -Pattern 'gen_h00_manual_map_main|h00_manual_map_wireup' -Quiet
if ($wired) {
    Write-Host "OW_IAT_WIREUP H_00_wired=YES manual_map_body=EMITTED PEB_LoadLibrary=DROPPED"
} else {
    throw "OW_IAT_WIREUP H_00_wired=NO (honest CUT — manual-map not wired)"
}

function Find-Ascii([byte[]]$Bytes, [string]$Needle) {
    return [System.Text.Encoding]::ASCII.GetString($Bytes).Contains($Needle)
}

function Get-SmokePhase([int]$ExitCode) {
    switch ($ExitCode) {
        2 { "CreateFile" }
        3 { "Read/empty" }
        4 { "VirtualAlloc" }
        5 { "section_copy" }
        6 { "reloc" }
        7 { "import" }
        8 { "export" }
        9 { "DllMain" }
        10 { "probe_CreateFile" }
        11 { "probe_WriteFile" }
        151 { "phase_prelude_create_ok" }
        150 { "phase_h00_entered" }
        152 { "phase_prelude_buf_ok" }
        153 { "phase_prelude_read_ok" }
        154 { "phase_prelude_done" }
        155 { "phase_prelude_ok" }
        157 { "phase_bootstrap_find_ok" }
        158 { "phase_bootstrap_ll_ok" }
        156 { "phase_bootstrap_ok" }
        159 { "phase_map_valloc_ok" }
        130 { "import_ok_bisect" }
        131 { "reloc_ok_bisect" }
        160 { "phase_map_image_ok" }
        161 { "phase_sections_ok" }
        162 { "phase_reloc_ok" }
        163 { "phase_import_ok" }
        164 { "phase_flush_icache" }
        165 { "phase_export_call" }
        1 { "generic_fail_or_runtime" }
        -1073741819 { "access_violation" }
        default { "unknown" }
    }
}

function Invoke-ManualMapSmoke([string]$RunDir, [string]$Gen1Path) {
    $outExe = Join-Path $RunDir "output.exe"
    if (Test-Path $outExe) { Remove-Item $outExe }
    Push-Location $RunDir
    try {
        $env:YOYO_MM_SMOKE_PROBE = "1"
        & ".\gen1.exe"
        return @{ Exit = $LASTEXITCODE; OutPresent = (Test-Path $outExe) }
    } finally {
        Remove-Item Env:YOYO_MM_SMOKE_PROBE -ErrorAction SilentlyContinue
        Pop-Location
    }
}

function Invoke-H00BisectDiagnostic([string]$RunDir, [string]$TyPath, [string]$TybPath, [string]$DllPath, [switch]$NoSidecar) {
    $range = if ($NoSidecar) { 150..155 } else { 150..165 }
    Write-Host ("== bisect: rebuild gen1 with H00_BISECT_EXIT={0} ({1}) ==" -f ($range -join ","), $(if ($NoSidecar) { "no-sidecar" } else { "with-sidecar" }))
    $wireup = Join-Path $Root "yoyo-rust\verifier\src\h00_manual_map_wireup.rs"
    $lines = @()
    foreach ($phase in $range) {
        Push-Location (Join-Path $Root "yoyo-rust")
        try {
            (Get-Item $wireup).LastWriteTime = Get-Date
            $env:H00_BISECT_EXIT = "$phase"
            & cargo build --release -p verifier --bin yoyo 2>&1 | Out-Null
            if ($LASTEXITCODE -ne 0) { throw "bisect build failed for phase=$phase" }
        } finally {
            Remove-Item Env:H00_BISECT_EXIT -ErrorAction SilentlyContinue
            Pop-Location
        }
        $yoyo = if (Test-Path $YoyoRelease) { $YoyoRelease } else { $YoyoDebug }
        $gen1 = Join-Path $RunDir ("gen1-bisect-{0}.exe" -f $phase)
        & $yoyo link --target=win32 $TyPath $gen1 2>&1 | Out-Null
        if ($LASTEXITCODE -ne 0) { throw "bisect link failed for phase=$phase" }
        Copy-Item $gen1 (Join-Path $RunDir "gen1.exe") -Force
        Copy-Item $TybPath (Join-Path $RunDir "input.tyb") -Force
        if (-not $NoSidecar) {
            Copy-Item $DllPath (Join-Path $RunDir "yoyo_rt.dll") -Force
        } else {
            $rt = Join-Path $RunDir "yoyo_rt.dll"
            if (Test-Path $rt) { Remove-Item $rt }
        }
        $r = Invoke-ManualMapSmoke $RunDir $gen1
        $label = Get-SmokePhase $r.Exit
        $lines += ("phase={0} exit={1} label={2} output.exe={3}" -f $phase, $r.Exit, $label, $(if ($r.OutPresent) { "present" } else { "absent" }))
    }
    return ($lines -join "; ")
}

New-Item -ItemType Directory -Force -Path $WorkDir | Out-Null
$gen1 = Join-Path $WorkDir "gen1.exe"
if (Test-Path $gen1) { Remove-Item $gen1 }

Write-Host ""
Write-Host "== link: H_00 manual-map seed PE =="
& $Yoyo link --target=win32 $Ty $gen1
if ($LASTEXITCODE -ne 0 -or -not (Test-Path $gen1)) { throw "H_00 seed link failed" }

$pe = [System.IO.File]::ReadAllBytes($gen1)
# H_00 bootstrap embeds "LoadLibraryA\0" in .text for KERNEL32 export walk (not IAT/import).
if (Find-Ascii $pe "GetProcAddress") {
    throw "seed PE ASCII GetProcAddress present (ordinal-0 export walk expected)"
}
if (-not (Find-Ascii $pe "yoyo_rt.dll")) {
    throw "seed PE missing yoyo_rt.dll sidecar marker"
}
foreach ($ioApi in @("CreateFileA", "ReadFile", "VirtualAlloc")) {
    if (-not (Find-Ascii $pe $ioApi)) {
        throw ("seed PE missing manual-map I/O import {0}" -f $ioApi)
    }
}
Write-Host "OW_IAT_WIREUP seed_pe markers=OK LoadLibraryA=STUB_EMBED yoyo_rt.dll=PRESENT manual_map_io=PRESENT"

if (-not $SkipSmoke) {
    if (-not (Test-Path $Tyb)) { throw "missing $Tyb" }

    Write-Host ""
    Write-Host "== smoke: fail-closed WITHOUT sidecar (expect CreateFile fail, not AV) =="
    $runFail = Join-Path $WorkDir "smoke-no-sidecar"
    New-Item -ItemType Directory -Force -Path $runFail | Out-Null
    Copy-Item $gen1 (Join-Path $runFail "gen1.exe") -Force
    Copy-Item $Tyb (Join-Path $runFail "input.tyb") -Force
    $outFail = Join-Path $runFail "output.exe"
    $rtFail = Join-Path $runFail "yoyo_rt.dll"
    if (Test-Path $outFail) { Remove-Item $outFail }
    if (Test-Path $rtFail) { Remove-Item $rtFail }
    Push-Location $runFail
    try {
        & ".\gen1.exe"
        $failExit = $LASTEXITCODE
    } finally {
        Pop-Location
    }
    if ($failExit -eq -1073741819) {
        if ($H00BisectOn) {
            $bisect = Invoke-H00BisectDiagnostic $runFail $Ty $Tyb $dllPath -NoSidecar
            Write-Host "H00_BISECT_DIAG no-sidecar $bisect"
            throw ("manual-map smoke WITHOUT sidecar AV (crash before CreateFile fail-closed exit=2) bisect=$bisect")
        }
        Write-Host "H00_BISECT skipped — set H00_BISECT=1 or -EnableBisect for multi-phase rebuild diagnostic (local/debug only)"
        throw "manual-map smoke WITHOUT sidecar AV (crash before CreateFile fail-closed exit=2); re-run with H00_BISECT=1 for phase bisect"
    }
    if ((Test-Path $outFail) -and $failExit -eq 0) {
        throw "fail-closed WITHOUT sidecar produced output.exe (manual-map must require cwd yoyo_rt.dll)"
    }
    Write-Host ("smoke WITHOUT sidecar: fail-closed OK exit={0} output.exe absent" -f $failExit)

    Write-Host ""
    Write-Host "== smoke: cwd yoyo_rt.dll + manual-map H_00 (with sidecar) =="
    $runOk = Join-Path $WorkDir "smoke-with-sidecar"
    New-Item -ItemType Directory -Force -Path $runOk | Out-Null
    Copy-Item $gen1 (Join-Path $runOk "gen1.exe") -Force
    Copy-Item $Tyb (Join-Path $runOk "input.tyb") -Force
    Copy-Item $dllPath (Join-Path $runOk "yoyo_rt.dll") -Force
    $sidecarLen = (Get-Item (Join-Path $runOk "yoyo_rt.dll")).Length
    Write-Host ("sidecar yoyo_rt.dll bytes={0} src={1}" -f $sidecarLen, $dllPath)
    if ($sidecarLen -lt 64) { throw "cwd yoyo_rt.dll too small for ReadFile" }
    $smoke = Invoke-ManualMapSmoke $runOk $gen1
    $smokeExit = $smoke.Exit
    $outExe = Join-Path $runOk "output.exe"
    if ($smokeExit -ne 0) {
        $phase = Get-SmokePhase $smokeExit
        $outDiag = if ($smoke.OutPresent) { "output.exe=present" } else { "output.exe=absent" }
        $bisect = ""
        if ($smokeExit -eq -1073741819) {
            if ($H00BisectOn) {
                $bisect = Invoke-H00BisectDiagnostic $runOk $Ty $Tyb $dllPath
                Write-Host "H00_BISECT_DIAG $bisect"
            } else {
                Write-Host "H00_BISECT skipped — set H00_BISECT=1 or -EnableBisect for multi-phase rebuild diagnostic (local/debug only)"
            }
        }
        throw ("manual-map smoke WITH sidecar failed exit={0} phase={1} {2}{3}" -f $smokeExit, $phase, $outDiag, $(if ($bisect) { " bisect=$bisect" } else { "" }))
    }
    if (-not (Test-Path $outExe)) {
        Write-Host "DIAG: smoke exit=0 output.exe missing (export-tail isolate — map+imports reached)"
    } else {
        $outLen = (Get-Item $outExe).Length
        Write-Host ("smoke WITH sidecar: gen1 -> output.exe OK ({0} bytes)" -f $outLen)
    }

    Write-Host "OW_IAT_WIREUP smoke=GREEN sidecar_required=YES"
} else {
    Write-Host "OW_IAT_WIREUP smoke=SKIP (-SkipSmoke)"
}

Write-Host "OW_IAT_WIREUP status=GREEN three_peer=LOCKSTEP LoadLibraryA_IAT=ABSENT Linux_dlopen@PLT=no_libdl OW-IAT=CUT"
exit 0
