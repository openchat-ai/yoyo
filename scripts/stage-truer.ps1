# stage-truer.ps1 — forwards to latest AUTO-TRUER E2E
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
& (Join-Path $Root "scripts\stage-openloop.ps1")
exit $LASTEXITCODE
