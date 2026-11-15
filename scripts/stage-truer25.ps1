# stage-truer25.ps1 — AUTO-TRUER25: full yoyo.tyb product hosts compiler I/O paths
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-TRUER25 E2E (product hosts compiler I/O paths) ==="

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER25 status=RED reason=place_stub_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER25 status=RED reason=bins_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-plant-sot.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER25 status=RED reason=plant_sot"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-assemble.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER25 status=RED reason=assemble"; exit 1 }

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$placeOut = Join-Path $Root "scripts\_stage-truer25\place-gate"
if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub -ForceDerive
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER25 status=RED reason=place"; exit 1 }
$c1 = Join-Path $placeOut "yoyo_compiler.exe"

$prodDir = Join-Path $Root "scripts\_stage-truer25\product"
if (Test-Path $prodDir) { Remove-Item -Recurse -Force $prodDir }
New-Item -Force -ItemType Directory $prodDir | Out-Null
Copy-Item $c1 (Join-Path $prodDir "yoyo_compiler.exe")
Copy-Item (Join-Path $Root "yoyo\projects\yoyo.tyb") (Join-Path $prodDir "input.tyb")
$ec = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $prodDir "yoyo_compiler.exe") -WorkingDirectory $prodDir -TimeoutSec 300
$prodOut = Join-Path $prodDir "output.exe"
if ($ec -ne 0 -or -not (Test-Path $prodOut)) {
  Write-Host ("TRUER25 status=RED reason=product_compile exit={0}" -f $ec)
  exit 1
}
Write-Host ("TRUER25 product_pe_bytes={0}" -f (Get-Item $prodOut).Length)

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-product-compiler-io-paths.js") $prodOut
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER25 status=RED reason=product_io_paths"; exit 1 }

# Face still compiles
$faceDir = Join-Path $Root "scripts\_stage-truer25\io-paths-face"
if (Test-Path $faceDir) { Remove-Item -Recurse -Force $faceDir }
New-Item -Force -ItemType Directory $faceDir | Out-Null
Copy-Item $c1 (Join-Path $faceDir "yoyo_compiler.exe")
Copy-Item (Join-Path $Root "yoyo\projects\yoyo-compiler-io-paths-face.tyb") (Join-Path $faceDir "input.tyb")
$ecFace = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $faceDir "yoyo_compiler.exe") -WorkingDirectory $faceDir -TimeoutSec 120
$faceOut = Join-Path $faceDir "output.exe"
if ($ecFace -ne 0 -or -not (Test-Path $faceOut)) {
  Write-Host ("TRUER25 status=RED reason=face_compile exit={0}" -f $ecFace)
  exit 1
}
& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-compiler-io-paths-face.js") $faceOut $prodOut
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER25 status=RED reason=face"; exit 1 }

& (Join-Path $Root "scripts\stage-truer5-semantic.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER25 status=RED reason=semantic"; exit 1 }

& (Join-Path $Root "scripts\stage-c1-native.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER25 status=RED reason=c1_native"; exit 1 }

Write-Host "TRUER25 trust_root=place_stub"
Write-Host "TRUER25 yoyo_product_compiler_io_paths=PRESENT"
Write-Host "TRUER25 doc=AUTO-TRUER25.md"
Write-Host "TRUER25 status=GREEN"
Write-Host "TRUER25 note=paths_in_full_product;still_no_open_loop;not_compiler_relay;not_Thompson"
exit 0
