# stage-truer37.ps1 — AUTO-TRUER37: open-loop glue C1 yes / product no
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-TRUER37 E2E (open-loop control-flow glue) ==="

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER37 status=RED reason=place_stub_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER37 status=RED reason=bins_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-code-derived-from-plant.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER37 status=RED reason=code_derive"; exit 1 }

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$placeOut = Join-Path $Root "scripts\_stage-truer37\place-gate"
if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub -ForceDerive
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER37 status=RED reason=place"; exit 1 }
$c1 = Join-Path $placeOut "yoyo_compiler.exe"

$faceDir = Join-Path $Root "scripts\_stage-truer37\glue-face"
if (Test-Path $faceDir) { Remove-Item -Recurse -Force $faceDir }
New-Item -Force -ItemType Directory $faceDir | Out-Null
Copy-Item $c1 (Join-Path $faceDir "yoyo_compiler.exe")
Copy-Item (Join-Path $Root "yoyo\projects\yoyo-compiler-open-loop-glue-face.tyb") (Join-Path $faceDir "input.tyb")
$ecFace = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $faceDir "yoyo_compiler.exe") -WorkingDirectory $faceDir -TimeoutSec 120
$faceOut = Join-Path $faceDir "output.exe"
if ($ecFace -ne 0 -or -not (Test-Path $faceOut)) {
  Write-Host ("TRUER37 status=RED reason=face_compile exit={0}" -f $ecFace)
  exit 1
}
Write-Host ("TRUER37 face_pe_bytes={0}" -f (Get-Item $faceOut).Length)

$prodDir = Join-Path $Root "scripts\_stage-truer37\product"
if (Test-Path $prodDir) { Remove-Item -Recurse -Force $prodDir }
New-Item -Force -ItemType Directory $prodDir | Out-Null
Copy-Item $c1 (Join-Path $prodDir "yoyo_compiler.exe")
Copy-Item (Join-Path $Root "yoyo\projects\yoyo.tyb") (Join-Path $prodDir "input.tyb")
$ecProd = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $prodDir "yoyo_compiler.exe") -WorkingDirectory $prodDir -TimeoutSec 300
$prodOut = Join-Path $prodDir "output.exe"
if ($ecProd -ne 0 -or -not (Test-Path $prodOut)) {
  Write-Host ("TRUER37 status=RED reason=product_compile exit={0}" -f $ecProd)
  exit 1
}

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-compiler-open-loop-glue.js") $faceOut $prodOut $c1
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER37 status=RED reason=glue"; exit 1 }

& (Join-Path $Root "scripts\stage-truer5-semantic.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER37 status=RED reason=semantic"; exit 1 }

& (Join-Path $Root "scripts\stage-c1-native.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER37 status=RED reason=c1_native"; exit 1 }

Write-Host "TRUER37 trust_root=place_stub"
Write-Host "TRUER37 yoyo_compiler_open_loop_glue=PRESENT"
Write-Host "TRUER37 yoyo_product_open_loop_glue=ABSENT"
Write-Host "TRUER37 doc=AUTO-TRUER37.md"
Write-Host "TRUER37 status=GREEN"
Write-Host "TRUER37 note=faces_present_glue_absent;not_wired;not_Thompson"
exit 0
