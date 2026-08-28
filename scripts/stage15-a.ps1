# stage15-a.ps1 — thin alias for Stage 15-A hole inventory.
# Canonical gate: scripts/stage15-hole-inventory.ps1
param([switch]$SkipBuild)
$here = $PSScriptRoot
if ($SkipBuild) {
    & (Join-Path $here "stage15-hole-inventory.ps1") -SkipBuild
} else {
    & (Join-Path $here "stage15-hole-inventory.ps1")
}
exit $LASTEXITCODE
