# stage16-b.ps1 — thin alias for Stage 16-B detection wording / RELEASE boundary.
# Canonical gate: scripts/stage16-detection-wording.ps1
param([switch]$SkipBuild)
$here = $PSScriptRoot
if ($SkipBuild) {
    & (Join-Path $here "stage16-detection-wording.ps1") -SkipBuild
} else {
    & (Join-Path $here "stage16-detection-wording.ps1")
}
exit $LASTEXITCODE
