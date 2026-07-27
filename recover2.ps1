#!/usr/bin/env pwsh
# Aggressive dedup: delete ALL content from the second occurrence of any section header
# onwards, UNTIL the line BEFORE the next section header (i.e., keep the last/rightmost copy).

$ErrorActionPreference = 'Stop'
$path = 'f:\yoyo\PROMPT-v3.md'
$utf8 = New-Object System.Text.UTF8Encoding($false)
$text = [System.IO.File]::ReadAllText($path, $utf8)
$lines = $text -split "`r?`n"
Write-Host ('Start: {0} lines' -f $lines.Length)

# Pass 1: Find all lines matching "# YOYO: Engineering Specification" header.
# After the first occurrence (idx 2 = v3.3.9), EVERY subsequent "# YOYO:" header marks
# the start of a "duplicate inserted block." Delete everything from the previous occurrence
# through (but not including) the next section header.

$headerIdxs = @()
for ($i = 0; $i -lt $lines.Length; $i++) {
  if ($lines[$i] -like '# YOYO: Engineering Specification*') { $headerIdxs += $i }
}
Write-Host ('Header markers: {0}' -f $headerIdxs.Count)

# Header idx 0 = v3.3.8 original, idx 1 = v3.3.9 new (kept). idx 2+ = duplicates to remove.
# For each duplicate, delete from (headerIdx[k-1]+something) to (headerIdx[k]-1)
# Actually we want to keep the LAST header (the v3.3.9 just before the original content).
# Let me think: original structure was:
#   line 1: # YOYO: ... v3.3.8 (original)
#   ... original content ...
# Then insertions:
#   line 3: # YOYO: ... v3.3.9 (my new)
#   ... my template content ...
#   line 13: # YOYO: ... v3.3.8 (original header, kept)
#   ... original content (a few lines) ...
#   ... more template inserts interleaved with original sections ...

# Strategy: Identify all "# YOYO:" headers. For duplicates at idx >= 2, delete from idx-1 backwards
# until we hit a non-duplicate header OR a section anchor like "## Part N".
# Actually, simpler: each `# YOYO: ... v3.3.8` after idx 2 is preceded by my inserted template.
# The template ends at "*End of v3.3.9 spec*" line. So delete from "*End of v3.3.9 spec*" 
# (or just before the duplicate header) to (just before the next section like "## Part N:" or similar).

# Simplest: remove all `# YOYO:` headers EXCEPT the very first v3.3.9 line, and remove everything
# in between consecutive headers. After headers[idx+1], the next "# YOYO:" header at idx+2 marks
# the start of the next duplicate block.

# Process headers in pairs:
# - Between header[0] and header[1]: keep header[0] (v3.3.8 original) + everything in between except header[1]... 
#   Actually header[1] is my v3.3.9 template start.
# - I want: KEEP header[1] (v3.3.9), DELETE everything else until we get to header[1]+content.

# Final structure I want:
# header v3.3.9 (with my changelog) + content of original (v3.3.8 text + all parts) - duplicates

# The duplicated content is mostly:
# - At line 13-22: duplicate v3.3.8 header block (12 lines)
# - At lines 1067-1069: leftover "*End of v3.3.9 spec*" markers (3 lines)
# - At lines 1278-1279: duplicate Part N start
# - All the way through.

# Let me just delete lines 13-1277 (which is the v3.3.8 header + everything after until 
# the start of the second Part N duplicate). But Part 4S ends and Part G begins somewhere in there.

# Actually after first dedup:
# Lines 1-12: my v3.3.9 header (good)
# Lines 13-1266: messed up content
# Lines 1267-1276: end of file content (last Part 4S section?)

# Let me just delete lines 13 onwards and reinsert the clean originals from a known good backup.
# But I don't have a backup. Let me read line 13-200 to see what's there.

Write-Host '--- Lines 13-25 ---'
for ($i = 12; $i -lt 25; $i++) {
  Write-Host ('{0,5}: {1}' -f ($i+1), $lines[$i])
}