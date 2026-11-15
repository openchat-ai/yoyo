# stage-openloop.ps1 — AUTO-OPENLOOP: product open_loop + relay PRESENT
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-OPENLOOP E2E (H_00 open/read/write; relay PRESENT) ==="

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "OPENLOOP status=RED reason=place_stub_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "OPENLOOP status=RED reason=bins_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-code-derived-from-plant.js")
if ($LASTEXITCODE -ne 0) { Write-Host "OPENLOOP status=RED reason=code_derive"; exit 1 }

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$placeOut = Join-Path $Root "scripts\_stage-openloop\place-gate"
if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub -ForceDerive
if ($LASTEXITCODE -ne 0) { Write-Host "OPENLOOP status=RED reason=place"; exit 1 }
$c1 = Join-Path $placeOut "yoyo_compiler.exe"

$prodDir = Join-Path $Root "scripts\_stage-openloop\product"
if (Test-Path $prodDir) { Remove-Item -Recurse -Force $prodDir }
New-Item -Force -ItemType Directory $prodDir | Out-Null
Copy-Item $c1 (Join-Path $prodDir "yoyo_compiler.exe")
Copy-Item (Join-Path $Root "yoyo\projects\yoyo.tyb") (Join-Path $prodDir "input.tyb")
$ecProd = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $prodDir "yoyo_compiler.exe") -WorkingDirectory $prodDir -TimeoutSec 300
$prodOut = Join-Path $prodDir "output.exe"
if ($ecProd -ne 0 -or -not (Test-Path $prodOut)) {
  Write-Host ("OPENLOOP status=RED reason=product_compile exit={0}" -f $ecProd)
  exit 1
}

$relayDir = Join-Path $Root "scripts\_stage-openloop\relay"
if (Test-Path $relayDir) { Remove-Item -Recurse -Force $relayDir }
New-Item -Force -ItemType Directory $relayDir | Out-Null
Copy-Item $prodOut (Join-Path $relayDir "yoyo_compiler.exe")
Copy-Item (Join-Path $Root "yoyo\projects\yoyo-compact-ret.tyb") (Join-Path $relayDir "input.tyb")
$ecRelay = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $relayDir "yoyo_compiler.exe") -WorkingDirectory $relayDir -TimeoutSec 60
$relayOut = Join-Path $relayDir "output.exe"
Write-Host ("OPENLOOP relay_exit={0} has_output={1}" -f $ecRelay, (Test-Path $relayOut))
if (-not (Test-Path $relayOut)) {
  Write-Host "OPENLOOP status=RED reason=relay_missing_output"
  exit 1
}

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-product-open-loop-relay.js") $prodOut $relayOut
if ($LASTEXITCODE -ne 0) { Write-Host "OPENLOOP status=RED reason=verify"; exit 1 }

& (Join-Path $Root "scripts\stage-truer5-semantic.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "OPENLOOP status=RED reason=semantic"; exit 1 }

& (Join-Path $Root "scripts\stage-c1-native.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "OPENLOOP status=RED reason=c1_native"; exit 1 }

Write-Host "OPENLOOP trust_root=place_stub"
Write-Host "OPENLOOP yoyo_product_open_loop=PRESENT"
Write-Host "OPENLOOP yoyo_product_compiler_relay=PRESENT"
Write-Host "OPENLOOP doc=AUTO-OPENLOOP.md"
Write-Host "OPENLOOP status=GREEN"
Write-Host "OPENLOOP note=h00_open_read_write_ret_pe;relay_alive;not_full_tyb_semantics;not_Thompson"
exit 0
