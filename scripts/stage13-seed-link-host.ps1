# stage13-seed-link-host.ps1 — alias of stage13-link-host.ps1 (Stage 13-A)
param(
    [switch]$SkipBuild,
    [switch]$SkipLinux,
    [switch]$SkipSelfhostDiff
)
$here = $PSScriptRoot
& (Join-Path $here "stage13-link-host.ps1") @PSBoundParameters
exit $LASTEXITCODE
