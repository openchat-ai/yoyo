# stage-truer14.ps1 — AUTO-TRUER14 E2E (plant SoT = YOYO tyb halves)
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-TRUER14 E2E ==="

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER14 status=RED reason=place_stub_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER14 status=RED reason=bins_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-no-sidecar-face.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER14 status=RED reason=sidecar"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-h00-bake-zero.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER14 status=RED reason=h00_clean"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-prefab-bins-sot.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER14 status=RED reason=prefab"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-assemble.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER14 status=RED reason=assemble"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-compact-code.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER14 status=RED reason=compact"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-plant-sot.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER14 status=RED reason=plant_sot"; exit 1 }

# C1 × halves → extract text (strip FF→C3) → concat EQUAL code.bin
$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$placeOut = Join-Path $Root "scripts\_stage-truer14\place-gate"
if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER14 status=RED reason=place"; exit 1 }
$c1 = Join-Path $placeOut "yoyo_compiler.exe"

function Invoke-PlantHalf([string]$Name, [string]$Tyb) {
    $d = Join-Path $Root ("scripts\_stage-truer14\regen-" + $Name)
    if (Test-Path $d) { Remove-Item -Recurse -Force $d }
    New-Item -Force -ItemType Directory $d | Out-Null
    Copy-Item $c1 (Join-Path $d "yoyo_compiler.exe")
    Copy-Item $Tyb (Join-Path $d "input.tyb")
    $ec = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $d "yoyo_compiler.exe") -WorkingDirectory $d -TimeoutSec 120
    $out = Join-Path $d "output.exe"
    if ($ec -ne 0 -or -not (Test-Path $out)) {
        throw ("plant half {0} failed exit={1}" -f $Name, $ec)
    }
    return $out
}

$pePlace = Invoke-PlantHalf "place" (Join-Path $Root "yoyo\projects\yoyo-plant-place.tyb")
$peParse1 = Invoke-PlantHalf "parse1" (Join-Path $Root "yoyo\projects\yoyo-plant-parse1.tyb")

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-plant-c1-regen.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER14 status=RED reason=c1_regen"; exit 1 }

& (Join-Path $Root "scripts\stage-truer5-semantic.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER14 status=RED reason=semantic"; exit 1 }

& (Join-Path $Root "scripts\stage-c1-native.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER14 status=RED reason=c1_native"; exit 1 }

Write-Host "TRUER14 trust_root=place_stub"
Write-Host "TRUER14 code_sot=yoyo_plant_tyb_halves"
Write-Host "TRUER14 prefab_sot=yoyo_plant_tyb_plus_seeds"
Write-Host "TRUER14 c1_halves_regen=EQUAL"
Write-Host "TRUER14 doc=AUTO-TRUER14.md"
Write-Host "TRUER14 status=GREEN"
Write-Host "TRUER14 note=plant_yoyo_ir;not_semantic_rewrite;not_Thompson"
exit 0
