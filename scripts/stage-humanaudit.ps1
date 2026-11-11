# stage-humanaudit.ps1 — AUTO-HUMANAUDIT: regen audit pack ≡ pins
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== AUTO-HUMANAUDIT ==="

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "HUMANAUDIT status=RED reason=stub_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\gen-human-audit-stub.js")
if ($LASTEXITCODE -ne 0) { Write-Host "HUMANAUDIT status=RED reason=gen"; exit 1 }

& python (Join-Path $Root "scripts\gen-stub-disasm.py")
if ($LASTEXITCODE -ne 0) { Write-Host "HUMANAUDIT status=RED reason=disasm"; exit 1 }
# First run may WRITE callseq pin — re-run once so EQUAL is enforced in same gate
& python (Join-Path $Root "scripts\gen-stub-disasm.py")
if ($LASTEXITCODE -ne 0) { Write-Host "HUMANAUDIT status=RED reason=disasm_recheck"; exit 1 }
foreach ($f in @('STUB.disasm.txt','STUB.WALKTHROUGH.md','STUB.AI.md','STUB.CALLSEQ.pin')) {
  if (-not (Test-Path (Join-Path $Root "scripts\fixtures\audit-stub\$f"))) {
    Write-Host "HUMANAUDIT status=RED reason=missing_$f"
    exit 1
  }
}
# Fail-closed: walkthrough must not list FLAG opens; AI verdict must be GREEN
$walk = Get-Content (Join-Path $Root "scripts\fixtures\audit-stub\STUB.WALKTHROUGH.md") -Raw
if ($walk -match '\|\s+\*\*FLAG\*\*\s+\|') {
  Write-Host "HUMANAUDIT status=RED reason=createfile_path_FLAG"
  exit 1
}
$ai = Get-Content (Join-Path $Root "scripts\fixtures\audit-stub\STUB.AI.md") -Raw
if ($ai -notmatch '\*\*VERDICT:\s*GREEN\*\*') {
  Write-Host "HUMANAUDIT status=RED reason=ai_verdict_not_green"
  exit 1
}

$dir = Join-Path $Root "scripts\fixtures\audit-stub"
foreach ($f in @('AUDIT-STUB.md','hdr_0x200.bin','iat_0x100.bin','text_raw.bin','MANIFEST.sha256')) {
  if (-not (Test-Path (Join-Path $dir $f))) {
    Write-Host "HUMANAUDIT status=RED reason=missing_$f"
    exit 1
  }
}

& node (Join-Path $Root "yoyo-js\scripts\verify-pe-slices-pin.js") $stub (Join-Path $Root "scripts\fixtures\yoyo_stub_slices.pin")
if ($LASTEXITCODE -ne 0) { Write-Host "HUMANAUDIT status=RED reason=slices"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-a1-imports-pin.js") $stub
if ($LASTEXITCODE -ne 0) { Write-Host "HUMANAUDIT status=RED reason=a1"; exit 1 }

Write-Host "HUMANAUDIT note=human_pack_for_stub_TCB;bins_not_eyeball;not_Thompson"
Write-Host "HUMANAUDIT status=GREEN"
Write-Host "HUMANAUDIT doc=AUTO-HUMANAUDIT.md"
exit 0
