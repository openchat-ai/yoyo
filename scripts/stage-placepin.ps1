# stage-placepin.ps1 — AUTO-PLACEPIN: stub×bins C1 sha pinned; poison caught
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root
. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-PLACEPIN (place C1 sha pin) ==="

. (Join-Path $Root "scripts\lib\Assert-YoyoFilePin.ps1")
$Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
if (-not (Test-Path -LiteralPath $Yoyo)) { throw "PLACEPIN status=RED reason=missing_yoyo" }

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
try {
  Assert-YoyoShaPin -Yoyo $Yoyo -File $stub `
    -PinPath (Join-Path $Root "scripts\fixtures\yoyo_place_stub.pin") `
    -Label PLACE_STUB -Doc AUTO-TRUER4.md
} catch {
  Write-Host "PLACEPIN status=RED reason=stub_pin"; exit 1
}

$work = Join-Path $Root "scripts\_stage-placepin"
if (Test-Path $work) { Remove-Item -Recurse -Force $work }
New-Item -Force -ItemType Directory $work | Out-Null

$placeOut = Join-Path $work "place"
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub
if ($LASTEXITCODE -ne 0) { Write-Host "PLACEPIN status=RED reason=place"; exit 1 }
$c1 = Join-Path $placeOut "yoyo_compiler.exe"

try {
  Assert-YoyoShaPin -Yoyo $Yoyo -File $c1 `
    -PinPath (Join-Path $Root "scripts\fixtures\yoyo_place_c1.pin") `
    -Label PLACE_C1 -Doc AUTO-PLACEPIN.md
} catch {
  Write-Host "PLACEPIN status=RED reason=c1_pin"; exit 1
}

# Poison must be caught
$poison = Join-Path $work "poison_c1.exe"
Copy-Item -Force $c1 $poison
$b = [IO.File]::ReadAllBytes($poison)
$b[0x200] = $b[0x200] -bxor 0xFF
[IO.File]::WriteAllBytes($poison, $b)
$poisonCaught = $false
try {
  Assert-YoyoShaPin -Yoyo $Yoyo -File $poison `
    -PinPath (Join-Path $Root "scripts\fixtures\yoyo_place_c1.pin") `
    -Label PLACE_C1 -Doc AUTO-PLACEPIN.md
} catch {
  $poisonCaught = $true
}
if (-not $poisonCaught) {
  Write-Host "PLACEPIN status=RED reason=poison_not_caught"
  exit 1
}
Write-Host "PLACEPIN poison_caught=RED_OK"

Write-Host "PLACEPIN note=place_product_detection;yoyo_hash_pin;node_ABSENT;not_Thompson"
Write-Host "PLACEPIN status=GREEN"
Write-Host "PLACEPIN doc=AUTO-LESSHOST.md"
exit 0
