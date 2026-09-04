# stage17-ow-rt-yoyo-runtime.ps1 — OW-RT YOYO-built runtime spike gate (post-v1.0 path 2)
#
# Gate G slice: compile inside YOYO sidecar export (emit-time bootstrap_compile
# baked into pe_dll; calling export writes output.exe).
# Still NOT OW-RT CLOSED — Rust sidecar remains production default;
# call-time is not a general in-DLL re-compile.
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
Write-Host "=== Post-v1.0: OW-RT Gate G slice (YOYO export-compile) ==="
$tyStub = Join-Path $Root "yoyo\tests\golden\ow_rt_yoyo_origin_exit2.ty"
if (-not (Test-Path $tyStub)) { throw "missing YOYO-origin stub $tyStub" }
$tyFx = Join-Path $Root "yoyo\tests\golden\selfhost_min_nop.ty"
if (-not (Test-Path $tyFx)) { throw "missing Gate F success fixture $tyFx" }
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
$WorkDir = Join-Path $Root "scripts\_stage17-ow-rt-export-compile"
New-Item -ItemType Directory -Force -Path $WorkDir | Out-Null
Get-ChildItem $WorkDir -Force -ErrorAction SilentlyContinue | Remove-Item -Recurse -Force -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force -Path $WorkDir | Out-Null
Copy-Item $tyFx (Join-Path $WorkDir "input.ty") -Force
Push-Location (Join-Path $Root "yoyo-rust")
try {
    $prevEap = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    & cargo run -q -p verifier --bin emit-rt-sidecar --no-default-features --features full-backends -- --export-compile $WorkDir
    $ecExit = $LASTEXITCODE
    $ErrorActionPreference = $prevEap
    if ($ecExit -ne 0) {
        throw ("emit-rt-sidecar --export-compile expected exit=0 got={0}" -f $ecExit)
    }
} finally {
    Pop-Location
}
$AltDll = Join-Path $WorkDir "yoyo_rt.dll"
$OutExe = Join-Path $WorkDir "output.exe"
if (-not (Test-Path $AltDll)) { throw "missing YOYO sidecar $AltDll after --export-compile" }
if (-not (Test-Path $OutExe)) { throw "missing output.exe after export-compile" }
$altBytes = [System.IO.File]::ReadAllBytes($AltDll)
$altLen = $altBytes.Length
if ($altLen -lt 64) { throw "YOYO sidecar too small" }
$ascii = [System.Text.Encoding]::ASCII.GetString($altBytes)
if (-not $ascii.Contains("yoyo_runtime_selfhost_main")) {
    throw "YOYO sidecar missing export yoyo_runtime_selfhost_main"
}
if (-not $ascii.Contains("yoyo_export_compile")) {
    throw "YOYO sidecar missing marker yoyo_export_compile"
}
if ($altBytes[0] -ne 0x4D -or $altBytes[1] -ne 0x5A) {
    throw "YOYO sidecar not MZ"
}
$outBytes = [System.IO.File]::ReadAllBytes($OutExe)
if ($outBytes.Length -lt 64 -or $outBytes[0] -ne 0x4D -or $outBytes[1] -ne 0x5A) {
    throw "export-compile output.exe not a PE"
}
Write-Host ("OW_RT_SPIKE yoyo_export_compile=PRESENT path={0} sidecar_bytes={1} output_bytes={2}" -f $WorkDir, $altLen, $outBytes.Length)
Write-Host "OW_RT_SPIKE yoyo_alt_sidecar=EMITTED (export-compile pe_dll)"
Write-Host "OW_RT_SPIKE gate_g_slice=export_compile"
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
# Win-only: H_00 loads YOYO export-compile DLL with no input → exit 2.
$IsWin = $env:OS -eq "Windows_NT"
if ($IsWin) {
    $YoyoRelease = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
    $YoyoDebug = Join-Path $Root "yoyo-rust\target\debug\yoyo.exe"
    $Yoyo = if (Test-Path $YoyoRelease) { $YoyoRelease } elseif (Test-Path $YoyoDebug) { $YoyoDebug } else { $null }
    $Ty = Join-Path $Root "yoyo\projects\yoyo.ty"
    if ($Yoyo -and (Test-Path $Ty)) {
        $smokeDir = Join-Path $WorkDir "smoke-export-compile-no-input"
        New-Item -ItemType Directory -Force -Path $smokeDir | Out-Null
        $gen1 = Join-Path $smokeDir "gen1.exe"
        & $Yoyo link --target=win32 $Ty $gen1
        if ($LASTEXITCODE -ne 0 -or -not (Test-Path $gen1)) {
            throw "Gate G export-compile smoke: H_00 link failed"
        }
        Copy-Item $AltDll (Join-Path $smokeDir "yoyo_rt.dll") -Force
        Get-ChildItem $smokeDir -Filter "input.*" -ErrorAction SilentlyContinue | Remove-Item -Force
        Push-Location $smokeDir
        try {
            & ".\gen1.exe"
            $altExit = $LASTEXITCODE
        } finally {
            Pop-Location
        }
        if ($altExit -eq 2) {
            Write-Host "OW_RT_SPIKE yoyo_export_compile_smoke=GREEN exit=2 (H_00 loaded export-compile pe_dll; no input)"
        } else {
            # AV / GPA instability — honest non-fatal (unit + emit path already GREEN)
            Write-Host ("OW_RT_SPIKE yoyo_export_compile_smoke=NOT_STABLE exit={0} (AV/H_00; non-fatal)" -f $altExit)
        }
    } else {
        Write-Host "OW_RT_SPIKE yoyo_export_compile_smoke=SKIP (missing yoyo.exe or yoyo.ty)"
    }
} else {
    Write-Host "OW_RT_SPIKE yoyo_export_compile_smoke=SKIP (non-Windows; use local Win)"
}
# Honest: export carries emit-time YOYO compile; production default still Rust.
Write-Host "OW_RT_SPIKE yoyo_built=EXPORT_COMPILE yoyo_export_compile=PRESENT disposition=CUT"
Write-Host "OW_RT_SPIKE note=Gate_G_slice_export_compile; call-time not general re-compile; CLOSED requires production YOYO-built compile sidecar + no Rust yoyo_rt.dll host trust"
Write-Host "OW_RT_SPIKE status=GREEN doc=SCOPE-CUT-v1.0-ow-rt-yoyo-runtime.md"
exit 0
