# Alias → stage16-v09-regress.ps1 (Stage 16-C)
param([switch]$SkipBuild, [switch]$SkipWsl)
$here = $PSScriptRoot
if ($SkipWsl) {
    Write-Host "stage16-c: SkipWsl forbidden for graduation" -ForegroundColor Red
    exit 1
}
if ($SkipBuild) {
    & (Join-Path $here "stage16-v09-regress.ps1") -SkipBuild
} else {
    & (Join-Path $here "stage16-v09-regress.ps1")
}
exit $LASTEXITCODE
