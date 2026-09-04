# stage17-ow-rt-yoyo-runtime.ps1 — OW-RT YOYO-built runtime spike gate (post-v1.0 path 2)
#
# Gate G slice: emit YOYO pe_dll as alternative cwd yoyo_rt.dll (opt-in path).
# Still NOT OW-RT CLOSED — Rust sidecar remains production default.
#
# Script name stage17-* = post-v1.0 gate id (NOT ROADMAP Stage 17).
#
# Local Windows verify (cloud Linux has no Win PE smoke):
#   cd F:\yoyo\yoyo-rust
#   cargo build --release -p verifier
#   cargo build --profile release-runtime -p yoyo-runtime   # production Rust default
#   & ..\scripts\stage17-ow-rt-yoyo-runtime.ps1
# Optional H_00 load of YOYO alt (expect exit 2, no input.tyb):
#   $work = Join-Path $env:TEMP 'yoyo-ow-rt-alt'
#   New-Item -ItemType Directory -Force -Path $work | Out-Null
#   cargo run -p verifier --bin emit-rt-sidecar -- (Join-Path $work 'yoyo_rt.dll')
#   & ..\target\release\yoyo.exe link --target=win32 ..\yoyo\projects\yoyo.ty (Join-Path $work 'gen1.exe')
#   Push-Location $work; & .\gen1.exe; Pop-Location   # expect exit 2
param(
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== Post-v1.0: OW-RT Gate G slice (YOYO pe_dll alt sidecar emit) ==="

$tyStub = Join-Path $Root "yoyo\tests\golden\ow_rt_yoyo_origin_exit2.ty"
if (-not (Test-Path $tyStub)) { throw "missing YOYO-origin stub $tyStub" }
$tyFx = Join-Path $Root "yoyo\tests\golden\selfhost_min_nop.ty"
if (-not (Test-Path $tyFx)) { throw "missing Gate F success fixture $tyFx" }

Push-Location (Join-Path $Root "yoyo-rust")
try {
    # cargo writes warnings to stderr; don't treat as terminating under Stop
    $prevEap = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    # --lib: pe_dll_link lives in verifier lib; avoid bin/wasmtime when toolchain lacks edition2024
    & cargo test -p verifier --lib pe_dll_link --no-default-features --features full-backends
    $cargoExit = $LASTEXITCODE
    $ErrorActionPreference = $prevEap
    if ($cargoExit -ne 0) { throw "pe_dll_link tests failed" }
} finally {
    Pop-Location
}
Write-Host "OW_RT_SPIKE pe_dll_link_tests=GREEN"
Write-Host "OW_RT_SPIKE yoyo_origin_export=PRESENT stub=$tyStub"
Write-Host "OW_RT_SPIKE yoyo_built_effect=PRESENT fixture=$tyFx exits=0/1/2/3"

$spikeDoc = Join-Path $Root "SCOPE-CUT-v1.0-ow-rt-yoyo-runtime.md"
if (-not (Test-Path $spikeDoc)) { throw "missing $spikeDoc" }

$WorkDir = Join-Path $Root "scripts\_stage17-ow-rt-alt-sidecar"
New-Item -ItemType Directory -Force -Path $WorkDir | Out-Null
$AltDll = Join-Path $WorkDir "yoyo_rt.dll"
if (Test-Path $AltDll) { Remove-Item $AltDll -Force }

Push-Location (Join-Path $Root "yoyo-rust")
try {
    $prevEap = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    & cargo run -q -p verifier --bin emit-rt-sidecar --no-default-features --features full-backends -- $AltDll
    $emitExit = $LASTEXITCODE
    $ErrorActionPreference = $prevEap
    if ($emitExit -ne 0) { throw "emit-rt-sidecar failed" }
} finally {
    Pop-Location
}
if (-not (Test-Path $AltDll)) { throw "missing YOYO alt sidecar $AltDll" }

$altBytes = [System.IO.File]::ReadAllBytes($AltDll)
$altLen = $altBytes.Length
if ($altLen -lt 64) { throw "YOYO alt sidecar too small" }
$ascii = [System.Text.Encoding]::ASCII.GetString($altBytes)
if (-not $ascii.Contains("yoyo_runtime_selfhost_main")) {
    throw "YOYO alt sidecar missing export yoyo_runtime_selfhost_main"
}
if (-not $ascii.Contains("yoyo_rt.dll")) {
    throw "YOYO alt sidecar missing dll name yoyo_rt.dll"
}
if ($altBytes[0] -ne 0x4D -or $altBytes[1] -ne 0x5A) {
    throw "YOYO alt sidecar not MZ"
}
Write-Host ("OW_RT_SPIKE yoyo_alt_sidecar=EMITTED path={0} bytes={1}" -f $AltDll, $altLen)

$RuntimePreferred = Join-Path $Root "yoyo-rust\target\release-runtime\yoyo_runtime.dll"
$RuntimeCompat = Join-Path $Root "yoyo-rust\target\release\yoyo_runtime.dll"
$RuntimeDll = $null
if (Test-Path $RuntimePreferred) {
    $RuntimeDll = $RuntimePreferred
} elseif (Test-Path $RuntimeCompat) {
    $RuntimeDll = $RuntimeCompat
}

if (-not $RuntimeDll) {
    if ($SkipBuild) {
        Write-Host "OW_RT_SPIKE rust_sidecar=ABSENT_ON_DISK (SkipBuild; production default still Rust)"
    } else {
        Push-Location (Join-Path $Root "yoyo-rust")
        try {
            Write-Host "== build yoyo-runtime (release-runtime) =="
            & cargo build --profile release-runtime -p yoyo-runtime
            if ($LASTEXITCODE -ne 0) { throw "yoyo-runtime build failed" }
        } finally {
            Pop-Location
        }
        if (Test-Path $RuntimePreferred) {
            $RuntimeDll = $RuntimePreferred
        } elseif (Test-Path $RuntimeCompat) {
            $RuntimeDll = $RuntimeCompat
        } else {
            throw "yoyo_runtime.dll still missing after build"
        }
    }
}

if ($RuntimeDll) {
    $dllBytes = (Get-Item $RuntimeDll).Length
    $dllSha = (Get-FileHash -Algorithm SHA256 -Path $RuntimeDll).Hash.ToLowerInvariant().Substring(0, 16)
    Write-Host "OW_RT_SPIKE rust_sidecar path=$RuntimeDll bytes=$dllBytes sha256_prefix=$dllSha"
    Write-Host "OW_RT_SPIKE production_default=RUST rust_sidecar=PRESENT"
} else {
    Write-Host "OW_RT_SPIKE production_default=RUST rust_sidecar=NOT_BUILT_HERE"
}

# Win-only: optional no-input smoke with YOYO alt (expect exit 2).
$IsWin = $env:OS -eq "Windows_NT"
if ($IsWin) {
    $YoyoRelease = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
    $YoyoDebug = Join-Path $Root "yoyo-rust\target\debug\yoyo.exe"
    $Yoyo = if (Test-Path $YoyoRelease) { $YoyoRelease } elseif (Test-Path $YoyoDebug) { $YoyoDebug } else { $null }
    $Ty = Join-Path $Root "yoyo\projects\yoyo.ty"
    if ($Yoyo -and (Test-Path $Ty)) {
        $smokeDir = Join-Path $WorkDir "smoke-alt-no-input"
        New-Item -ItemType Directory -Force -Path $smokeDir | Out-Null
        $gen1 = Join-Path $smokeDir "gen1.exe"
        & $Yoyo link --target=win32 $Ty $gen1
        if ($LASTEXITCODE -ne 0 -or -not (Test-Path $gen1)) {
            throw "Gate G alt-sidecar smoke: H_00 link failed"
        }
        Copy-Item $AltDll (Join-Path $smokeDir "yoyo_rt.dll") -Force
        # No input.* → export should return 2 (YOYO-origin probe)
        Get-ChildItem $smokeDir -Filter "input.*" -ErrorAction SilentlyContinue | Remove-Item -Force
        Push-Location $smokeDir
        try {
            & ".\gen1.exe"
            $altExit = $LASTEXITCODE
        } finally {
            Pop-Location
        }
        if ($altExit -ne 2) {
            throw ("Gate G alt-sidecar no-input smoke expected exit=2 got={0}" -f $altExit)
        }
        Write-Host "OW_RT_SPIKE yoyo_alt_sidecar_smoke=GREEN exit=2 (H_00 loaded YOYO pe_dll)"
    } else {
        Write-Host "OW_RT_SPIKE yoyo_alt_sidecar_smoke=SKIP (missing yoyo.exe or yoyo.ty)"
    }
} else {
    Write-Host "OW_RT_SPIKE yoyo_alt_sidecar_smoke=SKIP (non-Windows; use local Win commands above)"
}

# Honest: alt emit wired; production default still Rust → OW-RT remains CUT.
Write-Host "OW_RT_SPIKE yoyo_built=ALT_SIDECAR yoyo_alt_sidecar=EMITTED disposition=CUT"
Write-Host "OW_RT_SPIKE note=Gate_G_slice_alt_emit_only; CLOSED requires production YOYO-built sidecar + no Rust yoyo_rt.dll host trust"
Write-Host "OW_RT_SPIKE status=GREEN doc=SCOPE-CUT-v1.0-ow-rt-yoyo-runtime.md"
exit 0
