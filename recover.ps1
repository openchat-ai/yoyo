#!/usr/bin/env pwsh
# Recovery: detect inserted templates and remove them.
# An inserted template is identified by the marker "*End of v3.3.9 spec.*"
# followed by the duplicate `# YOYO:` header marker.
# We delete everything from "*End of v3.3.9 spec.*" up to (but not including)
# the next `# YOYO: Engineering Specification (v3.3.8)` header line.

$ErrorActionPreference = 'Stop'
$path = 'f:\yoyo\PROMPT-v3.md'
$utf8 = New-Object System.Text.UTF8Encoding($false)
$text = [System.IO.File]::ReadAllText($path, $utf8)
$lines = $text -split "`r?`n"
Write-Host ('Start: {0} lines' -f $lines.Length)

# Strategy:
# 1. The first v3.3.9 header (line 3) and the immediately following content are the NEW header. Keep.
# 2. Each subsequent "## Part X" / "## Appendix X" appears once. But duplicated inserted sections exist.
# 3. Detect by pattern: a sequence of lines that begins with "*End of v3.3.9 spec*" and ends with
#    a "# YOYO:" header line. Remove that sequence (inclusive of both markers).

# Find all duplicate-header positions
$dupHeaderIdxs = @()
for ($i = 0; $i -lt $lines.Length; $i++) {
  $line = $lines[$i]
  if ($line -like '# YOYO: Engineering Specification*') { $dupHeaderIdxs += $i }
}
Write-Host ('Header markers at lines: {0}' -f (($dupHeaderIdxs | ForEach-Object { $_ + 1 }) -join ', '))

# First header (idx 0): my new v3.3.9. KEEP.
# Subsequent headers: identify the inserted blocks.

# Find all "End of v3.3.9 spec" markers
$endSpecIdxs = @()
for ($i = 0; $i -lt $lines.Length; $i++) {
  if ($lines[$i] -like '*End of v3.3.9 spec*') { $endSpecIdxs += $i }
}
Write-Host ('End-of-spec markers at lines: {0}' -f (($endSpecIdxs | ForEach-Object { $_ + 1 }) -join ', '))

# For each "End of v3.3.9 spec" marker, find the next "# YOYO:" header and delete [endSpec..nextHeader-1]
$toDelete = @()
foreach ($endIdx in $endSpecIdxs) {
  # Find next # YOYO: header after endIdx
  $nextHdr = -1
  for ($j = $endIdx + 1; $j -lt $lines.Length; $j++) {
    if ($lines[$j] -like '# YOYO: Engineering Specification*') { $nextHdr = $j; break }
  }
  if ($nextHdr -gt $endIdx) {
    # Delete [endIdx..nextHdr-1] inclusive
    $toDelete += @{ Start = $endIdx; End = $nextHdr - 1 }
  }
}

Write-Host ('Blocks to delete: {0}' -f $toDelete.Count)
foreach ($b in $toDelete) {
  Write-Host ('  Delete lines {0}..{1}' -f ($b.Start + 1), ($b.End + 1))
}

# Apply deletions (in reverse order so indices stay valid)
$toDelete = $toDelete | Sort-Object -Property Start -Descending
foreach ($b in $toDelete) {
  $start = $b.Start
  $end = $b.End
  $len = $end - $start + 1
  $lines = $lines[0..($start-1)] + $lines[($end+1)..($lines.Length-1)]
}

# Now also clean up: there may be leftover duplicate sections that don't have the "End of v3.3.9" marker.
# These are the inserts for parts (like Part N, Part L, etc.) that I prepended.
# Detect: any sequence between a "# YOYO:" header and the next "## Part N" or similar that
# is duplicated. For now, manual check: lines 1278+ duplicates Part N which starts at line 209.
# We need to remove [1278..next-##Part N - 1] style duplicates.

# Look for duplicate Part N: there should be only ONE Part N header.
# Find all "## Part N: Normative Conventions" markers
$partNIdxs = @()
for ($i = 0; $i -lt $lines.Length; $i++) {
  if ($lines[$i] -like '## Part N: Normative Conventions*') { $partNIdxs += $i }
}
Write-Host ('Part N markers at lines: {0}' -f (($partNIdxs | ForEach-Object { $_ + 1 }) -join ', '))

# Keep the first, delete the rest
$partNToDelete = @()
for ($k = 1; $k -lt $partNIdxs.Length; $k++) {
  $start = $partNIdxs[$k]
  $end = $partNIdxs[$k+1] - 1
  if ($end -le 0) { $end = $lines.Length - 1 }
  $partNToDelete += @{ Start = $start; End = $end }
}
$partNToDelete = $partNToDelete | Sort-Object -Property Start -Descending
foreach ($b in $partNToDelete) {
  Write-Host ('  Deleting duplicate Part N at line {0}' -f ($b.Start + 1))
  $lines = $lines[0..($b.Start-1)] + $lines[($b.End+1)..($lines.Length-1)]
}

# Repeat for each section that may be duplicated
$sectionHeaders = @(
  '## Part L:',
  '## Part 0:',
  '## Part 1:',
  '## Part 2:',
  '## Part 3:',
  '## Part 4:',
  '## Part 4S:',
  '## Part G:',
  '## Part 5:',
  '## Part 5B:',
  '## Part 6:',
  '## Part 7:',
  '## Part 8:',
  '## Part 9:',
  '## Part E:',
  '## Part F:',
  '## Part Deduce:',
  '## Part Gnd:',
  '## Part S:',
  '## Part 10:',
  '## Part 11:',
  '## Part 12:',
  '## Part 13:',
  '## Part 14:',
  '## Part 15:',
  '## Part 16:',
  '## Appendix A:',
  '## Appendix B:',
  '## Appendix C:',
  '## Appendix D:',
  '## Appendix E:',
  '## Appendix F:',
  '## Appendix G:',
  '## Appendix H:',
  '## Appendix T:',
  '## Appendix Bib:',
  '## Appendix CH:'
)

foreach ($hdr in $sectionHeaders) {
  $matches = @()
  for ($i = 0; $i -lt $lines.Length; $i++) {
    if ($lines[$i] -like ($hdr + '*')) { $matches += $i }
  }
  if ($matches.Length -gt 1) {
    # Keep first, delete later ones
    Write-Host ('{0} has {1} occurrences at lines {2}' -f $hdr, $matches.Length, (($matches | ForEach-Object { $_ + 1 }) -join ', '))
    $sorted = $matches | Sort-Object -Descending
    for ($k = 1; $k -lt $sorted.Length; $k++) {
      $start = $sorted[$k]
      $end = $sorted[$k-1] - 1
      if ($end -lt $start) { continue }
      Write-Host ('  Deleting duplicate at line {0}..{1}' -f ($start + 1), ($end + 1))
      $lines = $lines[0..($start-1)] + $lines[($end+1)..($lines.Length-1)]
    }
  }
}

# Save
$joined = $lines -join "`n"
[System.IO.File]::WriteAllText($path, $joined, $utf8)
Write-Host ('After dedup: {0} lines. Saved.' -f $lines.Length)