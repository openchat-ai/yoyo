# stage-nocopy.ps1 — AUTO-NOCOPY: no copy_self; Y1PL only for large
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root
. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-NOCOPY E2E (copy_self ABSENT) ==="

& node (Join-Path $Root "yoyo-js\scripts\bake-nocopy-into-code.js")
if ($LASTEXITCODE -ne 0) { Write-Host "NOCOPY status=RED reason=bake"; exit 1 }

& node -e @"
const fs=require('fs');
const pin=fs.readFileSync('yoyo-js/src/platform/yoyo-compiler-payload/payload.pin','utf8');
const frag=fs.readFileSync('yoyo/projects/yoyo-openloop-h00.fragment.ty','utf8');
if(frag.includes('yoyo_compiler.exe')){console.log('NOCOPY status=RED reason=fragment_strSelf');process.exit(1);}
if(!pin.includes('yoyo_nocopy_copy_self=ABSENT')){console.log('NOCOPY status=RED reason=pin');process.exit(1);}
console.log('NOCOPY copy_self=ABSENT');
"@
if ($LASTEXITCODE -ne 0) { exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\build-place-stub.js")
if ($LASTEXITCODE -ne 0) { exit 1 }
& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { exit 1 }
& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { exit 1 }
& node (Join-Path $Root "yoyo-js\scripts\verify-code-derived-from-plant.js")
if ($LASTEXITCODE -ne 0) { exit 1 }
& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-plant-sot.js")
if ($LASTEXITCODE -ne 0) { exit 1 }

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$placeOut = Join-Path $Root "scripts\_stage-nocopy\place-gate"
if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub
if ($LASTEXITCODE -ne 0) { Write-Host "NOCOPY status=RED reason=place"; exit 1 }
$c1 = Join-Path $placeOut "yoyo_compiler.exe"

# pad80 must NOT copy_self — expect fail (no output or exit!=0)
$padDir = Join-Path $Root "scripts\_stage-nocopy\pad80"
if (Test-Path $padDir) { Remove-Item -Recurse -Force $padDir }
New-Item -Force -ItemType Directory $padDir | Out-Null
Copy-Item $c1 (Join-Path $padDir "yoyo_compiler.exe")
& node -e "const fs=require('fs');const b=Buffer.alloc(80);b.write('TYB\0');b.writeUInt16LE(1,6);b[8]=0xff;fs.writeFileSync(process.argv[1],b);" (Join-Path $padDir "input.tyb")
$ecP = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $padDir "yoyo_compiler.exe") -WorkingDirectory $padDir -TimeoutSec 30
$pOut = Join-Path $padDir "output.exe"
if ((Test-Path $pOut) -and ((Get-Item $pOut).Length -ge 1000)) {
  Write-Host "NOCOPY status=RED reason=pad80_still_emits_pe"
  exit 1
}
Write-Host ("NOCOPY pad80_fail_closed=1 exit={0}" -f $ecP)

# Y1PL poison still EQUAL exit0
$reproDir = Join-Path $Root "scripts\_stage-nocopy\y1pl-poison"
if (Test-Path $reproDir) { Remove-Item -Recurse -Force $reproDir }
New-Item -Force -ItemType Directory $reproDir | Out-Null
& node -e @"
const fs=require('fs');
const pe=Buffer.from(fs.readFileSync(process.argv[1]));
const s=Buffer.from('yoyo_compiler.exe'); const r=Buffer.from('xxxx_compiler.exe');
let n=0; for(let i=0;i<=pe.length-s.length;i++){ if(pe.subarray(i,i+s.length).equals(s)){ r.copy(pe,i); n++; } }
fs.writeFileSync(process.argv[2], pe); console.log('NOCOPY poison_hits='+n);
"@ $c1 (Join-Path $reproDir "poison.exe")
Copy-Item (Join-Path $Root "yoyo\projects\yoyo-semcomp-self.tyb") (Join-Path $reproDir "input.tyb")
$ecY = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $reproDir "poison.exe") -WorkingDirectory $reproDir -TimeoutSec 120
$yOut = Join-Path $reproDir "output.exe"
if ($ecY -ne 0 -or -not (Test-Path $yOut)) { Write-Host "NOCOPY status=RED reason=y1pl_fail exit=$ecY"; exit 1 }
& node -e @"
const fs=require('fs');const crypto=require('crypto');
const stock=fs.readFileSync(process.argv[1]); const out=fs.readFileSync(process.argv[2]); const poison=fs.readFileSync(process.argv[3]);
if(out.equals(poison)||!out.equals(stock)){console.log('NOCOPY status=RED reason=y1pl_ne_stock');process.exit(1);}
console.log('NOCOPY y1pl_semantic=EQUAL exit=0 sha='+crypto.createHash('sha256').update(out).digest('hex').slice(0,16));
"@ $c1 $yOut (Join-Path $reproDir "poison.exe")
if ($LASTEXITCODE -ne 0) { exit 1 }

$slices = @("ret","nop","sub","add","get","set","mov","cmp","inc","dec","addv","orv","subv","imul")
foreach ($name in $slices) {
  $d = Join-Path $Root ("scripts\_stage-nocopy\sem-" + $name)
  if (Test-Path $d) { Remove-Item -Recurse -Force $d }
  New-Item -Force -ItemType Directory $d | Out-Null
  Copy-Item $c1 (Join-Path $d "yoyo_compiler.exe")
  Copy-Item (Join-Path $Root ("yoyo\projects\yoyo-sem-" + $name + ".tyb")) (Join-Path $d "input.tyb")
  $ec = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $d "yoyo_compiler.exe") -WorkingDirectory $d -TimeoutSec 60
  $out = Join-Path $d "output.exe"
  if ($ec -ne 0 -or -not (Test-Path $out)) { Write-Host "NOCOPY status=RED reason=slice $name"; exit 1 }
  $expPath = Join-Path $Root ("yoyo\projects\yoyo-sem-" + $name + ".expect.hex")
  & node -e @"
const fs=require('fs');
const pe=fs.readFileSync(process.argv[1]);
const exp=Buffer.from(fs.readFileSync(process.argv[2],'utf8').trim().replace(/\s+/g,''),'hex');
const e=pe.readUInt32LE(0x3c);const opt=pe.readUInt16LE(e+20);const sec=e+24+opt;
const text=pe.subarray(pe.readUInt32LE(sec+20),pe.readUInt32LE(sec+20)+pe.readUInt32LE(sec+16));
if(text.indexOf(exp)<0){console.log('NOCOPY status=RED reason=slice_MISS');process.exit(1);}
console.log('NOCOPY slice_'+process.argv[3]+'_expect=EQUAL');
"@ (Join-Path $d "output.exe") $expPath $name
  if ($LASTEXITCODE -ne 0) { exit 1 }
}
Write-Host "NOCOPY stock_sem_all14=EQUAL"
Write-Host "NOCOPY doc=AUTO-NOCOPY.md"
Write-Host "NOCOPY status=GREEN"
Write-Host "NOCOPY note=copy_self_ABSENT;y1pl_only;not_Thompson"
exit 0
