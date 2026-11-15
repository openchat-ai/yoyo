# stage-semself.ps1 — C1 semantic plant-tyb → recover code.bin → link self; hop2 EQUAL
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-SEMSELF E2E (plant tyb semantic → code → self; hop2) ==="

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMSELF status=RED reason=place_stub_pin"; exit 1 }
& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMSELF status=RED reason=bins_pin"; exit 1 }
& node (Join-Path $Root "yoyo-js\scripts\verify-code-derived-from-plant.js")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMSELF status=RED reason=code_derive"; exit 1 }

$pinPath = Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\payload.pin"
$pin = @{}
Get-Content $pinPath | ForEach-Object {
  if ($_ -match '^\s*([^=]+)=(.*)$') { $pin[$matches[1].Trim()] = $matches[2].Trim() }
}
$placeBytes = [int]$pin["code_place_bytes"]
$parse1Bytes = [int]$pin["code_parse1_bytes"]
if ($placeBytes -le 0 -or $parse1Bytes -le 0) {
  Write-Host "SEMSELF status=RED reason=pin_place_parse1_bytes"
  exit 1
}

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$placeOut = Join-Path $Root "scripts\_stage-semself\place-gate"
if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub -ForceDerive
if ($LASTEXITCODE -ne 0) { Write-Host "SEMSELF status=RED reason=place"; exit 1 }
$c1 = Join-Path $placeOut "yoyo_compiler.exe"
$c1Hash = (Get-FileHash $c1 -Algorithm SHA256).Hash.ToLowerInvariant()

function Invoke-C1Tyb {
  param([string]$Compiler, [string]$Tyb, [string]$WorkDir)
  if (Test-Path $WorkDir) { Remove-Item -Recurse -Force $WorkDir }
  New-Item -Force -ItemType Directory $WorkDir | Out-Null
  Copy-Item $Compiler (Join-Path $WorkDir "yoyo_compiler.exe")
  Copy-Item $Tyb (Join-Path $WorkDir "input.tyb")
  $ec = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $WorkDir "yoyo_compiler.exe") -WorkingDirectory $WorkDir -TimeoutSec 180
  $out = Join-Path $WorkDir "output.exe"
  if ($ec -ne 0 -or -not (Test-Path $out)) {
    throw ("compile_fail exit={0} tyb={1}" -f $ec, $Tyb)
  }
  return $out
}

function Get-TextPayload {
  param([string]$PePath, [int]$ExpectLen)
  $bytes = [IO.File]::ReadAllBytes($PePath)
  $e = [BitConverter]::ToInt32($bytes, 0x3c)
  $optSize = [BitConverter]::ToInt16($bytes, $e + 20)
  $sec = $e + 24 + $optSize
  $rs = [BitConverter]::ToInt32($bytes, $sec + 16)
  $rp = [BitConverter]::ToInt32($bytes, $sec + 20)
  if ($rp + 13 + $ExpectLen -gt $bytes.Length -or $rs -lt 13 + $ExpectLen) {
    throw ("text_short pe={0} rs={1} need={2}" -f $PePath, $rs, (13 + $ExpectLen))
  }
  $buf = New-Object byte[] $ExpectLen
  [Array]::Copy($bytes, $rp + 13, $buf, 0, $ExpectLen)
  return $buf
}

try {
  $pePlace = Invoke-C1Tyb -Compiler $c1 -Tyb (Join-Path $Root "yoyo\projects\yoyo-plant-place.tyb") -WorkDir (Join-Path $Root "scripts\_stage-semself\plant-place")
  $peParse1 = Invoke-C1Tyb -Compiler $c1 -Tyb (Join-Path $Root "yoyo\projects\yoyo-plant-parse1.tyb") -WorkDir (Join-Path $Root "scripts\_stage-semself\plant-parse1")
} catch {
  Write-Host ("SEMSELF status=RED reason=plant_compile detail={0}" -f $_.Exception.Message)
  exit 1
}

$halfPlace = Get-TextPayload -PePath $pePlace -ExpectLen $placeBytes
$halfParse1 = Get-TextPayload -PePath $peParse1 -ExpectLen $parse1Bytes
$codePath = Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\code.bin"
$code = [IO.File]::ReadAllBytes($codePath)
$recovered = New-Object byte[] ($placeBytes + $parse1Bytes)
[Array]::Copy($halfPlace, 0, $recovered, 0, $placeBytes)
[Array]::Copy($halfParse1, 0, $recovered, $placeBytes, $parse1Bytes)

