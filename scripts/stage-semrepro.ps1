# stage-semrepro.ps1 — AUTO-SEMREPRO: Y1PL large → semantic body (not copy_self)
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-SEMREPRO E2E (Y1PL → body semantic self) ==="

& node (Join-Path $Root "yoyo-js\scripts\bake-semrepro-into-code.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMREPRO status=RED reason=bake"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\build-place-stub.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMREPRO status=RED reason=rebuild_stub"; exit 1 }
& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMREPRO status=RED reason=place_stub_pin"; exit 1 }
& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMREPRO status=RED reason=bins_pin"; exit 1 }
& node (Join-Path $Root "yoyo-js\scripts\verify-code-derived-from-plant.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMREPRO status=RED reason=code_derive"; exit 1 }
& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-plant-sot.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMREPRO status=RED reason=plant_sot"; exit 1 }

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$placeOut = Join-Path $Root "scripts\_stage-semrepro\place-gate"
if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub
if ($LASTEXITCODE -ne 0) { Write-Host "SEMREPRO status=RED reason=place"; exit 1 }
$c1 = Join-Path $placeOut "yoyo_compiler.exe"

& node -e @"
const fs=require('fs');
const {layoutFor}=require('./yoyo-js/src/platform/yoyo-compiler-parse1-native-link');
const code=fs.readFileSync('yoyo-js/src/platform/yoyo-compiler-payload/code.bin');
const data=fs.readFileSync('yoyo-js/src/platform/yoyo-compiler-payload/data.bin');
const pe=fs.readFileSync(process.argv[1]);
const L=layoutFor(code.length,data.length,0x38000);
if(pe.length!==L.peSize){console.log('SEMREPRO status=RED reason=pe_size');process.exit(1);}
const e=pe.readUInt32LE(0x3c); const sizeOpt=pe.readUInt16LE(e+20); const sec=e+24+sizeOpt;
const rs=pe.readUInt32LE(sec+16); const ptr=pe.readUInt32LE(sec+20);
if(!pe.subarray(ptr+13,ptr+13+code.length).equals(code)){console.log('SEMREPRO status=RED reason=text_not_full');process.exit(1);}
console.log('SEMREPRO pe_geom=ALIGN peSize='+pe.length+' codeRaw='+rs);
console.log('SEMREPRO text_full=EQUAL');
"@ $c1
if ($LASTEXITCODE -ne 0) { exit 1 }

# Proof: poison copy_self name in runner — output must still EQUAL clean stock (from Y1PL plant).
$reproDir = Join-Path $Root "scripts\_stage-semrepro\y1pl-poison"
if (Test-Path $reproDir) { Remove-Item -Recurse -Force $reproDir }
New-Item -Force -ItemType Directory $reproDir | Out-Null
& node -e @"
const fs=require('fs');
const pe=Buffer.from(fs.readFileSync(process.argv[1]));
const s=Buffer.from('yoyo_compiler.exe');
const r=Buffer.from('xxxx_compiler.exe');
let n=0;
for(let i=0;i<=pe.length-s.length;i++){
  if(pe.subarray(i,i+s.length).equals(s)){ r.copy(pe,i); n++; }
}
if(n<1){console.log('SEMREPRO status=RED reason=poison_str_miss');process.exit(1);}
fs.writeFileSync(process.argv[2], pe);
console.log('SEMREPRO poison_str_hits='+n);
"@ $c1 (Join-Path $reproDir "poison.exe")
if ($LASTEXITCODE -ne 0) { exit 1 }
Copy-Item (Join-Path $Root "yoyo\projects\yoyo-semcomp-self.tyb") (Join-Path $reproDir "input.tyb")
$ecY = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $reproDir "poison.exe") -WorkingDirectory $reproDir -TimeoutSec 120
$yOut = Join-Path $reproDir "output.exe"
if (-not (Test-Path $yOut)) {
  Write-Host ("SEMREPRO status=RED reason=y1pl_no_output exit={0}" -f $ecY)
  exit 1
}
& node -e @"
const fs=require('fs');const crypto=require('crypto');
const stock=fs.readFileSync(process.argv[1]);
const out=fs.readFileSync(process.argv[2]);
const poison=fs.readFileSync(process.argv[3]);
if(out.equals(poison)){console.log('SEMREPRO status=RED reason=still_copy_self');process.exit(1);}
if(out.includes(Buffer.from('xxxx_compiler.exe'))){console.log('SEMREPRO status=RED reason=poison_leaked');process.exit(1);}
if(!out.equals(stock)){console.log('SEMREPRO status=RED reason=y1pl_ne_stock');process.exit(1);}
console.log('SEMREPRO y1pl_semantic=EQUAL bytes='+out.length+' sha256_prefix='+crypto.createHash('sha256').update(out).digest('hex').slice(0,16));
console.log('SEMREPRO poison_copy_self_impossible=1');
console.log('SEMREPRO runner_exit='+process.argv[4]+' (AV-after-write allowed)');
"@ $c1 $yOut (Join-Path $reproDir "poison.exe") "$ecY"
if ($LASTEXITCODE -ne 0) { exit 1 }

$slices = @("ret","nop","sub","add","get","set","mov","cmp","inc","dec","addv","orv","subv","imul")
foreach ($name in $slices) {
  $d = Join-Path $Root ("scripts\_stage-semrepro\sem-" + $name)
  if (Test-Path $d) { Remove-Item -Recurse -Force $d }
  New-Item -Force -ItemType Directory $d | Out-Null
  Copy-Item $c1 (Join-Path $d "yoyo_compiler.exe")
  Copy-Item (Join-Path $Root ("yoyo\projects\yoyo-sem-" + $name + ".tyb")) (Join-Path $d "input.tyb")
  $ec = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $d "yoyo_compiler.exe") -WorkingDirectory $d -TimeoutSec 60
  $out = Join-Path $d "output.exe"
  if ($ec -ne 0 -or -not (Test-Path $out)) { Write-Host ("SEMREPRO status=RED reason=slice name={0}" -f $name); exit 1 }
  $expPath = Join-Path $Root ("yoyo\projects\yoyo-sem-" + $name + ".expect.hex")
  & node -e @"
const fs=require('fs');
const pe=fs.readFileSync(process.argv[1]);
const exp=Buffer.from(fs.readFileSync(process.argv[2],'utf8').trim().replace(/\s+/g,''),'hex');
const e=pe.readUInt32LE(0x3c);const opt=pe.readUInt16LE(e+20);const sec=e+24+opt;
const text=pe.subarray(pe.readUInt32LE(sec+20),pe.readUInt32LE(sec+20)+pe.readUInt32LE(sec+16));
if(text.indexOf(exp)<0){console.log('SEMREPRO status=RED reason=slice_MISS name='+process.argv[3]);process.exit(1);}
console.log('SEMREPRO slice_'+process.argv[3]+'_expect=EQUAL');
"@ (Join-Path $d "output.exe") $expPath $name
  if ($LASTEXITCODE -ne 0) { exit 1 }
}
Write-Host "SEMREPRO stock_sem_all14=EQUAL"

Write-Host "SEMREPRO doc=AUTO-SEMREPRO.md"
Write-Host "SEMREPRO status=GREEN"
Write-Host "SEMREPRO note=y1pl_to_body_semantic;copy_self_fallback;not_Thompson"
exit 0
