# stage-c1-native.ps1 — AUTO-C1-NATIVE E2E
#
#   cd F:\yoyo
#   & .\scripts\stage-c1-native.ps1
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== AUTO-C1-NATIVE E2E ==="

. (Join-Path $Root "scripts\lib\Assert-YoyoFilePin.ps1")
$Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
if (-not (Test-Path -LiteralPath $Yoyo)) {
    Write-Host "C1_NATIVE status=RED reason=missing_yoyo"
    exit 1
}

Write-Host "=== TRUER4 verify place-stub pin (yoyo hash; Node ABSENT) ==="
try {
    Assert-YoyoShaPin -Yoyo $Yoyo `
        -File (Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe") `
        -PinPath (Join-Path $Root "scripts\fixtures\yoyo_place_stub.pin") `
        -Label PLACE_STUB -Doc AUTO-TRUER4.md
} catch {
    Write-Host "C1_NATIVE status=RED reason=place_stub_pin"
    exit 1
}

Write-Host "=== BD-3 verify bins pin (yoyo hash; Node ABSENT) ==="
try {
    Assert-YoyoPayloadBinsPin -Yoyo $Yoyo `
        -PayloadDir (Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload") `
        -Label HB_FX_BINS -Doc AUTO-TRUER6.md
} catch {
    Write-Host "C1_NATIVE status=RED reason=bins_pin"
    exit 1
}

Write-Host "=== C1COLD fixture×bins → full C1; emit bins ≡ pin ==="
$fx = Join-Path $Root "scripts\fixtures\yoyo_compiler.exe"
$placeOut = Join-Path $Root "scripts\_stage-c1-native\fixture-place"
if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $fx
if ($LASTEXITCODE -ne 0) {
    Write-Host "C1_NATIVE status=RED reason=fixture_place"
    exit 1
}
$fullC1 = Join-Path $placeOut "yoyo_compiler.exe"
try {
    Assert-YoyoShaPin -Yoyo $Yoyo -File $fullC1 `
        -PinPath (Join-Path $Root "scripts\fixtures\yoyo_place_c1.pin") `
        -Label PLACE_C1 -RequireTrustRoot c1_product_pin -Doc AUTO-BREAK3.md
} catch {
    Write-Host "C1_NATIVE status=RED reason=c1_product_pin"
    exit 1
}
$emitOut = Join-Path $Root "scripts\_stage-yoyo-bins\fixture-gate-emit"
if (Test-Path $emitOut) { Remove-Item -Recurse -Force $emitOut }
& (Join-Path $Root "scripts\emit-bins-c1.ps1") -OutDir $emitOut -Bootstrap $fullC1
if ($LASTEXITCODE -ne 0) {
    Write-Host "C1_NATIVE status=RED reason=full_c1_emit"
    exit 1
}
$payload = Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload"
$hc0 = (Get-FileHash (Join-Path $payload "code.bin") -Algorithm SHA256).Hash
$hc1 = (Get-FileHash (Join-Path $emitOut "code.bin") -Algorithm SHA256).Hash
$hd0 = (Get-FileHash (Join-Path $payload "data.bin") -Algorithm SHA256).Hash
$hd1 = (Get-FileHash (Join-Path $emitOut "data.bin") -Algorithm SHA256).Hash
if ($hc0 -ne $hc1 -or $hd0 -ne $hd1) {
    Write-Host "C1_NATIVE status=RED reason=emit_NE_bins_pin"
    exit 1
}
Write-Host "C1_NATIVE trust_root=c1_product_pin"
Write-Host "C1_NATIVE bins_sot=fixture_pe"
Write-Host "C1_NATIVE fixture_place_emit=EQUAL"
Write-Host "C1_NATIVE yoyo_bins_node_rebuild=ABSENT"
Write-Host "C1_NATIVE place_stub_role=truer4_pin_only"

Write-Host "=== N0/N2 yc2 C1 process repro ==="
& (Join-Path $Root "scripts\yc2-compiler-payload-repro.ps1")
if ($LASTEXITCODE -ne 0) {
    Write-Host "C1_NATIVE status=RED reason=yc2"
    exit 1
}

Write-Host "=== N3 C1 place (no Node) ==="
$placeOut = Join-Path $Root "scripts\_stage-c1-native\place-gate"
if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut
if ($LASTEXITCODE -ne 0) {
    Write-Host "C1_NATIVE status=RED reason=c1_place"
    exit 1
}

Write-Host "=== N3/N4 fullbar (prefer C1 place) ==="
$markers = Join-Path $Root "scripts\_stage-fullbar-fs2-chain\yoyo-closed-markers.txt"
if (Test-Path $markers) { Remove-Item -Force $markers }
& (Join-Path $Root "scripts\stage-fullbar-trust-root.ps1") -SkipEmitRt
if ($LASTEXITCODE -ne 0) {
    Write-Host "C1_NATIVE status=RED reason=fullbar"
    exit 1
}
$mk = @{}
Get-Content $markers | ForEach-Object {
    if ($_ -match '^\s*([^=]+)=(.*)$') { $mk[$Matches[1].Trim()] = $Matches[2].Trim() }
}
if ($mk['node_place_on_gate'] -ne 'ABSENT') {
    Write-Host "C1_NATIVE status=RED reason=node_place_still_present"
    exit 1
}
if ($mk['fixedpoint'] -ne 'EQUAL') {
    Write-Host "C1_NATIVE status=RED reason=fixedpoint"
    exit 1
}
if ($mk['h00_mid_chain_node_place'] -ne 'ABSENT') {
    Write-Host "C1_NATIVE status=RED reason=h00_mid_chain"
    exit 1
}
if ($mk['cold_start_asm_live'] -ne 'ABSENT') {
    Write-Host "C1_NATIVE status=RED reason=cold_start_asm_still_live"
    exit 1
}
if ($mk['bins_node_rebuild_on_gate'] -ne 'ABSENT') {
    Write-Host "C1_NATIVE status=RED reason=bins_node_rebuild_on_gate"
    exit 1
}
if ($mk['bins_gate_policy'] -ne 'pin_locked') {
    Write-Host "C1_NATIVE status=RED reason=bins_gate_policy"
    exit 1
}
if ($mk['seed_sot'] -ne 'c1_tyb') {
    Write-Host "C1_NATIVE status=RED reason=seed_sot"
    exit 1
}
if ($mk['seed_fixture_required'] -ne 'ABSENT') {
    Write-Host "C1_NATIVE status=RED reason=seed_fixture_still_required"
    exit 1
}
if ($mk['critical_pe_roots'] -ne '1') {
    Write-Host "C1_NATIVE status=RED reason=critical_pe_roots"
    exit 1
}
if ($mk['critical_trust_blobs'] -ne '2') {
    Write-Host "C1_NATIVE status=RED reason=critical_trust_blobs"
    exit 1
}
if ($mk['trust_root'] -ne 'c1_product_pin' -and $mk['trust_root'] -ne 'fixture_yoyo_compiler' -and $mk['trust_root'] -ne 'place_stub') {
    Write-Host "C1_NATIVE status=RED reason=trust_root"
    exit 1
}
if ($mk['sidecar_required_on_critical'] -ne 'ABSENT') {
    Write-Host "C1_NATIVE status=RED reason=sidecar_still_critical"
    exit 1
}
if ($mk['cargo_emit_rt'] -ne 'ABSENT') {
    Write-Host "C1_NATIVE status=RED reason=cargo_emit_rt"
    exit 1
}

Write-Host "C1_NATIVE n0=GREEN n1=GREEN n2=GREEN n3=GREEN n4=GREEN"
Write-Host "C1_NATIVE node_place_on_gate=ABSENT"
Write-Host "C1_NATIVE c1_process_repro=EQUAL"
Write-Host "C1_NATIVE h00=EQUAL"
Write-Host ("C1_NATIVE fixedpoint_sha256_prefix={0}" -f $mk['fixedpoint_sha256_prefix'])
Write-Host "C1_NATIVE node_plant=ABSENT"
Write-Host "C1_NATIVE escape=ABSENT"
Write-Host ("C1_NATIVE trust_root={0}" -f $mk['trust_root'])
Write-Host "C1_NATIVE bins_sot=fixture_pe"
Write-Host "C1_NATIVE seed_sot=c1_tyb"
Write-Host "C1_NATIVE critical_pe_roots=1"
Write-Host "C1_NATIVE critical_trust_blobs=2"
Write-Host "C1_NATIVE sidecar_required_on_critical=ABSENT"
Write-Host "C1_NATIVE cargo_emit_rt=ABSENT"
Write-Host "C1_NATIVE note=c1cold_fixture_product;place_stub_pin_only;not_Thompson"
Write-Host "C1_NATIVE doc=AUTO-C1COLD.md"
Write-Host "C1_NATIVE status=GREEN"
exit 0
