# stage-truer26.ps1 — AUTO-TRUER26: product has io paths but cannot relay-compile
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-TRUER26 E2E (product relay ABSENT) ==="

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER26 status=RED reason=place_stub_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER26 status=RED reason=bins_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-assemble.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER26 status=RED reason=assemble"; exit 1 }

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$placeOut = Join-Path $Root "scripts\_stage-truer26\place-gate"
if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub -ForceDerive
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER26 status=RED reason=place"; exit 1 }
$c1 = Join-Path $placeOut "yoyo_compiler.exe"

$prodDir = Join-Path $Root "scripts\_stage-truer26\product"
if (Test-Path $prodDir) { Remove-Item -Recurse -Force $prodDir }
New-Item -Force -ItemType Directory $prodDir | Out-Null
Copy-Item $c1 (Join-Path $prodDir "yoyo_compiler.exe")
Copy-Item (Join-Path $Root "yoyo\projects\yoyo.tyb") (Join-Path $prodDir "input.tyb")
$ecProd = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $prodDir "yoyo_compiler.exe") -WorkingDirectory $prodDir -TimeoutSec 300
$prodOut = Join-Path $prodDir "output.exe"
if ($ecProd -ne 0 -or -not (Test-Path $prodOut)) {
  Write-Host ("TRUER26 status=RED reason=product_compile exit={0}" -f $ecProd)
  exit 1
}

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-product-compiler-io-paths.js") $prodOut
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER26 status=RED reason=product_io"; exit 1 }

# Relay attempt: product-as-compiler × compact-ret must NOT emit output.exe
$relayDir = Join-Path $Root "scripts\_stage-truer26\relay"
if (Test-Path $relayDir) { Remove-Item -Recurse -Force $relayDir }
New-Item -Force -ItemType Directory $relayDir | Out-Null
Copy-Item $prodOut (Join-Path $relayDir "yoyo_compiler.exe")
Copy-Item (Join-Path $Root "yoyo\projects\yoyo-compact-ret.tyb") (Join-Path $relayDir "input.tyb")
"1" | Set-Content -Path (Join-Path $relayDir "_relay_attempted") -Encoding ascii
$ecRelay = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $relayDir "yoyo_compiler.exe") -WorkingDirectory $relayDir -TimeoutSec 60
$relayOut = Join-Path $relayDir "output.exe"
if (Test-Path $relayOut) {
  Write-Host ("TRUER26 status=RED reason=relay_unexpected_output exit={0} bytes={1}" -f $ecRelay, (Get-Item $relayOut).Length)
  exit 1
}
Write-Host ("TRUER26 relay_exit={0} output.exe=ABSENT" -f $ecRelay)

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-product-compiler-relay.js") $relayDir
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER26 status=RED reason=relay_pin"; exit 1 }

& (Join-Path $Root "scripts\stage-truer5-semantic.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER26 status=RED reason=semantic"; exit 1 }

& (Join-Path $Root "scripts\stage-c1-native.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER26 status=RED reason=c1_native"; exit 1 }

Write-Host "TRUER26 trust_root=place_stub"
Write-Host "TRUER26 yoyo_product_compiler_io_paths=PRESENT"
Write-Host "TRUER26 yoyo_product_compiler_relay=ABSENT"
Write-Host "TRUER26 doc=AUTO-TRUER26.md"
Write-Host "TRUER26 status=GREEN"
Write-Host "TRUER26 note=paths_present_relay_absent;need_open_loop;not_Thompson"
exit 0
