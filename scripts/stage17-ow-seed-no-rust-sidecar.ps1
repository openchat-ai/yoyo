# stage17-ow-seed-no-rust-sidecar.ps1 - OW-SEED Gate I experiment: run pinned
# YOYO emitter with a YOYO-built sidecar (no Rust yoyo_rt.dll in cwd)
# (post-v1.0 path 2)
#
# Honest experiment, expected to be RED today: the in-DLL-recompile pe_dll is
# an oracle-table sidecar, not a full H_00 runtime contract. Machine evidence
# must show the actual exit / mismatch, not an assumption.
#
#   cd F:\yoyo
#   & .\scripts\stage17-ow-seed-no-rust-sidecar.ps1
param(
    [switch]$SkipStage9,
    [switch]$SkipEmitRt
)
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden

Write-Host "=== Post-v1.0: OW-SEED Gate I experiment (no Rust sidecar) ==="

# Gate H: fail-closed trust-root pin
& (Join-Path $Root "scripts\stage17-ow-seed-trust-root.ps1") @PSBoundParameters
if ($LASTEXITCODE -ne 0) {
    Write-Host "OW_SEED_NO_SIDECAR status=RED reason=trust_root_pin_failed"
    exit 1
}

$Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
$Ty = Join-Path $Root "yoyo\projects\yoyo.ty"
$Tyb = Join-Path $Root "yoyo\projects\yoyo.tyb"
$TrustRoot = Join-Path $Root "scripts\_stage9-pure-m4\gen4.exe"
$Base = Join-Path $Root "scripts\_stage17-ow-seed-no-rust-sidecar"
$RustDir = Join-Path $Base "rust"
$YoyoDir = Join-Path $Base "yoyo"
$AltSrc = Join-Path $Root "scripts\_stage17-ow-rt-in-dll-recompile\yoyo_rt.dll"
New-Item -ItemType Directory -Force -Path $RustDir | Out-Null
New-Item -ItemType Directory -Force -Path $YoyoDir | Out-Null

function Get-ShaPrefix([string]$Path, [int]$N = 16) {
    $h = (Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash.ToLowerInvariant()
    return $h.Substring(0, [Math]::Min($N, $h.Length))
}

# --- Rust baseline sidecar (contrast) ---
$RustSidecar = Join-Path $Root "yoyo-rust\target\release-runtime\yoyo_runtime.dll"
if (-not (Test-Path $RustSidecar)) {
    $RustSidecar = Join-Path $Root "yoyo-rust\target\release\yoyo_runtime.dll"
}
if (-not (Test-Path $RustSidecar)) {
    Write-Host "OW_SEED_NO_SIDECAR status=RED reason=missing_rust_runtime_dll"
    exit 1
}

$RustSeed = Join-Path $RustDir "seed_rust.exe"
Copy-Item -Force $RustSidecar (Join-Path $RustDir "yoyo_rt.dll")
Copy-Item -Force $TrustRoot (Join-Path $RustDir "emitter_gen4.exe")
Copy-Item -Force $Tyb (Join-Path $RustDir "input.tyb")
Copy-Item -Force $Ty (Join-Path $RustDir "input.ky")
Push-Location $RustDir
try {
    if (Test-Path "output.exe") { Remove-Item "output.exe" -Force }
    Write-Host "=== contrast: gen4 + RUST sidecar (unattended-ui) ==="
    $ecRust = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $RustDir "emitter_gen4.exe") -WorkingDirectory $RustDir -TimeoutSec 180
    if ($ecRust -eq -1 -or $ecRust -ne 0 -or -not (Test-Path "output.exe")) {
        Write-Host "OW_SEED_NO_SIDECAR status=RED reason=rust_baseline_failed exit=$ecRust"
        exit 1
    }
    Copy-Item -Force "output.exe" $RustSeed
} finally { Pop-Location }

# --- YOYO-built sidecar (no Rust yoyo_rt.dll in cwd) ---
if (-not (Test-Path $AltSrc)) {
    if ($SkipEmitRt) {
        Write-Host "OW_SEED_NO_SIDECAR status=RED reason=missing_yoyo_sidecar_and_SkipEmitRt"
        exit 1
    }
    Push-Location (Join-Path $Root "yoyo-rust")
    try {
        $prevEap = $ErrorActionPreference
        $ErrorActionPreference = "Continue"
        Write-Host "=== build YOYO in-DLL-recompile sidecar ==="
        & cargo run -q -p verifier --bin emit-rt-sidecar --no-default-features --features full-backends -- --in-dll-recompile (Join-Path $Root "scripts\_stage17-ow-rt-in-dll-recompile")
        $ecEmit = $LASTEXITCODE
        $ErrorActionPreference = $prevEap
        if ($ecEmit -ne 0 -or -not (Test-Path $AltSrc)) {
            Write-Host "OW_SEED_NO_SIDECAR status=RED reason=emit_rt_sidecar_failed exit=$ecEmit"
            exit 1
        }
    } finally { Pop-Location }
}

