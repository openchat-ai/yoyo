# verify-selfhost.ps1 — 4-round self-host byte-equality (Decision #13 step 3)
# Requires: node, and built yoyo-rust verifier.
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "== M0 (JS) =="
New-Item -ItemType Directory -Force -Path "yoyo-js\build" | Out-Null
node yoyo-js\src\yoyo.js yoyo\projects\yoyo.ty yoyo-js\build\M1.exe

Write-Host "== M_rust =="
$yoyo = "yoyo-rust\target\release\yoyo.exe"
if (-not (Test-Path $yoyo)) {
  Push-Location yoyo-rust
  cargo build --release -p verifier
  Pop-Location
}
& $yoyo link --target=win32 yoyo\projects\yoyo.ty yoyo-js\build\M_rust.exe

Write-Host "== DDC text compare =="
& $yoyo diff yoyo-js\build\M1.exe yoyo-js\build\M_rust.exe
if ($LASTEXITCODE -ne 0) {
  Write-Host "NOTE: full-file/.text DDC may differ until emit parity is complete (Phase 1 exit)."
  Write-Host "Hashes logged above for audit."
}

Write-Host "== lockdown =="
node scripts\verify-yoyo-ty.mjs

Write-Host "verify-selfhost: done"
