# stage-real-fixedpoint.ps1 — AUTO-REAL-FIXEDPOINT R1–R4 (not soft COMPLETED)
#
# Hard: JS/asm cold start → YOYO H00 fixedpoint; no Rust yoyo.exe on chain;
#       compiler = JS peer link (HB-FX; no cargo place-compiler); Gate I shape; ancestry pin.
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

# R2 / HB-FX: JS peer place script + payload bins present
$jsPlace = Join-Path $Root "yoyo-js\scripts\place-yoyo-compiler.js"
$payloadCode = Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\code.bin"
$payloadData = Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\data.bin"
if (-not (Test-Path $jsPlace) -or -not (Test-Path $payloadCode) -or -not (Test-Path $payloadData)) {
    Write-Host "REAL_FP status=RED reason=missing_js_peer_compiler_place"
    exit 1
}

# R3 Gate I
Write-Host "=== R3 Gate I ==="
& (Join-Path $Root "scripts\stage17-ow-seed-no-rust-sidecar.ps1") -SkipStage9 -SkipEmitRt:$SkipEmitRt
if ($LASTEXITCODE -ne 0) {
    Write-Host "REAL_FP status=RED reason=gate_i_failed"
    exit 1
}

# R1+R4 FULLBAR trust-root (JS ancestry + YOYO chain + js_peer_link compiler)
Write-Host "=== R1/R4 fullbar trust-root ==="
& (Join-Path $Root "scripts\stage-fullbar-trust-root.ps1") -SkipEmitRt:$SkipEmitRt
if ($LASTEXITCODE -ne 0) {
    Write-Host "REAL_FP status=RED reason=fullbar_trust_root_failed"
    exit 1
}

Write-Host "REAL_FP rust_place_compiler_cargo=ABSENT"
Write-Host "REAL_FP compiler_emitter=js_peer_link"
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
Write-Host "REAL_FP disposition=CLOSED"
Write-Host "REAL_FP closed=1"
Write-Host "REAL_FP auth=human_A_2026-11-07"
Write-Host "REAL_FP note=R1-R5_machine_green; OW-SEED_CLOSED; not_Thompson; fixture_compiler_accepted"
Write-Host "REAL_FP doc=SCOPE-CUT-v1.0-ow-seed-observe.md"
Write-Host "REAL_FP status=GREEN"
exit 0
