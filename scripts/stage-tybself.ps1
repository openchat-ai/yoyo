# stage-tybself.ps1 — C1 semantic plant tyb → recover code+data → link self; hop2
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-TYBSELF E2E (code+data from plant tyb → self) ==="

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TYBSELF status=RED reason=place_stub_pin"; exit 1 }
& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TYBSELF status=RED reason=bins_pin"; exit 1 }
& node (Join-Path $Root "yoyo-js\scripts\verify-code-derived-from-plant.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TYBSELF status=RED reason=code_derive"; exit 1 }

$pinPath = Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\payload.pin"
$pin = @{}
Get-Content $pinPath | ForEach-Object {
  if ($_ -match '^\s*([^=]+)=(.*)$') { $pin[$matches[1].Trim()] = $matches[2].Trim() }
}
$placeBytes = [int]$pin["code_place_bytes"]
$parse1Bytes = [int]$pin["code_parse1_bytes"]
$dataChunkBytes = [int]$pin["data_plant_chunk_bytes"]
$dataChunks = [int]$pin["data_plant_chunks"]
if ($placeBytes -le 0 -or $parse1Bytes -le 0 -or $dataChunkBytes -le 0 -or $dataChunks -le 0) {
  Write-Host "TYBSELF status=RED reason=pin_plant_sizes"
  exit 1
}

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$placeOut = Join-Path $Root "scripts\_stage-tybself\place-gate"
if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub -ForceDerive
if ($LASTEXITCODE -ne 0) { Write-Host "TYBSELF status=RED reason=place"; exit 1 }
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

function Test-BytesEqual([byte[]]$a, [byte[]]$b) {
  if ($a.Length -ne $b.Length) { return $false }
  for ($i = 0; $i -lt $a.Length; $i++) { if ($a[$i] -ne $b[$i]) { return $false } }
  return $true
}

# Verify data plant tyb pin hashes (SoT present)
for ($h = 0; $h -lt $dataChunks; $h++) {
  $tybPath = Join-Path $Root ("yoyo\projects\yoyo-plant-data{0}.tyb" -f $h)
  if (-not (Test-Path $tybPath)) {
    Write-Host ("TYBSELF status=RED reason=missing_data_plant_tyb h={0}" -f $h)
    exit 1
  }
  $sha = (Get-FileHash $tybPath -Algorithm SHA256).Hash.ToLowerInvariant()
  $key = "data_plant_tyb{0}_sha256" -f $h
  if (-not $pin.ContainsKey($key) -or $pin[$key].ToLowerInvariant() -ne $sha) {
    Write-Host ("TYBSELF status=RED reason=data_plant_tyb_sha_DIFF h={0}" -f $h)
    exit 1
  }
}

try {
  $pePlace = Invoke-C1Tyb -Compiler $c1 -Tyb (Join-Path $Root "yoyo\projects\yoyo-plant-place.tyb") -WorkDir (Join-Path $Root "scripts\_stage-tybself\plant-place")
  $peParse1 = Invoke-C1Tyb -Compiler $c1 -Tyb (Join-Path $Root "yoyo\projects\yoyo-plant-parse1.tyb") -WorkDir (Join-Path $Root "scripts\_stage-tybself\plant-parse1")
  $dataPes = @()
  for ($h = 0; $h -lt $dataChunks; $h++) {
    $dataPes += Invoke-C1Tyb -Compiler $c1 -Tyb (Join-Path $Root ("yoyo\projects\yoyo-plant-data{0}.tyb" -f $h)) -WorkDir (Join-Path $Root ("scripts\_stage-tybself\plant-data{0}" -f $h))
  }
} catch {
  Write-Host ("TYBSELF status=RED reason=plant_compile detail={0}" -f $_.Exception.Message)
  exit 1
}

$halfPlace = Get-TextPayload -PePath $pePlace -ExpectLen $placeBytes
$halfParse1 = Get-TextPayload -PePath $peParse1 -ExpectLen $parse1Bytes
$codeRec = New-Object byte[] ($placeBytes + $parse1Bytes)
[Array]::Copy($halfPlace, 0, $codeRec, 0, $placeBytes)
[Array]::Copy($halfParse1, 0, $codeRec, $placeBytes, $parse1Bytes)

$dataRecList = New-Object System.Collections.Generic.List[byte]
for ($h = 0; $h -lt $dataChunks; $h++) {
  $chunk = Get-TextPayload -PePath $dataPes[$h] -ExpectLen $dataChunkBytes
  foreach ($b in $chunk) { [void]$dataRecList.Add($b) }
}
$dataRec = $dataRecList.ToArray()

