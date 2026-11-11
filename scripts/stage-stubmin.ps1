# stage-stubmin.ps1 — AUTO-STUBMIN: place-half length is minimal
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root
. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-STUBMIN (min code_place_bytes) ==="

$pinPath = Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\payload.pin"
$off = 0
foreach ($line in Get-Content $pinPath) {
  if ($line -match '^code_place_bytes=(\d+)') { $off = [int]$Matches[1] }
}
if ($off -ne 5876) {
  Write-Host "STUBMIN status=RED reason=code_place_bytes_not_5876 got=$off"
  exit 1
}
Write-Host "STUBMIN code_place_bytes=5876"

$work = Join-Path $Root "scripts\_stage-stubmin"
if (Test-Path $work) { Remove-Item -Recurse -Force $work }
New-Item -Force -ItemType Directory $work | Out-Null

# Build stubs at 5876 and 5875 via node
& node -e @"
const fs=require('fs');const {linkPeWin32}=require('./yoyo-js/src/platform/pe-builder');
const code=fs.readFileSync('yoyo-js/src/platform/yoyo-compiler-payload/code.bin');
const data=fs.readFileSync('yoyo-js/src/platform/yoyo-compiler-payload/data.bin').slice(0,0x800);
const out='scripts/_stage-stubmin';
fs.writeFileSync(out+'/stub_5876.exe', linkPeWin32(code.slice(0,5876), data, [], 0x2000));
fs.writeFileSync(out+'/stub_5875.exe', linkPeWin32(code.slice(0,5875), data, [], 0x2000));
const a=linkPeWin32(code.slice(0,5876), data, [], 0x2000);
const b=linkPeWin32(code.slice(0,5876), data, [], 0x1000);
if (a.length !== b.length) { console.log('STUBMIN status=RED reason=floor_still_binds'); process.exit(1); }
console.log('STUBMIN data_floor_nonbinding pe_bytes='+a.length);
"@
if ($LASTEXITCODE -ne 0) { exit 1 }

$odOk = Join-Path $work "place_5876"
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $odOk -Bootstrap (Join-Path $work "stub_5876.exe")
if ($LASTEXITCODE -ne 0 -or -not (Test-Path (Join-Path $odOk "yoyo_compiler.exe"))) {
  Write-Host "STUBMIN status=RED reason=min_len_place_fail"
  exit 1
}
Write-Host "STUBMIN place_5876=GREEN"

$odBad = Join-Path $work "place_5875"
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $odBad -Bootstrap (Join-Path $work "stub_5875.exe")
$badEc = $LASTEXITCODE
$badC1 = Test-Path (Join-Path $odBad "yoyo_compiler.exe")
if ($badEc -eq 0 -and $badC1) {
  Write-Host "STUBMIN status=RED reason=shorter_still_places"
  exit 1
}
Write-Host "STUBMIN place_5875=RED_OK (min locked)"

Write-Host "STUBMIN note=stub_text_at_place_min;not_Thompson"
Write-Host "STUBMIN status=GREEN"
Write-Host "STUBMIN doc=AUTO-STUBMIN.md"
exit 0
