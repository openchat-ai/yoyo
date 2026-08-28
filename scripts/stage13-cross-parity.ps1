# stage13-cross-parity.ps1 — alias of stage13-cross-platform-parity.ps1 (Stage 13-B)
param(
    [switch]$SkipBuild,
    [switch]$SkipWsl,
    [switch]$SkipPriorPeers
)
$here = $PSScriptRoot
& (Join-Path $here "stage13-cross-platform-parity.ps1") @PSBoundParameters
exit $LASTEXITCODE
