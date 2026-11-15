# stage-stubddc.ps1 — AUTO-STUBDDC: JS stub ≡ Rust stub ≡ pin
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root
. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-STUBDDC (js\equiv rust\equiv pin) ==="

& node (Join-Path $Root "yoyo-js\scripts\build-place-stub.js")
if ($LASTEXITCODE -ne 0) { Write-Host "STUBDDC status=RED reason=js_build"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "STUBDDC status=RED reason=pin"; exit 1 }

$fx = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$sha = (Get-FileHash -Algorithm SHA256 $fx).Hash.ToLower()
Write-Host ("STUBDDC js_pin_EQUAL sha256_prefix={0}" -f $sha.Substring(0, 16))

Push-Location (Join-Path $Root "yoyo-rust\verifier")
try {
  $log = Join-Path $Root "scripts\_stage-stubddc\cargo-test.log"
  New-Item -Force -ItemType Directory (Split-Path $log) | Out-Null
  # Avoid PS native-stderr → error; capture exit via cmd.
  cmd /c "cargo test --lib -- pe_link::tests::place_stub_rust_matches_js_fixture --exact --nocapture > `"$log`" 2>&1"
  $ec = $LASTEXITCODE
  Get-Content $log | Write-Host
  if ($ec -ne 0) {
    Write-Host "STUBDDC status=RED reason=rust_DIFF_js_fixture"
    exit 1
  }
  $text = Get-Content $log -Raw
  if ($text -notmatch 'running 1 test' -or $text -notmatch 'test result: ok\. 1 passed') {
    Write-Host "STUBDDC status=RED reason=rust_test_not_run"
    exit 1
  }
} finally {
  Pop-Location
}

Write-Host "STUBDDC rust_eq_js_fixture=EQUAL"
Write-Host "STUBDDC trust_root=place_stub"
Write-Host "STUBDDC note=linker_ddc_only;not_Thompson"
Write-Host "STUBDDC status=GREEN"
Write-Host "STUBDDC doc=AUTO-STUBDDC.md"
exit 0
