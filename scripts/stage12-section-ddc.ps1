# stage12-section-ddc.ps1 — thin alias for Stage 12-B selfhost body section-ddc.
# Canonical gate: scripts/stage12-selfhost-body-section-ddc.ps1
param(
    [switch]$SkipBuild
)
$ErrorActionPreference = "Stop"
$here = $PSScriptRoot
if ($SkipBuild) {
    & (Join-Path $here "stage12-selfhost-body-section-ddc.ps1") -SkipBuild
} else {
    & (Join-Path $here "stage12-selfhost-body-section-ddc.ps1")
}
exit $LASTEXITCODE
