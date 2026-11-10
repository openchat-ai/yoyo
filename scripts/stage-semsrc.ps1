# stage-semsrc.ps1 — high-level yoyo-compiler.ty SoT → stock compiler EQUAL
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-SEMSRC E2E (yoyo-compiler.ty → stock compiler) ==="

$ty = Join-Path $Root "yoyo\projects\yoyo-compiler.ty"
if (-not (Test-Path $ty)) {
  & node (Join-Path $Root "yoyo-js\scripts\assemble-compiler-ty.js") emit
  if ($LASTEXITCODE -ne 0) { Write-Host "SEMSRC status=RED reason=emit"; exit 1 }
}

& node (Join-Path $Root "yoyo-js\scripts\assemble-compiler-ty.js") check
if ($LASTEXITCODE -ne 0) { Write-Host "SEMSRC status=RED reason=ty_check"; exit 1 }

# Refresh derived Y1PL/plant from .ty (SoT); bins must stay EQUAL
& node (Join-Path $Root "yoyo-js\scripts\assemble-compiler-ty.js") assemble
if ($LASTEXITCODE -ne 0) { Write-Host "SEMSRC status=RED reason=assemble"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMSRC status=RED reason=place_stub_pin"; exit 1 }
& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMSRC status=RED reason=bins_pin"; exit 1 }
& node (Join-Path $Root "yoyo-js\scripts\verify-code-derived-from-plant.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMSRC status=RED reason=code_derive"; exit 1 }

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$placeOut = Join-Path $Root "scripts\_stage-semsrc\place-gate"
if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub -ForceDerive
if ($LASTEXITCODE -ne 0) { Write-Host "SEMSRC status=RED reason=place"; exit 1 }

$stock = Join-Path $placeOut "yoyo_compiler.exe"
$stockHash = (Get-FileHash $stock -Algorithm SHA256).Hash.ToLowerInvariant()
Write-Host ("SEMSRC stock_sha256_prefix={0} bytes={1}" -f $stockHash.Substring(0, 16), (Get-Item $stock).Length)

$T = Join-Path $Root "yoyo\projects\yoyo-semcomp-self.tyb"
$runDir = Join-Path $Root "scripts\_stage-semsrc\run"
if (Test-Path $runDir) { Remove-Item -Recurse -Force $runDir }
New-Item -Force -ItemType Directory $runDir | Out-Null
Copy-Item $stock (Join-Path $runDir "yoyo_compiler.exe")
Copy-Item $T (Join-Path $runDir "input.tyb")
$ec = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $runDir "yoyo_compiler.exe") -WorkingDirectory $runDir -TimeoutSec 180
$out = Join-Path $runDir "output.exe"
if ($ec -ne 0 -or -not (Test-Path $out)) {
  Write-Host ("SEMSRC status=RED reason=stock_run exit={0}" -f $ec)
  exit 1
}
$outHash = (Get-FileHash $out -Algorithm SHA256).Hash.ToLowerInvariant()
if ($outHash -ne $stockHash) {
  Write-Host "SEMSRC status=RED reason=stock_T_ne_stock"
  exit 1
}
Write-Host "SEMSRC stock_runs_T=EQUAL"

$hop = Join-Path $Root "scripts\_stage-semsrc\hop2"
if (Test-Path $hop) { Remove-Item -Recurse -Force $hop }
New-Item -Force -ItemType Directory $hop | Out-Null
Copy-Item $out (Join-Path $hop "yoyo_compiler.exe")
Copy-Item $T (Join-Path $hop "input.tyb")
$ec2 = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $hop "yoyo_compiler.exe") -WorkingDirectory $hop -TimeoutSec 180
$out2 = Join-Path $hop "output.exe"
if ($ec2 -ne 0 -or -not (Test-Path $out2)) {
  Write-Host ("SEMSRC status=RED reason=hop2_run exit={0}" -f $ec2)
  exit 1
}
$h2 = (Get-FileHash $out2 -Algorithm SHA256).Hash.ToLowerInvariant()
if ($h2 -ne $stockHash) {
  Write-Host "SEMSRC status=RED reason=hop2_DIFF"
  exit 1
}
Write-Host "SEMSRC hop2=EQUAL"

$cDir = Join-Path $Root "scripts\_stage-semsrc\compact"
if (Test-Path $cDir) { Remove-Item -Recurse -Force $cDir }
New-Item -Force -ItemType Directory $cDir | Out-Null
Copy-Item $stock (Join-Path $cDir "yoyo_compiler.exe")
Copy-Item (Join-Path $Root "yoyo\projects\yoyo-compact-ret.tyb") (Join-Path $cDir "input.tyb")
$ecC = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $cDir "yoyo_compiler.exe") -WorkingDirectory $cDir -TimeoutSec 60
$cOut = Join-Path $cDir "output.exe"
if ($ecC -ne 0 -or -not (Test-Path $cOut)) {
  Write-Host ("SEMSRC status=RED reason=compact_missing exit={0}" -f $ecC)
  exit 1
}
$cb = [IO.File]::ReadAllBytes($cOut)
if ($cb.Length -lt 2 -or $cb[0] -ne 0x4d -or $cb[1] -ne 0x5a) {
  Write-Host "SEMSRC status=RED reason=compact_not_mz"
  exit 1
}
Write-Host ("SEMSRC compact_out=PRESENT bytes={0}" -f $cb.Length)

# Prove SoT marker in pin
$pin = Get-Content (Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\payload.pin") -Raw
if ($pin -notmatch 'compiler_src_sot=yoyo/projects/yoyo-compiler\.ty') {
  Write-Host "SEMSRC status=RED reason=pin_missing_compiler_src_sot"
  exit 1
}
if ($pin -notmatch 'code_bin_role=derived_from_yoyo_compiler_ty') {
  Write-Host "SEMSRC status=RED reason=pin_code_bin_role"
  exit 1
}

& (Join-Path $Root "scripts\stage-semnative.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMSRC status=RED reason=semnative"; exit 1 }

Write-Host "SEMSRC compiler_src_sot=yoyo-compiler.ty"
Write-Host "SEMSRC doc=AUTO-SEMSRC.md"
Write-Host "SEMSRC status=GREEN"
Write-Host "SEMSRC note=ty_text_sot_assembles_to_y1pl;stock_times_T_equals_stock;A1_encoding;not_Thompson"
exit 0
