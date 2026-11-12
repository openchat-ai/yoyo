# validate-llmwrite-drift.ps1 — rebuild determinism + artifact pins + bitflip detect
param(
    [string]$Yoyo = "",
    [switch]$SkipBuild
)
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

if (-not $Yoyo) {
    $Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
}
if (-not (Test-Path $Yoyo)) {
    if ($SkipBuild) { throw "missing yoyo.exe" }
    Push-Location (Join-Path $Root "yoyo-rust")
    cargo build --release -p verifier
    if ($LASTEXITCODE -ne 0) { throw "verifier build failed" }
    Pop-Location
}

$ContractPath = Join-Path $Root "scripts\fixtures\anyprog\model-contract.json"
$Contract = Get-Content $ContractPath -Raw | ConvertFrom-Json
if ([int]$Contract.revision -lt 4) {
    throw "LLMDRIFT status=RED reason=contract_not_v4"
}
$Drift = $Contract.drift
$Pins = $Contract.artifact_pins
$Fix = Join-Path $Root ($Contract.samples_llm_dir -replace '/', '\')
$Work = Join-Path $Root "scripts\_stage-llmwrite-drift"
New-Item -ItemType Directory -Force -Path $Work | Out-Null

function Sha256File([string]$path) {
    return (Get-FileHash -LiteralPath $path -Algorithm SHA256).Hash.ToLowerInvariant()
}

function Link-Ty([string]$ty, [string]$exe) {
    if (Test-Path $exe) { Remove-Item -Force -LiteralPath $exe }
    & $Yoyo link --target=win32 $ty $exe | Out-Null
    if ($LASTEXITCODE -ne 0) { throw "LLMDRIFT link failed ty=$ty" }
    if (-not (Test-Path $exe)) { throw "LLMDRIFT missing exe=$exe" }
}

$inNames = @()
$Contract.surface_coverage.PSObject.Properties | ForEach-Object {
    if ($_.Value -eq 'in_contract') { $inNames += $_.Name }
}
$inNames = @($inNames | Sort-Object)
$N = [int]$Drift.rebuild_times
if ($N -lt 2) { $N = 2 }

foreach ($name in $inNames) {
    $want = [string]$Pins.$name
    if (-not $want) { throw "LLMDRIFT status=RED reason=artifact_pin_missing sample=$name" }
    $ty = Join-Path $Fix "$name.ty"
    if (-not (Test-Path $ty)) { throw "LLMDRIFT status=RED reason=sample_ty_missing sample=$name" }
    $hashes = @()
    for ($i = 0; $i -lt $N; $i++) {
        $exe = Join-Path $Work "$name-rebuild-$i.exe"
        Link-Ty $ty $exe
        $hashes += (Sha256File $exe)
    }
    $uniq = @($hashes | Select-Object -Unique)
    if ($uniq.Count -ne 1) {
        throw "LLMDRIFT status=RED reason=rebuild_nondeterministic sample=$name hashes=$($hashes -join ',')"
    }
    $got = $hashes[0]
    if ($got -ne $want.ToLowerInvariant()) {
        throw "LLMDRIFT status=RED reason=artifact_pin_DIFF sample=$name want=$want got=$got"
    }
    Write-Host "LLMDRIFT sample=$name rebuild=$N pin=EQUAL sha256_prefix=$($got.Substring(0,16)) status=GREEN"
}

# Source bitflip (semantic): XOR one nibble of first SET imm → must change artifact hash
$srcName = [string]$Drift.bitflip_source_sample
$srcTy = Join-Path $Fix "$srcName.ty"
$srcText = [IO.File]::ReadAllText($srcTy)
if ($srcText -notmatch '30 51 ([0-9A-F]{2})') {
    throw "LLMDRIFT status=RED reason=bitflip_source_no_set sample=$srcName"
}
$origImm = $Matches[1]
$nib = [Convert]::ToInt32($origImm, 16) -bxor 0x01
$newImm = ('{0:X2}' -f $nib)
$idx = $srcText.IndexOf("30 51 $origImm")
if ($idx -lt 0) { throw "LLMDRIFT status=RED reason=bitflip_source_imm_missing" }
$flippedText = $srcText.Substring(0, $idx) + "30 51 $newImm" + $srcText.Substring($idx + "30 51 $origImm".Length)
$flipTy = Join-Path $Work "bitflip-src.ty"
$flipExe = Join-Path $Work "bitflip-src.exe"
[IO.File]::WriteAllText($flipTy, $flippedText)
Link-Ty $flipTy $flipExe
$flipHash = Sha256File $flipExe
$baseHash = ([string]$Pins.$srcName).ToLowerInvariant()
if ($flipHash -eq $baseHash) {
    throw "LLMDRIFT status=RED reason=source_bitflip_silent sample=$srcName"
}
Write-Host "LLMDRIFT source_bitflip sample=$srcName imm $origImm->$newImm hash_changed status=GREEN"

# Invalid token bitflip must fail lex (contract fail-closed)
$badTy = Join-Path $Work "bitflip-bad.ty"
$badText = $srcText -replace '^40 00', '40 0G'
[IO.File]::WriteAllText($badTy, $badText)
$checkJs = Join-Path $Work "check-one.js"
@'
const fs = require("fs");
const path = require("path");
const root = process.argv[2];
const tyPath = process.argv[3];
const contract = JSON.parse(
  fs.readFileSync(path.join(root, "scripts", "fixtures", "anyprog", "model-contract.json"), "utf8")
);
const HEX2 = /^[0-9A-F]{2}$/;
const text = fs.readFileSync(tyPath, "utf8");
const errors = [];
const lines = text.split("\n");
if (lines.length && lines[lines.length - 1] === "") lines.pop();
if (lines[0] !== contract.entry_line) errors.push("entry_line_mismatch");
for (let i = 1; i < lines.length; i++) {
  const body = lines[i].slice(contract.body_indent.length);
  for (const t of body.trim().split(" ")) {
    if (!HEX2.test(t)) errors.push("token_not_upper_hex2");
  }
}
for (const t of lines[0].split(" ")) {
  if (!HEX2.test(t)) errors.push("token_not_upper_hex2_entry");
}
if (!errors.length) {
  console.log("LLMDRIFT status=RED reason=invalid_bitflip_not_caught");
  process.exit(1);
}
console.log("LLMDRIFT invalid_token_bitflip status=RED_as_expected errs=" + errors[0]);
process.exit(0);
'@ | Set-Content -LiteralPath $checkJs -Encoding ascii
& node $checkJs $Root $badTy
if ($LASTEXITCODE -ne 0) { throw "LLMDRIFT status=RED reason=invalid_token_bitflip_gate" }

# Artifact bitflip: mutate one byte → hash ≠ pin
$artName = [string]$Drift.bitflip_artifact_sample
$artExe = Join-Path $Work "$artName-rebuild-0.exe"
if (-not (Test-Path $artExe)) { throw "LLMDRIFT status=RED reason=artifact_for_bitflip_missing" }
$bytes = [IO.File]::ReadAllBytes($artExe)
if ($bytes.Length -lt 64) { throw "LLMDRIFT status=RED reason=artifact_too_small" }
$bytes[64] = $bytes[64] -bxor 0x01
$mut = Join-Path $Work "bitflip-art.exe"
[IO.File]::WriteAllBytes($mut, $bytes)
$mutHash = Sha256File $mut
$wantArt = ([string]$Pins.$artName).ToLowerInvariant()
if ($mutHash -eq $wantArt) {
    throw "LLMDRIFT status=RED reason=artifact_bitflip_silent"
}
Write-Host "LLMDRIFT artifact_bitflip sample=$artName pin_mismatch_detected status=GREEN"

Write-Host "LLMDRIFT samples=$($inNames.Count) rebuild=$N status=GREEN"
Write-Host "LLMDRIFT doc=AUTO-LLMWRITEDRIFT.md"
Write-Host "LLMDRIFT note=software_hash_integrity;not_SEU_silicon;not_Thompson"
exit 0
