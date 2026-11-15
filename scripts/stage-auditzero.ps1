# stage-auditzero.ps1 — close attack paths 1-7 with poison; path 8 HARD_BLOCK
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== AUTO-AUDITZERO ==="

function Assert-PoisonRed([scriptblock]$Probe, [string]$Label) {
  & $Probe
  if ($LASTEXITCODE -eq 0) {
    Write-Host "AUDITZERO status=RED reason=poison_not_red label=$Label"
    exit 1
  }
  Write-Host "AUDITZERO poison=$Label RED_as_expected"
}

# --- positive equals ---
& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "AUDITZERO status=RED reason=bins"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "AUDITZERO status=RED reason=stub"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-beyond-place-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "AUDITZERO status=RED reason=beyond"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-stub-oow-landing-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "AUDITZERO status=RED reason=oow"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-live-allow-files-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "AUDITZERO status=RED reason=live"; exit 1 }

# --- poison fail-closed (temp copies) ---
$code = Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\code.bin"
$data = Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\data.bin"
$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"

# #1 code.bin (full sha + beyond + oow)
$bak = "$code.auditzero.bak"
Copy-Item $code $bak -Force
$b = [IO.File]::ReadAllBytes($code); $b[0] = ($b[0] + 1) -band 0xff; [IO.File]::WriteAllBytes($code, $b)
Assert-PoisonRed { & node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js") } "code_bin"
Copy-Item $bak $code -Force
$b = [IO.File]::ReadAllBytes($code); $b[$b.Length - 1] = ($b[$b.Length - 1] + 1) -band 0xff; [IO.File]::WriteAllBytes($code, $b)
Assert-PoisonRed { & node (Join-Path $Root "yoyo-js\scripts\verify-beyond-place-pin.js") } "code_beyond"
Copy-Item $bak $code -Force
$b = [IO.File]::ReadAllBytes($code); $b[0x4255] = ($b[0x4255] + 1) -band 0xff; [IO.File]::WriteAllBytes($code, $b)
Assert-PoisonRed { & node (Join-Path $Root "yoyo-js\scripts\verify-stub-oow-landing-pin.js") } "code_oow"
Copy-Item $bak $code -Force
Remove-Item $bak -Force

# #2 data.bin
$bak = "$data.auditzero.bak"
Copy-Item $data $bak -Force
$b = [IO.File]::ReadAllBytes($data); $b[0] = ($b[0] + 1) -band 0xff; [IO.File]::WriteAllBytes($data, $b)
Assert-PoisonRed { & node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js") } "data_bin"
Copy-Item $bak $data -Force
Remove-Item $bak -Force

# #3/#4/#6 stub
$bak = "$stub.auditzero.bak"
Copy-Item $stub $bak -Force
$b = [IO.File]::ReadAllBytes($stub); $b[0x500] = ($b[0x500] + 1) -band 0xff; [IO.File]::WriteAllBytes($stub, $b)
Assert-PoisonRed { & node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js") } "stub"
Copy-Item $bak $stub -Force
Remove-Item $bak -Force

# #7 live compiler
$comp = Join-Path $Root "scripts\fixtures\yoyo_compiler.exe"
$bak = "$comp.auditzero.bak"
Copy-Item $comp $bak -Force
$b = [IO.File]::ReadAllBytes($comp); $b[0x1000] = ($b[0x1000] + 1) -band 0xff; [IO.File]::WriteAllBytes($comp, $b)
Assert-PoisonRed { & node (Join-Path $Root "yoyo-js\scripts\verify-live-allow-files-pin.js") } "live_compiler"
Copy-Item $bak $comp -Force
Remove-Item $bak -Force

# matrix 1-7 CLOSED, 8 HARD_BLOCK
& node (Join-Path $Root "yoyo-js\scripts\verify-auditzero-matrix.js")
if ($LASTEXITCODE -ne 0) { Write-Host "AUDITZERO status=RED reason=matrix"; exit 1 }

Write-Host "AUDITZERO note=paths_1_7_CLOSED;path_8_HARD_BLOCK_collusion;not_Thompson"
Write-Host "AUDITZERO status=GREEN"
Write-Host "AUDITZERO doc=AUTO-AUDITZERO.md"
exit 0
