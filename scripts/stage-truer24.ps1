# stage-truer24.ps1 — AUTO-TRUER24: YOYO io-paths face into PE; full product still ABSENT
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-TRUER24 E2E (YOYO io-paths face in PE) ==="

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER24 status=RED reason=place_stub_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER24 status=RED reason=bins_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-plant-sot.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER24 status=RED reason=plant_sot"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-assemble.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER24 status=RED reason=assemble"; exit 1 }

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$placeOut = Join-Path $Root "scripts\_stage-truer24\place-gate"
if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub -ForceDerive
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER24 status=RED reason=place"; exit 1 }
$c1 = Join-Path $placeOut "yoyo_compiler.exe"
if (-not (Test-Path $c1)) { Write-Host "TRUER24 status=RED reason=missing_c1"; exit 1 }

# Face: C1 × io-paths-face.tyb
$faceDir = Join-Path $Root "scripts\_stage-truer24\io-paths-face"
if (Test-Path $faceDir) { Remove-Item -Recurse -Force $faceDir }
New-Item -Force -ItemType Directory $faceDir | Out-Null
Copy-Item $c1 (Join-Path $faceDir "yoyo_compiler.exe")
Copy-Item (Join-Path $Root "yoyo\projects\yoyo-compiler-io-paths-face.tyb") (Join-Path $faceDir "input.tyb")
$ecFace = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $faceDir "yoyo_compiler.exe") -WorkingDirectory $faceDir -TimeoutSec 120
$faceOut = Join-Path $faceDir "output.exe"
if ($ecFace -ne 0 -or -not (Test-Path $faceOut)) {
  Write-Host ("TRUER24 status=RED reason=face_compile exit={0}" -f $ecFace)
  exit 1
}
Write-Host ("TRUER24 face_pe_bytes={0}" -f (Get-Item $faceOut).Length)

# Full product still ABSENT paths
$prodDir = Join-Path $Root "scripts\_stage-truer24\product"
if (Test-Path $prodDir) { Remove-Item -Recurse -Force $prodDir }
New-Item -Force -ItemType Directory $prodDir | Out-Null
Copy-Item $c1 (Join-Path $prodDir "yoyo_compiler.exe")
Copy-Item (Join-Path $Root "yoyo\projects\yoyo.tyb") (Join-Path $prodDir "input.tyb")
$ecProd = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $prodDir "yoyo_compiler.exe") -WorkingDirectory $prodDir -TimeoutSec 300
$prodOut = Join-Path $prodDir "output.exe"
if ($ecProd -ne 0 -or -not (Test-Path $prodOut)) {
  Write-Host ("TRUER24 status=RED reason=product_compile exit={0}" -f $ecProd)
  exit 1
}

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-compiler-io-paths-face.js") $faceOut $prodOut
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER24 status=RED reason=io_paths_face"; exit 1 }

& (Join-Path $Root "scripts\stage-truer5-semantic.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER24 status=RED reason=semantic"; exit 1 }

& (Join-Path $Root "scripts\stage-c1-native.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER24 status=RED reason=c1_native"; exit 1 }

Write-Host "TRUER24 trust_root=place_stub"
Write-Host "TRUER24 yoyo_compiler_io_paths_face=PRESENT"
Write-Host "TRUER24 yoyo_product_compiler_io_paths=ABSENT"
Write-Host "TRUER24 doc=AUTO-TRUER24.md"
Write-Host "TRUER24 status=GREEN"
Write-Host "TRUER24 note=io_paths_face_in_pe;full_product_still_absent;not_compiler_loop;not_Thompson"
exit 0
