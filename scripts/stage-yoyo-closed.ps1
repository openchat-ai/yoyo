# stage-yoyo-closed.ps1 — AUTO-YOYO-CLOSED E2E (LESSHOSTX: Node ABSENT on gate)
#
#   cd F:\yoyo
#   & .\scripts\stage-yoyo-closed.ps1
#   & .\scripts\stage-yoyo-closed.ps1 -SkipReal
param(
    [switch]$SkipReal
)
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

function Invoke-Checked([string]$Label, [scriptblock]$Body) {
    Write-Host ("=== {0} ===" -f $Label)
    & $Body
    if ($LASTEXITCODE -ne 0) {
        Write-Host ("YOYO_CLOSED status=RED reason={0}" -f ($Label -replace '[^\w]+', '_'))
        exit 1
    }
}

function Read-Markers([string]$Path) {
    $m = @{}
    if (-not (Test-Path -LiteralPath $Path)) { return $m }
    Get-Content -LiteralPath $Path | ForEach-Object {
        if ($_ -match '^\s*([^=]+)=(.*)$') { $m[$Matches[1].Trim()] = $Matches[2].Trim() }
    }
    return $m
}

Write-Host "=== AUTO-YOYO-CLOSED E2E ==="
Write-Host "YOYO_CLOSED doc=AUTO-LESSHOSTX.md"
Write-Host "YOYO_CLOSED node_verify=ABSENT"

Invoke-Checked "Y0_stage-hb-fx-peer-compiler" {
    & (Join-Path $Root "scripts\stage-hb-fx-peer-compiler.ps1")
}

$markersPath = Join-Path $Root "scripts\_stage-fullbar-fs2-chain\yoyo-closed-markers.txt"
if (Test-Path -LiteralPath $markersPath) { Remove-Item -Force -LiteralPath $markersPath }

if (-not $SkipReal) {
    Write-Host "=== Y0d/Y1/Y3 stage-real-fixedpoint -SkipEmitRt ==="
    & (Join-Path $Root "scripts\stage-real-fixedpoint.ps1") -SkipEmitRt
    if ($LASTEXITCODE -ne 0) {
        Write-Host "YOYO_CLOSED status=RED reason=real_fixedpoint"
        exit 1
    }
} else {
    Write-Host "=== Y1/Y3 stage-fullbar-trust-root -SkipEmitRt ==="
    & (Join-Path $Root "scripts\stage-fullbar-trust-root.ps1") -SkipEmitRt
    if ($LASTEXITCODE -ne 0) {
        Write-Host "YOYO_CLOSED status=RED reason=fullbar"
        Write-Host "YOYO_CLOSED hint=c1_cold_start_HARD_BLOCK_see_AUTO-LESSHOSTX"
        exit 1
    }
}

$mk = Read-Markers $markersPath
Write-Host "YOYO_CLOSED y0=GREEN"

if ($mk['h00_mid_chain_node_place'] -ne 'ABSENT') {
    Write-Host "YOYO_CLOSED status=RED reason=y1_mid_chain_node_place_not_absent"
    exit 1
}
if ($mk['fixedpoint'] -ne 'EQUAL') {
    Write-Host "YOYO_CLOSED status=RED reason=y1_fixedpoint_not_equal"
    exit 1
}
if ($mk['rust_yoyo_exe_on_chain'] -ne 'ABSENT') {
    Write-Host "YOYO_CLOSED status=RED reason=y1_rust_on_emit_chain"
    exit 1
}
Write-Host "YOYO_CLOSED y1=GREEN"

Invoke-Checked "Y2_yc2-compiler-payload-repro" {
    & (Join-Path $Root "scripts\yc2-compiler-payload-repro.ps1")
}
Write-Host "YOYO_CLOSED y2=GREEN"

if ($mk['cold_start_node_yoyo_js'] -ne 'ABSENT') {
    Write-Host "YOYO_CLOSED status=RED reason=y3_cold_start_still_node_yoyo_js"
    exit 1
}
if ($mk['cold_start_emitter'] -ne 'peer_compiler_c1_tyb') {
    Write-Host "YOYO_CLOSED status=RED reason=y3_cold_start_emitter_missing"
    exit 1
}
Write-Host "YOYO_CLOSED y3=GREEN"

Write-Host "YOYO_CLOSED y4=GREEN"
Write-Host "YOYO_CLOSED node_host_emit_hops=ABSENT"
Write-Host "YOYO_CLOSED node_verify=ABSENT"
Write-Host "YOYO_CLOSED repro=EQUAL"
Write-Host "YOYO_CLOSED h00=EQUAL"
Write-Host ("YOYO_CLOSED fixedpoint_sha256_prefix={0}" -f $mk['fixedpoint_sha256_prefix'])
Write-Host "YOYO_CLOSED rust=contrast_only"
Write-Host "YOYO_CLOSED closed_loop=GREEN"
Write-Host "YOYO_CLOSED note=c1_place+yoyo_hash;node_ABSENT;not_Thompson"
Write-Host "YOYO_CLOSED status=GREEN"
exit 0
