# stage-y1plpin.ps1 — AUTO-Y1PLPIN: Y1PL output ≡ place C1 pin; poison caught
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root
. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-Y1PLPIN (Y1PL ≡ place pin) ==="

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$work = Join-Path $Root "scripts\_stage-y1plpin"
if (Test-Path $work) { Remove-Item -Recurse -Force $work }
New-Item -Force -ItemType Directory $work | Out-Null

$placeOut = Join-Path $work "place"
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub
if ($LASTEXITCODE -ne 0) { Write-Host "Y1PLPIN status=RED reason=place"; exit 1 }
$c1 = Join-Path $placeOut "yoyo_compiler.exe"
& node (Join-Path $Root "yoyo-js\scripts\verify-place-c1-pin.js") $c1
if ($LASTEXITCODE -ne 0) { Write-Host "Y1PLPIN status=RED reason=place_pin"; exit 1 }

$yDir = Join-Path $work "y1pl"
New-Item -Force -ItemType Directory $yDir | Out-Null
Copy-Item $c1 (Join-Path $yDir "yoyo_compiler.exe")
Copy-Item (Join-Path $Root "yoyo\projects\yoyo-semcomp-self.tyb") (Join-Path $yDir "input.tyb")
$ec = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $yDir "yoyo_compiler.exe") -WorkingDirectory $yDir -TimeoutSec 120
$yOut = Join-Path $yDir "output.exe"
if ($ec -ne 0 -or -not (Test-Path $yOut)) {
  Write-Host "Y1PLPIN status=RED reason=y1pl_fail exit=$ec"
  exit 1
}
& node (Join-Path $Root "yoyo-js\scripts\verify-place-c1-pin.js") $yOut
if ($LASTEXITCODE -ne 0) { Write-Host "Y1PLPIN status=RED reason=y1pl_ne_place_pin"; exit 1 }
Write-Host "Y1PLPIN y1pl_eq_place_pin=EQUAL"

# Poison tyb: flip a byte → must not still equal place pin
$pDir = Join-Path $work "poison"
New-Item -Force -ItemType Directory $pDir | Out-Null
Copy-Item $c1 (Join-Path $pDir "yoyo_compiler.exe")
$tyb = [IO.File]::ReadAllBytes((Join-Path $Root "yoyo\projects\yoyo-semcomp-self.tyb"))
$tyb[[Math]::Min($tyb.Length - 1, 64)] = $tyb[[Math]::Min($tyb.Length - 1, 64)] -bxor 0xFF
[IO.File]::WriteAllBytes((Join-Path $pDir "input.tyb"), $tyb)
$ecP = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $pDir "yoyo_compiler.exe") -WorkingDirectory $pDir -TimeoutSec 120
$pOut = Join-Path $pDir "output.exe"
if ($ecP -eq 0 -and (Test-Path $pOut)) {
  & node (Join-Path $Root "yoyo-js\scripts\verify-place-c1-pin.js") $pOut
  if ($LASTEXITCODE -eq 0) {
    Write-Host "Y1PLPIN status=RED reason=poison_tyb_still_pin_EQUAL"
    exit 1
  }
  Write-Host "Y1PLPIN poison_tyb_DIFF_pin=RED_OK"
} else {
  Write-Host ("Y1PLPIN poison_tyb_fail_closed=OK exit={0}" -f $ecP)
}

Write-Host "Y1PLPIN note=selfhost_detection;not_Thompson"
Write-Host "Y1PLPIN status=GREEN"
Write-Host "Y1PLPIN doc=AUTO-Y1PLPIN.md"
exit 0
