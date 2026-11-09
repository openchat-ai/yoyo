# stage-no-node-plant.ps1 — AUTO-NO-NODE-PLANT E2E
#
#   cd F:\yoyo
#   & .\scripts\stage-no-node-plant.ps1
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== AUTO-NO-NODE-PLANT E2E ==="

Write-Host "=== NP1/NE Node plant refuse (incl. escape env) ==="
$prevAllow = $env:YOYO_ALLOW_NODE_PLANT
$env:YOYO_ALLOW_NODE_PLANT = '1'
try {
    $out = & node (Join-Path $Root "yoyo-js\scripts\rebuild-compiler-payload.js") 2>&1 | Out-String
    if ($LASTEXITCODE -eq 0) {
        Write-Host "NO_NODE_PLANT status=RED reason=rebuild_should_refuse"
        exit 1
    }
    if ($out -notmatch 'NODE_PLANT status=ABSENT' -or $out -notmatch 'escape=ABSENT') {
        Write-Host "NO_NODE_PLANT status=RED reason=rebuild_refuse_marker_missing"
        Write-Host $out
        exit 1
    }
    Write-Host "NO_NODE_PLANT rebuild_even_with_allow=REFUSED"

    $placeTmp = Join-Path $Root "scripts\_stage-no-node-plant\place-refuse"
    New-Item -ItemType Directory -Force -Path $placeTmp | Out-Null
    $pout = & node (Join-Path $Root "yoyo-js\scripts\place-yoyo-compiler.js") $placeTmp 2>&1 | Out-String
    if ($LASTEXITCODE -eq 0) {
        Write-Host "NO_NODE_PLANT status=RED reason=live_place_should_refuse"
        exit 1
    }
    if ($pout -notmatch 'node_plant_refused' -or $pout -notmatch 'escape=ABSENT') {
        Write-Host "NO_NODE_PLANT status=RED reason=place_refuse_marker_missing"
        Write-Host $pout
        exit 1
    }
    Write-Host "NO_NODE_PLANT live_place_even_with_allow=REFUSED"
    Write-Host "NO_NODE_PLANT escape=ABSENT"
} finally {
    if ($null -eq $prevAllow) { Remove-Item Env:YOYO_ALLOW_NODE_PLANT -ErrorAction SilentlyContinue }
    else { $env:YOYO_ALLOW_NODE_PLANT = $prevAllow }
}

Write-Host "=== NP3 wipe bins → C1 regen ≡ saved pin ==="
$payload = Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload"
$bak = Join-Path $Root "scripts\_stage-no-node-plant\bins-bak"
New-Item -ItemType Directory -Force -Path $bak | Out-Null
Copy-Item -Force (Join-Path $payload "code.bin") (Join-Path $bak "code.bin")
Copy-Item -Force (Join-Path $payload "data.bin") (Join-Path $bak "data.bin")
Copy-Item -Force (Join-Path $payload "payload.pin") (Join-Path $bak "payload.pin")
$hc0 = (Get-FileHash (Join-Path $bak "code.bin") -Algorithm SHA256).Hash
$hd0 = (Get-FileHash (Join-Path $bak "data.bin") -Algorithm SHA256).Hash

Remove-Item -Force (Join-Path $payload "code.bin")
Remove-Item -Force (Join-Path $payload "data.bin")

& (Join-Path $Root "scripts\regen-payload-from-c1.ps1")
if ($LASTEXITCODE -ne 0) {
    # restore before fail
    Copy-Item -Force (Join-Path $bak "code.bin") (Join-Path $payload "code.bin")
    Copy-Item -Force (Join-Path $bak "data.bin") (Join-Path $payload "data.bin")
    Copy-Item -Force (Join-Path $bak "payload.pin") (Join-Path $payload "payload.pin")
    Write-Host "NO_NODE_PLANT status=RED reason=regen_failed"
    exit 1
}

$hc1 = (Get-FileHash (Join-Path $payload "code.bin") -Algorithm SHA256).Hash
$hd1 = (Get-FileHash (Join-Path $payload "data.bin") -Algorithm SHA256).Hash
if ($hc0 -ne $hc1 -or $hd0 -ne $hd1) {
    Write-Host "NO_NODE_PLANT status=RED reason=regen_NE_original"
    exit 1
}
Write-Host "NO_NODE_PLANT regen_after_wipe=EQUAL"

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) {
    Write-Host "NO_NODE_PLANT status=RED reason=pin_after_regen"
    exit 1
}

Write-Host "NO_NODE_PLANT node_plant=ABSENT"
Write-Host "NO_NODE_PLANT escape=ABSENT"
Write-Host "NO_NODE_PLANT sot=c1_fixture_pe"
Write-Host "NO_NODE_PLANT doc=AUTO-NO-PLANT-ESCAPE.md"
Write-Host "NO_NODE_PLANT status=GREEN"
exit 0
