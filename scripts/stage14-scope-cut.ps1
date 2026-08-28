# stage14-scope-cut.ps1 — thin alias for Stage 14-A outside-window SCOPE-CUT.
# Canonical gate: scripts/stage14-outside-window-scope-cut.ps1
param([switch]$SkipBuild)
$here = $PSScriptRoot
if ($SkipBuild) {
    & (Join-Path $here "stage14-outside-window-scope-cut.ps1") -SkipBuild
} else {
    & (Join-Path $here "stage14-outside-window-scope-cut.ps1")
}
exit $LASTEXITCODE
