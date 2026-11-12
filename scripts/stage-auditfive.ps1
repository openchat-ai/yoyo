# stage-auditfive.ps1 — residual 5 paths: close F2/F3; hard-block F1/F4/F5
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== AUTO-AUDITFIVE ==="

# Keep 1-7 floor
& (Join-Path $Root "scripts\stage-auditzero.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "AUDITFIVE status=RED reason=auditzero"; exit 1 }

# F3 expanded scan
& node (Join-Path $Root "yoyo-js\scripts\verify-live-scan-allow-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "AUDITFIVE status=RED reason=scan"; exit 1 }

# F5 dual hash (mitigation; true collision still HARD_BLOCK in matrix)
& node (Join-Path $Root "yoyo-js\scripts\verify-dualhash-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "AUDITFIVE status=RED reason=dualhash"; exit 1 }

# F2 semantic/poison nets (existing)
& (Join-Path $Root "scripts\stage-plantpoison.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "AUDITFIVE status=RED reason=plantpoison"; exit 1 }
& (Join-Path $Root "scripts\stage-y1plpin.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "AUDITFIVE status=RED reason=y1plpin"; exit 1 }
& (Join-Path $Root "scripts\stage-binspoison.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "AUDITFIVE status=RED reason=binspoison"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-auditfive-matrix.js")
if ($LASTEXITCODE -ne 0) { Write-Host "AUDITFIVE status=RED reason=matrix"; exit 1 }

Write-Host "AUDITFIVE note=F2_F3_CLOSED;F1_F4_F5_HARD_BLOCK;not_Thompson"
Write-Host "AUDITFIVE status=GREEN"
Write-Host "AUDITFIVE doc=AUTO-AUDITFIVE.md"
exit 0
