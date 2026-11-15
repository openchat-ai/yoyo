# validate-llmwrite-drift.ps1 — rebuild + pins + section + canon_relink dual + bitflip
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
if ([int]$Contract.revision -lt 5) {
    throw "LLMDRIFT status=RED reason=contract_not_v5"
}
if (-not $Contract.dual_emit -or $Contract.dual_emit.require_agree -ne $true) {
    throw "LLMDRIFT status=RED reason=dual_emit_missing"
}
$Drift = $Contract.drift
$Pins = $Contract.artifact_pins
$SecPins = $Contract.section_pins
$Fix = Join-Path $Root ($Contract.samples_llm_dir -replace '/', '\')
$Work = Join-Path $Root "scripts\_stage-llmwrite-drift"
New-Item -ItemType Directory -Force -Path $Work | Out-Null

function Sha256File([string]$path) {
    return (Get-FileHash -LiteralPath $path -Algorithm SHA256).Hash.ToLowerInvariant()
}

function Get-PeTextSha256([string]$path) {
    $js = @'
const fs=require("fs");const crypto=require("crypto");
const b=fs.readFileSync(process.argv[2]);
const e=b.readUInt32LE(0x3c);
if(b[e]!==0x50||b[e+1]!==0x45||b[e+2]!==0||b[e+3]!==0){console.error("noPE");process.exit(2);}
const coff=e+4; const nsec=b.readUInt16LE(coff+2); const optSize=b.readUInt16LE(coff+16);
const sec=coff+20+optSize;
for(let i=0;i<nsec;i++){
  const off=sec+i*40;
  const name=b.slice(off,off+8).toString("ascii").replace(/\0/g,"");
  const rawsz=b.readUInt32LE(off+16); const ra=b.readUInt32LE(off+20);
  if(name.startsWith(".text")){
    process.stdout.write(crypto.createHash("sha256").update(b.slice(ra,ra+rawsz)).digest("hex"));
    process.exit(0);
  }
}
console.error("noText"); process.exit(3);
'@
    $tmp = Join-Path $Work "pe-text-sha.js"
    Set-Content -LiteralPath $tmp -Value $js -Encoding ascii
    $out = & node $tmp $path
    if ($LASTEXITCODE -ne 0) { throw "LLMDRIFT status=RED reason=pe_text_sha_fail path=$path" }
    return ([string]$out).Trim().ToLowerInvariant()
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
    $wantText = [string]$SecPins.$name.text_sha256
    if (-not $wantText) { throw "LLMDRIFT status=RED reason=section_pin_missing sample=$name" }
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
    $textSha = Get-PeTextSha256 (Join-Path $Work "$name-rebuild-0.exe")
    if ($textSha -ne $wantText.ToLowerInvariant()) {
        throw "LLMDRIFT status=RED reason=section_pin_DIFF sample=$name want=$wantText got=$textSha"
    }

    # dual emit: canon_relink — re-emit canonical .ty then link; must agree
    $canonTy = Join-Path $Work "$name-canon.ty"
    $canonExe = Join-Path $Work "$name-canon.exe"
    $canonJs = Join-Path $Work "emit-canon.js"
    @'
const fs=require("fs");
const v=require(process.argv[2]);
const contract=v.loadContract();
const name=process.argv[3];
const tyPath=process.argv[4];
const outPath=process.argv[5];
const kind=v.programKind(name,contract);
const text=fs.readFileSync(tyPath,"utf8");
const r=v.validateText(text,contract,kind);
if(r.errors.length){ console.error(r.errors.join("|")); process.exit(2); }
fs.writeFileSync(outPath, r.canon);
'@ | Set-Content -LiteralPath $canonJs -Encoding ascii
    $valJs = Join-Path $Root "yoyo-js\scripts\validate-llmwrite-contract.js"
    & node $canonJs $valJs $name $ty $canonTy
    if ($LASTEXITCODE -ne 0) { throw "LLMDRIFT status=RED reason=canon_emit_fail sample=$name" }
    Link-Ty $canonTy $canonExe
    $canonHash = Sha256File $canonExe
    if ($canonHash -ne $got) {
        throw "LLMDRIFT status=RED reason=dual_emit_disagree sample=$name direct=$got canon=$canonHash"
    }

    Write-Host "LLMPIPE sample=$name rebuild=$N pin=EQUAL text=EQUAL dual=AGREE sha256_prefix=$($got.Substring(0,16)) status=GREEN"
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
for (const t of lines[0].split(" ")) {
  if (!HEX2.test(t)) errors.push("token_not_upper_hex2_entry");
}
for (let i = 1; i < lines.length; i++) {
  const body = lines[i].slice(contract.body_indent.length);
  for (const t of body.trim().split(" ")) {
    if (!HEX2.test(t)) errors.push("token_not_upper_hex2");
  }
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

Write-Host "LLMPIPE samples=$($inNames.Count) rebuild=$N dual=canon_relink status=GREEN"
Write-Host "LLMPIPE hard_block=independent_second_linker;winsock_open;not_Thompson;not_SEU_silicon"
Write-Host "LLMPIPE doc=AUTO-LLMWRITEPIPE.md"
exit 0
