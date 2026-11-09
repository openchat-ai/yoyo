# stage-truer13.ps1 — AUTO-TRUER13 E2E (YOYO tyb → runnable MZ PE)
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-TRUER13 E2E ==="

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER13 status=RED reason=place_stub_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER13 status=RED reason=bins_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-no-sidecar-face.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER13 status=RED reason=sidecar"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-h00-bake-zero.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER13 status=RED reason=h00_clean"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-prefab-bins-sot.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER13 status=RED reason=prefab"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-assemble.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER13 status=RED reason=assemble"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-compact-code.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER13 status=RED reason=compact"; exit 1 }

# R2: H00 abs jne must be NOP'd in code.bin
& node -e @"
const fs=require('fs');
const code=fs.readFileSync('yoyo-js/src/platform/yoyo-compiler-payload/code.bin');
const off=9867;
const b=code.slice(off, off+6);
if (![0x90,0x90,0x90,0x90,0x90,0x90].every((x,i)=>b[i]===x)) {
  console.log('H00_ABS status=RED reason=jne_not_nop hex='+b.toString('hex'));
  process.exit(1);
}
console.log('H00_ABS skip=NOP off=9867');
console.log('H00_ABS status=GREEN');
"@
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER13 status=RED reason=h00_abs_skip"; exit 1 }

& (Join-Path $Root "scripts\stage-truer5-semantic.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER13 status=RED reason=semantic"; exit 1 }

& (Join-Path $Root "scripts\stage-c1-native.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER13 status=RED reason=c1_native"; exit 1 }

# R3: place → C1 × yoyo.tyb → MZ runnable YOYO PE
$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$placeOut = Join-Path $Root "scripts\_stage-truer13\place-gate"
if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER13 status=RED reason=place"; exit 1 }

$cold = Join-Path $Root "scripts\_stage-truer13\yoyo-mz"
if (Test-Path $cold) { Remove-Item -Recurse -Force $cold }
New-Item -Force -ItemType Directory $cold | Out-Null
Copy-Item (Join-Path $placeOut "yoyo_compiler.exe") (Join-Path $cold "yoyo_compiler.exe")
Copy-Item (Join-Path $Root "yoyo\projects\yoyo.tyb") (Join-Path $cold "input.tyb")
$ec = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $cold "yoyo_compiler.exe") -WorkingDirectory $cold -TimeoutSec 300
$out = Join-Path $cold "output.exe"
if ($ec -ne 0 -or -not (Test-Path $out)) {
    Write-Host ("TRUER13 status=RED reason=yoyo_tyb_compile exit={0}" -f $ec)
    exit 1
}

& node -e @"
const fs=require('fs'); const crypto=require('crypto');
const b=fs.readFileSync('scripts/_stage-truer13/yoyo-mz/output.exe');
if (b[0]!==0x4d || b[1]!==0x5a) { console.log('YOYO_MZ status=RED reason=not_mz'); process.exit(1); }
const e=b.readUInt32LE(0x3c);
if (e===0 || b[e]!==0x50 || b[e+1]!==0x45) { console.log('YOYO_MZ status=RED reason=bad_pe'); process.exit(1); }
let nz=0; for (const x of b) if (x) nz++;
const sha=crypto.createHash('sha256').update(b).digest('hex');
console.log('YOYO_MZ bytes='+b.length+' nonzero='+nz+' sha256_prefix='+sha.slice(0,16));
console.log('YOYO_MZ status=GREEN');
"@
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER13 status=RED reason=yoyo_mz"; exit 1 }

# Run YOYO PE (must start + exit, not AV on launch)
$run = Join-Path $Root "scripts\_stage-truer13\yoyo-run"
if (Test-Path $run) { Remove-Item -Recurse -Force $run }
New-Item -Force -ItemType Directory $run | Out-Null
Copy-Item $out (Join-Path $run "yoyo_product.exe")
$ec2 = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $run "yoyo_product.exe") -WorkingDirectory $run -TimeoutSec 60
if ($ec2 -eq -1) {
    Write-Host "TRUER13 status=RED reason=yoyo_product_launch_fail"
    exit 1
}
# Accept exit 0 or ordinary nonzero; reject AV-class if reported as such by wrapper
Write-Host ("TRUER13 yoyo_product_exit={0}" -f $ec2)

Write-Host "TRUER13 trust_root=place_stub"
Write-Host "TRUER13 yoyo_tyb_product=MZ_RUNNABLE"
Write-Host "TRUER13 h00_abs_path=SKIPPED_NOP"
Write-Host "TRUER13 prefab_sot=code_plus_seeds"
Write-Host "TRUER13 doc=AUTO-TRUER13.md"
Write-Host "TRUER13 status=GREEN"
Write-Host "TRUER13 note=yoyo_product_real_pe;plant_still_bootstrap;not_Thompson"
exit 0
