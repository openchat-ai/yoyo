# stage-truer19.ps1 — AUTO-TRUER19: place with checked-in code.bin/data.bin ABSENT
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-TRUER19 E2E (place without checked-in bins) ==="

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER19 status=RED reason=place_stub_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER19 status=RED reason=bins_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-plant-sot.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER19 status=RED reason=plant_sot"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-code-derived-from-plant.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER19 status=RED reason=code_derive_auth"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-assemble.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER19 status=RED reason=assemble"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-place-bins-absent-ok.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER19 status=RED reason=absent_pin"; exit 1 }

$payload = Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload"
$codePath = Join-Path $payload "code.bin"
$dataPath = Join-Path $payload "data.bin"
$hideDir = Join-Path $Root "scripts\_stage-truer19\_hidden_bins"
$derivedDir = Join-Path $Root "scripts\_stage-truer19\derived"
New-Item -Force -ItemType Directory $hideDir | Out-Null
New-Item -Force -ItemType Directory $derivedDir | Out-Null

$hiddenCode = Join-Path $hideDir "code.bin"
$hiddenData = Join-Path $hideDir "data.bin"
$derivedCode = Join-Path $derivedDir "code.bin"
$derivedData = Join-Path $derivedDir "data.bin"

function Restore-PayloadBins {
  if (Test-Path $hiddenCode) {
    Move-Item -Force $hiddenCode $codePath
  }
  if (Test-Path $hiddenData) {
    Move-Item -Force $hiddenData $dataPath
  }
}

try {
  # Hide checked-in bins so place cannot accidentally use them.
  if (-not (Test-Path $codePath) -or -not (Test-Path $dataPath)) {
    Write-Host "TRUER19 status=RED reason=payload_bins_missing_before_hide"
    exit 1
  }
  Move-Item -Force $codePath $hiddenCode
  Move-Item -Force $dataPath $hiddenData
  if ((Test-Path $codePath) -or (Test-Path $dataPath)) {
    Write-Host "TRUER19 status=RED reason=hide_failed"
    exit 1
  }
  Write-Host "TRUER19 checked_in_bins=HIDDEN"

  & node (Join-Path $Root "yoyo-js\scripts\derive-code-from-plant-tyb.js") $derivedCode
  if ($LASTEXITCODE -ne 0) { throw "derive_code failed" }
  & node (Join-Path $Root "yoyo-js\scripts\derive-data-from-seeds.js") $derivedData
  if ($LASTEXITCODE -ne 0) { throw "derive_data failed" }

  # Coherence vs hidden cache
  $hCode = Get-FileHash $hiddenCode -Algorithm SHA256
  $dCode = Get-FileHash $derivedCode -Algorithm SHA256
  $hData = Get-FileHash $hiddenData -Algorithm SHA256
  $dData = Get-FileHash $derivedData -Algorithm SHA256
  if ($hCode.Hash -ne $dCode.Hash) { throw "derived_code_ne_hidden_cache" }
  if ($hData.Hash -ne $dData.Hash) { throw "derived_data_ne_hidden_cache" }
  Write-Host "TRUER19 derive_vs_cache=EQUAL"

  $stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
  $placeOut = Join-Path $Root "scripts\_stage-truer19\place-gate"
  if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }
  & (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub -CodeBin $derivedCode -DataBin $derivedData
  if ($LASTEXITCODE -ne 0) { throw "place_derived_failed" }
  $c1 = Join-Path $placeOut "yoyo_compiler.exe"

  # Prove payload bins still absent during compile
  if ((Test-Path $codePath) -or (Test-Path $dataPath)) {
    throw "payload_bins_reappeared"
  }

  $ffDir = Join-Path $Root "scripts\_stage-truer19\compile-ff"
  if (Test-Path $ffDir) { Remove-Item -Recurse -Force $ffDir }
  New-Item -Force -ItemType Directory $ffDir | Out-Null
  Copy-Item $c1 (Join-Path $ffDir "yoyo_compiler.exe")
  Copy-Item (Join-Path $Root "yoyo\projects\yoyo-compact-ret.tyb") (Join-Path $ffDir "input.tyb")
  $ec = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $ffDir "yoyo_compiler.exe") -WorkingDirectory $ffDir -TimeoutSec 120
  $ffOut = Join-Path $ffDir "output.exe"
  if ($ec -ne 0 -or -not (Test-Path $ffOut)) { throw ("ff_compile_failed exit={0}" -f $ec) }
  $mz = [IO.File]::ReadAllBytes($ffOut)[0..1]
  if ($mz[0] -ne 0x4D -or $mz[1] -ne 0x5A) { throw "ff_not_mz" }

  $semDir = Join-Path $Root "scripts\_stage-truer19\compile-sem-sub"
  if (Test-Path $semDir) { Remove-Item -Recurse -Force $semDir }
  New-Item -Force -ItemType Directory $semDir | Out-Null
  Copy-Item $c1 (Join-Path $semDir "yoyo_compiler.exe")
  Copy-Item (Join-Path $Root "yoyo\projects\yoyo-sem-sub.tyb") (Join-Path $semDir "input.tyb")
  $ec2 = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $semDir "yoyo_compiler.exe") -WorkingDirectory $semDir -TimeoutSec 120
  $semOut = Join-Path $semDir "output.exe"
  if ($ec2 -ne 0 -or -not (Test-Path $semOut)) { throw ("sem_sub_compile_failed exit={0}" -f $ec2) }
  Write-Host ("TRUER19 hide_compile ff={0} sem_sub={1}" -f (Get-Item $ffOut).Length, (Get-Item $semOut).Length)
}
catch {
  Write-Host ("TRUER19 status=RED reason={0}" -f $_)
  Restore-PayloadBins
  exit 1
}

