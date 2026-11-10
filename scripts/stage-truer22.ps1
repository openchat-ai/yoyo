# stage-truer22.ps1 — AUTO-TRUER22: product sem faces source-driven by yoyo.tyb
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-TRUER22 E2E (product sem faces source-driven) ==="

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER22 status=RED reason=place_stub_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER22 status=RED reason=bins_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-plant-sot.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER22 status=RED reason=plant_sot"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-code-derived-from-plant.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER22 status=RED reason=code_derive_auth"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-product-sem-faces-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER22 status=RED reason=product_sem_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-product-sem-faces-source-driven-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER22 status=RED reason=source_driven_pin"; exit 1 }

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$placeOut = Join-Path $Root "scripts\_stage-truer22\place-gate"
if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }

& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub -ForceDerive
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER22 status=RED reason=place_force_derive"; exit 1 }
$c1 = Join-Path $placeOut "yoyo_compiler.exe"
if (-not (Test-Path $c1)) { Write-Host "TRUER22 status=RED reason=missing_c1"; exit 1 }

# Base product: pristine yoyo.tyb
$baseDir = Join-Path $Root "scripts\_stage-truer22\product-base"
if (Test-Path $baseDir) { Remove-Item -Recurse -Force $baseDir }
New-Item -Force -ItemType Directory $baseDir | Out-Null
Copy-Item $c1 (Join-Path $baseDir "yoyo_compiler.exe")
Copy-Item (Join-Path $Root "yoyo\projects\yoyo.tyb") (Join-Path $baseDir "input.tyb")
$ecBase = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $baseDir "yoyo_compiler.exe") -WorkingDirectory $baseDir -TimeoutSec 300
$baseOut = Join-Path $baseDir "output.exe"
if ($ecBase -ne 0 -or -not (Test-Path $baseOut)) {
  Write-Host ("TRUER22 status=RED reason=base_compile exit={0}" -f $ecBase)
  exit 1
}
& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-product-hosts-sem-faces.js") $baseOut
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER22 status=RED reason=base_faces"; exit 1 }
Write-Host "TRUER22 base_faces=14/14"

# Mut product: flip locked IMUL record 63→6A
$mutDir = Join-Path $Root "scripts\_stage-truer22\product-mut"
if (Test-Path $mutDir) { Remove-Item -Recurse -Force $mutDir }
New-Item -Force -ItemType Directory $mutDir | Out-Null
Copy-Item $c1 (Join-Path $mutDir "yoyo_compiler.exe")
$mutTyb = Join-Path $mutDir "input.tyb"
$bytes = [IO.File]::ReadAllBytes((Join-Path $Root "yoyo\projects\yoyo.tyb"))
$mutOff = 1736
if ($bytes[$mutOff] -ne 0x63) {
  Write-Host ("TRUER22 status=RED reason=mut_anchor_drift off={0} got={1:X2}" -f $mutOff, $bytes[$mutOff])
  exit 1
}
$bytes[$mutOff] = 0x6A
[IO.File]::WriteAllBytes($mutTyb, $bytes)
$ecMut = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $mutDir "yoyo_compiler.exe") -WorkingDirectory $mutDir -TimeoutSec 300
$mutOut = Join-Path $mutDir "output.exe"
if ($ecMut -ne 0 -or -not (Test-Path $mutOut)) {
  Write-Host ("TRUER22 status=RED reason=mut_compile exit={0}" -f $ecMut)
  exit 1
}
Write-Host "TRUER22 mut_tyb_off=1736 from=63 to=6a"

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-product-sem-faces-source-driven.js") $baseOut $mutOut
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER22 status=RED reason=source_driven"; exit 1 }

& (Join-Path $Root "scripts\stage-truer5-semantic.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER22 status=RED reason=semantic"; exit 1 }

& (Join-Path $Root "scripts\stage-c1-native.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER22 status=RED reason=c1_native"; exit 1 }

Write-Host "TRUER22 trust_root=place_stub"
Write-Host "TRUER22 yoyo_product_hosts_sem_emit_faces=PRESENT"
Write-Host "TRUER22 yoyo_product_sem_faces_source_driven=PRESENT"
Write-Host "TRUER22 mut_face_miss=yoyo-sem-imul"
Write-Host "TRUER22 doc=AUTO-TRUER22.md"
Write-Host "TRUER22 status=GREEN"
Write-Host "TRUER22 note=faces_source_driven;handler_pe_not_compiler;plant_body_still_a1;not_Thompson"
exit 0
