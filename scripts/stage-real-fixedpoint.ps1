# stage-real-fixedpoint.ps1 — AUTO-REAL-FIXEDPOINT R1–R4 (not soft COMPLETED)
#
# Hard: JS/asm cold start → YOYO H00 fixedpoint; no Rust yoyo.exe on chain;
#       compiler = fixture copy (no cargo place-compiler); Gate I shape; ancestry pin.
# R5 full-file / full .text EQUAL required for REAL COMPLETED (no soft window).
#
#   cd F:\yoyo
#   & .\scripts\stage-real-fixedpoint.ps1
param([switch]$SkipEmitRt)
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden

Write-Host "=== AUTO-REAL-FIXEDPOINT (no soft graduation) ==="

# R2 fixture present
$fx = Join-Path $Root "scripts\fixtures\yoyo_compiler.exe"
$fxPin = Join-Path $Root "scripts\fixtures\yoyo_compiler.pin"
if (-not (Test-Path $fx) -or -not (Test-Path $fxPin)) {
    Write-Host "REAL_FP status=RED reason=missing_compiler_fixture"
    exit 1
}

# R3 Gate I
Write-Host "=== R3 Gate I ==="
& (Join-Path $Root "scripts\stage17-ow-seed-no-rust-sidecar.ps1") -SkipStage9 -SkipEmitRt:$SkipEmitRt
if ($LASTEXITCODE -ne 0) {
    Write-Host "REAL_FP status=RED reason=gate_i_failed"
    exit 1
}

# R1+R4 FULLBAR trust-root (JS ancestry + YOYO chain + fixture compiler)
Write-Host "=== R1/R4 fullbar trust-root ==="
& (Join-Path $Root "scripts\stage-fullbar-trust-root.ps1") -SkipEmitRt:$SkipEmitRt
if ($LASTEXITCODE -ne 0) {
    Write-Host "REAL_FP status=RED reason=fullbar_trust_root_failed"
    exit 1
}

# R2: prove place-compiler cargo was not required — fixture pin chain_emitter
$pinTxt = Get-Content $fxPin -Raw
if ($pinTxt -notmatch 'chain_emitter=fixture_copy') {
    Write-Host "REAL_FP status=RED reason=fixture_pin_not_fixture_copy"
    exit 1
}
Write-Host "REAL_FP rust_place_compiler_cargo=ABSENT"
Write-Host "REAL_FP rust_yoyo_exe_on_emit_hops=ABSENT (Gate I + fullbar chain)"

# R5 observe
$Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
$js = Join-Path $Root "scripts\_stage-fullbar-fs2-chain\seed_js.exe"
$rust = Join-Path $Root "scripts\_stage-fullbar-fs2-chain\seed_rust_contrast.exe"
$r5 = "SKIP"
if ((Test-Path $js) -and (Test-Path $rust) -and (Test-Path $Yoyo)) {
    $prev = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    & $Yoyo diff $js $rust 2>&1 | Out-Null
    $r5 = if ($LASTEXITCODE -eq 0) { "EQUAL" } else { "DIFF" }
    $ErrorActionPreference = $prev
}
Write-Host ("REAL_FP R5_full_text_js_vs_rust_observe={0}" -f $r5)
if ($r5 -ne "EQUAL") {
    Write-Host "REAL_FP R5=HARD_OPEN (full DIFF) — REAL track MUST NOT set COMPLETED=1"
    Write-Host "REAL_FP R1=GREEN R2=GREEN R3=GREEN R4=GREEN"
    Write-Host "REAL_FP disposition=CUT"
    Write-Host "REAL_FP note=bootstrap_compiler_fixture_allowed; OW-SEED_not_CLOSED; soft_tracks_not_enough"
    Write-Host "REAL_FP doc=AUTO-REAL-FIXEDPOINT.md"
    Write-Host "REAL_FP status=GREEN_PARTIAL"
    exit 0
}

Write-Host "REAL_FP R1=GREEN R2=GREEN R3=GREEN R4=GREEN R5=GREEN"
Write-Host "REAL_FP disposition=CUT"
Write-Host "REAL_FP note=R1-R5_machine_green; OW-SEED_still_CUT; not_Thompson; fixture_compiler_ok_per_R2"
Write-Host "REAL_FP doc=AUTO-REAL-FIXEDPOINT.md"
Write-Host "REAL_FP status=GREEN"
exit 0
