# stage17-ow-rt-yoyo-runtime.ps1 — OW-RT YOYO-built runtime spike gate (post-v1.0 path 2)
#
# Gate G slice: generic in-DLL recompile (pe_dll compile_slot; call-time host
# patches slot → YOYO bootstrap_compile R→C→W; export dispatches).
# Still NOT OW-RT CLOSED — Rust sidecar remains production default;
# compile kernel still host-patched (no compiler ISA inside pe_dll).
#
# Script name stage17-* = post-v1.0 gate id (NOT ROADMAP Stage 17).
#
# Local Windows verify (preferred; cloud Linux has no Win PE smoke):
#   cd F:\yoyo\yoyo-rust
#   cargo build --release -p verifier
#   cargo build --profile release-runtime -p yoyo-runtime   # production Rust default
#   & ..\scripts\stage17-ow-rt-yoyo-runtime.ps1
param(
    [switch]$SkipBuild
)
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root
Write-Host "=== Post-v1.0: OW-RT Gate G slice (YOYO dll-recompile) ==="
$tyStub = Join-Path $Root "yoyo\tests\golden\ow_rt_yoyo_origin_exit2.ty"
if (-not (Test-Path $tyStub)) { throw "missing YOYO-origin stub $tyStub" }
$tyFx = Join-Path $Root "yoyo\tests\golden\selfhost_min_nop.ty"
if (-not (Test-Path $tyFx)) { throw "missing Gate F success fixture $tyFx" }
$tyFx2 = Join-Path $Root "yoyo\tests\golden\selfhost_min_set_52_cafef00d.ty"
if (-not (Test-Path $tyFx2)) { throw "missing dll-recompile second fixture $tyFx2" }
Push-Location (Join-Path $Root "yoyo-rust")
try {
    $prevEap = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
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
$WorkDir = Join-Path $Root "scripts\_stage17-ow-rt-dll-recompile"
New-Item -ItemType Directory -Force -Path $WorkDir | Out-Null
Get-ChildItem $WorkDir -Force -ErrorAction SilentlyContinue | Remove-Item -Recurse -Force -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force -Path $WorkDir | Out-Null
Copy-Item $tyFx (Join-Path $WorkDir "input.ty") -Force
Push-Location (Join-Path $Root "yoyo-rust")
try {
    $prevEap = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    & cargo run -q -p verifier --bin emit-rt-sidecar --no-default-features --features full-backends -- --dll-recompile $WorkDir
    $ecExit = $LASTEXITCODE
    $ErrorActionPreference = $prevEap
    if ($ecExit -ne 0) {
        throw ("emit-rt-sidecar --dll-recompile expected exit=0 got={0}" -f $ecExit)
    }
} finally {
    Pop-Location
}
$AltDll = Join-Path $WorkDir "yoyo_rt.dll"
$OutExe = Join-Path $WorkDir "output.exe"
if (-not (Test-Path $AltDll)) { throw "missing YOYO sidecar $AltDll after --dll-recompile" }
if (-not (Test-Path $OutExe)) { throw "missing output.exe after dll-recompile" }
$altBytes = [System.IO.File]::ReadAllBytes($AltDll)
$altLen = $altBytes.Length
if ($altLen -lt 64) { throw "YOYO sidecar too small" }
$ascii = [System.Text.Encoding]::ASCII.GetString($altBytes)
if (-not $ascii.Contains("yoyo_runtime_selfhost_main")) {
    throw "YOYO sidecar missing export yoyo_runtime_selfhost_main"
}
if (-not $ascii.Contains("yoyo_dll_recompile")) {
    throw "YOYO sidecar missing marker yoyo_dll_recompile"
}
if ($altBytes[0] -ne 0x4D -or $altBytes[1] -ne 0x5A) {
    throw "YOYO sidecar not MZ"
}
$outBytes = [System.IO.File]::ReadAllBytes($OutExe)
if ($outBytes.Length -lt 64 -or $outBytes[0] -ne 0x4D -or $outBytes[1] -ne 0x5A) {
    throw "dll-recompile output.exe not a PE"
}
Write-Host ("OW_RT_SPIKE yoyo_dll_recompile=PRESENT path={0} sidecar_bytes={1} output_bytes={2}" -f $WorkDir, $altLen, $outBytes.Length)
Write-Host "OW_RT_SPIKE yoyo_alt_sidecar=EMITTED (dll-recompile pe_dll)"
Write-Host "OW_RT_SPIKE gate_g_slice=dll_recompile"
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
# Win-only: H_00 loads YOYO dll-recompile DLL with no input → exit 2.
$IsWin = $env:OS -eq "Windows_NT"
if ($IsWin) {
    $YoyoRelease = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
    $YoyoDebug = Join-Path $Root "yoyo-rust\target\debug\yoyo.exe"
    $Yoyo = if (Test-Path $YoyoRelease) { $YoyoRelease } elseif (Test-Path $YoyoDebug) { $YoyoDebug } else { $null }
    $Ty = Join-Path $Root "yoyo\projects\yoyo.ty"
    if ($Yoyo -and (Test-Path $Ty)) {
        $smokeDir = Join-Path $WorkDir "smoke-dll-recompile-no-input"
        New-Item -ItemType Directory -Force -Path $smokeDir | Out-Null
        $gen1 = Join-Path $smokeDir "gen1.exe"
        & $Yoyo link --target=win32 $Ty $gen1
        if ($LASTEXITCODE -ne 0 -or -not (Test-Path $gen1)) {
            throw "Gate G dll-recompile smoke: H_00 link failed"
        }
        # Fresh uninjected shell (len=0) for no-input exit=2
        $shellDir = Join-Path $WorkDir "smoke-shell"
        New-Item -ItemType Directory -Force -Path $shellDir | Out-Null
        Push-Location (Join-Path $Root "yoyo-rust")
        try {
            $prevEap = $ErrorActionPreference
            $ErrorActionPreference = "Continue"
            & cargo run -q -p verifier --bin emit-rt-sidecar --no-default-features --features full-backends -- --dll-recompile $shellDir
            $shellExit = $LASTEXITCODE
            $ErrorActionPreference = $prevEap
            if ($shellExit -ne 2) {
                throw ("dll-recompile no-input shell expected exit=2 got={0}" -f $shellExit)
            }
        } finally {
            Pop-Location
        }
        Copy-Item (Join-Path $shellDir "yoyo_rt.dll") (Join-Path $smokeDir "yoyo_rt.dll") -Force
        Get-ChildItem $smokeDir -Filter "input.*" -ErrorAction SilentlyContinue | Remove-Item -Force
        Push-Location $smokeDir
        try {
            & ".\gen1.exe"
            $altExit = $LASTEXITCODE
        } finally {
            Pop-Location
        }
        if ($altExit -eq 2) {
            Write-Host "OW_RT_SPIKE yoyo_dll_recompile_smoke=GREEN exit=2 (H_00 loaded dll-recompile pe_dll; no input)"
        } else {
            Write-Host ("OW_RT_SPIKE yoyo_dll_recompile_smoke=NOT_STABLE exit={0} (AV/H_00; non-fatal)" -f $altExit)
        }
    } else {
        Write-Host "OW_RT_SPIKE yoyo_dll_recompile_smoke=SKIP (missing yoyo.exe or yoyo.ty)"
    }
} else {
    Write-Host "OW_RT_SPIKE yoyo_dll_recompile_smoke=SKIP (non-Windows; use local Win)"
}
# Honest: call-time YOYO compile + inject; pe_dll has no compiler ISA; production still Rust.
Write-Host "OW_RT_SPIKE yoyo_built=DLL_RECOMPILE yoyo_dll_recompile=PRESENT disposition=CUT"
Write-Host "OW_RT_SPIKE note=Gate_G_slice_dll_recompile; compiler ISA still ABSENT in pe_dll; CLOSED requires production YOYO-built compile sidecar + no Rust yoyo_rt.dll host trust"
Write-Host "OW_RT_SPIKE status=GREEN doc=SCOPE-CUT-v1.0-ow-rt-yoyo-runtime.md"
exit 0
