# stage-plantpoison.ps1 — AUTO-PLANTPOISON: poisoned code-plant must be caught
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root
. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-PLANTPOISON ==="

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-plant-sot.js")
if ($LASTEXITCODE -ne 0) { Write-Host "PLANTPOISON status=RED reason=baseline_Y14"; exit 1 }

$placeTyb = Join-Path $Root "yoyo\projects\yoyo-plant-place.tyb"
$bak = Join-Path $Root "scripts\_stage-plantpoison\yoyo-plant-place.tyb.bak"
New-Item -Force -ItemType Directory (Split-Path $bak) | Out-Null
Copy-Item -Force $placeTyb $bak

try {
  $b = [IO.File]::ReadAllBytes($placeTyb)
  # Flip a payload byte inside first A1 record (offset 8+2) if large enough
  $ix = [Math]::Min($b.Length - 1, 10)
  $b[$ix] = $b[$ix] -bxor 0xFF
  [IO.File]::WriteAllBytes($placeTyb, $b)

  & node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-plant-sot.js")
  if ($LASTEXITCODE -eq 0) {
    Write-Host "PLANTPOISON status=RED reason=Y14_missed_poison"
    exit 1
  }
  Write-Host "PLANTPOISON Y14_caught_poison=RED_OK"

  # Derive poisoned code → place must miss C1 pin
  $work = Join-Path $Root "scripts\_stage-plantpoison"
  $derivedCode = Join-Path $work "code_poison.bin"
  & node (Join-Path $Root "yoyo-js\scripts\derive-code-from-plant-tyb.js") $derivedCode
  if ($LASTEXITCODE -ne 0) {
    Write-Host "PLANTPOISON derive_fail_closed=OK"
  } else {
    $data = Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\data.bin"
    $stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
    $pod = Join-Path $work "place"
    & (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $pod -Bootstrap $stub -CodeBin $derivedCode -DataBin $data
    if ($LASTEXITCODE -eq 0 -and (Test-Path (Join-Path $pod "yoyo_compiler.exe"))) {
      & node (Join-Path $Root "yoyo-js\scripts\verify-place-c1-pin.js") (Join-Path $pod "yoyo_compiler.exe")
      if ($LASTEXITCODE -eq 0) {
        Write-Host "PLANTPOISON status=RED reason=poison_place_still_pin_EQUAL"
        exit 1
      }
      Write-Host "PLANTPOISON poison_place_DIFF_pin=RED_OK"
    } else {
      Write-Host "PLANTPOISON poison_place_fail_closed=OK"
    }
  }
} finally {
  Copy-Item -Force $bak $placeTyb
}

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-plant-sot.js")
if ($LASTEXITCODE -ne 0) { Write-Host "PLANTPOISON status=RED reason=restore_Y14"; exit 1 }

Write-Host "PLANTPOISON note=content_sot_detection;not_Thompson"
Write-Host "PLANTPOISON status=GREEN"
Write-Host "PLANTPOISON doc=AUTO-PLANTPOISON.md"
exit 0