# Separate verifies after restore for clarity
Restore-PayloadBins
if (-not (Test-Path $codePath) -or -not (Test-Path $dataPath)) {
  Write-Host "TRUER19 status=RED reason=restore_failed"
  exit 1
}
Write-Host "TRUER19 checked_in_bins=RESTORED"

# Re-check sem-sub expect properly on the PE produced while bins were hidden
& node -e "const fs=require('fs');const pe=fs.readFileSync(process.argv[1]);const exp=Buffer.from(fs.readFileSync(process.argv[2],'utf8').trim().replace(/\s+/g,''),'hex');const e=pe.readUInt32LE(0x3c);const sizeOpt=pe.readUInt16LE(e+20);const sect=e+24+sizeOpt;const text=pe.subarray(pe.readUInt32LE(sect+20),pe.readUInt32LE(sect+20)+pe.readUInt32LE(sect+16));if(pe[0]!==0x4d||pe[1]!==0x5a)process.exit(1);if(text.indexOf(exp)<0){console.log('TRUER19 status=RED reason=sem_sub_expect');process.exit(1);}console.log('TRUER19 sem_sub_expect_at='+text.indexOf(exp));" (Join-Path $Root "scripts\_stage-truer19\compile-sem-sub\output.exe") (Join-Path $Root "yoyo\projects\yoyo-sem-sub.expect.hex")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER19 status=RED reason=sem_sub"; exit 1 }

& node -e "const fs=require('fs');const pe=fs.readFileSync(process.argv[1]);const e=pe.readUInt32LE(0x3c);const sizeOpt=pe.readUInt16LE(e+20);const sect=e+24+sizeOpt;const text=pe.subarray(pe.readUInt32LE(sect+20),pe.readUInt32LE(sect+20)+pe.readUInt32LE(sect+16));if(text.indexOf(Buffer.from([0xc3]))<0)process.exit(1);console.log('TRUER19 ff_text_has_c3=1 bytes='+pe.length);" (Join-Path $Root "scripts\_stage-truer19\compile-ff\output.exe")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER19 status=RED reason=ff_c3"; exit 1 }

& (Join-Path $Root "scripts\stage-truer5-semantic.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER19 status=RED reason=semantic"; exit 1 }

& (Join-Path $Root "scripts\stage-c1-native.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER19 status=RED reason=c1_native"; exit 1 }

Write-Host "TRUER19 trust_root=place_stub"
Write-Host "TRUER19 code_bin_required_on_place=ABSENT"
Write-Host "TRUER19 data_bin_required_on_place=ABSENT"
Write-Host "TRUER19 place_bins_input=plant_derive_plus_assemble"
Write-Host "TRUER19 hide_place_compile=GREEN"
Write-Host "TRUER19 doc=AUTO-TRUER19.md"
Write-Host "TRUER19 status=GREEN"
Write-Host "TRUER19 note=place_without_checked_in_bins;plant_body_still_a1;not_Thompson"
exit 0
