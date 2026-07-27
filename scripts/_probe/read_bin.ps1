# Read the binary file's hex
$f = 'f:\yoyo\scripts\_probe\add_imm_probe.bin'
$bytes = [System.IO.File]::ReadAllBytes($f)
$hex = ($bytes | ForEach-Object { $_.ToString('x2') }) -join ''
Write-Host "Total bytes: $($bytes.Length)"
Write-Host "Hex: $hex"
