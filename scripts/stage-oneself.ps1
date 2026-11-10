# stage-oneself.ps1 — P(T)=P oneshot selfhost (one tyb, one CreateProcess)
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-ONESELF E2E (P(T)=P; one tyb; one run) ==="

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "ONESELF status=RED reason=place_stub_pin"; exit 1 }
& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "ONESELF status=RED reason=bins_pin"; exit 1 }

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$placeOut = Join-Path $Root "scripts\_stage-oneself\place-gate"
if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub -ForceDerive
if ($LASTEXITCODE -ne 0) { Write-Host "ONESELF status=RED reason=place"; exit 1 }

$stock = Join-Path $placeOut "yoyo_compiler.exe"
$pDir = Join-Path $Root "scripts\_stage-oneself\product"
if (Test-Path $pDir) { Remove-Item -Recurse -Force $pDir }
New-Item -Force -ItemType Directory $pDir | Out-Null
$P = Join-Path $pDir "yoyo_compiler.exe"
& node (Join-Path $Root "yoyo-js\scripts\install-oneshot-self.js") $stock $P
if ($LASTEXITCODE -ne 0) { Write-Host "ONESELF status=RED reason=install"; exit 1 }

$T = Join-Path $Root "yoyo\projects\yoyo-oneshot-self.tyb"
if (-not (Test-Path $T)) { Write-Host "ONESELF status=RED reason=missing_T"; exit 1 }

$runDir = Join-Path $Root "scripts\_stage-oneself\run"
if (Test-Path $runDir) { Remove-Item -Recurse -Force $runDir }
New-Item -Force -ItemType Directory $runDir | Out-Null
Copy-Item $P (Join-Path $runDir "yoyo_compiler.exe")
Copy-Item $T (Join-Path $runDir "input.tyb")
$ec = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $runDir "yoyo_compiler.exe") -WorkingDirectory $runDir -TimeoutSec 120
$out = Join-Path $runDir "output.exe"
if ($ec -ne 0 -or -not (Test-Path $out)) {
  Write-Host ("ONESELF status=RED reason=oneshot_run exit={0}" -f $ec)
  exit 1
}
$hP = (Get-FileHash $P -Algorithm SHA256).Hash.ToLowerInvariant()
$hO = (Get-FileHash $out -Algorithm SHA256).Hash.ToLowerInvariant()
Write-Host ("ONESELF p_sha256_prefix={0} bytes={1}" -f $hP.Substring(0, 16), (Get-Item $P).Length)
Write-Host ("ONESELF out_sha256_prefix={0} bytes={1}" -f $hO.Substring(0, 16), (Get-Item $out).Length)
if ($hP -ne $hO) {
  Write-Host "ONESELF status=RED reason=P_T_ne_P"
  exit 1
}
Write-Host "ONESELF oneshot_self=EQUAL"

$hop = Join-Path $Root "scripts\_stage-oneself\hop2"
if (Test-Path $hop) { Remove-Item -Recurse -Force $hop }
New-Item -Force -ItemType Directory $hop | Out-Null
Copy-Item $out (Join-Path $hop "yoyo_compiler.exe")
Copy-Item $T (Join-Path $hop "input.tyb")
$ec2 = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $hop "yoyo_compiler.exe") -WorkingDirectory $hop -TimeoutSec 120
$out2 = Join-Path $hop "output.exe"
if ($ec2 -ne 0 -or -not (Test-Path $out2)) {
  Write-Host ("ONESELF status=RED reason=hop2_run exit={0}" -f $ec2)
  exit 1
}
$h2 = (Get-FileHash $out2 -Algorithm SHA256).Hash.ToLowerInvariant()
if ($h2 -ne $hP) {
  Write-Host "ONESELF status=RED reason=hop2_DIFF"
  exit 1
}
Write-Host "ONESELF hop2=EQUAL"

$cDir = Join-Path $Root "scripts\_stage-oneself\compact"
if (Test-Path $cDir) { Remove-Item -Recurse -Force $cDir }
New-Item -Force -ItemType Directory $cDir | Out-Null
Copy-Item $P (Join-Path $cDir "yoyo_compiler.exe")
Copy-Item (Join-Path $Root "yoyo\projects\yoyo-compact-ret.tyb") (Join-Path $cDir "input.tyb")
$ecC = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $cDir "yoyo_compiler.exe") -WorkingDirectory $cDir -TimeoutSec 60
$cOut = Join-Path $cDir "output.exe"
if ($ecC -ne 0 -or -not (Test-Path $cOut)) {
  Write-Host ("ONESELF status=RED reason=compact_missing exit={0}" -f $ecC)
  exit 1
}
$cb = [IO.File]::ReadAllBytes($cOut)
if ($cb.Length -lt 2 -or $cb[0] -ne 0x4d -or $cb[1] -ne 0x5a) {
  Write-Host "ONESELF status=RED reason=compact_not_mz"
  exit 1
}
Write-Host ("ONESELF compact_out=PRESENT bytes={0}" -f $cb.Length)

Write-Host "ONESELF doc=AUTO-ONESELF.md"
Write-Host "ONESELF status=GREEN"
Write-Host "ONESELF note=P_times_T_equals_P;y1sf_embed_pe;text_pad_trampoline;not_plant_semantic_compiler;not_Thompson"
exit 0
