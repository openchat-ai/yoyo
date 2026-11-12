# stage-auditlive.ps1 — AUTO-AUDITLIVE: OOW landing + live allow-files + poison fail-closed
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== AUTO-AUDITLIVE ==="

# Need annotated disasm for OOW count contract
if (-not (Test-Path (Join-Path $Root "scripts\fixtures\audit-stub\STUB.disasm.txt"))) {
  & python (Join-Path $Root "scripts\gen-stub-disasm.py")
  if ($LASTEXITCODE -ne 0) { Write-Host "AUDITLIVE status=RED reason=disasm"; exit 1 }
}

& node (Join-Path $Root "yoyo-js\scripts\verify-stub-oow-landing-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "AUDITLIVE status=RED reason=oow"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-live-allow-files-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "AUDITLIVE status=RED reason=live"; exit 1 }

# Poison: flip one byte in a TEMP copy of code.bin at OOW landing → must RED
$code = Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\code.bin"
$tmp = Join-Path $env:TEMP ("yoyo-auditlive-poison-{0}.bin" -f [guid]::NewGuid().ToString("n"))
Copy-Item $code $tmp -Force
$bytes = [IO.File]::ReadAllBytes($tmp)
$off = 0x4255
$bytes[$off] = ($bytes[$off] + 1) -band 0xff
[IO.File]::WriteAllBytes($tmp, $bytes)
& node (Join-Path $Root "yoyo-js\scripts\verify-stub-oow-landing-pin.js") $tmp
if ($LASTEXITCODE -eq 0) {
  Remove-Item $tmp -Force -ErrorAction SilentlyContinue
  Write-Host "AUDITLIVE status=RED reason=poison_did_not_fail"
  exit 1
}
Remove-Item $tmp -Force -ErrorAction SilentlyContinue
Write-Host "AUDITLIVE poison=RED_as_expected"

# Poison live compiler: temp copy wrong hash path — verify live script only checks fixed paths,
# so instead poison by temporarily renaming is too invasive. Covered by OOW poison + bins pin.
Write-Host "AUDITLIVE note=oow_landing+live_files;not_Thompson"
Write-Host "AUDITLIVE status=GREEN"
Write-Host "AUDITLIVE doc=AUTO-AUDITLIVE.md"
exit 0
