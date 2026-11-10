# stage-truer42.ps1 — AUTO-TRUER42: install entry trampoline to connected; open_loop still ABSENT
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-TRUER42 E2E (install entry trampoline → connected) ==="

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER42 status=RED reason=place_stub_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER42 status=RED reason=bins_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-code-derived-from-plant.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER42 status=RED reason=code_derive"; exit 1 }

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$placeOut = Join-Path $Root "scripts\_stage-truer42\place-gate"
if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub -ForceDerive
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER42 status=RED reason=place"; exit 1 }
$c1 = Join-Path $placeOut "yoyo_compiler.exe"

$prodDir = Join-Path $Root "scripts\_stage-truer42\product"
if (Test-Path $prodDir) { Remove-Item -Recurse -Force $prodDir }
New-Item -Force -ItemType Directory $prodDir | Out-Null
Copy-Item $c1 (Join-Path $prodDir "yoyo_compiler.exe")
Copy-Item (Join-Path $Root "yoyo\projects\yoyo.tyb") (Join-Path $prodDir "input.tyb")
$ecProd = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $prodDir "yoyo_compiler.exe") -WorkingDirectory $prodDir -TimeoutSec 300
$prodOut = Join-Path $prodDir "output.exe"
if ($ecProd -ne 0 -or -not (Test-Path $prodOut)) {
  Write-Host ("TRUER42 status=RED reason=product_compile exit={0}" -f $ecProd)
  exit 1
}

$instDir = Join-Path $Root "scripts\_stage-truer42\installed"
if (Test-Path $instDir) { Remove-Item -Recurse -Force $instDir }
New-Item -Force -ItemType Directory $instDir | Out-Null
$instOut = Join-Path $instDir "output.exe"
& node (Join-Path $Root "yoyo-js\scripts\install-yoyo-product-entry-trampoline.js") $prodOut $instOut
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER42 status=RED reason=install"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-product-open-loop-entry-trampoline-installed.js") $instOut
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER42 status=RED reason=installed_verify"; exit 1 }

$relayDir = Join-Path $Root "scripts\_stage-truer42\relay"
if (Test-Path $relayDir) { Remove-Item -Recurse -Force $relayDir }
New-Item -Force -ItemType Directory $relayDir | Out-Null
Copy-Item $instOut (Join-Path $relayDir "yoyo_compiler.exe")
Copy-Item (Join-Path $Root "yoyo\projects\yoyo-compact-ret.tyb") (Join-Path $relayDir "input.tyb")
$ecRelay = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $relayDir "yoyo_compiler.exe") -WorkingDirectory $relayDir -TimeoutSec 60
Write-Host ("TRUER42 relay_exit={0} has_output={1}" -f $ecRelay, (Test-Path (Join-Path $relayDir "output.exe")))
if (Test-Path (Join-Path $relayDir "output.exe")) {
  Write-Host "TRUER42 status=RED reason=unexpected_relay"
  exit 1
}

& (Join-Path $Root "scripts\stage-truer5-semantic.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER42 status=RED reason=semantic"; exit 1 }

& (Join-Path $Root "scripts\stage-c1-native.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER42 status=RED reason=c1_native"; exit 1 }

Write-Host "TRUER42 trust_root=place_stub"
Write-Host "TRUER42 yoyo_product_open_loop_entry_trampoline_installed=PRESENT"
Write-Host "TRUER42 yoyo_product_open_loop_entry_wired=PRESENT"
Write-Host "TRUER42 yoyo_product_open_loop=ABSENT"
Write-Host "TRUER42 yoyo_product_compiler_relay=ABSENT"
Write-Host "TRUER42 install_kind=post_emit_pe_poke"
Write-Host "TRUER42 doc=AUTO-TRUER42.md"
Write-Host "TRUER42 status=GREEN"
Write-Host "TRUER42 note=entry_jmp_to_connected_not_full_open_loop;not_Thompson"
exit 0
