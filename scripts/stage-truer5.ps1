# stage-truer5.ps1 — AUTO-TRUER5 E2E (no sidecar face · thinner SoT · tyb semantic)
#
#   cd F:\yoyo
#   & .\scripts\stage-truer5.ps1
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== AUTO-TRUER5 E2E ==="

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) {
    Write-Host "TRUER5 status=RED reason=place_stub_pin"
    exit 1
}

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) {
    Write-Host "TRUER5 status=RED reason=bins_pin"
    exit 1
}

& node (Join-Path $Root "yoyo-js\scripts\verify-no-sidecar-face.js")
if ($LASTEXITCODE -ne 0) {
    Write-Host "TRUER5 status=RED reason=sidecar_face"
    exit 1
}

& (Join-Path $Root "scripts\stage-truer5-semantic.ps1")
if ($LASTEXITCODE -ne 0) {
    Write-Host "TRUER5 status=RED reason=semantic"
    exit 1
}

& (Join-Path $Root "scripts\stage-c1-native.ps1")
if ($LASTEXITCODE -ne 0) {
    Write-Host "TRUER5 status=RED reason=c1_native"
    exit 1
}

Write-Host "TRUER5 trust_root=place_stub"
Write-Host ("TRUER5 stub_bytes={0}" -f (Get-Item (Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe")).Length)
Write-Host "TRUER5 sidecar_face_in_data=ABSENT"
Write-Host "TRUER5 sidecar_required_on_critical=ABSENT"
Write-Host "TRUER5 sidecar_sot=contrast_only"
Write-Host "TRUER5 bins_sot=pin_locked_content"
Write-Host "TRUER5 seed_sot=c1_tyb"
Write-Host "TRUER5 source_driven=PRESENT"
Write-Host "TRUER5 critical_pe_roots=1"
Write-Host "TRUER5 critical_trust_blobs=2"
Write-Host "TRUER5 full_c1_fixture=fallthrough_only"
Write-Host "TRUER5 yoyo_rt_fixture=contrast_only"
Write-Host "TRUER5 doc=AUTO-TRUER5.md"
Write-Host "TRUER5 status=GREEN"
Write-Host "TRUER5 note=no_yoyo_rt_face;tyb_semantic;not_Thompson"
exit 0
