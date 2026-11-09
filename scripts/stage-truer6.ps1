# stage-truer6.ps1 — AUTO-TRUER6 E2E (zero dll-name face · data-trim honesty · gates)
#
#   cd F:\yoyo
#   & .\scripts\stage-truer6.ps1
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== AUTO-TRUER6 E2E ==="

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) {
    Write-Host "TRUER6 status=RED reason=place_stub_pin"
    exit 1
}

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) {
    Write-Host "TRUER6 status=RED reason=bins_pin"
    exit 1
}

& node (Join-Path $Root "yoyo-js\scripts\verify-no-sidecar-face.js")
if ($LASTEXITCODE -ne 0) {
    Write-Host "TRUER6 status=RED reason=sidecar_name_face"
    exit 1
}

# Z2 honesty: trailing-zero trim was tried; C1 place exit=2 → keep data_bytes pinned.
$dataBytes = (Get-Item (Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\data.bin")).Length
if ($dataBytes -ne 421968) {
    Write-Host ("TRUER6 status=RED reason=data_len_unexpected got={0}" -f $dataBytes)
    exit 1
}
Write-Host "TRUER6 data_trim=BLOCKED_BY_C1_PLACE"
Write-Host "TRUER6 data_bytes=421968"

& (Join-Path $Root "scripts\stage-truer5-semantic.ps1")
if ($LASTEXITCODE -ne 0) {
    Write-Host "TRUER6 status=RED reason=semantic"
    exit 1
}

& (Join-Path $Root "scripts\stage-c1-native.ps1")
if ($LASTEXITCODE -ne 0) {
    Write-Host "TRUER6 status=RED reason=c1_native"
    exit 1
}

Write-Host "TRUER6 trust_root=place_stub"
Write-Host ("TRUER6 stub_bytes={0}" -f (Get-Item (Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe")).Length)
Write-Host "TRUER6 sidecar_name_face=ABSENT"
Write-Host "TRUER6 sidecar_required_on_critical=ABSENT"
Write-Host "TRUER6 bins_sot=pin_locked_content"
Write-Host "TRUER6 seed_sot=c1_tyb"
Write-Host "TRUER6 source_driven=PRESENT"
Write-Host "TRUER6 data_trim=BLOCKED_BY_C1_PLACE"
Write-Host "TRUER6 doc=AUTO-TRUER6.md"
Write-Host "TRUER6 status=GREEN"
Write-Host "TRUER6 note=zero_dll_name_face;trim_blocked;not_Thompson"
exit 0
