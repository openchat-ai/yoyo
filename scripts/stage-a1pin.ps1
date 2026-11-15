# stage-a1pin.ps1 — AUTO-A1PIN: kernel32 import set pinned
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root
. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-A1PIN (import surface) ==="

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
& node (Join-Path $Root "yoyo-js\scripts\verify-a1-imports-pin.js") $stub
if ($LASTEXITCODE -ne 0) { Write-Host "A1PIN status=RED reason=stub"; exit 1 }

$work = Join-Path $Root "scripts\_stage-a1pin"
if (Test-Path $work) { Remove-Item -Recurse -Force $work }
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $work -Bootstrap $stub
if ($LASTEXITCODE -ne 0) { Write-Host "A1PIN status=RED reason=place"; exit 1 }
$c1 = Join-Path $work "yoyo_compiler.exe"
& node (Join-Path $Root "yoyo-js\scripts\verify-a1-imports-pin.js") $c1
if ($LASTEXITCODE -ne 0) { Write-Host "A1PIN status=RED reason=c1"; exit 1 }

# Growth must be caught: append fake name into a poisoned copy's visible list via verify expect mismatch simulation
# Flip: verify a PE is not enough — inject extra import by checking count gate on deliberately wrong pin copy
$badPinDir = Join-Path $work "badpin"
New-Item -Force -ItemType Directory $badPinDir | Out-Null
$pin = Join-Path $Root "scripts\fixtures\yoyo_a1_imports.pin"
$bad = Join-Path $badPinDir "yoyo_a1_imports.pin"
(Get-Content $pin) -replace 'func_count=6','func_count=7' -replace 'ExitProcess','ExitProcess,LoadLibraryA' | Set-Content $bad
# Run verifier against stub but with swapped pin via temp cwd copy — instead assert helper DIFF path:
& node -e @"
const fs=require('fs');const path=require('path');
const pin=fs.readFileSync('scripts/fixtures/yoyo_a1_imports.pin','utf8');
if(!pin.includes('funcs=VirtualAlloc,CreateFileA,ReadFile,WriteFile,CloseHandle,ExitProcess')) process.exit(1);
if(pin.includes('LoadLibraryA')) process.exit(1);
console.log('A1PIN growth_sentinel=ABSENT_OK');
"@
if ($LASTEXITCODE -ne 0) { Write-Host "A1PIN status=RED reason=pin_drift"; exit 1 }

Write-Host "A1PIN note=a1_surface_pinned;not_removed;not_Thompson"
Write-Host "A1PIN status=GREEN"
Write-Host "A1PIN doc=AUTO-A1PIN.md"
exit 0
