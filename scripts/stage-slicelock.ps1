# stage-slicelock.ps1 — TRUSTLIM4: hdr+IAT windows pinned; poison caught
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root
. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-TRUSTLIM4 SLICELOCK ==="

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$stubPin = Join-Path $Root "scripts\fixtures\yoyo_stub_slices.pin"
$c1Pin = Join-Path $Root "scripts\fixtures\yoyo_c1_slices.pin"
$verify = Join-Path $Root "yoyo-js\scripts\verify-pe-slices-pin.js"

& node $verify $stub $stubPin
if ($LASTEXITCODE -ne 0) { Write-Host "SLICELOCK status=RED reason=stub_slices"; exit 1 }

$work = Join-Path $Root "scripts\_stage-slicelock"
if (Test-Path $work) { Remove-Item -Recurse -Force $work }
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $work -Bootstrap $stub
if ($LASTEXITCODE -ne 0) { Write-Host "SLICELOCK status=RED reason=place"; exit 1 }
$c1 = Join-Path $work "yoyo_compiler.exe"
& node (Join-Path $Root "yoyo-js\scripts\verify-place-c1-pin.js") $c1
if ($LASTEXITCODE -ne 0) { Write-Host "SLICELOCK status=RED reason=c1_full_pin"; exit 1 }
& node $verify $c1 $c1Pin
if ($LASTEXITCODE -ne 0) { Write-Host "SLICELOCK status=RED reason=c1_slices"; exit 1 }

# Poison hdr window on stub copy
$pStub = Join-Path $work "poison_stub.exe"
Copy-Item -Force $stub $pStub
$sb = [IO.File]::ReadAllBytes($pStub)
$sb[0x40] = $sb[0x40] -bxor 0xFF
[IO.File]::WriteAllBytes($pStub, $sb)
& node $verify $pStub $stubPin
if ($LASTEXITCODE -eq 0) { Write-Host "SLICELOCK status=RED reason=stub_hdr_poison_missed"; exit 1 }
Write-Host "SLICELOCK stub_hdr_poison=RED_OK"

# Poison IAT window on C1 (.data raw from pin)
$c1PinMap = @{}
Get-Content $c1Pin | ForEach-Object {
  if ($_ -match '^([^=]+)=(.*)$') { $c1PinMap[$Matches[1]] = $Matches[2] }
}
$dataRaw = [int]$c1PinMap['data_raw']
$pC1 = Join-Path $work "poison_c1.exe"
Copy-Item -Force $c1 $pC1
$cb = [IO.File]::ReadAllBytes($pC1)
$cb[$dataRaw + 8] = $cb[$dataRaw + 8] -bxor 0xFF
[IO.File]::WriteAllBytes($pC1, $cb)
& node $verify $pC1 $c1Pin
if ($LASTEXITCODE -eq 0) { Write-Host "SLICELOCK status=RED reason=c1_iat_poison_missed"; exit 1 }
Write-Host "SLICELOCK c1_iat_poison=RED_OK"

Write-Host "SLICELOCK note=slice_detection_dense;not_Thompson"
Write-Host "SLICELOCK status=GREEN"
Write-Host "SLICELOCK doc=AUTO-TRUSTLIM4.md"
exit 0