$recoverEqual = $true
if ($recovered.Length -ne $code.Length) { $recoverEqual = $false }
else {
  for ($i = 0; $i -lt $code.Length; $i++) {
    if ($recovered[$i] -ne $code[$i]) { $recoverEqual = $false; break }
  }
}
if (-not $recoverEqual) {
  Write-Host "SEMSELF status=RED reason=plant_code_recover_DIFF"
  exit 1
}
Write-Host ("SEMSELF plant_code_recover=EQUAL place={0} parse1={1}" -f $placeBytes, $parse1Bytes)

$linkDir = Join-Path $Root "scripts\_stage-semself\link-self"
if (Test-Path $linkDir) { Remove-Item -Recurse -Force $linkDir }
New-Item -Force -ItemType Directory $linkDir | Out-Null
Copy-Item $c1 (Join-Path $linkDir "yoyo_compiler.exe")
[IO.File]::WriteAllBytes((Join-Path $linkDir "code.bin"), $recovered)
Copy-Item (Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\data.bin") (Join-Path $linkDir "data.bin")
$ecLink = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $linkDir "yoyo_compiler.exe") -WorkingDirectory $linkDir -TimeoutSec 120
$linked = Join-Path $linkDir "output.exe"
if ($ecLink -ne 0 -or -not (Test-Path $linked)) {
  Write-Host ("SEMSELF status=RED reason=link_self exit={0}" -f $ecLink)
  exit 1
}
$linkedHash = (Get-FileHash $linked -Algorithm SHA256).Hash.ToLowerInvariant()
Write-Host ("SEMSELF c1_sha256_prefix={0} bytes={1}" -f $c1Hash.Substring(0, 16), (Get-Item $c1).Length)
Write-Host ("SEMSELF linked_sha256_prefix={0} bytes={1}" -f $linkedHash.Substring(0, 16), (Get-Item $linked).Length)
if ($linkedHash -ne $c1Hash) {
  Write-Host "SEMSELF status=RED reason=self_repro_DIFF"
  exit 1
}
Write-Host "SEMSELF self_repro=EQUAL"

try {
  $hopPlace = Invoke-C1Tyb -Compiler $linked -Tyb (Join-Path $Root "yoyo\projects\yoyo-plant-place.tyb") -WorkDir (Join-Path $Root "scripts\_stage-semself\hop2-place")
  $hopParse1 = Invoke-C1Tyb -Compiler $linked -Tyb (Join-Path $Root "yoyo\projects\yoyo-plant-parse1.tyb") -WorkDir (Join-Path $Root "scripts\_stage-semself\hop2-parse1")
} catch {
  Write-Host ("SEMSELF status=RED reason=hop2_compile detail={0}" -f $_.Exception.Message)
  exit 1
}
$hPlace0 = (Get-FileHash $pePlace -Algorithm SHA256).Hash.ToLowerInvariant()
$hPlace1 = (Get-FileHash $hopPlace -Algorithm SHA256).Hash.ToLowerInvariant()
$hParse0 = (Get-FileHash $peParse1 -Algorithm SHA256).Hash.ToLowerInvariant()
$hParse1 = (Get-FileHash $hopParse1 -Algorithm SHA256).Hash.ToLowerInvariant()
if ($hPlace0 -ne $hPlace1 -or $hParse0 -ne $hParse1) {
  Write-Host "SEMSELF status=RED reason=hop2_DIFF"
  Write-Host ("SEMSELF hop2_place {0} vs {1}" -f $hPlace0.Substring(0, 16), $hPlace1.Substring(0, 16))
  Write-Host ("SEMSELF hop2_parse1 {0} vs {1}" -f $hParse0.Substring(0, 16), $hParse1.Substring(0, 16))
  exit 1
}
Write-Host "SEMSELF hop2=EQUAL"

$cDir = Join-Path $Root "scripts\_stage-semself\compact"
try {
  $cOut = Invoke-C1Tyb -Compiler $linked -Tyb (Join-Path $Root "yoyo\projects\yoyo-compact-ret.tyb") -WorkDir $cDir
} catch {
  Write-Host ("SEMSELF status=RED reason=compact_compile detail={0}" -f $_.Exception.Message)
  exit 1
}
$cb = [IO.File]::ReadAllBytes($cOut)
if ($cb.Length -lt 2 -or $cb[0] -ne 0x4d -or $cb[1] -ne 0x5a) {
  Write-Host "SEMSELF status=RED reason=compact_not_mz"
  exit 1
}
Write-Host ("SEMSELF compact_out=PRESENT bytes={0}" -f $cb.Length)

& (Join-Path $Root "scripts\stage-c1-native.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "SEMSELF status=RED reason=c1_native"; exit 1 }

Write-Host "SEMSELF doc=AUTO-SEMSELF.md"
Write-Host "SEMSELF status=GREEN"
Write-Host "SEMSELF note=plant_tyb_semantic_recover_code_link_self;data_bin_pin_sot;not_28k_handler_product;not_Thompson"
exit 0
