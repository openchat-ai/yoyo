# stage-truer21.ps1 — AUTO-TRUER21: YOYO product hosts semantic emit faces
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-TRUER21 E2E (product hosts sem emit faces) ==="

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER21 status=RED reason=place_stub_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER21 status=RED reason=bins_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-plant-sot.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER21 status=RED reason=plant_sot"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-code-derived-from-plant.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER21 status=RED reason=code_derive_auth"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-product-sem-faces-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER21 status=RED reason=product_sem_pin"; exit 1 }

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$placeOut = Join-Path $Root "scripts\_stage-truer21\place-gate"
if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }

& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub -ForceDerive
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER21 status=RED reason=place_force_derive"; exit 1 }
$c1 = Join-Path $placeOut "yoyo_compiler.exe"
if (-not (Test-Path $c1)) { Write-Host "TRUER21 status=RED reason=missing_c1"; exit 1 }
Write-Host "TRUER21 place_force_derive=ON"

$prodDir = Join-Path $Root "scripts\_stage-truer21\product"
if (Test-Path $prodDir) { Remove-Item -Recurse -Force $prodDir }
New-Item -Force -ItemType Directory $prodDir | Out-Null
Copy-Item $c1 (Join-Path $prodDir "yoyo_compiler.exe")
Copy-Item (Join-Path $Root "yoyo\projects\yoyo.tyb") (Join-Path $prodDir "input.tyb")
$ec = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $prodDir "yoyo_compiler.exe") -WorkingDirectory $prodDir -TimeoutSec 300
$prodOut = Join-Path $prodDir "output.exe"
if ($ec -ne 0 -or -not (Test-Path $prodOut)) {
  Write-Host ("TRUER21 status=RED reason=yoyo_tyb_compile exit={0}" -f $ec)
  exit 1
}
$mz = [IO.File]::ReadAllBytes($prodOut)[0..1]
if ($mz[0] -ne 0x4D -or $mz[1] -ne 0x5A) {
  Write-Host "TRUER21 status=RED reason=product_not_mz"
  exit 1
}
Write-Host ("TRUER21 product_pe_bytes={0}" -f (Get-Item $prodOut).Length)

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-product-hosts-sem-faces.js") $prodOut
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER21 status=RED reason=sem_faces"; exit 1 }

& (Join-Path $Root "scripts\stage-truer5-semantic.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER21 status=RED reason=semantic"; exit 1 }

& (Join-Path $Root "scripts\stage-c1-native.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER21 status=RED reason=c1_native"; exit 1 }

Write-Host "TRUER21 trust_root=place_stub"
Write-Host "TRUER21 yoyo_product_hosts_sem_emit_faces=PRESENT"
Write-Host "TRUER21 sem_faces_hit=14/14"
Write-Host "TRUER21 doc=AUTO-TRUER21.md"
Write-Host "TRUER21 status=GREEN"
Write-Host "TRUER21 note=product_hosts_faces;handler_pe_not_compiler;plant_body_still_a1;not_Thompson"
exit 0
