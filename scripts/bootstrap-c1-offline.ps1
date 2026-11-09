# bootstrap-c1-offline.ps1 — OFFLINE only (not a gate).
# Copies pin-locked scripts/fixtures/yoyo_compiler.exe → _stage-c1-native.
# AUTO-NO-NODE-BOOT: MUST NOT call Node place-yoyo-compiler.js.
#
#   cd F:\yoyo
#   & .\scripts\bootstrap-c1-offline.ps1
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

$out = Join-Path $Root "scripts\_stage-c1-native"
$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$place = Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1"
New-Item -ItemType Directory -Force -Path $out | Out-Null

if (-not (Test-Path $stub)) {
    Write-Host "BOOTSTRAP_C1 status=RED reason=missing_place_stub"
    Write-Host "BOOTSTRAP_C1 hint=node yoyo-js/scripts/build-place-stub.js"
    exit 1
}

Write-Host "=== OFFLINE stub→place full C1 (no Node plant) ==="
& $place -OutDir $out -Bootstrap $stub
if ($LASTEXITCODE -ne 0 -or -not (Test-Path (Join-Path $out "yoyo_compiler.exe"))) {
    Write-Host "BOOTSTRAP_C1 status=RED reason=stub_place_failed"
    exit 1
}
$pe = Join-Path $out "yoyo_compiler.exe"
$sha = (Get-FileHash $pe -Algorithm SHA256).Hash.ToLowerInvariant()
$bytes = (Get-Item $pe).Length
Write-Host ("BOOTSTRAP_C1 out={0} sha256_prefix={1} bytes={2}" -f $pe, $sha.Substring(0, 16), $bytes)
Write-Host "BOOTSTRAP_C1 emitter=place_stub"
Write-Host "BOOTSTRAP_C1 node_place=ABSENT"
Write-Host "BOOTSTRAP_C1 node_plant=ABSENT"
Write-Host "BOOTSTRAP_C1 doc=AUTO-TRUER4.md"
Write-Host "BOOTSTRAP_C1 status=GREEN"
exit 0
