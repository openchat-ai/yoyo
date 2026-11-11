# stage-placepin.ps1 — AUTO-PLACEPIN: stub×bins C1 sha pinned; poison caught
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root
. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-PLACEPIN (place C1 sha pin) ==="

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "PLACEPIN status=RED reason=stub_pin"; exit 1 }

$work = Join-Path $Root "scripts\_stage-placepin"
if (Test-Path $work) { Remove-Item -Recurse -Force $work }
New-Item -Force -ItemType Directory $work | Out-Null

$placeOut = Join-Path $work "place"
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub
if ($LASTEXITCODE -ne 0) { Write-Host "PLACEPIN status=RED reason=place"; exit 1 }
$c1 = Join-Path $placeOut "yoyo_compiler.exe"

& node (Join-Path $Root "yoyo-js\scripts\verify-place-c1-pin.js") $c1
if ($LASTEXITCODE -ne 0) { Write-Host "PLACEPIN status=RED reason=c1_pin"; exit 1 }

# Poison must be caught
$poison = Join-Path $work "poison_c1.exe"
Copy-Item -Force $c1 $poison
$b = [IO.File]::ReadAllBytes($poison)
$b[0x200] = $b[0x200] -bxor 0xFF
[IO.File]::WriteAllBytes($poison, $b)
& node (Join-Path $Root "yoyo-js\scripts\verify-place-c1-pin.js") $poison
if ($LASTEXITCODE -eq 0) {
  Write-Host "PLACEPIN status=RED reason=poison_not_caught"
  exit 1
}
Write-Host "PLACEPIN poison_caught=RED_OK"

Write-Host "PLACEPIN note=place_product_detection;not_Thompson"
Write-Host "PLACEPIN status=GREEN"
Write-Host "PLACEPIN doc=AUTO-PLACEPIN.md"
exit 0
