# regen-payload-from-c1.ps1 — regenerate code.bin/data.bin+pin from C1 fixture (no Node plant)
#
#   cd F:\yoyo
#   & .\scripts\regen-payload-from-c1.ps1
param(
    [string]$Bootstrap = ""
)
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

$payload = Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload"
$emit = Join-Path $Root "scripts\emit-bins-c1.ps1"
if (-not (Test-Path $emit)) { throw "missing emit-bins-c1.ps1" }

Write-Host "=== NP regen payload from C1 (no Node plant) ==="
Write-Host "NODE_PLANT status=ABSENT"
Write-Host "NODE_PLANT regen=c1_extract_self_pe"

& $emit -OutDir $payload -Bootstrap $Bootstrap
if ($LASTEXITCODE -ne 0) {
    Write-Host "NP_REGEN status=RED reason=emit_failed"
    exit 1
}

$codePath = Join-Path $payload "code.bin"
$dataPath = Join-Path $payload "data.bin"
$codeSha = (Get-FileHash $codePath -Algorithm SHA256).Hash.ToLowerInvariant()
$dataSha = (Get-FileHash $dataPath -Algorithm SHA256).Hash.ToLowerInvariant()
$codeBytes = (Get-Item $codePath).Length
$dataBytes = (Get-Item $dataPath).Length
$pin = @"
name=yoyo_compiler_payload
code_bytes=$codeBytes
data_bytes=$dataBytes
code_sha256=$codeSha
data_sha256=$dataSha
code_sha256_prefix=$($codeSha.Substring(0,16))
data_sha256_prefix=$($dataSha.Substring(0,16))
payload_origin=c1_extract_self_pe
peer_link=c1_native_linkPeWin32
chain_emitter=c1_try_place_bins
bins_emitter=c1_extract_self_pe
lineage=c1_fixture_pe
x6_native=PRESENT
bins_gate_policy=pin_locked
bins_sot=fixture_pe
trust_root=c1_fixture
node_plant=ABSENT
escape=ABSENT
note=TRUER_derived_bins;not_Thompson
doc=AUTO-TRUER.md
"@
Set-Content -Path (Join-Path $payload "payload.pin") -Value $pin.TrimEnd() -Encoding ascii

Write-Host ("NP_REGEN code_bytes={0} sha256_prefix={1}" -f $codeBytes, $codeSha.Substring(0, 16))
Write-Host ("NP_REGEN data_bytes={0} sha256_prefix={1}" -f $dataBytes, $dataSha.Substring(0, 16))
Write-Host "NP_REGEN node_plant=ABSENT"
Write-Host "NP_REGEN doc=AUTO-NO-NODE-PLANT.md"
Write-Host "NP_REGEN status=GREEN"
exit 0
