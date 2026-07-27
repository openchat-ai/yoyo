$ErrorActionPreference = 'Stop'
Set-Location F:\yoyo\yoyo-rust
$bin = 'F:\_scratch_h33_out.bin'
# Link run, suppress stderr (only warnings there)
$ErrorActionPreference = 'Continue'
$out = & cargo run -q -p verifier --bin yoyo -- link --target=stub F:/yoyo/yoyo/tests/golden/_scratch_h33.ty $bin 2>$null
$ec = $LASTEXITCODE
Write-Host "EXIT=$ec"
$out | Where-Object { $_ -is [string] -and $_.Trim() -ne '' } | ForEach-Object { Write-Host ("C: " + $_) }
# Now read the file
if (Test-Path -LiteralPath $bin) {
  $fi = Get-Item -LiteralPath $bin
  Write-Host ("SIZE=" + $fi.Length)
  $bytes = [System.IO.File]::ReadAllBytes($bin)
  $sb = New-Object System.Text.StringBuilder
  foreach ($byte in $bytes) { [void]$sb.AppendFormat("{0:X2}", $byte) }
  $hex = $sb.ToString().ToLower()
  Write-Host ("HEX=" + $hex)
  Write-Host ("COUNT=" + $bytes.Length)
} else {
  Write-Host "NO_BIN_AT_LINK_TIME"
}
