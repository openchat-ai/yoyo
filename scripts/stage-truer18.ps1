# stage-truer18.ps1 — AUTO-TRUER18 E2E (axis: place from plant-derived code, not code.bin SoT)
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-TRUER18 E2E (axis change) ==="

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER18 status=RED reason=place_stub_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER18 status=RED reason=bins_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-no-sidecar-face.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER18 status=RED reason=sidecar"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-h00-bake-zero.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER18 status=RED reason=h00_clean"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-prefab-bins-sot.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER18 status=RED reason=prefab"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-assemble.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER18 status=RED reason=assemble"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-compact-code.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER18 status=RED reason=compact"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-plant-sot.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER18 status=RED reason=plant_sot"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-code-derived-from-plant.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER18 status=RED reason=code_derive_auth"; exit 1 }

# Derive code from plant tyb — tip place MUST use this, not payload code.bin as authority.
$derivedDir = Join-Path $Root "scripts\_stage-truer18\derived"
New-Item -Force -ItemType Directory $derivedDir | Out-Null
$derivedCode = Join-Path $derivedDir "code.bin"
& node (Join-Path $Root "yoyo-js\scripts\derive-code-from-plant-tyb.js") $derivedCode
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER18 status=RED reason=derive"; exit 1 }

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$placeOut = Join-Path $Root "scripts\_stage-truer18\place-gate"
if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub -CodeBin $derivedCode
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER18 status=RED reason=place_derived"; exit 1 }
$c1 = Join-Path $placeOut "yoyo_compiler.exe"

# Cold compile FF via plant-derived C1
$ffDir = Join-Path $Root "scripts\_stage-truer18\compile-ff"
if (Test-Path $ffDir) { Remove-Item -Recurse -Force $ffDir }
New-Item -Force -ItemType Directory $ffDir | Out-Null
Copy-Item $c1 (Join-Path $ffDir "yoyo_compiler.exe")
Copy-Item (Join-Path $Root "yoyo\projects\yoyo-compact-ret.tyb") (Join-Path $ffDir "input.tyb")
$ec = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $ffDir "yoyo_compiler.exe") -WorkingDirectory $ffDir -TimeoutSec 120
$ffOut = Join-Path $ffDir "output.exe"
if ($ec -ne 0 -or -not (Test-Path $ffOut)) {
    Write-Host ("TRUER18 status=RED reason=ff_compile exit={0}" -f $ec)
    exit 1
}
$mz = [IO.File]::ReadAllBytes($ffOut)[0..1]
if ($mz[0] -ne 0x4D -or $mz[1] -ne 0x5A) {
    Write-Host "TRUER18 status=RED reason=ff_not_mz"
    exit 1
}
& node -e @"
const fs=require('fs');
const pe=fs.readFileSync(process.argv[1]);
const e=pe.readUInt32LE(0x3c); const sizeOpt=pe.readUInt16LE(e+20); const sect=e+24+sizeOpt;
const text=pe.subarray(pe.readUInt32LE(sect+20), pe.readUInt32LE(sect+20)+pe.readUInt32LE(sect+16));
if (text.indexOf(Buffer.from([0xc3])) < 0) { console.log('TRUER18 status=RED reason=ff_no_c3'); process.exit(1); }
console.log('TRUER18 ff_pe_bytes=' + pe.length + ' text_has_c3=1');
"@ $ffOut
if ($LASTEXITCODE -ne 0) { exit 1 }

# Cold compile one semantic slice (inc) via same derived C1
$semDir = Join-Path $Root "scripts\_stage-truer18\compile-sem-inc"
if (Test-Path $semDir) { Remove-Item -Recurse -Force $semDir }
New-Item -Force -ItemType Directory $semDir | Out-Null
Copy-Item $c1 (Join-Path $semDir "yoyo_compiler.exe")
Copy-Item (Join-Path $Root "yoyo\projects\yoyo-sem-inc.tyb") (Join-Path $semDir "input.tyb")
$ec2 = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $semDir "yoyo_compiler.exe") -WorkingDirectory $semDir -TimeoutSec 120
$semOut = Join-Path $semDir "output.exe"
if ($ec2 -ne 0 -or -not (Test-Path $semOut)) {
    Write-Host ("TRUER18 status=RED reason=sem_inc_compile exit={0}" -f $ec2)
    exit 1
}
& node -e @"
const fs=require('fs');
const pe=fs.readFileSync(process.argv[1]);
const exp=Buffer.from(fs.readFileSync(process.argv[2],'utf8').trim().replace(/\s+/g,''),'hex');
const e=pe.readUInt32LE(0x3c); const sizeOpt=pe.readUInt16LE(e+20); const sect=e+24+sizeOpt;
const text=pe.subarray(pe.readUInt32LE(sect+20), pe.readUInt32LE(sect+20)+pe.readUInt32LE(sect+16));
const idx=text.indexOf(exp);
if (idx < 0) { console.log('TRUER18 status=RED reason=sem_inc_expect_missing'); process.exit(1); }
console.log('TRUER18 sem_inc_pe_bytes=' + pe.length + ' expect_at=' + idx);
"@ $semOut (Join-Path $Root "yoyo\projects\yoyo-sem-inc.expect.hex")
if ($LASTEXITCODE -ne 0) { exit 1 }

& (Join-Path $Root "scripts\stage-truer5-semantic.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER18 status=RED reason=semantic"; exit 1 }

& (Join-Path $Root "scripts\stage-c1-native.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER18 status=RED reason=c1_native"; exit 1 }

Write-Host "TRUER18 trust_root=place_stub"
Write-Host "TRUER18 code_authority=yoyo_plant_tyb_halves"
Write-Host "TRUER18 code_bin_role=derived_from_plant_tyb"
Write-Host "TRUER18 place_code_input=plant_derive"
Write-Host "TRUER18 derived_c1_ff=MZ_C3"
Write-Host "TRUER18 derived_c1_sem_inc=EXPECT_IN_TEXT"
Write-Host "TRUER18 doc=AUTO-TRUER18.md"
Write-Host "TRUER18 status=GREEN"
Write-Host "TRUER18 note=axis_place_from_plant_derive;not_product_as_compiler;plant_body_still_a1;not_Thompson"
exit 0
