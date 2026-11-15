# stage-semtxt.ps1 — stock reads yoyo-compiler.ty text → EQUAL stock (no Node assemble on run)
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-SEMTXT E2E (stock reads YOYC .ty text) ==="

$codePath = Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\code.bin"
# Re-bake from 15596 baseline whenever requested / first tip
& node (Join-Path $Root "yoyo-js\scripts\bake-semtxt-into-code.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMTXT status=RED reason=bake"; exit 1 }
& node (Join-Path $Root "yoyo-js\scripts\build-place-stub.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMTXT status=RED reason=rebuild_stub"; exit 1 }

# Emit YOYC text SoT from (new) bins; refresh derived plant/Y1PL for pin
& node (Join-Path $Root "yoyo-js\scripts\assemble-compiler-ty.js") emit
if ($LASTEXITCODE -ne 0) { Write-Host "SEMTXT status=RED reason=emit"; exit 1 }
& node (Join-Path $Root "yoyo-js\scripts\assemble-compiler-ty.js") assemble
if ($LASTEXITCODE -ne 0) { Write-Host "SEMTXT status=RED reason=assemble_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMTXT status=RED reason=place_stub_pin"; exit 1 }
& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMTXT status=RED reason=bins_pin"; exit 1 }
& node (Join-Path $Root "yoyo-js\scripts\verify-code-derived-from-plant.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMTXT status=RED reason=code_derive"; exit 1 }

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$placeOut = Join-Path $Root "scripts\_stage-semtxt\place-gate"
if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub -ForceDerive
if ($LASTEXITCODE -ne 0) { Write-Host "SEMTXT status=RED reason=place"; exit 1 }

$stock = Join-Path $placeOut "yoyo_compiler.exe"
$stockHash = (Get-FileHash $stock -Algorithm SHA256).Hash.ToLowerInvariant()
Write-Host ("SEMTXT stock_sha256_prefix={0} bytes={1}" -f $stockHash.Substring(0, 16), (Get-Item $stock).Length)

$T = Join-Path $Root "yoyo\projects\yoyo-compiler.ty"
$runDir = Join-Path $Root "scripts\_stage-semtxt\run"
if (Test-Path $runDir) { Remove-Item -Recurse -Force $runDir }
New-Item -Force -ItemType Directory $runDir | Out-Null
Copy-Item $stock (Join-Path $runDir "yoyo_compiler.exe")
# Feed TEXT SoT as input.tyb (content is YOYC text — stock parses natively)
Copy-Item $T (Join-Path $runDir "input.tyb")
Write-Host "SEMTXT gate_node_assemble=ABSENT"
$ec = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $runDir "yoyo_compiler.exe") -WorkingDirectory $runDir -TimeoutSec 180
$out = Join-Path $runDir "output.exe"
if ($ec -ne 0 -or -not (Test-Path $out)) {
  Write-Host ("SEMTXT status=RED reason=stock_run_ty exit={0}" -f $ec)
  exit 1
}
$outHash = (Get-FileHash $out -Algorithm SHA256).Hash.ToLowerInvariant()
if ($outHash -ne $stockHash) {
  Write-Host "SEMTXT status=RED reason=stock_ty_ne_stock"
  exit 1
}
Write-Host "SEMTXT stock_reads_ty=EQUAL"

$hop = Join-Path $Root "scripts\_stage-semtxt\hop2"
if (Test-Path $hop) { Remove-Item -Recurse -Force $hop }
New-Item -Force -ItemType Directory $hop | Out-Null
Copy-Item $out (Join-Path $hop "yoyo_compiler.exe")
Copy-Item $T (Join-Path $hop "input.tyb")
$ec2 = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $hop "yoyo_compiler.exe") -WorkingDirectory $hop -TimeoutSec 180
$out2 = Join-Path $hop "output.exe"
if ($ec2 -ne 0 -or -not (Test-Path $out2)) {
  Write-Host ("SEMTXT status=RED reason=hop2_run exit={0}" -f $ec2)
  exit 1
}
$h2 = (Get-FileHash $out2 -Algorithm SHA256).Hash.ToLowerInvariant()
if ($h2 -ne $stockHash) {
  Write-Host "SEMTXT status=RED reason=hop2_DIFF"
  exit 1
}
Write-Host "SEMTXT hop2=EQUAL"

$cDir = Join-Path $Root "scripts\_stage-semtxt\compact"
if (Test-Path $cDir) { Remove-Item -Recurse -Force $cDir }
New-Item -Force -ItemType Directory $cDir | Out-Null
Copy-Item $stock (Join-Path $cDir "yoyo_compiler.exe")
Copy-Item (Join-Path $Root "yoyo\projects\yoyo-compact-ret.tyb") (Join-Path $cDir "input.tyb")
$ecC = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $cDir "yoyo_compiler.exe") -WorkingDirectory $cDir -TimeoutSec 60
$cOut = Join-Path $cDir "output.exe"
if ($ecC -ne 0 -or -not (Test-Path $cOut)) {
  Write-Host ("SEMTXT status=RED reason=compact_missing exit={0}" -f $ecC)
  exit 1
}
$cb = [IO.File]::ReadAllBytes($cOut)
if ($cb.Length -lt 2 -or $cb[0] -ne 0x4d -or $cb[1] -ne 0x5a) {
  Write-Host "SEMTXT status=RED reason=compact_not_mz"
  exit 1
}
Write-Host ("SEMTXT compact_out=PRESENT bytes={0}" -f $cb.Length)

& (Join-Path $Root "scripts\stage-c1-native.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMTXT status=RED reason=c1_native"; exit 1 }

Write-Host "SEMTXT compiler_src_sot=yoyo-compiler.ty"
Write-Host "SEMTXT doc=AUTO-SEMTXT.md"
Write-Host "SEMTXT status=GREEN"
Write-Host "SEMTXT note=stock_reads_yoyc_ty_text;no_gate_assemble_on_run;not_Thompson"
exit 0
