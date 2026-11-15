# stage-truer7.ps1 — AUTO-TRUER7 E2E (trimmed data.bin + patched place contract)
#
#   cd F:\yoyo
#   & .\scripts\stage-truer7.ps1
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== AUTO-TRUER7 E2E ==="

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) {
    Write-Host "TRUER7 status=RED reason=place_stub_pin"
    exit 1
}

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) {
    Write-Host "TRUER7 status=RED reason=bins_pin"
    exit 1
}

& node (Join-Path $Root "yoyo-js\scripts\verify-no-sidecar-face.js")
if ($LASTEXITCODE -ne 0) {
    Write-Host "TRUER7 status=RED reason=sidecar_name_face"
    exit 1
}

$dataBytes = (Get-Item (Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\data.bin")).Length
if ($dataBytes -ne 414016) {
    Write-Host ("TRUER7 status=RED reason=data_not_trimmed got={0} want=414016" -f $dataBytes)
    exit 1
}
$c1Bytes = (Get-Item (Join-Path $Root "scripts\fixtures\yoyo_compiler.exe")).Length
if ($c1Bytes -ne 438784) {
    Write-Host ("TRUER7 status=RED reason=c1_pe_not_shrunk got={0} want=438784" -f $c1Bytes)
    exit 1
}
Write-Host "TRUER7 data_bytes=414016"
Write-Host "TRUER7 data_trim_saved=7952"
Write-Host "TRUER7 full_c1_pe_bytes=438784"

& (Join-Path $Root "scripts\stage-truer5-semantic.ps1")
if ($LASTEXITCODE -ne 0) {
    Write-Host "TRUER7 status=RED reason=semantic"
    exit 1
}

& (Join-Path $Root "scripts\stage-c1-native.ps1")
if ($LASTEXITCODE -ne 0) {
    Write-Host "TRUER7 status=RED reason=c1_native"
    exit 1
}

Write-Host "TRUER7 trust_root=place_stub"
Write-Host ("TRUER7 stub_bytes={0}" -f (Get-Item (Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe")).Length)
Write-Host "TRUER7 sidecar_name_face=ABSENT"
Write-Host "TRUER7 bins_sot=pin_locked_content"
Write-Host "TRUER7 seed_sot=c1_tyb"
Write-Host "TRUER7 source_driven=PRESENT"
Write-Host "TRUER7 data_trim=TRAILING_ZEROS"
Write-Host "TRUER7 place_size_contract=PATCHED"
Write-Host "TRUER7 doc=AUTO-TRUER7.md"
Write-Host "TRUER7 status=GREEN"
Write-Host "TRUER7 note=data_414016;pe_438784;not_Thompson"
exit 0
