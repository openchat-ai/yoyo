# Alias → stage15-v08-regress.ps1 (Stage 15-C)
param([switch]$SkipBuild, [switch]$SkipWsl)
$here = $PSScriptRoot
if ($SkipWsl) {
    Write-Host "stage15-c: SkipWsl forbidden for graduation" -ForegroundColor Red
    exit 1
}
if ($SkipBuild) {
    & (Join-Path $here "stage15-v08-regress.ps1") -SkipBuild
} else {
    & (Join-Path $here "stage15-v08-regress.ps1")
}
exit $LASTEXITCODE
