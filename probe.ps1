$lines = Get-Content -Encoding UTF8 'f:\yoyo\PROMPT-v3.md'
Write-Host 'Total lines:' $lines.Length
for ($i=5576; $i -lt 5583; $i++) {
  Write-Host ('--- Line {0}: ---' -f ($i+1))
  $lines[$i] | Format-Hex
}