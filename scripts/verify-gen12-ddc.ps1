# verify-gen12-ddc.ps1 — gen1≡gen2 monitor (.ty link vs .tyb bootstrap, .text DDC EQUAL)
# Exit 0 when gen1≡gen2 EQUAL; exit 1 on DIFF or build failure.
param(
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

$Ty = Join-Path $Root "yoyo\projects\yoyo.ty"
$Tyb = Join-Path $Root "yoyo\projects\yoyo.tyb"
$OutDir = Join-Path $Root "yoyo-js\build\gen12-ddc"
New-Item -ItemType Directory -Force -Path $OutDir | Out-Null

$Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
if (-not (Test-Path $Yoyo)) {
    Write-Host "== build yoyo (release) =="
    Push-Location (Join-Path $Root "yoyo-rust")
    cargo build --release -p verifier
    Pop-Location
}

if (-not (Test-Path $Tyb)) {
    Write-Host "== ty2tyb =="
    python (Join-Path $Root "scripts\ty2tyb.py")
    if (-not (Test-Path $Tyb)) { throw "ty2tyb failed: missing $Tyb" }
}

$Gen1 = Join-Path $OutDir "gen1_ty.exe"
$Gen2 = Join-Path $OutDir "gen2_tyb.exe"

if (-not $SkipBuild) {
    Write-Host "== gen1: yoyo link yoyo.ty =="
    & $Yoyo link --target=win32 $Ty $Gen1
    if ($LASTEXITCODE -ne 0) { throw "gen1 link failed (exit $LASTEXITCODE)" }

    Write-Host "== gen2: yoyo bootstrap yoyo.tyb =="
    & $Yoyo bootstrap $Tyb $Gen2
    if ($LASTEXITCODE -ne 0) { throw "gen2 bootstrap failed (exit $LASTEXITCODE)" }
} else {
    foreach ($f in @($Gen1, $Gen2)) {
        if (-not (Test-Path $f)) { throw "Missing $f - run without -SkipBuild first" }
    }
    Write-Host "== skip build (reuse existing artifacts) =="
}

Write-Host ""
Write-Host "=== gen1≡gen2: .ty link vs .tyb bootstrap ==="
$out = & $Yoyo diff $Gen1 $Gen2 2>&1 | ForEach-Object { "$_" }
$diffOk = ($LASTEXITCODE -eq 0)
$out | ForEach-Object { Write-Host $_ }

$sha = $null
foreach ($line in $out) {
    if ($line -match 'hash_a: ([0-9a-f]+)') {
        $sha = $Matches[1]
        break
    }
}

Write-Host ""
if ($sha) {
    Write-Host "SHA-256 (.text): $sha"
    Write-Host "SHA prefix: $($sha.Substring(0, 8))"
}

if ($diffOk) {
    Write-Host "gen1≡gen2 (.ty==.tyb DDC): EQUAL"
    exit 0
} else {
    Write-Host "gen1≡gen2 (.ty==.tyb DDC): DIFF"
    exit 1
}
