# generate-pe-ledger.ps1 — deterministic PE inventory for quarantine-gen Q3
# Usage:
#   .\scripts\generate-pe-ledger.ps1 -Gen gen4 -Root F:\yoyo
# This inventories current .exe/.dll/.node files only. It does not advertise LOCKED / C-ddc.

[CmdletBinding()]
param(
  [Parameter(Mandatory = $true)]
  [ValidatePattern('^gen\d+$')]
  [string]$Gen,

  [Parameter(Mandatory = $false)]
  [string]$Root
)

$ErrorActionPreference = "Stop"

if ([string]::IsNullOrWhiteSpace($Root)) {
  $Root = Split-Path -Parent $PSScriptRoot
}
$Root = (Resolve-Path -LiteralPath $Root).Path

$ledgerRelativePath = "docs\PE_LEDGER_$Gen.txt"
$ledgerPath = Join-Path $Root $ledgerRelativePath
$ledgerDirectory = Split-Path -Parent $ledgerPath
if (-not (Test-Path -LiteralPath $ledgerDirectory)) {
  throw "Ledger directory does not exist: $ledgerDirectory"
}

$binaries = @(
  Get-ChildItem -LiteralPath $Root -Recurse -File -ErrorAction Stop |
    Where-Object {
      $_.Extension -match '^\.(exe|dll|node)$' -and
      $_.FullName -notmatch '\\(\.git)\\'
    } |
    Sort-Object FullName
)

$lines = @(
  "# PE_LEDGER_$Gen — inventory for quarantine-gen Q3 ($Gen local smoke)"
  "# Generated: $((Get-Date).ToString('yyyy-MM-dd'))"
  "# Format: SHA256  relative-path"
  "# Note: includes build caches when present; refresh after rebuilds that change PE hashes."
  "# Does NOT advertise LOCKED / C-ddc — inventory only."
  ""
)

foreach ($binary in $binaries) {
  $hash = (Get-FileHash -LiteralPath $binary.FullName -Algorithm SHA256).Hash
  $relativePath = $binary.FullName.Substring($Root.Length).TrimStart('\', '/')
  $lines += "$hash  $relativePath"
}

Set-Content -LiteralPath $ledgerPath -Value $lines -Encoding UTF8
Write-Host "PE ledger refreshed: $ledgerRelativePath ($($binaries.Count) binaries)"
exit 0
