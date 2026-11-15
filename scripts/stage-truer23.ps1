# stage-truer23.ps1 — AUTO-TRUER23: compiler I/O names SoT = YOYO tyb
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-TRUER23 E2E (compiler I/O names YOYO SoT) ==="

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER23 status=RED reason=place_stub_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER23 status=RED reason=bins_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-plant-sot.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER23 status=RED reason=plant_sot"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-code-derived-from-plant.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER23 status=RED reason=code_derive_auth"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-assemble.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER23 status=RED reason=assemble"; exit 1 }

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$placeOut = Join-Path $Root "scripts\_stage-truer23\place-gate"
if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }

& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub -ForceDerive
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER23 status=RED reason=place"; exit 1 }
$c1 = Join-Path $placeOut "yoyo_compiler.exe"
if (-not (Test-Path $c1)) { Write-Host "TRUER23 status=RED reason=missing_c1"; exit 1 }

$prodDir = Join-Path $Root "scripts\_stage-truer23\product"
if (Test-Path $prodDir) { Remove-Item -Recurse -Force $prodDir }
New-Item -Force -ItemType Directory $prodDir | Out-Null
Copy-Item $c1 (Join-Path $prodDir "yoyo_compiler.exe")
Copy-Item (Join-Path $Root "yoyo\projects\yoyo.tyb") (Join-Path $prodDir "input.tyb")
$ec = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $prodDir "yoyo_compiler.exe") -WorkingDirectory $prodDir -TimeoutSec 300
$prodOut = Join-Path $prodDir "output.exe"
if ($ec -ne 0 -or -not (Test-Path $prodOut)) {
  Write-Host ("TRUER23 status=RED reason=product_compile exit={0}" -f $ec)
  exit 1
}

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-compiler-io-names-sot.js") $c1 $prodOut
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER23 status=RED reason=io_names_sot"; exit 1 }

& (Join-Path $Root "scripts\stage-truer5-semantic.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER23 status=RED reason=semantic"; exit 1 }

& (Join-Path $Root "scripts\stage-c1-native.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER23 status=RED reason=c1_native"; exit 1 }

Write-Host "TRUER23 trust_root=place_stub"
Write-Host "TRUER23 compiler_io_names_sot=yoyo_compiler_io_names_tyb"
Write-Host "TRUER23 yoyo_product_compiler_io_paths=ABSENT"
Write-Host "TRUER23 doc=AUTO-TRUER23.md"
Write-Host "TRUER23 status=GREEN"
Write-Host "TRUER23 note=io_names_yoyo_sot;still_a1_literals;product_not_compiler;not_Thompson"
exit 0