$codePin = [IO.File]::ReadAllBytes((Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\code.bin"))
$dataPin = [IO.File]::ReadAllBytes((Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\data.bin"))
if (-not (Test-BytesEqual $codeRec $codePin)) {
  Write-Host "TYBSELF status=RED reason=code_from_tyb_DIFF"
  exit 1
}
if (-not (Test-BytesEqual $dataRec $dataPin)) {
  Write-Host ("TYBSELF status=RED reason=data_from_tyb_DIFF got={0} want={1}" -f $dataRec.Length, $dataPin.Length)
  exit 1
}
Write-Host ("TYBSELF bins_from_tyb=EQUAL code={0} data={1}" -f $codeRec.Length, $dataRec.Length)

# Link using ONLY recovered bins (no copy from payload data.bin)
$linkDir = Join-Path $Root "scripts\_stage-tybself\link-self"
if (Test-Path $linkDir) { Remove-Item -Recurse -Force $linkDir }
New-Item -Force -ItemType Directory $linkDir | Out-Null
Copy-Item $c1 (Join-Path $linkDir "yoyo_compiler.exe")
[IO.File]::WriteAllBytes((Join-Path $linkDir "code.bin"), $codeRec)
[IO.File]::WriteAllBytes((Join-Path $linkDir "data.bin"), $dataRec)
$ecLink = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $linkDir "yoyo_compiler.exe") -WorkingDirectory $linkDir -TimeoutSec 120
$linked = Join-Path $linkDir "output.exe"
if ($ecLink -ne 0 -or -not (Test-Path $linked)) {
  Write-Host ("TYBSELF status=RED reason=link_self exit={0}" -f $ecLink)
  exit 1
}
$linkedHash = (Get-FileHash $linked -Algorithm SHA256).Hash.ToLowerInvariant()
Write-Host ("TYBSELF c1_sha256_prefix={0} bytes={1}" -f $c1Hash.Substring(0, 16), (Get-Item $c1).Length)
Write-Host ("TYBSELF linked_sha256_prefix={0} bytes={1}" -f $linkedHash.Substring(0, 16), (Get-Item $linked).Length)
if ($linkedHash -ne $c1Hash) {
  Write-Host "TYBSELF status=RED reason=self_repro_DIFF"
  exit 1
}
Write-Host "TYBSELF self_repro=EQUAL"

# hop2: rebuilt compiler recompiles all plant tybs EQUAL
try {
  $hopPlace = Invoke-C1Tyb -Compiler $linked -Tyb (Join-Path $Root "yoyo\projects\yoyo-plant-place.tyb") -WorkDir (Join-Path $Root "scripts\_stage-tybself\hop2-place")
  $hopParse1 = Invoke-C1Tyb -Compiler $linked -Tyb (Join-Path $Root "yoyo\projects\yoyo-plant-parse1.tyb") -WorkDir (Join-Path $Root "scripts\_stage-tybself\hop2-parse1")
  $hopData = @()
  for ($h = 0; $h -lt $dataChunks; $h++) {
    $hopData += Invoke-C1Tyb -Compiler $linked -Tyb (Join-Path $Root ("yoyo\projects\yoyo-plant-data{0}.tyb" -f $h)) -WorkDir (Join-Path $Root ("scripts\_stage-tybself\hop2-data{0}" -f $h))
  }
} catch {
  Write-Host ("TYBSELF status=RED reason=hop2_compile detail={0}" -f $_.Exception.Message)
  exit 1
}

function Assert-PeEqual([string]$a, [string]$b, [string]$tag) {
  $ha = (Get-FileHash $a -Algorithm SHA256).Hash.ToLowerInvariant()
  $hb = (Get-FileHash $b -Algorithm SHA256).Hash.ToLowerInvariant()
  if ($ha -ne $hb) {
    Write-Host ("TYBSELF status=RED reason=hop2_DIFF tag={0}" -f $tag)
    exit 1
  }
}
Assert-PeEqual $pePlace $hopPlace "place"
Assert-PeEqual $peParse1 $hopParse1 "parse1"
for ($h = 0; $h -lt $dataChunks; $h++) {
  Assert-PeEqual $dataPes[$h] $hopData[$h] ("data{0}" -f $h)
}
Write-Host "TYBSELF hop2=EQUAL"

$cDir = Join-Path $Root "scripts\_stage-tybself\compact"
try {
  $cOut = Invoke-C1Tyb -Compiler $linked -Tyb (Join-Path $Root "yoyo\projects\yoyo-compact-ret.tyb") -WorkDir $cDir
} catch {
  Write-Host ("TYBSELF status=RED reason=compact_compile detail={0}" -f $_.Exception.Message)
  exit 1
}
$cb = [IO.File]::ReadAllBytes($cOut)
if ($cb.Length -lt 2 -or $cb[0] -ne 0x4d -or $cb[1] -ne 0x5a) {
  Write-Host "TYBSELF status=RED reason=compact_not_mz"
  exit 1
}
Write-Host ("TYBSELF compact_out=PRESENT bytes={0}" -f $cb.Length)

& (Join-Path $Root "scripts\stage-c1-native.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TYBSELF status=RED reason=c1_native"; exit 1 }

Write-Host "TYBSELF doc=AUTO-TYBSELF.md"
Write-Host "TYBSELF status=GREEN"
Write-Host "TYBSELF note=code_and_data_from_plant_tyb_c1_semantic;link_self;multi_chunk;not_single_tyb_quine;not_Thompson"
exit 0
