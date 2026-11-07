# stage-fixedpoint-three-peer.ps1 — AUTO-FIXEDPOINT FP-5 / D6
#
# D6 DONE window (honest, fail-closed):
#   Same source yoyo/projects/yoyo.ty → JS / Rust / asm each emit a PE;
#   `yoyo diff --selfhost-body` EQUAL on all three pairs;
#   compared_bytes >= 17013 (startup + post-H_00 handlers).
#
# Explicitly NOT required for D6 GREEN:
#   full-file PE EQUAL; full .text EQUAL (may DIFF; observed only).
#
# Implementation: embeds Stage 12-B gate (canonical three-peer body window).
#
#   cd F:\yoyo
#   & .\scripts\stage-fixedpoint-three-peer.ps1
#   & .\scripts\stage-fixedpoint-three-peer.ps1 -SkipBuild
param(
    [switch]$SkipBuild
)
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden

Write-Host "=== AUTO-FIXEDPOINT FP-5 / D6 (three-peer ultimate DONE window) ==="

$Ty = Join-Path $Root "yoyo\projects\yoyo.ty"
if (-not (Test-Path $Ty)) {
    Write-Host "FP5_D6 status=RED reason=missing_yoyo_ty"
    exit 1
}
$srcSha = (Get-FileHash -LiteralPath $Ty -Algorithm SHA256).Hash.ToLowerInvariant()
Write-Host ("FP5_D6 source=yoyo.ty sha256_prefix={0}" -f $srcSha.Substring(0, 16))

$gate12 = Join-Path $Root "scripts\stage12-selfhost-body-section-ddc.ps1"
if (-not (Test-Path $gate12)) {
    Write-Host "FP5_D6 status=RED reason=missing_stage12_body_gate"
    exit 1
}

$log = Join-Path $Root "scripts\_stage-fixedpoint-three-peer\gate12.log"
New-Item -ItemType Directory -Force -Path (Split-Path $log) | Out-Null
$gArgs = @()
if ($SkipBuild) { $gArgs += "-SkipBuild" }

Write-Host "=== embed: stage12-selfhost-body-section-ddc (JS=Rust=asm selfhost-body) ==="
try {
    Start-Transcript -Path $log -Force | Out-Null
    & $gate12 @gArgs
    $ec = $LASTEXITCODE
} finally {
    Stop-Transcript | Out-Null
}
if ($ec -ne 0) {
    Write-Host "FP5_D6 status=RED reason=stage12_body_ddc_failed"
    exit 1
}

$text = Get-Content -LiteralPath $log -Raw
if ($text -notmatch 'Stage 12-B: GREEN' -and $text -notmatch 'three-peer EQUAL') {
    # Stage 12 prints "Stage 12-B: GREEN" (encoding may mangle dash); also accept EQUAL window line.
    if ($text -notmatch 'selfhost body section-ddc' -and $text -notmatch 'three-peer EQUAL window') {
        Write-Host "FP5_D6 status=RED reason=missing_stage12_green_marker"
        exit 1
    }
}

$WorkDir = Join-Path $Root "scripts\_stage12-selfhost-body-section-ddc"
$jsOut = Join-Path $WorkDir "M_js.exe"
$rustOut = Join-Path $WorkDir "M_rust.exe"
$asmOut = Join-Path $WorkDir "M_asm.exe"
foreach ($f in @($jsOut, $rustOut, $asmOut)) {
    if (-not (Test-Path $f)) {
        Write-Host ("FP5_D6 status=RED reason=missing_peer_pe path={0}" -f $f)
        exit 1
    }
}

$Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
# Re-assert three-peer body EQUAL (fail-closed; do not trust transcript alone).
$pairs = @(
    @{ A = "JS"; B = "Rust"; PathA = $jsOut; PathB = $rustOut },
    @{ A = "JS"; B = "Asm"; PathA = $jsOut; PathB = $asmOut },
    @{ A = "Rust"; B = "Asm"; PathA = $rustOut; PathB = $asmOut }
)
$minCompared = [int]::MaxValue
foreach ($p in $pairs) {
    Write-Host ("=== recheck selfhost-body: {0} vs {1} ===" -f $p.A, $p.B)
    $prev = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    $lines = & $Yoyo diff --selfhost-body $p.PathA $p.PathB 2>&1
    $de = $LASTEXITCODE
    $ErrorActionPreference = $prev
    $lines | ForEach-Object { Write-Host "  $_" }
    if ($de -ne 0) {
        Write-Host ("FP5_D6 status=RED reason=body_diff pair={0}-{1}" -f $p.A, $p.B)
        exit 1
    }
    $joined = ($lines | ForEach-Object { "$_" }) -join "`n"
    if ($joined -notmatch 'DDC:\s*EQUAL') {
        Write-Host ("FP5_D6 status=RED reason=body_not_EQUAL pair={0}-{1}" -f $p.A, $p.B)
        exit 1
    }
    if ($joined -match 'compared_bytes:\s*(\d+)') {
        $n = [int]$Matches[1]
        if ($n -lt $minCompared) { $minCompared = $n }
    }
}
if ($minCompared -lt 17013) {
    Write-Host ("FP5_D6 status=RED reason=window_too_small compared={0}" -f $minCompared)
    exit 1
}

# Observe-only full .text (must not fail D6).
$prev = $ErrorActionPreference
$ErrorActionPreference = "Continue"
& $Yoyo diff $jsOut $rustOut 2>&1 | Out-Null
$fullJr = $LASTEXITCODE
$ErrorActionPreference = $prev
$fullObs = if ($fullJr -eq 0) { "EQUAL" } else { "DIFF" }

Write-Host ("FP5_D6 done_window=selfhost-body compared_bytes_min={0}" -f $minCompared)
Write-Host "FP5_D6 peers=JS,Rust,asm source=same_yoyo.ty"
Write-Host "FP5_D6 three_peer_body=EQUAL"
Write-Host ("FP5_D6 full_text_js_vs_rust_observe={0}" -f $fullObs)
Write-Host "FP5_D6 note=full_text_DIFF_ok; D6_is_DONE_window_not_full_PE"
Write-Host "FP5_D6 doc=AUTO-FIXEDPOINT.md"
Write-Host "FP5_D6 status=GREEN"
exit 0
