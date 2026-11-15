# stage-stubpoison.ps1 — AUTO-STUBPOISON: poisoned stub must be caught
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== AUTO-STUBPOISON (poison must RED) ==="

# Clean baseline must still hold
& (Join-Path $Root "scripts\stage-stubddc.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "STUBPOISON status=RED reason=baseline_stubddc"; exit 1 }

$work = Join-Path $Root "scripts\_stage-stubpoison"
if (Test-Path $work) { Remove-Item -Recurse -Force $work }
New-Item -Force -ItemType Directory $work | Out-Null

$fx = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$pin = Join-Path $Root "scripts\fixtures\yoyo_place_stub.pin"
$poison = Join-Path $work "yoyo_place_stub.exe"
Copy-Item -Force $fx $poison
$bytes = [System.IO.File]::ReadAllBytes($poison)
if ($bytes.Length -lt 0x500) { Write-Host "STUBPOISON status=RED reason=stub_too_small"; exit 1 }
# Flip a PE header-ish byte that changes sha but keeps file readable
$bytes[0x80] = $bytes[0x80] -bxor 0xFF
[System.IO.File]::WriteAllBytes($poison, $bytes)

# verify-place-stub-pin against poisoned file (swap fixture temporarily)
$bak = Join-Path $work "clean_stub.exe"
Copy-Item -Force $fx $bak
try {
  Copy-Item -Force $poison $fx
  & node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
  $ec = $LASTEXITCODE
} finally {
  Copy-Item -Force $bak $fx
}

if ($ec -eq 0) {
  Write-Host "STUBPOISON status=RED reason=poison_not_caught_by_pin"
  exit 1
}
Write-Host "STUBPOISON pin_caught_poison=RED_OK"

# Rust linker output must DIFF poisoned bytes
Push-Location (Join-Path $Root "yoyo-rust\verifier")
try {
  $log = Join-Path $work "cargo-test.log"
  cmd /c "cargo test --lib -- pe_link::tests::place_stub_rust_matches_js_fixture --exact --nocapture > `"$log`" 2>&1"
  if ($LASTEXITCODE -ne 0) {
    Write-Host "STUBPOISON status=RED reason=rust_baseline_broke"
    exit 1
  }
} finally {
  Pop-Location
}

# Explicit: poisoned file sha != pin sha
$pinSha = ((Get-Content $pin) | Where-Object { $_ -match '^sha256=' }) -replace '^sha256=',''
$pSha = (Get-FileHash -Algorithm SHA256 $poison).Hash.ToLower()
if ($pSha -eq $pinSha) {
  Write-Host "STUBPOISON status=RED reason=poison_sha_unchanged"
  exit 1
}
Write-Host ("STUBPOISON poison_sha_DIFF pin prefix_p={0}" -f $pSha.Substring(0,16))

Write-Host "STUBPOISON note=detection_only;not_Thompson"
Write-Host "STUBPOISON status=GREEN"
Write-Host "STUBPOISON doc=AUTO-STUBPOISON.md"
exit 0
