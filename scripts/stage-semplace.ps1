# stage-semplace.ps1 — AUTO-SEMPLACE: place_mode from YOYO fragment.ty
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-SEMPLACE E2E (place_mode fragment.ty SoT) ==="

& node (Join-Path $Root "yoyo-js\scripts\bake-semplace-into-code.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMPLACE status=RED reason=bake"; exit 1 }

& node -e @"
const fs=require('fs');
function unpackFragment(text){
  const out=[];
  for(const line of text.split(/\r?\n/)){
    const t=line.trim();
    if(!t||t.startsWith(';')||/^40\s+0$/.test(t)||/^FF$/i.test(t)) continue;
    const m1=t.match(/^A1\s+([0-9a-fA-F]{2})\s+([0-9a-fA-F]{2})\s+([0-9a-fA-F]{2})$/);
    if(m1){out.push(parseInt(m1[1],16),parseInt(m1[2],16),parseInt(m1[3],16));continue;}
    const m0=t.match(/^A0\s+([0-9a-fA-F]{2})$/);
    if(m0){out.push(parseInt(m0[1],16));continue;}
  }
  return Buffer.from(out);
}
const pin=fs.readFileSync('yoyo-js/src/platform/yoyo-compiler-payload/payload.pin','utf8');
const code=fs.readFileSync('yoyo-js/src/platform/yoyo-compiler-payload/code.bin');
const coreN=parseInt(pin.match(/yoyo_semplace_core_bytes=(\d+)/)[1],10);
const pref=parseInt(pin.match(/yoyo_semplace_prefix_bytes=(\d+)/)[1],10);
const placeFrag=unpackFragment(fs.readFileSync('yoyo/projects/yoyo-openloop-place-mode.fragment.ty','utf8'));
if(!pin.includes('yoyo_semplace_place_mode_sot=yoyo-openloop-place-mode.fragment.ty')){
  console.log('SEMPLACE status=RED reason=sot_missing');process.exit(1);
}
if(placeFrag.length!==12||placeFrag[0]!==0x48||placeFrag[7]!==0xe9){
  console.log('SEMPLACE status=RED reason=bad_place_frag');process.exit(1);
}
const liveEpi=code.subarray(coreN, pref);
if(liveEpi.length!==12||liveEpi[0]!==0x48||liveEpi[7]!==0xe9){
  console.log('SEMPLACE status=RED reason=live_epi_bad');process.exit(1);
}
// opcode/imm frame matches fragment; only jmp rel32 may differ (patched).
for(let i=0;i<12;i++){
  if(i>=8&&i<=11) continue;
  if(liveEpi[i]!==placeFrag[i]){console.log('SEMPLACE status=RED reason=epi_frame_MISS at='+i);process.exit(1);}
}
console.log('SEMPLACE place_mode_sot=yoyo-openloop-place-mode.fragment.ty');
console.log('SEMPLACE place_mode_frame=EQUAL');
console.log('SEMPLACE prefix_bytes='+pref);
"@
if ($LASTEXITCODE -ne 0) { exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\build-place-stub.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMPLACE status=RED reason=rebuild_stub"; exit 1 }
& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMPLACE status=RED reason=place_stub_pin"; exit 1 }
& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMPLACE status=RED reason=bins_pin"; exit 1 }
& node (Join-Path $Root "yoyo-js\scripts\verify-code-derived-from-plant.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMPLACE status=RED reason=code_derive"; exit 1 }
& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-plant-sot.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMPLACE status=RED reason=plant_sot"; exit 1 }

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$placeOut = Join-Path $Root "scripts\_stage-semplace\place-gate"
if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub
if ($LASTEXITCODE -ne 0) { Write-Host "SEMPLACE status=RED reason=place"; exit 1 }
$c1 = Join-Path $placeOut "yoyo_compiler.exe"

& node -e @"
const fs=require('fs');
const {layoutFor}=require('./yoyo-js/src/platform/yoyo-compiler-parse1-native-link');
const code=fs.readFileSync('yoyo-js/src/platform/yoyo-compiler-payload/code.bin');
const data=fs.readFileSync('yoyo-js/src/platform/yoyo-compiler-payload/data.bin');
const pe=fs.readFileSync(process.argv[1]);
const L=layoutFor(code.length,data.length,0x38000);
if(pe.length!==L.peSize){console.log('SEMPLACE status=RED reason=pe_size');process.exit(1);}
const e=pe.readUInt32LE(0x3c); const sizeOpt=pe.readUInt16LE(e+20); const sec=e+24+sizeOpt;
const rs=pe.readUInt32LE(sec+16); const ptr=pe.readUInt32LE(sec+20);
if(rs!==L.codeRaw){console.log('SEMPLACE status=RED reason=code_raw');process.exit(1);}
if(!pe.subarray(ptr+13,ptr+13+code.length).equals(code)){console.log('SEMPLACE status=RED reason=text_not_full');process.exit(1);}
if(!pe.includes(Buffer.from('SEMHOST1'))){console.log('SEMPLACE status=RED reason=no_SEMHOST1');process.exit(1);}
console.log('SEMPLACE pe_geom=ALIGN peSize='+pe.length);
console.log('SEMPLACE text_full=EQUAL');
console.log('SEMPLACE SEMHOST1=PRESENT');
"@ $c1
if ($LASTEXITCODE -ne 0) { exit 1 }

$slices = @("ret","nop","sub","add","get","set","mov","cmp","inc","dec","addv","orv","subv","imul")
foreach ($name in $slices) {
  $d = Join-Path $Root ("scripts\_stage-semplace\sem-" + $name)
  if (Test-Path $d) { Remove-Item -Recurse -Force $d }
  New-Item -Force -ItemType Directory $d | Out-Null
  Copy-Item $c1 (Join-Path $d "yoyo_compiler.exe")
  Copy-Item (Join-Path $Root ("yoyo\projects\yoyo-sem-" + $name + ".tyb")) (Join-Path $d "input.tyb")
  $ec = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $d "yoyo_compiler.exe") -WorkingDirectory $d -TimeoutSec 60
  $out = Join-Path $d "output.exe"
  if ($ec -ne 0 -or -not (Test-Path $out)) { Write-Host ("SEMPLACE status=RED reason=slice name={0} exit={1}" -f $name,$ec); exit 1 }
  $expPath = Join-Path $Root ("yoyo\projects\yoyo-sem-" + $name + ".expect.hex")
  & node -e @"
const fs=require('fs');
const pe=fs.readFileSync(process.argv[1]);
const exp=Buffer.from(fs.readFileSync(process.argv[2],'utf8').trim().replace(/\s+/g,''),'hex');
const e=pe.readUInt32LE(0x3c);const opt=pe.readUInt16LE(e+20);const sec=e+24+opt;
const text=pe.subarray(pe.readUInt32LE(sec+20),pe.readUInt32LE(sec+20)+pe.readUInt32LE(sec+16));
if(text.indexOf(exp)<0){console.log('SEMPLACE status=RED reason=slice_MISS name='+process.argv[3]);process.exit(1);}
console.log('SEMPLACE slice_'+process.argv[3]+'_expect=EQUAL');
"@ (Join-Path $d "output.exe") $expPath $name
  if ($LASTEXITCODE -ne 0) { exit 1 }
}
Write-Host "SEMPLACE stock_sem_all14=EQUAL"

