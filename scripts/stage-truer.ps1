# stage-truer.ps1 — forwards to latest deepen tip E2E
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
& (Join-Path $Root "scripts\stage-tybself.ps1")
exit $LASTEXITCODE
