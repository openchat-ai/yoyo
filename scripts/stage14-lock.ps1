# stage14-lock.ps1 — thin alias for Stage 14-B Lock harden.
# Canonical gate: scripts/stage14-lock-harden.ps1
param(
    [switch]$SkipBuild
)
$here = $PSScriptRoot
if ($SkipBuild) {
    & (Join-Path $here "stage14-lock-harden.ps1") -SkipBuild
} else {
    & (Join-Path $here "stage14-lock-harden.ps1")
}
exit $LASTEXITCODE
