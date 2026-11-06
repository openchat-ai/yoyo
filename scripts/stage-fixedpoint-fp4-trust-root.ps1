# stage-fixedpoint-fp4-trust-root.ps1 — AUTO-FIXEDPOINT FP-4 / D4
#
# D4: trust-root provenance must NOT list rust_stage9_gen1_* as the *sole* ancestor.
# Recipe (machine):
#   1) Gate H pin uses dual provenance (ancestry + operational YOYO PE)
#   2) Gate G YOYO emit hops GREEN · rust_yoyo_exe_on_emit_hops=ABSENT
#   3) operational seed_yoyo.exe full-file sha ≡ trust-root gen4.exe ≡ pin
# disposition stays CUT — dual pin ≠ OW-SEED CLOSED.
#
#   cd F:\yoyo
#   & .\scripts\stage-fixedpoint-fp4-trust-root.ps1
#   & .\scripts\stage-fixedpoint-fp4-trust-root.ps1 -SkipStage9
param(
    [switch]$SkipStage9
)
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden

Write-Host "=== AUTO-FIXEDPOINT FP-4 / D4 (dual trust-root provenance) ==="

$trust = Join-Path $Root "scripts\stage17-ow-seed-trust-root.ps1"
$emit = Join-Path $Root "scripts\stage17-ow-seed-yoyo-emit.ps1"
$gen4 = Join-Path $Root "scripts\_stage9-pure-m4\gen4.exe"
$seedYoyo = Join-Path $Root "scripts\_stage17-ow-seed-yoyo-emit\seed_yoyo.exe"

$tArgs = @()
$eArgs = @()
if ($SkipStage9) {
    $tArgs += "-SkipStage9"
    $eArgs += "-SkipStage9"
}

Write-Host "=== Gate H: dual-provenance trust-root pin ==="
& $trust @tArgs
if ($LASTEXITCODE -ne 0) {
    Write-Host "FP4_D4 status=RED reason=trust_root_gate_failed"
    exit 1
}

Write-Host "=== Gate G: YOYO PE emit hops (operational provenance evidence) ==="
$emitLog = Join-Path $Root "scripts\_stage-fixedpoint-fp4\emit.log"
New-Item -ItemType Directory -Force -Path (Split-Path $emitLog) | Out-Null
try {
    Start-Transcript -Path $emitLog -Force | Out-Null
    & $emit @eArgs
    $emitEc = $LASTEXITCODE
} finally {
    Stop-Transcript | Out-Null
}
if ($emitEc -ne 0) {
    Write-Host "FP4_D4 status=RED reason=yoyo_emit_failed"
    exit 1
}
$emitText = Get-Content -LiteralPath $emitLog -Raw
if ($emitText -notmatch 'rust_yoyo_exe_on_emit_hops=ABSENT') {
    Write-Host "FP4_D4 status=RED reason=rust_exe_still_on_emit_hops"
    exit 1
}
if ($emitText -notmatch 'OW_SEED_EMIT status=GREEN') {
    Write-Host "FP4_D4 status=RED reason=emit_not_green"
    exit 1
}

if (-not (Test-Path $gen4) -or -not (Test-Path $seedYoyo)) {
    Write-Host "FP4_D4 status=RED reason=missing_gen4_or_seed_yoyo"
    exit 1
}

$shaGen4 = (Get-FileHash -LiteralPath $gen4 -Algorithm SHA256).Hash.ToLowerInvariant()
$shaSeed = (Get-FileHash -LiteralPath $seedYoyo -Algorithm SHA256).Hash.ToLowerInvariant()
if ($shaGen4 -ne $shaSeed) {
    Write-Host ("FP4_D4 status=RED reason=operational_seed_ne_trust_root gen4={0} seed={1}" -f $shaGen4.Substring(0,16), $shaSeed.Substring(0,16))
    exit 1
}

Write-Host ("FP4_D4 trust_root_sha256_prefix={0}" -f $shaGen4.Substring(0,16))
Write-Host "FP4_D4 provenance=dual (ancestry=rust_stage9_gen1_then_H00_chain; operational=yoyo_pe_emit)"
Write-Host "FP4_D4 sole_rust_ancestry=ABSENT"
Write-Host "FP4_D4 rust_yoyo_exe_on_emit_hops=ABSENT"
Write-Host "FP4_D4 operational_seed_equiv_trust_root=EQUAL"
Write-Host "FP4_D4 disposition=CUT"
Write-Host "FP4_D4 note=NOT_OW_SEED_CLOSED; ancestry_still_records_Rust_gen1"
Write-Host "FP4_D4 doc=AUTO-FIXEDPOINT.md"
Write-Host "FP4_D4 status=GREEN"
exit 0
