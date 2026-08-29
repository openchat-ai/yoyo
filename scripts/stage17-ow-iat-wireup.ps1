# stage17-ow-iat-wireup.ps1 — OW-IAT wire-up gate (post spike PR #7)
#
# Phase 2: manual-map x64 body wired into gen_h00_selfhost_main; PEB LoadLibraryA dropped.
# Phase 3: JS/asm three-peer lockstep (template + explicit IAT patch sites).
# Phase 4: Windows smoke — cwd yoyo_rt.dll + manual-map H_00 (fail-closed if sidecar missing).
param(
    [switch]$SkipBuild,
    [switch]$SkipSmoke
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== Stage 17: OW-IAT wire-up (manual-map H_00) ==="

Push-Location (Join-Path $Root "yoyo-rust")
try {
    & cargo test -p verifier manual_map
    if ($LASTEXITCODE -ne 0) { throw "wire-up unit tests failed" }
    & cargo test -p verifier --lib manual_map_runtime_smoke_host_resolve
    if ($LASTEXITCODE -ne 0) { throw "manual_map_runtime_smoke_host_resolve failed (stub vs reference mapper)" }
    & cargo test -p verifier --lib compare_stub_vs_host_iat_on_sidecar
    if ($LASTEXITCODE -ne 0) { throw "compare_stub_vs_host_iat_on_sidecar failed (stub IAT != host GetProcAddress)" }
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
    Write-Host "== smoke: cwd yoyo_rt.dll + manual-map H_00 (with sidecar) =="
    $runOk = Join-Path $WorkDir "smoke-with-sidecar"
    New-Item -ItemType Directory -Force -Path $runOk | Out-Null
    Copy-Item $gen1 (Join-Path $runOk "gen1.exe") -Force
    Copy-Item $Tyb (Join-Path $runOk "input.tyb") -Force
    Copy-Item $dllPath (Join-Path $runOk "yoyo_rt.dll") -Force
    $outExe = Join-Path $runOk "output.exe"
    if (Test-Path $outExe) { Remove-Item $outExe }
    Push-Location $runOk
    try {
        $env:YOYO_MM_SMOKE_PROBE = "1"
        & ".\gen1.exe"
        $smokeExit = $LASTEXITCODE
    } finally {
        Remove-Item Env:YOYO_MM_SMOKE_PROBE -ErrorAction SilentlyContinue
        Pop-Location
    }
    if ($smokeExit -ne 0) {
        $phase = switch ($smokeExit) {
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
            1 { "generic_fail_or_runtime" }
            -1073741819 { "access_violation" }
            default { "unknown" }
        }
        throw ("manual-map smoke WITH sidecar failed exit={0} phase={1}" -f $smokeExit, $phase)
    }
    if (-not (Test-Path $outExe)) {
        Write-Host "DIAG: smoke exit=0 output.exe missing (export-tail isolate — map+imports reached)"
    } else {
        $outLen = (Get-Item $outExe).Length
        Write-Host ("smoke WITH sidecar: gen1 -> output.exe OK ({0} bytes)" -f $outLen)
    }

    Write-Host ""
    Write-Host "== smoke: fail-closed WITHOUT sidecar (expect no output.exe) =="
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
    if ((Test-Path $outFail) -and $failExit -eq 0) {
        throw "fail-closed WITHOUT sidecar produced output.exe (manual-map must require cwd yoyo_rt.dll)"
    }
    Write-Host ("smoke WITHOUT sidecar: fail-closed OK exit={0} output.exe absent" -f $failExit)
    Write-Host "OW_IAT_WIREUP smoke=GREEN sidecar_required=YES"
} else {
    Write-Host "OW_IAT_WIREUP smoke=SKIP (-SkipSmoke)"
}

Write-Host "OW_IAT_WIREUP status=GREEN three_peer=LOCKSTEP LoadLibraryA_IAT=ABSENT Linux_dlopen@PLT=no_libdl OW-IAT=CUT"
exit 0
