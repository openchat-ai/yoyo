# stage-binspoison.ps1 — AUTO-BINSPOISON: poisoned bins change place C1
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root
. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-BINSPOISON (poison bins must DIFF place pin) ==="

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$code = Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\code.bin"
$data = Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\data.bin"
$work = Join-Path $Root "scripts\_stage-binspoison"
if (Test-Path $work) { Remove-Item -Recurse -Force $work }
New-Item -Force -ItemType Directory $work | Out-Null

# Baseline place must match pin
$base = Join-Path $work "base"
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $base -Bootstrap $stub -CodeBin $code -DataBin $data
if ($LASTEXITCODE -ne 0) { Write-Host "BINSPOISON status=RED reason=baseline_place"; exit 1 }
& node (Join-Path $Root "yoyo-js\scripts\verify-place-c1-pin.js") (Join-Path $base "yoyo_compiler.exe")
if ($LASTEXITCODE -ne 0) { Write-Host "BINSPOISON status=RED reason=baseline_pin"; exit 1 }

# Poison a copy of code.bin (flip late place/parse boundary byte)
$poisonCode = Join-Path $work "code_poison.bin"
Copy-Item -Force $code $poisonCode
$b = [IO.File]::ReadAllBytes($poisonCode)
$ix = [Math]::Min($b.Length - 1, 6000)
$b[$ix] = $b[$ix] -bxor 0xFF
[IO.File]::WriteAllBytes($poisonCode, $b)

$pod = Join-Path $work "poison_place"
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $pod -Bootstrap $stub -CodeBin $poisonCode -DataBin $data
if ($LASTEXITCODE -ne 0 -or -not (Test-Path (Join-Path $pod "yoyo_compiler.exe"))) {
  # AV/fail also counts as caught (not silent equal)
  Write-Host "BINSPOISON poison_place_fail_closed=OK"
  Write-Host "BINSPOISON note=bins_poison_breaks_or_diffs;not_Thompson"
  Write-Host "BINSPOISON status=GREEN"
  Write-Host "BINSPOISON doc=AUTO-BINSPOISON.md"
  exit 0
}

& node (Join-Path $Root "yoyo-js\scripts\verify-place-c1-pin.js") (Join-Path $pod "yoyo_compiler.exe")
if ($LASTEXITCODE -eq 0) {
  Write-Host "BINSPOISON status=RED reason=poison_still_pin_EQUAL"
  exit 1
}
Write-Host "BINSPOISON poison_c1_DIFF_pin=RED_OK"
Write-Host "BINSPOISON note=content_sot_detection;not_Thompson"
Write-Host "BINSPOISON status=GREEN"
Write-Host "BINSPOISON doc=AUTO-BINSPOISON.md"
exit 0
