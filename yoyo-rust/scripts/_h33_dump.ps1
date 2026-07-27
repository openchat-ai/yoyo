$ErrorActionPreference = 'Continue'
$bin = 'F:\_scratch_h33_out.bin'
Write-Host "checking $bin"
if (-not (Test-Path -LiteralPath $bin)) {
  Write-Host "NOT_FOUND"
  exit 1
}
$fi = Get-Item -LiteralPath $bin
Write-Host "size=$($fi.Length)"
$bytes = [System.IO.File]::ReadAllBytes($bin)
Write-Host "len=$($bytes.Length)"
$hex = ($bytes | ForEach-Object { '{0:X2}' -f $_ }) -join ''
Write-Host "hex=$hex"