$YoyoSeed = Join-Path $YoyoDir "seed_yoyo.exe"
Copy-Item -Force $AltSrc (Join-Path $YoyoDir "yoyo_rt.dll")
Copy-Item -Force $TrustRoot (Join-Path $YoyoDir "emitter_gen4.exe")
Copy-Item -Force $Tyb (Join-Path $YoyoDir "input.tyb")
Copy-Item -Force $Ty (Join-Path $YoyoDir "input.ky")
# HB-FX: CreateProcess child via JS peer link (no fixture copy, no cargo place-compiler).
Write-Host "=== co-place yoyo_compiler.exe via JS peer link (HB-FX) ==="
$jsPlace = Join-Path $Root "yoyo-js\scripts\place-yoyo-compiler.js"
if (-not (Test-Path $jsPlace)) {
    Write-Host "OW_SEED_NO_SIDECAR status=RED reason=missing_js_place_script"
    exit 1
}
& node $jsPlace $YoyoDir
if ($LASTEXITCODE -ne 0 -or -not (Test-Path (Join-Path $YoyoDir "yoyo_compiler.exe"))) {
    Write-Host ("OW_SEED_NO_SIDECAR status=RED reason=js_peer_place_failed exit={0}" -f $LASTEXITCODE)
    exit 1
}
$compSha = (Get-FileHash (Join-Path $YoyoDir "yoyo_compiler.exe") -Algorithm SHA256).Hash.ToLowerInvariant()
Write-Host ("OW_SEED_NO_SIDECAR yoyo_compiler_bytes={0} sha256_prefix={1} emitter=js_peer_link" -f (Get-Item (Join-Path $YoyoDir "yoyo_compiler.exe")).Length, $compSha.Substring(0,16))
Write-Host "OW_SEED_NO_SIDECAR rust_place_compiler_cargo=ABSENT"
Write-Host "OW_SEED_NO_SIDECAR fixture_copy=ABSENT"
Push-Location $YoyoDir
try {
    if (Test-Path "output.exe") { Remove-Item "output.exe" -Force }
    Write-Host "=== experiment: gen4 + YOYO sidecar + yoyo_compiler (no Rust DLL, unattended-ui) ==="
    try {
        $ecYoyo = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $YoyoDir "emitter_gen4.exe") -WorkingDirectory $YoyoDir -TimeoutSec 180
    } catch {
        Write-Host ("exception: {0}" -f $_.Exception.Message)
        $ecYoyo = -2
    }
} finally { Pop-Location }

$hasOut = Test-Path (Join-Path $YoyoDir "output.exe")
$altBytes = (Get-Item (Join-Path $YoyoDir "yoyo_rt.dll")).Length
$rustBytes = (Get-Item $RustSeed).Length
$rustSha = Get-ShaPrefix $RustSeed
$altSha = ""
$parity = "N/A"
if ($hasOut) {
    Copy-Item -Force (Join-Path $YoyoDir "output.exe") $YoyoSeed
    $altSha = Get-ShaPrefix $YoyoSeed
    if ((Get-FileHash $YoyoSeed -Algorithm SHA256).Hash -eq (Get-FileHash $RustSeed -Algorithm SHA256).Hash) {
        $parity = "EQUAL"
    } else {
        $parity = "DIFF"
    }
}

Write-Host ""
Write-Host ("OW_SEED_NO_SIDECAR contrast_rust_sidecar_exit={0} seed_bytes={1} sha256_prefix={2}" -f $ecRust, $rustBytes, $rustSha)
Write-Host ("OW_SEED_NO_SIDECAR yoyo_sidecar_bytes={0}" -f $altBytes)
Write-Host ("OW_SEED_NO_SIDECAR yoyo_sidecar_exit={0} output_present={1}" -f $ecYoyo, $hasOut)
if ($hasOut) {
    Write-Host ("OW_SEED_NO_SIDECAR seed_yoyo_sha256_prefix={0} parity={1}" -f $altSha, $parity)
}
Write-Host "OW_SEED_NO_SIDECAR rust_yoyo_rt_dll_in_cwd=ABSENT"
Write-Host "OW_SEED_NO_SIDECAR yoyo_compiler_cwd=PRESENT"
Write-Host "OW_SEED_NO_SIDECAR disposition=CLOSED"
Write-Host "OW_SEED_NO_SIDECAR closed=1"
Write-Host "OW_SEED_NO_SIDECAR auth=human_A_2026-11-07"
Write-Host "OW_SEED_NO_SIDECAR note=CreateProcess_js_peer_link;payload_origin_rust_parse1_emit;not_Thompson"
Write-Host "OW_SEED_NO_SIDECAR doc=SCOPE-CUT-v1.0-ow-seed-observe.md"

if ($ecYoyo -ne 0 -or -not $hasOut -or $parity -ne "EQUAL") {
    Write-Host ("OW_SEED_NO_SIDECAR status=RED reason=yoyo_sidecar_cannot_replace_rust_runtime exit={0} output={1} parity={2}" -f $ecYoyo, $hasOut, $parity)
    exit 1
}
Write-Host "OW_SEED_NO_SIDECAR status=GREEN"
exit 0
