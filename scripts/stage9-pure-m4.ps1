# stage9-pure-m4.ps1 — Stage 9-C: M4 via H_00 in-process (no bootstrap --selfhost)
# Seed: yoyo link → gen1 (H_00 entry, not genNrt startup wrapper)
# Chain: gen1 → gen2 → gen3 → gen4 (each zero-arg H_00 runtime compile)
# Parity: gen4 ≡ gen3_direct (.text section-ddc); gen3_direct = bootstrap WITHOUT --selfhost
# Trust: M3→M4 algebra runs inside prior YOYO PE; host never calls bootstrap --selfhost here.
param(
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

$WorkDir = Join-Path $Root "scripts\_stage9-pure-m4"
New-Item -ItemType Directory -Force -Path $WorkDir | Out-Null

$Ty = Join-Path $Root "yoyo\projects\yoyo.ty"
$Tyb = Join-Path $Root "yoyo\projects\yoyo.tyb"
$Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
$RuntimeDllBuilt = Join-Path $Root "yoyo-rust\target\release\yoyo_runtime.dll"

if (-not (Test-Path $Yoyo) -or -not (Test-Path $RuntimeDllBuilt)) {
    if ($SkipBuild) { throw "missing yoyo.exe or yoyo_runtime.dll (and -SkipBuild)" }
    Write-Host "== build yoyo + yoyo-runtime (release) =="
    Push-Location (Join-Path $Root "yoyo-rust")
    cargo build --release -p verifier
    if ($LASTEXITCODE -ne 0) { throw "verifier build failed" }
    cargo build --release -p yoyo-runtime
    if ($LASTEXITCODE -ne 0) { throw "yoyo-runtime build failed" }
    Pop-Location
}

if (-not (Test-Path $Tyb)) {
    Write-Host "== ty2tyb =="
    python (Join-Path $Root "scripts\ty2tyb.py")
    if (-not (Test-Path $Tyb)) { throw "ty2tyb failed: missing $Tyb" }
}

$Gen1 = Join-Path $WorkDir "gen1.exe"
$Gen2 = Join-Path $WorkDir "gen2.exe"
$Gen3 = Join-Path $WorkDir "gen3.exe"
$Gen4 = Join-Path $WorkDir "gen4.exe"
$Gen3Direct = Join-Path $WorkDir "gen3_direct.exe"
$InputTyb = Join-Path $WorkDir "input.tyb"
$InputKy = Join-Path $WorkDir "input.ky"

Copy-Item -Force $Tyb $InputTyb
Copy-Item -Force $Ty $InputKy

$chainGreen = $false
$parityEqual = $false
$trustSha = ""
$textSha = ""

Write-Host ""
Write-Host "=== Stage 9-C: seed gen1 via yoyo link (H_00 path, NOT bootstrap --selfhost) ==="
Remove-Item $Gen1 -Force -ErrorAction SilentlyContinue
& $Yoyo link --target=win32 $Ty $Gen1
if ($LASTEXITCODE -ne 0 -or -not (Test-Path $Gen1)) {
    Write-Host "Stage 9-C: RED (gen1 link failed)"
    exit 1
}
Write-Host "gen1: $((Get-Item $Gen1).Length) bytes (PE entry → H_00)"

function Invoke-H00Gen {
    param(
        [string]$ExePath,
        [string]$OutPath,
        [string]$Label
    )
    Push-Location $WorkDir
    try {
        if (Test-Path "output.exe") { Remove-Item "output.exe" }
        Write-Host "running $Label (zero-arg H_00)..."
        & $ExePath
        $ec = $LASTEXITCODE
        if ($ec -eq 0xC0000005) {
            Write-Host "${Label}: RED (STATUS_ACCESS_VIOLATION 0xC0000005)"
            return $false
        }
        if ($ec -ne 0 -or -not (Test-Path "output.exe")) {
            Write-Host "${Label}: RED (exit=$ec, no output.exe)"
            return $false
        }
        Copy-Item -Force "output.exe" $OutPath
        Write-Host "${Label}: GREEN ($((Get-Item $OutPath).Length) bytes)"
        return $true
    } finally {
        Pop-Location
    }
}

Write-Host ""
Write-Host "=== gen1 → gen2 (H_00 in-process) ==="
if (-not (Invoke-H00Gen -ExePath $Gen1 -OutPath $Gen2 -Label "gen1→gen2")) { exit 1 }

Write-Host ""
Write-Host "=== gen2 → gen3 (H_00 in-process) ==="
if (-not (Invoke-H00Gen -ExePath $Gen2 -OutPath $Gen3 -Label "gen2→gen3")) { exit 1 }

Write-Host ""
Write-Host "=== gen3 → gen4 (H_00 in-process; M3→M4 without gen3rt) ==="
if (-not (Invoke-H00Gen -ExePath $Gen3 -OutPath $Gen4 -Label "gen3→gen4")) { exit 1 }
$chainGreen = $true

Write-Host ""
Write-Host "=== reference: gen3_direct via bootstrap (no --selfhost) ==="
if (Test-Path $Gen3Direct) { Remove-Item $Gen3Direct }
& $Yoyo bootstrap $InputTyb $Gen3Direct
if ($LASTEXITCODE -ne 0 -or -not (Test-Path $Gen3Direct)) {
    Write-Host "Stage 9-C: RED (gen3_direct bootstrap failed)"
    exit 1
}
Write-Host "gen3_direct: $((Get-Item $Gen3Direct).Length) bytes"

Write-Host ""
Write-Host "=== trust chain: gen4 vs gen3_direct (.text section-ddc) ==="
& $Yoyo diff $Gen4 $Gen3Direct 2>&1 | ForEach-Object { Write-Host $_ }
if ($LASTEXITCODE -eq 0) {
    $parityEqual = $true
    $trustSha = (Get-FileHash -Algorithm SHA256 -Path $Gen4).Hash.Substring(0, 8).ToLower()
    # Capture .text hash line from prior diff output is awkward; re-diff quiet via exit only.
    Write-Host "gen4 ≡ gen3_direct (.text DDC): EQUAL (file sha256 prefix $trustSha)"
} else {
    Write-Host "gen4 ≡ gen3_direct (.text DDC): DIFF"
}

Write-Host ""
Write-Host "=== gen12 window: gen3 vs gen4 (.text DDC) ==="
& $Yoyo diff $Gen3 $Gen4 2>&1 | ForEach-Object {
    Write-Host "  $_"
    if ($_ -match '^hash_a:\s*([0-9a-fA-F]{8})') {
        $textSha = $Matches[1].ToLower()
    }
}

Write-Host ""
Write-Host "=== summary ==="
Write-Host "H_00 chain gen1→gen4: $(if ($chainGreen) { 'GREEN' } else { 'RED' })"
Write-Host "gen4 DDC parity:      $(if ($parityEqual) { "EQUAL (file sha256 prefix $trustSha)" } else { 'DIFF or N/A' })"
Write-Host "bootstrap --selfhost:  NOT USED (Stage 9-C gate)"
Write-Host "Stage 9-C:            $(if ($chainGreen -and $parityEqual) { 'may check [x]' } else { 'keep [ ]' })"
Write-Host ""
Write-Host "Trust chain: M4 algebra completed inside H_00-patched YOYO PEs (gen1→gen4)."
Write-Host "  Seed = yoyo link (H_00 extract stub + embedded runtime DLL)"
Write-Host "  Reference = yoyo bootstrap input.tyb WITHOUT --selfhost"
Write-Host "  gen4 = gen3 H_00 runtime output (no genNrt entry wrapper)"
Write-Host "Remaining host surface (honest):"
Write-Host "  - host link/bootstrap seed + gen3_direct reference"
Write-Host "  - embedded yoyo_runtime.dll (Rust compile) inside each genN"
Write-Host "  - Linux pure M4: see scripts/stage10-linux-pure-m4.sh (ELF H_00; Stage 10-B)"
if ($textSha) {
    Write-Host "  gen4 .text SHA256 prefix: $textSha"
} elseif ($trustSha) {
    Write-Host "  gen4 file SHA256 prefix: $trustSha"
}

if ($chainGreen -and $parityEqual) { exit 0 } else { exit 1 }
