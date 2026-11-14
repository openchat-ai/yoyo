# stage-lang1.ps1 — v1.1 independent-language first cut
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== LANG1 (v1.1 independent language surface) ==="

$surface = Join-Path $Root "LANG-SURFACE.md"
$scope = Join-Path $Root "SCOPE-v1.1.md"
if (-not (Test-Path $surface) -or -not (Test-Path $scope)) {
    Write-Host "LANG1 status=RED reason=missing_surface_or_scope"
    exit 1
}
$surf = Get-Content -LiteralPath $surface -Raw
foreach ($needle in @('yoyo link', '.ty', 'anyprog', '独立语言')) {
    if ($surf -notlike "*$needle*") {
        Write-Host ("LANG1 status=RED reason=surface_missing token={0}" -f $needle)
        exit 1
    }
}
Write-Host "LANG1 surface_doc=GREEN"

& (Join-Path $Root "scripts\stage-anyprog.ps1") -SkipBuild
if ($LASTEXITCODE -ne 0) {
    Write-Host "LANG1 status=RED reason=anyprog"
    exit 1
}
Write-Host "LANG1 programs_anyprog=GREEN"

Write-Host "LANG1 north_star=independent_language_first_cut"
Write-Host "LANG1 trust_chain=regression_appendix"
Write-Host "LANG1 note=v1.0_selfhost_foundation;v1.1_language_surface;not_C_replacement;not_Thompson"
Write-Host "LANG1 doc=SCOPE-v1.1.md"
Write-Host "LANG1 status=GREEN"
exit 0
