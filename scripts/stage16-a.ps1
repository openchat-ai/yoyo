# stage16-a.ps1 — thin alias for Stage 16-A v1.0 FINAL SCOPE-CUT.
# Canonical gate: scripts/stage16-scope-cut-finalize.ps1
param([switch]$SkipBuild)
$here = $PSScriptRoot
if ($SkipBuild) {
    & (Join-Path $here "stage16-scope-cut-finalize.ps1") -SkipBuild
} else {
    & (Join-Path $here "stage16-scope-cut-finalize.ps1")
}
exit $LASTEXITCODE
