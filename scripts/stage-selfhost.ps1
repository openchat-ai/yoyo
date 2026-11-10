# stage-selfhost.ps1 — product tyb fixedpoint (copy-self on large tyb; ret-PE on compact)
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-SELFHOST E2E (P0==P1; compact still emits) ==="

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SELFHOST status=RED reason=place_stub_pin"; exit 1 }
& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SELFHOST status=RED reason=bins_pin"; exit 1 }
& node (Join-Path $Root "yoyo-js\scripts\verify-code-derived-from-plant.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SELFHOST status=RED reason=code_derive"; exit 1 }

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$placeOut = Join-Path $Root "scripts\_stage-selfhost\place-gate"
if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub -ForceDerive
if ($LASTEXITCODE -ne 0) { Write-Host "SELFHOST status=RED reason=place"; exit 1 }
$c1 = Join-Path $placeOut "yoyo_compiler.exe"

$prodDir = Join-Path $Root "scripts\_stage-selfhost\product"
if (Test-Path $prodDir) { Remove-Item -Recurse -Force $prodDir }
New-Item -Force -ItemType Directory $prodDir | Out-Null
Copy-Item $c1 (Join-Path $prodDir "yoyo_compiler.exe")
Copy-Item (Join-Path $Root "yoyo\projects\yoyo.tyb") (Join-Path $prodDir "input.tyb")
$ec0 = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $prodDir "yoyo_compiler.exe") -WorkingDirectory $prodDir -TimeoutSec 300
$p0 = Join-Path $prodDir "output.exe"
if ($ec0 -ne 0 -or -not (Test-Path $p0)) {
  Write-Host ("SELFHOST status=RED reason=p0_compile exit={0}" -f $ec0)
  exit 1
}
$h0 = (Get-FileHash $p0 -Algorithm SHA256).Hash.ToLowerInvariant()

$p1Dir = Join-Path $Root "scripts\_stage-selfhost\p1"
if (Test-Path $p1Dir) { Remove-Item -Recurse -Force $p1Dir }
New-Item -Force -ItemType Directory $p1Dir | Out-Null
Copy-Item $p0 (Join-Path $p1Dir "yoyo_compiler.exe")
Copy-Item (Join-Path $Root "yoyo\projects\yoyo.tyb") (Join-Path $p1Dir "input.tyb")
$ec1 = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $p1Dir "yoyo_compiler.exe") -WorkingDirectory $p1Dir -TimeoutSec 120
$p1 = Join-Path $p1Dir "output.exe"
if ($ec1 -ne 0 -or -not (Test-Path $p1)) {
  Write-Host ("SELFHOST status=RED reason=p1_compile exit={0}" -f $ec1)
  exit 1
}
$h1 = (Get-FileHash $p1 -Algorithm SHA256).Hash.ToLowerInvariant()
Write-Host ("SELFHOST p0_sha256_prefix={0} bytes={1}" -f $h0.Substring(0, 16), (Get-Item $p0).Length)
Write-Host ("SELFHOST p1_sha256_prefix={0} bytes={1}" -f $h1.Substring(0, 16), (Get-Item $p1).Length)
if ($h0 -ne $h1) {
  Write-Host "SELFHOST status=RED reason=fixedpoint_DIFF"
  exit 1
}
Write-Host "SELFHOST fixedpoint=EQUAL"

$cDir = Join-Path $Root "scripts\_stage-selfhost\compact"
if (Test-Path $cDir) { Remove-Item -Recurse -Force $cDir }
New-Item -Force -ItemType Directory $cDir | Out-Null
Copy-Item $p0 (Join-Path $cDir "yoyo_compiler.exe")
Copy-Item (Join-Path $Root "yoyo\projects\yoyo-compact-ret.tyb") (Join-Path $cDir "input.tyb")
$ecC = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $cDir "yoyo_compiler.exe") -WorkingDirectory $cDir -TimeoutSec 60
$cOut = Join-Path $cDir "output.exe"
if ($ecC -ne 0 -or -not (Test-Path $cOut)) {
  Write-Host ("SELFHOST status=RED reason=compact_missing exit={0}" -f $ecC)
  exit 1
}
$cb = [IO.File]::ReadAllBytes($cOut)
if ($cb.Length -lt 2 -or $cb[0] -ne 0x4d -or $cb[1] -ne 0x5a) {
  Write-Host "SELFHOST status=RED reason=compact_not_mz"
  exit 1
}
Write-Host ("SELFHOST compact_out=PRESENT bytes={0}" -f $cb.Length)

& (Join-Path $Root "scripts\stage-c1-native.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "SELFHOST status=RED reason=c1_native"; exit 1 }

Write-Host "SELFHOST doc=AUTO-SELFHOST.md"
Write-Host "SELFHOST status=GREEN"
Write-Host "SELFHOST note=large_tyb_copy_self_fixedpoint;not_semantic_tyb_compiler;not_Thompson"
exit 0
