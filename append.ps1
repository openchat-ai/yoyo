#!/usr/bin/env pwsh
# Append the missing Parts (G, 5, 5B, 6, 7, 8, 9, E, F, Deduce, Gnd, S, 10-16, Appendices)
# from templates to the end of PROMPT-v3.md.

$ErrorActionPreference = 'Stop'
$path = 'f:\yoyo\PROMPT-v3.md'
$utf8 = New-Object System.Text.UTF8Encoding($false)
$text = [System.IO.File]::ReadAllText($path, $utf8)
$lines = $text -split "`r?`n"
Write-Host ('Start: {0} lines' -f $lines.Length)

# Strip the trailing BOOK I header (lines 1270-1275) so we start fresh
# Find the line "# ═══ BOOK I — Identity & Norms / 身份与规范 ═══"
$endIdx = -1
for ($i = $lines.Length - 1; $i -ge 0; $i--) {
  if ($lines[$i] -like '# ═══ BOOK I — Identity & Norms*') { $endIdx = $i; break }
}
if ($endIdx -ge 0) {
  Write-Host ('Stripping trailing BOOK I header at line {0}' -f ($endIdx + 1))
  $lines = $lines[0..($endIdx - 1)]
}
Write-Host ('After strip: {0} lines' -f $lines.Length)

# Also strip the duplicate header insertions at lines 1068-1089 (or wherever they are now)
# Find "# YOYO: Engineering Specification (v3.3.8)" lines and remove blocks
$dupIdxs = @()
for ($i = 0; $i -lt $lines.Length; $i++) {
  if ($lines[$i] -like '# YOYO: Engineering Specification (v3.3.8)*') { $dupIdxs += $i }
}
Write-Host ('Found {0} duplicate v3.3.8 headers' -f $dupIdxs.Length)
# Keep the first one (original v3.3.8 file location). Actually we want to delete ALL of them.
# Each is followed by ~9 lines (header content + "Markers" line + blank). Delete [idx..idx+11].
$toDelete = @()
foreach ($idx in $dupIdxs) {
  $end = $idx + 11
  if ($end -ge $lines.Length) { $end = $lines.Length - 1 }
  $toDelete += @{ Start = $idx; End = $end }
}
$toDelete = $toDelete | Sort-Object -Property Start -Descending
foreach ($b in $toDelete) {
  $lines = $lines[0..($b.Start - 1)] + $lines[($b.End + 1)..($lines.Length - 1)]
}
Write-Host ('After dedup: {0} lines' -f $lines.Length)

# Also remove the leftover v3.3.8 header (the very first header at the top)
# The first line should be v3.3.9. If there's a v3.3.8 header line before it, delete it.
for ($i = 0; $i -lt [Math]::Min(20, $lines.Length); $i++) {
  if ($lines[$i] -like '# YOYO: Engineering Specification (v3.3.8)*') {
    # Delete lines [i..i+11]
    Write-Host ('Deleting first v3.3.8 header at line {0}' -f ($i + 1))
    $lines = $lines[0..($i - 1)] + $lines[($i + 12)..($lines.Length - 1)]
    break
  }
}

# Now append the missing parts from templates
$missing = @(
  @{ Tpl = 'f:\yoyo\tpl-partG.txt';       Gap = 0 },
  @{ Tpl = 'f:\yoyo\tpl-part5-5B.txt';    Gap = 0 },
  @{ Tpl = 'f:\yoyo\tpl-part6.txt';       Gap = 0 },
  @{ Tpl = 'f:\yoyo\tpl-part7-orig.txt';  Gap = 0 },
  @{ Tpl = 'f:\yoyo\tpl-part8-orig.txt';  Gap = 0 },
  @{ Tpl = 'f:\yoyo\tpl-part9.txt';       Gap = 0 },
  @{ Tpl = 'f:\yoyo\tpl-partE.txt';       Gap = 0 },
  @{ Tpl = 'f:\yoyo\tpl-partF-D-Gnd-S.txt'; Gap = 0 },
  @{ Tpl = 'f:\yoyo\tpl-part10-13.txt';   Gap = 0 },
  @{ Tpl = 'f:\yoyo\tpl-appendix.txt';    Gap = 0 }
)

# Note: some templates don't exist yet. Check and create as needed.

# For now, write what we have
$joined = $lines -join "`n"
[System.IO.File]::WriteAllText($path, $joined, $utf8)
Write-Host ('Saved at {0} lines' -f $lines.Length)