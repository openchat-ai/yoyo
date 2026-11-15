# stage-semav.ps1 — AUTO-SEMAV: Y1PL semantic exit=0
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root
. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-SEMAV E2E (Y1PL success → ExitProcess 0) ==="

& node (Join-Path $Root "yoyo-js\scripts\bake-semav-into-code.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMAV status=RED reason=bake"; exit 1 }

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
$placeOut = Join-Path $Root "scripts\_stage-semav\place-gate"
if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub
if ($LASTEXITCODE -ne 0) { Write-Host "SEMAV status=RED reason=place"; exit 1 }
$c1 = Join-Path $placeOut "yoyo_compiler.exe"

$reproDir = Join-Path $Root "scripts\_stage-semav\y1pl-poison"
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
if(n<1){console.log('SEMAV status=RED reason=poison_miss');process.exit(1);}
fs.writeFileSync(process.argv[2], pe);
console.log('SEMAV poison_str_hits='+n);
"@ $c1 (Join-Path $reproDir "poison.exe")
if ($LASTEXITCODE -ne 0) { exit 1 }
Copy-Item (Join-Path $Root "yoyo\projects\yoyo-semcomp-self.tyb") (Join-Path $reproDir "input.tyb")
$ecY = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $reproDir "poison.exe") -WorkingDirectory $reproDir -TimeoutSec 120
$yOut = Join-Path $reproDir "output.exe"
if (-not (Test-Path $yOut)) { Write-Host "SEMAV status=RED reason=no_output exit=$ecY"; exit 1 }
if ($ecY -ne 0) { Write-Host "SEMAV status=RED reason=exit_not_zero exit=$ecY"; exit 1 }
& node -e @"
const fs=require('fs');const crypto=require('crypto');
const stock=fs.readFileSync(process.argv[1]);
const out=fs.readFileSync(process.argv[2]);
const poison=fs.readFileSync(process.argv[3]);
if(out.equals(poison)){console.log('SEMAV status=RED reason=still_copy_self');process.exit(1);}
if(!out.equals(stock)){console.log('SEMAV status=RED reason=ne_stock');process.exit(1);}
console.log('SEMAV y1pl_semantic=EQUAL sha256_prefix='+crypto.createHash('sha256').update(out).digest('hex').slice(0,16));
console.log('SEMAV runner_exit=0');
console.log('SEMAV poison_copy_self_impossible=1');
"@ $c1 $yOut (Join-Path $reproDir "poison.exe")
if ($LASTEXITCODE -ne 0) { exit 1 }

$slices = @("ret","nop","sub","add","get","set","mov","cmp","inc","dec","addv","orv","subv","imul")
foreach ($name in $slices) {
  $d = Join-Path $Root ("scripts\_stage-semav\sem-" + $name)
  if (Test-Path $d) { Remove-Item -Recurse -Force $d }
  New-Item -Force -ItemType Directory $d | Out-Null
  Copy-Item $c1 (Join-Path $d "yoyo_compiler.exe")
  Copy-Item (Join-Path $Root ("yoyo\projects\yoyo-sem-" + $name + ".tyb")) (Join-Path $d "input.tyb")
  $ec = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $d "yoyo_compiler.exe") -WorkingDirectory $d -TimeoutSec 60
  $out = Join-Path $d "output.exe"
  if ($ec -ne 0 -or -not (Test-Path $out)) { Write-Host "SEMAV status=RED reason=slice name=$name"; exit 1 }
  $expPath = Join-Path $Root ("yoyo\projects\yoyo-sem-" + $name + ".expect.hex")
  & node -e @"
const fs=require('fs');
const pe=fs.readFileSync(process.argv[1]);
const exp=Buffer.from(fs.readFileSync(process.argv[2],'utf8').trim().replace(/\s+/g,''),'hex');
const e=pe.readUInt32LE(0x3c);const opt=pe.readUInt16LE(e+20);const sec=e+24+opt;
const text=pe.subarray(pe.readUInt32LE(sec+20),pe.readUInt32LE(sec+20)+pe.readUInt32LE(sec+16));
if(text.indexOf(exp)<0){console.log('SEMAV status=RED reason=slice_MISS');process.exit(1);}
console.log('SEMAV slice_'+process.argv[3]+'_expect=EQUAL');
"@ (Join-Path $d "output.exe") $expPath $name
  if ($LASTEXITCODE -ne 0) { exit 1 }
}
Write-Host "SEMAV stock_sem_all14=EQUAL"
Write-Host "SEMAV doc=AUTO-SEMAV.md"
Write-Host "SEMAV status=GREEN"
Write-Host "SEMAV note=try_place_success_ExitProcess;not_Thompson"
exit 0
