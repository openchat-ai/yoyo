$ErrorActionPreference = 'Continue'
Set-Location f:\yoyo\yoyo-rust
$out = & cargo run -q -p verifier --bin yoyo -- link --target=stub F:/yoyo/yoyo/tests/golden/_scratch_h33.ty F:/yoyo/_scratch_h33_out.bin 2>&1
$ec = $LASTEXITCODE
Write-Host "EXIT=$ec"
Write-Host "--- cargo output (last 20 non-warning lines) ---"
$out | Where-Object { $_ -is [string] -and $_.Trim() -ne '' -and $_ -notmatch 'warning|^-->' -and $_ -notmatch '^$' } | Select-Object -Last 20
$bin = 'F:\_scratch_h33_out.bin'
if (Test-Path -LiteralPath $bin) {
  $bytes = [System.IO.File]::ReadAllBytes($bin)
  Write-Host "BIN_LEN=$($bytes.Length)"
  $hex = ($bytes | ForEach-Object { '{0:X2}' -f $_ }) -join ''
  Write-Host "BIN_HEX=$hex"
} else {
  Write-Host "BIN_NOT_PRESENT"
}
