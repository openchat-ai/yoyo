# stage-semgeom.ps1 — AUTO-SEMGEOM: place PE geometry matches full code.bin
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-SEMGEOM E2E (place PE geometry = full code.bin) ==="

& node (Join-Path $Root "yoyo-js\scripts\bake-semgeom-into-code.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMGEOM status=RED reason=bake"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\build-place-stub.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMGEOM status=RED reason=rebuild_stub"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMGEOM status=RED reason=place_stub_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMGEOM status=RED reason=bins_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-code-derived-from-plant.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMGEOM status=RED reason=code_derive"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-plant-sot.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMGEOM status=RED reason=plant_sot"; exit 1 }

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$placeOut = Join-Path $Root "scripts\_stage-semgeom\place-gate"
if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub
if ($LASTEXITCODE -ne 0) { Write-Host "SEMGEOM status=RED reason=place"; exit 1 }
$c1 = Join-Path $placeOut "yoyo_compiler.exe"

& node -e @"
const fs=require('fs');
const {layoutFor}=require('./yoyo-js/src/platform/yoyo-compiler-parse1-native-link');
const code=fs.readFileSync('yoyo-js/src/platform/yoyo-compiler-payload/code.bin');
const data=fs.readFileSync('yoyo-js/src/platform/yoyo-compiler-payload/data.bin');
const pe=fs.readFileSync(process.argv[1]);
const L=layoutFor(code.length,data.length,0x38000);
if(pe.length!==L.peSize){console.log('SEMGEOM status=RED reason=pe_size got='+pe.length+' expect='+L.peSize);process.exit(1);}
const e=pe.readUInt32LE(0x3c); const sizeOpt=pe.readUInt16LE(e+20); const sec=e+24+sizeOpt;
const rs=pe.readUInt32LE(sec+16); const ptr=pe.readUInt32LE(sec+20);
if(rs!==L.codeRaw){console.log('SEMGEOM status=RED reason=code_raw got='+rs+' expect='+L.codeRaw);process.exit(1);}
const text=pe.subarray(ptr, ptr+rs);
const startup=13;
if(!text.subarray(startup, startup+code.length).equals(code)){
  console.log('SEMGEOM status=RED reason=text_not_full_code');
  process.exit(1);
}
if(!pe.includes(Buffer.from('SEMHOST1'))){console.log('SEMGEOM status=RED reason=stock_missing_SEMHOST1');process.exit(1);}
console.log('SEMGEOM pe_geom=ALIGN peSize='+pe.length+' codeRaw='+rs);
console.log('SEMGEOM text_full=EQUAL code_bytes='+code.length);
console.log('SEMGEOM SEMHOST1=PRESENT');
"@ $c1
if ($LASTEXITCODE -ne 0) { exit 1 }

$slices = @(
  "ret", "nop", "sub", "add", "get",
  "set", "mov", "cmp", "inc", "dec",
  "addv", "orv", "subv", "imul"
)

foreach ($name in $slices) {
  $d = Join-Path $Root ("scripts\_stage-semgeom\sem-" + $name)
  if (Test-Path $d) { Remove-Item -Recurse -Force $d }
  New-Item -Force -ItemType Directory $d | Out-Null
  Copy-Item $c1 (Join-Path $d "yoyo_compiler.exe")
  Copy-Item (Join-Path $Root ("yoyo\projects\yoyo-sem-" + $name + ".tyb")) (Join-Path $d "input.tyb")
  $ec = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $d "yoyo_compiler.exe") -WorkingDirectory $d -TimeoutSec 60
  $out = Join-Path $d "output.exe"
  if ($ec -ne 0 -or -not (Test-Path $out)) {
    Write-Host ("SEMGEOM status=RED reason=slice_compile name={0} exit={1}" -f $name, $ec)
    exit 1
  }
  $expPath = Join-Path $Root ("yoyo\projects\yoyo-sem-" + $name + ".expect.hex")
  & node -e @"
const fs=require('fs');
const pe=fs.readFileSync(process.argv[1]);
const exp=Buffer.from(fs.readFileSync(process.argv[2],'utf8').trim().replace(/\s+/g,''),'hex');
if(pe[0]!==0x4d||pe[1]!==0x5a){console.log('SEMGEOM status=RED reason=slice_not_mz name='+process.argv[3]);process.exit(1);}
const e=pe.readUInt32LE(0x3c);const opt=pe.readUInt16LE(e+20);const sec=e+24+opt;
const text=pe.subarray(pe.readUInt32LE(sec+20),pe.readUInt32LE(sec+20)+pe.readUInt32LE(sec+16));
const at=text.indexOf(exp);
if(at<0){console.log('SEMGEOM status=RED reason=slice_expect_MISS name='+process.argv[3]);process.exit(1);}
console.log('SEMGEOM slice_'+process.argv[3]+'_expect=EQUAL at='+at+' out='+pe.length);
"@ (Join-Path $d "output.exe") $expPath $name
  if ($LASTEXITCODE -ne 0) { exit 1 }
}
Write-Host "SEMGEOM stock_sem_all14=EQUAL"

$largeDir = Join-Path $Root "scripts\_stage-semgeom\large-pad"
if (Test-Path $largeDir) { Remove-Item -Recurse -Force $largeDir }
New-Item -Force -ItemType Directory $largeDir | Out-Null
$pad = Join-Path $largeDir "pad80.tyb"
& node -e "const fs=require('fs');const b=Buffer.alloc(80);b.write('TYB\0');b.writeUInt16LE(1,6);b[8]=0xff;fs.writeFileSync(process.argv[1],b);" $pad
Copy-Item $c1 (Join-Path $largeDir "yoyo_compiler.exe")
Copy-Item $pad (Join-Path $largeDir "input.tyb")
$ecL = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $largeDir "yoyo_compiler.exe") -WorkingDirectory $largeDir -TimeoutSec 60
$lOut = Join-Path $largeDir "output.exe"
if ($ecL -ne 0 -or -not (Test-Path $lOut)) {
  Write-Host ("SEMGEOM status=RED reason=large_fail exit={0}" -f $ecL)
  exit 1
}
$lb = [IO.File]::ReadAllBytes($lOut)
if ($lb.Length -lt 2 -or $lb[0] -ne 0x4d -or $lb[1] -ne 0x5a) {
  Write-Host "SEMGEOM status=RED reason=large_not_mz"
  exit 1
}
Write-Host ("SEMGEOM large_still_mz=PRESENT bytes={0}" -f $lb.Length)

Write-Host "SEMGEOM doc=AUTO-SEMGEOM.md"
Write-Host "SEMGEOM status=GREEN"
Write-Host "SEMGEOM note=place_pe_geom_aligned;large_defaults_ret_pe;not_Thompson"
exit 0
