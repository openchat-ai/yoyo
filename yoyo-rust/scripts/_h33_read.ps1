$ErrorActionPreference = 'Stop'
$bin = 'F:\_scratch_h33_out.bin'
$fi = Get-Item -LiteralPath $bin
Write-Host ("LENGTH=" + $fi.Length)
$bytes = [System.IO.File]::ReadAllBytes($bin)
$sb = New-Object System.Text.StringBuilder
foreach ($byte in $bytes) { [void]$sb.AppendFormat("{0:X2}", $byte) }
Write-Host ("HEX=" + $sb.ToString().ToLower())
Write-Host ("COUNT=" + $bytes.Length)
