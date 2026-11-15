# Expand-YoyoReadable.ps1 — LANG-READABLE (.yoyor → .ty); no Node
param(
    [Parameter(Mandatory = $true)][string]$InputPath,
    [Parameter(Mandatory = $true)][string]$OutputPath
)
$ErrorActionPreference = "Stop"
if (-not (Test-Path -LiteralPath $InputPath)) {
    throw "Expand-YoyoReadable missing input: $InputPath"
}

$out = New-Object System.Collections.Generic.List[string]
$out.Add("; expanded from $(Split-Path $InputPath -Leaf) by Expand-YoyoReadable.ps1")
$program = $null
$target = $null
$lineNo = 0

Get-Content -LiteralPath $InputPath | ForEach-Object {
    $lineNo++
    $raw = $_
    $line = $raw.Trim()
    if ($line.Length -eq 0) { return }
    if ($line.StartsWith(";;") -or $line.StartsWith("#")) { return }
    if ($line.StartsWith(";")) {
        $out.Add($line)
        return
    }
    if ($line -match '^(?i)program\s+(\S+)\s*$') {
        $program = $Matches[1]
        return
    }
    if ($line -match '^(?i)target\s+(\S+)\s*$') {
        $target = $Matches[1].ToLowerInvariant()
        return
    }
    if ($line -match '^(?i)(raw|db)\s+(.+)$') {
        $hex = $Matches[2].Trim()
        if ($hex -notmatch '^(?:[0-9A-Fa-f]{2})(?:\s+[0-9A-Fa-f]{2})*$') {
            throw "Expand-YoyoReadable bad hex at line $lineNo : $hex"
        }
        $out.Add(($hex.ToUpperInvariant() -replace '\s+', ' '))
        return
    }
    throw "Expand-YoyoReadable syntax at line $lineNo : $raw"
}

if (-not $program) { throw "Expand-YoyoReadable missing program <name>" }
if (-not $target) { throw "Expand-YoyoReadable missing target <os>" }
if ($out.Count -lt 2) { throw "Expand-YoyoReadable no raw/db bytes" }

$dir = Split-Path -Parent $OutputPath
if ($dir -and -not (Test-Path $dir)) { New-Item -ItemType Directory -Force -Path $dir | Out-Null }
Set-Content -LiteralPath $OutputPath -Value $out -Encoding ascii
Write-Output "YOYOR_EXPAND program=$program target=$target out=$OutputPath lines=$($out.Count)"
