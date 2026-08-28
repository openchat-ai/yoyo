# stage12-regression.ps1 — thin alias for Stage 12-C v0.5 regression gate.
# Canonical gate: scripts/stage12-v05-regress.ps1
param(
    [switch]$SkipBuild,
    [switch]$SkipWsl
)
$ErrorActionPreference = "Stop"
$here = $PSScriptRoot
$argsList = @()
if ($SkipBuild) { $argsList += "-SkipBuild" }
if ($SkipWsl) { $argsList += "-SkipWsl" }
& (Join-Path $here "stage12-v05-regress.ps1") @argsList
exit $LASTEXITCODE
