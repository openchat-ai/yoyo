# stage-dataplant.ps1 — AUTO-DATAPLANT: data plant SoT + poison caught
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== AUTO-DATAPLANT ==="

& node (Join-Path $Root "yoyo-js\scripts\verify-data-plant-sot.js")
if ($LASTEXITCODE -ne 0) { Write-Host "DATAPLANT status=RED reason=baseline"; exit 1 }

$tyb0 = Join-Path $Root "yoyo\projects\yoyo-plant-data0.tyb"
$work = Join-Path $Root "scripts\_stage-dataplant"
New-Item -Force -ItemType Directory $work | Out-Null
$bak = Join-Path $work "data0.bak"
Copy-Item -Force $tyb0 $bak

try {
  $b = [IO.File]::ReadAllBytes($tyb0)
  $ix = [Math]::Min($b.Length - 1, 10)
  $b[$ix] = $b[$ix] -bxor 0xFF
  [IO.File]::WriteAllBytes($tyb0, $b)

  & node (Join-Path $Root "yoyo-js\scripts\verify-data-plant-sot.js")
  if ($LASTEXITCODE -eq 0) {
    Write-Host "DATAPLANT status=RED reason=poison_not_caught"
    exit 1
  }
  Write-Host "DATAPLANT poison_caught=RED_OK"
} finally {
  Copy-Item -Force $bak $tyb0
}

& node (Join-Path $Root "yoyo-js\scripts\verify-data-plant-sot.js")
if ($LASTEXITCODE -ne 0) { Write-Host "DATAPLANT status=RED reason=restore"; exit 1 }

Write-Host "DATAPLANT note=data_sot_detection;not_Thompson"
Write-Host "DATAPLANT status=GREEN"
Write-Host "DATAPLANT doc=AUTO-DATAPLANT.md"
exit 0
