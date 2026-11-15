# stage-truer20.ps1 — AUTO-TRUER20: place auto-derive on absent (no -CodeBin/-DataBin)
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-TRUER20 E2E (place auto-derive default) ==="

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER20 status=RED reason=place_stub_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER20 status=RED reason=bins_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-plant-sot.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER20 status=RED reason=plant_sot"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-code-derived-from-plant.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER20 status=RED reason=code_derive_auth"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-assemble.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER20 status=RED reason=assemble"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-place-bins-absent-ok.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER20 status=RED reason=absent_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-place-auto-derive-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER20 status=RED reason=auto_derive_pin"; exit 1 }

$payload = Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload"
$codePath = Join-Path $payload "code.bin"
$dataPath = Join-Path $payload "data.bin"
$hideDir = Join-Path $Root "scripts\_stage-truer20\_hidden_bins"
New-Item -Force -ItemType Directory $hideDir | Out-Null
$hiddenCode = Join-Path $hideDir "code.bin"
$hiddenData = Join-Path $hideDir "data.bin"

function Restore-PayloadBins {
  if (Test-Path $hiddenCode) { Move-Item -Force $hiddenCode $codePath }
  if (Test-Path $hiddenData) { Move-Item -Force $hiddenData $dataPath }
}

try {
  Move-Item -Force $codePath $hiddenCode
  Move-Item -Force $dataPath $hiddenData
  if ((Test-Path $codePath) -or (Test-Path $dataPath)) { throw "hide_failed" }
  Write-Host "TRUER20 checked_in_bins=HIDDEN"

  $stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
  $placeOut = Join-Path $Root "scripts\_stage-truer20\place-gate"
  if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }

  # No -CodeBin/-DataBin: place must auto-derive into OutDir/_derived.
  $log = Join-Path $Root "scripts\_stage-truer20\place.log"
  $placeOutLog = @(& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub 2>&1 | ForEach-Object { "$_" })
  $placeEc = $LASTEXITCODE
  $placeOutLog | Set-Content -Path $log -Encoding utf8
  $placeOutLog | ForEach-Object { Write-Host $_ }
  if ($placeEc -ne 0) { throw ("place_auto_derive_failed exit={0}" -f $placeEc) }

  $derivedCode = Join-Path $placeOut "_derived\code.bin"
  $derivedData = Join-Path $placeOut "_derived\data.bin"
  if (-not (Test-Path $derivedCode)) { throw "missing_auto_derived_code" }
  if (-not (Test-Path $derivedData)) { throw "missing_auto_derived_data" }
  if (-not (Test-Path (Join-Path $placeOut "yoyo_compiler.exe"))) { throw "missing_placed_c1" }
  $logText = ($placeOutLog -join "`n")
  if ($logText -notmatch "auto_derive=ON") { throw "auto_derive_not_logged" }
  if ($logText -notmatch "place_bins_input=plant_derive_plus_assemble") { throw "place_bins_input_not_logged" }
  if ($logText -notmatch "auto_derive_used=1") { throw "auto_derive_used_not_1" }
  # Derived must EQUAL hidden cache (authority coherence).
  if ((Get-FileHash $derivedCode -Algorithm SHA256).Hash -ne (Get-FileHash $hiddenCode -Algorithm SHA256).Hash) {
    throw "auto_derived_code_ne_cache"
  }
  if ((Get-FileHash $derivedData -Algorithm SHA256).Hash -ne (Get-FileHash $hiddenData -Algorithm SHA256).Hash) {
    throw "auto_derived_data_ne_cache"
  }
  Write-Host "TRUER20 place_auto_derive=ON"

  if ((Test-Path $codePath) -or (Test-Path $dataPath)) { throw "payload_bins_reappeared" }

  $c1 = Join-Path $placeOut "yoyo_compiler.exe"
  $ffDir = Join-Path $Root "scripts\_stage-truer20\compile-ff"
  if (Test-Path $ffDir) { Remove-Item -Recurse -Force $ffDir }
  New-Item -Force -ItemType Directory $ffDir | Out-Null
  Copy-Item $c1 (Join-Path $ffDir "yoyo_compiler.exe")
  Copy-Item (Join-Path $Root "yoyo\projects\yoyo-compact-ret.tyb") (Join-Path $ffDir "input.tyb")
  $ec = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $ffDir "yoyo_compiler.exe") -WorkingDirectory $ffDir -TimeoutSec 120
  $ffOut = Join-Path $ffDir "output.exe"
  if ($ec -ne 0 -or -not (Test-Path $ffOut)) { throw ("ff_compile_failed exit={0}" -f $ec) }
  $mz = [IO.File]::ReadAllBytes($ffOut)[0..1]
  if ($mz[0] -ne 0x4D -or $mz[1] -ne 0x5A) { throw "ff_not_mz" }
  Write-Host ("TRUER20 ff_pe_bytes={0}" -f (Get-Item $ffOut).Length)
}
catch {
  Write-Host ("TRUER20 status=RED reason={0}" -f $_)
  Restore-PayloadBins
  exit 1
}

Restore-PayloadBins
if (-not (Test-Path $codePath) -or -not (Test-Path $dataPath)) {
  Write-Host "TRUER20 status=RED reason=restore_failed"
  exit 1
}
Write-Host "TRUER20 checked_in_bins=RESTORED"

& node -e "const fs=require('fs');const pe=fs.readFileSync(process.argv[1]);const e=pe.readUInt32LE(0x3c);const sizeOpt=pe.readUInt16LE(e+20);const sect=e+24+sizeOpt;const text=pe.subarray(pe.readUInt32LE(sect+20),pe.readUInt32LE(sect+20)+pe.readUInt32LE(sect+16));if(text.indexOf(Buffer.from([0xc3]))<0)process.exit(1);console.log('TRUER20 ff_text_has_c3=1');" (Join-Path $Root "scripts\_stage-truer20\compile-ff\output.exe")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER20 status=RED reason=ff_c3"; exit 1 }

& (Join-Path $Root "scripts\stage-truer5-semantic.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER20 status=RED reason=semantic"; exit 1 }

& (Join-Path $Root "scripts\stage-c1-native.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER20 status=RED reason=c1_native"; exit 1 }

Write-Host "TRUER20 trust_root=place_stub"
Write-Host "TRUER20 place_auto_derive_on_absent=PRESENT"
Write-Host "TRUER20 place_bins_input=plant_derive_plus_assemble"
Write-Host "TRUER20 hide_place_no_overrides=GREEN"
Write-Host "TRUER20 doc=AUTO-TRUER20.md"
Write-Host "TRUER20 status=GREEN"
Write-Host "TRUER20 note=place_tool_auto_derive;plant_body_still_a1;not_Thompson"
exit 0