$largeDir = Join-Path $Root "scripts\_stage-semplace\large-pad"
if (Test-Path $largeDir) { Remove-Item -Recurse -Force $largeDir }
New-Item -Force -ItemType Directory $largeDir | Out-Null
$pad = Join-Path $largeDir "pad80.tyb"
& node -e "const fs=require('fs');const b=Buffer.alloc(80);b.write('TYB\0');b.writeUInt16LE(1,6);b[8]=0xff;fs.writeFileSync(process.argv[1],b);" $pad
Copy-Item $c1 (Join-Path $largeDir "yoyo_compiler.exe")
Copy-Item $pad (Join-Path $largeDir "input.tyb")
$ecL = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $largeDir "yoyo_compiler.exe") -WorkingDirectory $largeDir -TimeoutSec 60
$lOut = Join-Path $largeDir "output.exe"
if ($ecL -ne 0 -or -not (Test-Path $lOut)) { Write-Host ("SEMPLACE status=RED reason=large_fail exit={0}" -f $ecL); exit 1 }
& node -e @"
const fs=require('fs');const crypto=require('crypto');
const stock=fs.readFileSync(process.argv[1]); const out=fs.readFileSync(process.argv[2]);
if(out.length!==stock.length||crypto.createHash('sha256').update(stock).digest('hex')!==crypto.createHash('sha256').update(out).digest('hex')){
  console.log('SEMPLACE status=RED reason=large_copy_self_MISS');process.exit(1);
}
console.log('SEMPLACE large_copy_self=EQUAL bytes='+out.length);
"@ $c1 $lOut
if ($LASTEXITCODE -ne 0) { exit 1 }

Write-Host "SEMPLACE doc=AUTO-SEMPLACE.md"
Write-Host "SEMPLACE status=GREEN"
Write-Host "SEMPLACE note=place_mode_fragment_sot;not_Thompson"
exit 0
