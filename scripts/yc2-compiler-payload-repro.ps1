# yc2-compiler-payload-repro.ps1 — Y2 place determinism (AUTO-C1COLD)
#
# Dual fixture×fixture-emit-bins place must EQUAL + placed C1 cold-compiles yoyo.tyb.
# SoT: C1 product = fixture bootstrap x payload bins (identity pin yoyo_place_c1).
# place_stub remains TRUER4 stub pin only — not this gate's bootstrap.
#
#   cd F:\yoyo
#   & .\scripts\yc2-compiler-payload-repro.ps1
param(
    [string]$OutDir = ""
)
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden

if (-not $OutDir) {
    $OutDir = Join-Path $Root "scripts\_stage-yc2-repro"
}
if (Test-Path $OutDir) { Remove-Item -Recurse -Force $OutDir }
New-Item -ItemType Directory -Force -Path $OutDir | Out-Null

$fx = Join-Path $Root "scripts\fixtures\yoyo_compiler.exe"
$codePath = Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\code.bin"
$dataPath = Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\data.bin"
$tyb = Join-Path $Root "yoyo\projects\yoyo.tyb"
if (-not (Test-Path $fx) -or -not (Test-Path $codePath) -or -not (Test-Path $dataPath)) {
    Write-Host "YOYO_CLOSED_Y2 status=RED reason=missing_fixture_or_bins"
    exit 1
}
if (-not (Test-Path $tyb)) {
    Write-Host "YOYO_CLOSED_Y2 status=RED reason=missing_yoyo_tyb"
    exit 1
}

Write-Host "=== YC-2 dual fixture×bins place + C1 cold compile ==="
Write-Host "YOYO_CLOSED_Y2 node_buildCompilerCode=ABSENT"
Write-Host "YOYO_CLOSED_Y2 node_linkPeWin32=ABSENT"
Write-Host "YOYO_CLOSED_Y2 node_place_yoyo_compiler=ABSENT"
Write-Host "YOYO_CLOSED_Y2 emitter=c1_try_place_bins"
Write-Host "YOYO_CLOSED_Y2 bootstrap=c1_product_pin_bits"

$p1 = Join-Path $OutDir "place1"
$p2 = Join-Path $OutDir "place2"
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $p1 -Bootstrap $fx
if ($LASTEXITCODE -ne 0) { Write-Host "YOYO_CLOSED_Y2 status=RED reason=place1"; exit 1 }
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $p2 -Bootstrap $fx
if ($LASTEXITCODE -ne 0) { Write-Host "YOYO_CLOSED_Y2 status=RED reason=place2"; exit 1 }

$c1 = Join-Path $p1 "yoyo_compiler.exe"
$c2 = Join-Path $p2 "yoyo_compiler.exe"
$h1 = (Get-FileHash $c1 -Algorithm SHA256).Hash.ToLowerInvariant()
$h2 = (Get-FileHash $c2 -Algorithm SHA256).Hash.ToLowerInvariant()
Write-Host ("YOYO_CLOSED_Y2 place1_sha256_prefix={0} bytes={1}" -f $h1.Substring(0, 16), (Get-Item $c1).Length)
Write-Host ("YOYO_CLOSED_Y2 place2_sha256_prefix={0} bytes={1}" -f $h2.Substring(0, 16), (Get-Item $c2).Length)
if ($h1 -ne $h2) {
    Write-Host "YOYO_CLOSED_Y2 status=RED reason=dual_fixture_place_DIFF"
    exit 1
}
Write-Host "YOYO_CLOSED_Y2 dual_fixture_place=EQUAL"

$cold = Join-Path $OutDir "cold"
New-Item -ItemType Directory -Force -Path $cold | Out-Null
Copy-Item -Force $c1 (Join-Path $cold "yoyo_compiler.exe")
Copy-Item -Force $tyb (Join-Path $cold "input.tyb")
if (Test-Path (Join-Path $cold "output.exe")) { Remove-Item -Force (Join-Path $cold "output.exe") }
$ec = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $cold "yoyo_compiler.exe") -WorkingDirectory $cold -TimeoutSec 300
$seed = Join-Path $cold "output.exe"
if ($ec -ne 0 -or -not (Test-Path $seed)) {
    Write-Host ("YOYO_CLOSED_Y2 status=RED reason=c1_cold_start_failed exit={0}" -f $ec)
    exit 1
}
$seedSha = (Get-FileHash $seed -Algorithm SHA256).Hash.ToLowerInvariant()
Write-Host ("YOYO_CLOSED_Y2 cold_start=GREEN seed_bytes={0} sha256_prefix={1}" -f (Get-Item $seed).Length, $seedSha.Substring(0, 16))
Write-Host "YOYO_CLOSED_Y2 note=dual_rebuild_place_EQUAL;c1_tyb_cold_GREEN;trust_root=c1_product_pin;not_Thompson"
Write-Host "YOYO_CLOSED_Y2 doc=AUTO-C1COLD.md"
Write-Host "YOYO_CLOSED_Y2 status=GREEN"
exit 0
