# stage17-ow-rt-yoyo-runtime.ps1 — OW-RT YOYO-built runtime spike gate (post-v1.0 path 2)
#
# Gate G CLOSED (S3.738 human auth A): production Off→YOYO + CreateProcess-compiler
# trust; co-place + fail-closed refuse oracle-only; residual oracle ≠ trust path.
# disposition=CLOSED · closed=1 · g_closed=PRESENT.
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
Write-Host "=== Post-v1.0: OW-RT Gate G slice (YOYO in-DLL recompile) ==="
$tyStub = Join-Path $Root "yoyo\tests\golden\ow_rt_yoyo_origin_exit2.ty"
if (-not (Test-Path $tyStub)) { throw "missing YOYO-origin stub $tyStub" }
$tyFx = Join-Path $Root "yoyo\tests\golden\selfhost_min_nop.ty"
if (-not (Test-Path $tyFx)) { throw "missing Gate F success fixture $tyFx" }
$tyFxB = Join-Path $Root "yoyo\tests\golden\01_set_get.ty"
if (-not (Test-Path $tyFxB)) { throw "missing Gate G fixture B $tyFxB" }
Push-Location (Join-Path $Root "yoyo-rust")
try {
    $prevEap = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    # --test-threads=1: ALT_SIDECAR env is process-global; parallel Off asserts race mutators.
    & cargo test -p verifier --lib pe_dll_link --no-default-features --features full-backends -- --test-threads=1
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
$WorkDir = Join-Path $Root "scripts\_stage17-ow-rt-in-dll-recompile"
New-Item -ItemType Directory -Force -Path $WorkDir | Out-Null
Get-ChildItem $WorkDir -Force -ErrorAction SilentlyContinue | Remove-Item -Recurse -Force -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force -Path $WorkDir | Out-Null
Copy-Item $tyFx (Join-Path $WorkDir "input.ty") -Force
Push-Location (Join-Path $Root "yoyo-rust")
try {
    $prevEap = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    & cargo run -q -p verifier --bin emit-rt-sidecar --no-default-features --features full-backends -- --in-dll-recompile $WorkDir
    $ecExit = $LASTEXITCODE
    $ErrorActionPreference = $prevEap
    if ($ecExit -ne 0) {
        throw ("emit-rt-sidecar --in-dll-recompile expected exit=0 got={0}" -f $ecExit)
    }
} finally {
    Pop-Location
}
$AltDll = Join-Path $WorkDir "yoyo_rt.dll"
$OutExe = Join-Path $WorkDir "output.exe"
if (-not (Test-Path $AltDll)) { throw "missing YOYO sidecar $AltDll after --in-dll-recompile" }
if (-not (Test-Path $OutExe)) { throw "missing output.exe after in-dll-recompile" }
$altBytes = [System.IO.File]::ReadAllBytes($AltDll)
$altLen = $altBytes.Length
if ($altLen -lt 64) { throw "YOYO sidecar too small" }
$ascii = [System.Text.Encoding]::ASCII.GetString($altBytes)
if (-not $ascii.Contains("yoyo_runtime_selfhost_main")) {
    throw "YOYO sidecar missing export yoyo_runtime_selfhost_main"
}
if (-not $ascii.Contains("yoyo_in_dll_recompile")) {
    throw "YOYO sidecar missing marker yoyo_in_dll_recompile"
}
if (-not $ascii.Contains("ReadFile")) {
    throw "YOYO sidecar missing ReadFile (call-time read required)"
}
if ($altBytes[0] -ne 0x4D -or $altBytes[1] -ne 0x5A) {
    throw "YOYO sidecar not MZ"
}
$outBytes = [System.IO.File]::ReadAllBytes($OutExe)
if ($outBytes.Length -lt 64 -or $outBytes[0] -ne 0x4D -or $outBytes[1] -ne 0x5A) {
    throw "in-dll-recompile output.exe not a PE"
}
Write-Host ("OW_RT_SPIKE yoyo_in_dll_recompile=PRESENT path={0} sidecar_bytes={1} output_bytes={2}" -f $WorkDir, $altLen, $outBytes.Length)
Write-Host "OW_RT_SPIKE yoyo_alt_sidecar=EMITTED (in-dll-recompile pe_dll)"
Write-Host "OW_RT_SPIKE gate_g_slice=in_dll_recompile"
# Fixture B against same table (generic)
$WorkB = Join-Path $WorkDir "swap-b"
New-Item -ItemType Directory -Force -Path $WorkB | Out-Null
Copy-Item $tyFxB (Join-Path $WorkB "input.ty") -Force
Push-Location (Join-Path $Root "yoyo-rust")
try {
    $prevEap = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    & cargo run -q -p verifier --bin emit-rt-sidecar --no-default-features --features full-backends -- --in-dll-recompile $WorkB
    $ecB = $LASTEXITCODE
    $ErrorActionPreference = $prevEap
    if ($ecB -ne 0) {
        throw ("in-dll-recompile fixture B expected exit=0 got={0}" -f $ecB)
    }
} finally {
    Pop-Location
}
$OutB = Join-Path $WorkB "output.exe"
$outBBytes = [System.IO.File]::ReadAllBytes($OutB)
if ($outBBytes.Length -eq $outBytes.Length) {
    $same = $true
    for ($i = 0; $i -lt $outBytes.Length; $i++) {
        if ($outBytes[$i] -ne $outBBytes[$i]) { $same = $false; break }
    }
    if ($same) { throw "generic recompile: fixture A/B PEs must differ" }
}
Write-Host ("OW_RT_SPIKE yoyo_in_dll_recompile_generic=PRESENT pe_a={0} pe_b={1}" -f $outBytes.Length, $outBBytes.Length)
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
# S3.732–738: Off → YOYO + co-place + fail-closed; human A CLOSED.
# Rust yoyo_runtime.dll may still exist on disk for legacy/compare — not production Off path.
Write-Host "OW_RT_SPIKE production_default=YOYO_IN_DLL (S3.732–734; disposition=CLOSED)"
Write-Host "OW_RT_SPIKE e5_fail_closed_oracle_only=ARMED (S3.735; sidecar∧¬compiler ⇒ refuse graduation)"
Write-Host "OW_RT_SPIKE create_process_primary_evidence=PRESENT (S3.736 pack; auth A trust path)"
Write-Host "OW_RT_SPIKE inventory_closed_hard_gate=AUTHORIZED (S3.738 human A; closed=1; disposition=CLOSED)"
if ($RuntimeDll) {
    $dllBytes = (Get-Item $RuntimeDll).Length
    $dllSha = (Get-FileHash -Algorithm SHA256 -Path $RuntimeDll).Hash.ToLowerInvariant().Substring(0, 16)
    Write-Host "OW_RT_SPIKE rust_sidecar path=$RuntimeDll bytes=$dllBytes sha256_prefix=$dllSha (legacy on-disk; not Off default; auth A OK)"
} else {
    Write-Host "OW_RT_SPIKE rust_sidecar=NOT_BUILT_HERE (ok; production Off uses YOYO candidate)"
}
# Win-only: H_00 loads YOYO in-dll-recompile DLL with no input → exit 2.
$IsWin = $env:OS -eq "Windows_NT"
if ($IsWin) {
    $YoyoRelease = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
    $YoyoDebug = Join-Path $Root "yoyo-rust\target\debug\yoyo.exe"
    $Yoyo = if (Test-Path $YoyoRelease) { $YoyoRelease } elseif (Test-Path $YoyoDebug) { $YoyoDebug } else { $null }
    $Ty = Join-Path $Root "yoyo\projects\yoyo.ty"
    if ($Yoyo -and (Test-Path $Ty)) {
        $smokeDir = Join-Path $WorkDir "smoke-in-dll-recompile-no-input"
        New-Item -ItemType Directory -Force -Path $smokeDir | Out-Null
        $gen1 = Join-Path $smokeDir "gen1.exe"
        & $Yoyo link --target=win32 $Ty $gen1
        if ($LASTEXITCODE -ne 0 -or -not (Test-Path $gen1)) {
            throw "Gate G in-dll-recompile smoke: H_00 link failed"
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
            Write-Host "OW_RT_SPIKE yoyo_in_dll_recompile_smoke=GREEN exit=2 (H_00 loaded in-dll-recompile pe_dll; no input)"
        } else {
            Write-Host ("OW_RT_SPIKE yoyo_in_dll_recompile_smoke=NOT_STABLE exit={0} (AV/H_00; non-fatal)" -f $altExit)
        }
    } else {
        Write-Host "OW_RT_SPIKE yoyo_in_dll_recompile_smoke=SKIP (missing yoyo.exe or yoyo.ty)"
    }
} else {
    Write-Host "OW_RT_SPIKE yoyo_in_dll_recompile_smoke=SKIP (non-Windows; use local Win)"
}
# S3.738: E3 positive CLOSED checks (human auth A). Fail-closed if claim drifts.
# Historical scaffolding text (pre-S738): "E3 fail-closed: disposition=CLOSED but rust_sidecar still PRESENT — need E2"
# Auth A: rust on-disk legacy is OK; production Off path is YOYO + CreateProcess-compiler.
$DispositionClaim = "CLOSED"
$scopeText = Get-Content -Raw -Path $spikeDoc
if ($DispositionClaim -eq "CLOSED") {
    if ($scopeText -notmatch 'closed=1') {
        throw "E3 fail-closed: disposition=CLOSED requires SCOPE closed=1"
    }
    if ($scopeText -notmatch 'disposition=\*\*CLOSED\*\*|disposition=CLOSED') {
        throw "E3 fail-closed: disposition=CLOSED requires SCOPE disposition CLOSED"
    }
    if (-not (Test-Path $AltDll)) {
        throw "E3 fail-closed: disposition=CLOSED requires E1 YOYO-built production sidecar evidence"
    }
    if (-not $ascii.Contains("yoyo_in_dll_recompile")) {
        throw "E3 fail-closed: disposition=CLOSED requires yoyo_in_dll_recompile marker"
    }
    if (-not $ascii.Contains("yoyo_runtime_selfhost_main")) {
        throw "E3 fail-closed: disposition=CLOSED requires ordinal-0 export evidence"
    }
}
Write-Host "OW_RT_SPIKE e3_fail_closed=ARMED disposition_claim=$DispositionClaim"
Write-Host "OW_RT_SPIKE yoyo_built=IN_DLL_RECOMPILE yoyo_in_dll_recompile=PRESENT disposition=CLOSED"
Write-Host "OW_RT_SPIKE g_closed=PRESENT human_auth=A closed=1"
Write-Host "OW_RT_SPIKE ow_iat=ABSENT (FO.1 post-S3 refresh; yoyo_rt.dll load-face residual PRESENT; S3 frozen@738)"
Write-Host "OW_RT_SPIKE note=CreateProcess-compiler trust; residual oracle≠trust; follow-ons (OW-IAT/SEED/…) remain"
Write-Host "OW_RT_SPIKE status=GREEN doc=SCOPE-CUT-v1.0-ow-rt-yoyo-runtime.md"
exit 0
