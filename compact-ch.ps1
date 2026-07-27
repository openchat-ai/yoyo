#!/usr/bin/env pwsh
# Read file as raw UTF-8
$path = 'f:\yoyo\PROMPT-v3.md'
$bytes = [System.IO.File]::ReadAllBytes($path)
$utf8 = New-Object System.Text.UTF8Encoding($false)
$text = $utf8.GetString($bytes)
$lines = $text -split "`r?`n"
Write-Host ('Start: {0} lines' -f $lines.Length)

function Replace-Range {
  param(
    [string[]]$Lines,
    [int]$StartLine,
    [int]$EndLine,
    [string]$NewContent
  )
  $before = $Lines[0..($StartLine-2)]
  $after  = $Lines[$EndLine..($Lines.Length-1)]
  return @($before) + @($NewContent) + @($after)
}

# Locate Appendix CH
$idx = -1
for ($i = 0; $i -lt $lines.Length; $i++) {
  if ($lines[$i] -like '## Appendix CH: Prior Changelog Archive (NON-NORMATIVE, slim)*') { $idx = $i; break }
}
if ($idx -lt 0) { throw 'Could not locate Appendix CH header' }
Write-Host ('Appendix CH header at line {0}' -f ($idx+1))

# Find the End-of-v3.3.9 marker
$endIdx = -1
for ($i = $idx; $i -lt $lines.Length; $i++) {
  if ($lines[$i] -like '*End of v3.3.9 spec*') { $endIdx = $i; break }
}
if ($endIdx -lt 0) { throw 'Could not locate End-of-v3.3.9 marker' }
Write-Host ('End of v3.3.9 marker at line {0}' -f ($endIdx+1))

$chStart = $idx
$chEndInclusive = $endIdx - 2

# Build the new slim CH content from the bytes of a UTF-8 file (no encoding issues)
$templatePath = 'f:\yoyo\ch-template.txt'
$newContent = [System.IO.File]::ReadAllText($templatePath, $utf8)

$lines = Replace-Range -Lines $lines -StartLine ($chStart+1) -EndLine ($chEndInclusive+1) -NewContent $newContent

Write-Host ('After Appendix CH: {0} lines' -f $lines.Length)

$joined = $lines -join "`n"
[System.IO.File]::WriteAllText($path, $joined, $utf8)
Write-Host 'Saved.'