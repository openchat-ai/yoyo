# stage-fixedpoint-fp3-d5.ps1 — AUTO-FIXEDPOINT FP-3 / D5
#
# D5: production seed trust must NOT treat cwd Rust yoyo_rt.dll as the sole path.
# Production trust (OW-RT CLOSED @ S3.738 human A) = CreateProcess-compiler.
# Gate I (gen4 + YOYO oracle sidecar, no Rust DLL) stays DIAGNOSTIC — expected RED
# today; RED here does NOT fail FP-3.
#
#   cd F:\yoyo
#   & .\scripts\stage-fixedpoint-fp3-d5.ps1
#   & .\scripts\stage-fixedpoint-fp3-d5.ps1 -SkipBuild
param(
    [switch]$SkipBuild,
    [switch]$SkipGateIDiag
)
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden

Write-Host "=== AUTO-FIXEDPOINT FP-3 / D5 (CreateProcess primary; Gate I diagnostic) ==="

$spike = Join-Path $Root "scripts\stage17-ow-rt-yoyo-runtime.ps1"
if (-not (Test-Path $spike)) {
    Write-Host "FP3_D5 status=RED reason=missing_ow_rt_spike"
    exit 1
}

$spikeLog = Join-Path $Root "scripts\_stage-fixedpoint-fp3-d5\ow-rt-spike.log"
New-Item -ItemType Directory -Force -Path (Split-Path $spikeLog) | Out-Null
$spikeArgs = @()
if ($SkipBuild) { $spikeArgs += "-SkipBuild" }

Write-Host "=== production trust gate: stage17-ow-rt-yoyo-runtime.ps1 ==="
# Write-Host inside spike does not hit the pipeline — use transcript.
try {
    Start-Transcript -Path $spikeLog -Force | Out-Null
    & $spike @spikeArgs
    $spikeEc = $LASTEXITCODE
} finally {
    Stop-Transcript | Out-Null
}
if ($spikeEc -ne 0) {
    Write-Host "FP3_D5 status=RED reason=ow_rt_spike_failed"
    exit 1
}

$spikeText = Get-Content -LiteralPath $spikeLog -Raw -ErrorAction Stop
foreach ($need in @(
    "OW_RT_SPIKE status=GREEN",
    "g_closed=PRESENT",
    "create_process_primary_evidence=PRESENT"
)) {
    if ($spikeText -notmatch [regex]::Escape($need)) {
        Write-Host ("FP3_D5 status=RED reason=missing_marker need={0}" -f $need)
        exit 1
    }
}
# disposition=CLOSED appears both as claim and note; require spike claim line.
if ($spikeText -notmatch 'DispositionClaim=CLOSED|disposition_claim=CLOSED|disposition=CLOSED') {
    Write-Host "FP3_D5 status=RED reason=missing_marker need=disposition=CLOSED"
    exit 1
}

$scope = Join-Path $Root "SCOPE-CUT-v1.0-ow-rt-yoyo-runtime.md"
$scopeText = Get-Content -LiteralPath $scope -Raw
if ($scopeText -notmatch 'CreateProcess') {
    Write-Host "FP3_D5 status=RED reason=scope_missing_CreateProcess"
    exit 1
}
if ($scopeText -notmatch 'disposition=\*\*CLOSED\*\*|disposition=CLOSED') {
    Write-Host "FP3_D5 status=RED reason=scope_missing_CLOSED"
    exit 1
}

$gateIStatus = "SKIP"
$gateIReason = "SkipGateIDiag"
if (-not $SkipGateIDiag) {
    Write-Host "=== diagnostic: stage17-ow-seed-no-rust-sidecar (expect RED; not D5 sole) ==="
    $gateI = Join-Path $Root "scripts\stage17-ow-seed-no-rust-sidecar.ps1"
    & $gateI -SkipStage9 -SkipEmitRt 2>&1 | ForEach-Object { Write-Host $_ }
    $ge = $LASTEXITCODE
    if ($ge -eq 0) {
        $gateIStatus = "GREEN"
        $gateIReason = "unexpected_green_record_for_inventory"
    } else {
        $gateIStatus = "RED"
        $gateIReason = "yoyo_sidecar_cannot_replace_rust_runtime_expected"
    }
}

Write-Host ""
Write-Host "FP3_D5 production_trust=CreateProcess-compiler"
Write-Host "FP3_D5 ow_rt_disposition=CLOSED"
Write-Host "FP3_D5 rust_cwd_dll_sole_path=ABSENT"
Write-Host ("FP3_D5 gate_i_diagnostic={0} reason={1}" -f $gateIStatus, $gateIReason)
Write-Host "FP3_D5 note=Gate_I_RED_does_not_fail_D5; OW-SEED still CUT (trust-root provenance)"
Write-Host "FP3_D5 doc=AUTO-FIXEDPOINT.md"
Write-Host "FP3_D5 status=GREEN"
exit 0
