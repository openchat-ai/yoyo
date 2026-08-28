# stage15-b.ps1 — thin alias for Stage 15-B prerun keep-green.
# Canonical gate: scripts/stage15-prerun.ps1
param([switch]$SkipBuild)
$here = $PSScriptRoot
if ($SkipBuild) {
    & (Join-Path $here "stage15-prerun.ps1") -SkipBuild
} else {
    & (Join-Path $here "stage15-prerun.ps1")
}
exit $LASTEXITCODE
