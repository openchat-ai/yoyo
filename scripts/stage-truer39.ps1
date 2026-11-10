# stage-truer39.ps1 — AUTO-TRUER39: product hosts connected VA+glue+CF; open_loop still ABSENT
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-TRUER39 E2E (product hosts open-loop connected face) ==="

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER39 status=RED reason=place_stub_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER39 status=RED reason=bins_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-code-derived-from-plant.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER39 status=RED reason=code_derive"; exit 1 }

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$placeOut = Join-Path $Root "scripts\_stage-truer39\place-gate"
if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub -ForceDerive
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER39 status=RED reason=place"; exit 1 }
$c1 = Join-Path $placeOut "yoyo_compiler.exe"

$prodDir = Join-Path $Root "scripts\_stage-truer39\product"
if (Test-Path $prodDir) { Remove-Item -Recurse -Force $prodDir }
New-Item -Force -ItemType Directory $prodDir | Out-Null
Copy-Item $c1 (Join-Path $prodDir "yoyo_compiler.exe")
Copy-Item (Join-Path $Root "yoyo\projects\yoyo.tyb") (Join-Path $prodDir "input.tyb")
$ecProd = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $prodDir "yoyo_compiler.exe") -WorkingDirectory $prodDir -TimeoutSec 300
$prodOut = Join-Path $prodDir "output.exe"
if ($ecProd -ne 0 -or -not (Test-Path $prodOut)) {
  Write-Host ("TRUER39 status=RED reason=product_compile exit={0}" -f $ecProd)
  exit 1
}

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-product-open-loop-connected.js") $prodOut
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER39 status=RED reason=connected"; exit 1 }

$relayDir = Join-Path $Root "scripts\_stage-truer39\relay"
if (Test-Path $relayDir) { Remove-Item -Recurse -Force $relayDir }
New-Item -Force -ItemType Directory $relayDir | Out-Null
Copy-Item $prodOut (Join-Path $relayDir "yoyo_compiler.exe")
Copy-Item (Join-Path $Root "yoyo\projects\yoyo-compact-ret.tyb") (Join-Path $relayDir "input.tyb")
$ecRelay = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $relayDir "yoyo_compiler.exe") -WorkingDirectory $relayDir -TimeoutSec 60
Write-Host ("TRUER39 relay_exit={0} has_output={1}" -f $ecRelay, (Test-Path (Join-Path $relayDir "output.exe")))
if (Test-Path (Join-Path $relayDir "output.exe")) {
  Write-Host "TRUER39 status=RED reason=unexpected_relay"
  exit 1
}

& (Join-Path $Root "scripts\stage-truer5-semantic.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER39 status=RED reason=semantic"; exit 1 }

& (Join-Path $Root "scripts\stage-c1-native.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER39 status=RED reason=c1_native"; exit 1 }

Write-Host "TRUER39 trust_root=place_stub"
Write-Host "TRUER39 yoyo_product_open_loop_connected=PRESENT"
Write-Host "TRUER39 yoyo_product_open_loop=ABSENT"
Write-Host "TRUER39 yoyo_product_compiler_relay=ABSENT"
Write-Host "TRUER39 doc=AUTO-TRUER39.md"
Write-Host "TRUER39 status=GREEN"
Write-Host "TRUER39 note=connected_in_product_still_not_wired;not_Thompson"
exit 0
