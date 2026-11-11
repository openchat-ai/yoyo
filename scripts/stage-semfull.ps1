# stage-semfull.ps1 — AUTO-SEMFULL: YOYO-sourced compiler covers all 14 parse1 sem slices
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-SEMFULL E2E (14-slice content-aware compiler) ==="

& node (Join-Path $Root "yoyo-js\scripts\emit-semops-compiler-ty.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMFULL status=RED reason=emit"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-semops-src.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMFULL status=RED reason=src_kind"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMFULL status=RED reason=place_stub_pin"; exit 1 }

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$placeOut = Join-Path $Root "scripts\_stage-semfull\place-gate"
if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub
if ($LASTEXITCODE -ne 0) { Write-Host "SEMFULL status=RED reason=place"; exit 1 }
$c1 = Join-Path $placeOut "yoyo_compiler.exe"

$buildDir = Join-Path $Root "scripts\_stage-semfull\build-P"
if (Test-Path $buildDir) { Remove-Item -Recurse -Force $buildDir }
New-Item -Force -ItemType Directory $buildDir | Out-Null
Copy-Item $c1 (Join-Path $buildDir "yoyo_compiler.exe")
Copy-Item (Join-Path $Root "yoyo\projects\yoyo-semops-compiler.tyb") (Join-Path $buildDir "input.tyb")
$ecP = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $buildDir "yoyo_compiler.exe") -WorkingDirectory $buildDir -TimeoutSec 120
$P = Join-Path $buildDir "output.exe"
if ($ecP -ne 0 -or -not (Test-Path $P)) {
  Write-Host ("SEMFULL status=RED reason=c1_semops_compile exit={0}" -f $ecP)
  exit 1
}
$pb = [IO.File]::ReadAllBytes($P)
if ($pb.Length -lt 2 -or $pb[0] -ne 0x4d -or $pb[1] -ne 0x5a) {
  Write-Host "SEMFULL status=RED reason=P_not_mz"
  exit 1
}
Write-Host ("SEMFULL semops_compiler=PRESENT bytes={0}" -f $pb.Length)

$slices = @(
  "ret", "nop", "sub", "add", "get",
  "set", "mov", "cmp", "inc", "dec",
  "addv", "orv", "subv", "imul"
)

foreach ($name in $slices) {
  $d = Join-Path $Root ("scripts\_stage-semfull\sem-" + $name)
  if (Test-Path $d) { Remove-Item -Recurse -Force $d }
  New-Item -Force -ItemType Directory $d | Out-Null
  Copy-Item $P (Join-Path $d "yoyo_compiler.exe")
  Copy-Item (Join-Path $Root ("yoyo\projects\yoyo-sem-" + $name + ".tyb")) (Join-Path $d "input.tyb")
  $ec = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $d "yoyo_compiler.exe") -WorkingDirectory $d -TimeoutSec 60
  $out = Join-Path $d "output.exe"
  if ($ec -ne 0 -or -not (Test-Path $out)) {
    Write-Host ("SEMFULL status=RED reason=slice_compile name={0} exit={1}" -f $name, $ec)
    exit 1
  }
  $expPath = Join-Path $Root ("yoyo\projects\yoyo-sem-" + $name + ".expect.hex")
  & node -e @"
const fs=require('fs');
const pe=fs.readFileSync(process.argv[1]);
const exp=Buffer.from(fs.readFileSync(process.argv[2],'utf8').trim().replace(/\s+/g,''),'hex');
if(pe[0]!==0x4d||pe[1]!==0x5a){console.log('SEMFULL status=RED reason=slice_not_mz name='+process.argv[3]);process.exit(1);}
const e=pe.readUInt32LE(0x3c);const opt=pe.readUInt16LE(e+20);const sec=e+24+opt;
const text=pe.subarray(pe.readUInt32LE(sec+20),pe.readUInt32LE(sec+20)+pe.readUInt32LE(sec+16));
const at=text.indexOf(exp);
if(at<0){console.log('SEMFULL status=RED reason=slice_expect_MISS name='+process.argv[3]);process.exit(1);}
console.log('SEMFULL slice_'+process.argv[3]+'_expect=EQUAL at='+at);
"@ (Join-Path $d "output.exe") $expPath $name
  if ($LASTEXITCODE -ne 0) { exit 1 }
}

Write-Host "SEMFULL parse1_sem_all14_expect=EQUAL"
Write-Host "SEMFULL src_kind=yoyo_ty_not_yoyc"
Write-Host "SEMFULL yoyc_role=stock_residual"
Write-Host "SEMFULL doc=AUTO-SEMFULL.md"
Write-Host "SEMFULL status=GREEN"
Write-Host "SEMFULL note=breaks_semops_batch1_only_wall;host_a1_face_remains;not_Thompson"
exit 0
