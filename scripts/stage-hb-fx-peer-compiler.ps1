# stage-hb-fx-peer-compiler.ps1 — AUTO-HB-FX gate
#
# Peer place yoyo_compiler.exe via JS linkPeWin32 (no fixture copy, no cargo place).
#
#   cd F:\yoyo
#   & .\scripts\stage-hb-fx-peer-compiler.ps1
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== AUTO-HB-FX peer compiler place ==="

$jsPlace = Join-Path $Root "yoyo-js\scripts\place-yoyo-compiler.js"
$fx = Join-Path $Root "scripts\fixtures\yoyo_compiler.exe"
$work = Join-Path $Root "scripts\_stage-hb-fx-peer-compiler"
New-Item -ItemType Directory -Force -Path $work | Out-Null
$peerOut = Join-Path $work "yoyo_compiler.exe"
if (Test-Path $peerOut) { Remove-Item -Force $peerOut }

if (-not (Test-Path $jsPlace)) {
    Write-Host "HB_FX status=RED reason=missing_place_script"
    exit 1
}

Write-Host "=== JS peer place ==="
& node $jsPlace $work
if ($LASTEXITCODE -ne 0 -or -not (Test-Path $peerOut)) {
    Write-Host ("HB_FX status=ABSENT reason=js_place_failed exit={0}" -f $LASTEXITCODE)
    exit 2
}

$peerSha = (Get-FileHash $peerOut -Algorithm SHA256).Hash.ToLowerInvariant()
Write-Host ("HB_FX peer_sha256_prefix={0} bytes={1}" -f $peerSha.Substring(0, 16), (Get-Item $peerOut).Length)
Write-Host "HB_FX chain_emitter=js_peer_link"
Write-Host "HB_FX rust_place_compiler_on_this_gate=ABSENT"
Write-Host "HB_FX fixture_copy_on_this_gate=ABSENT"

if (Test-Path $fx) {
    $fxSha = (Get-FileHash $fx -Algorithm SHA256).Hash.ToLowerInvariant()
    $eq = if ($peerSha -eq $fxSha) { "EQUAL" } else { "DIFF" }
    Write-Host ("HB_FX vs_fixture_bootstrap={0}" -f $eq)
}

Write-Host "=== FX-2b data full (H00+field-match) verify ==="
& node (Join-Path $Root "yoyo-js\scripts\verify-compiler-data-prelude.js") --with-field-match
if ($LASTEXITCODE -ne 0) {
    Write-Host "HB_FX status=RED reason=data_bin_mismatch"
    exit 1
}

Write-Host "=== FX-2b-4 code.bin JS emit verify ==="
& node (Join-Path $Root "yoyo-js\scripts\verify-compiler-code.js")
if ($LASTEXITCODE -ne 0) {
    Write-Host "HB_FX status=RED reason=code_bin_mismatch"
    exit 1
}

Write-Host "HB_FX payload_origin=js_parse1_source_interp"
Write-Host "HB_FX note=JS_links_payload; data+code_algorithmic_GREEN"
Write-Host "HB_FX doc=AUTO-HB-FX.md"
Write-Host "HB_FX status=GREEN"
exit 0
