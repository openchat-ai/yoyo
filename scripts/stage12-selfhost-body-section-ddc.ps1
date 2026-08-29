# stage12-selfhost-body-section-ddc.ps1 鈥?Stage 12-B: selfhost body section-ddc
# Fail-closed gate:
#   - Rust `yoyo test body-ddc` (gen1鈮en2 enlarged window + H_00 stub pin)
#   - Three-peer full yoyo.ty: PE startup + post-H_00 shared handlers EQUAL
#     (shrinks "whole .text DIFF 鈬?outside still green" blind spot)
#   - compared window >= 17013 (startup 13 + shared handlers >= 17000)
#   - Rust H_00 LoadLibrary stub present (stub_tail_nonzero >= 40; matches body-ddc)
# Honest remaining DIFF (documented, not fail):
#   - H_00 entry slot (Rust JMP+NOPs vs JS/asm SET+RET)
#   - Rust-only H_00 LoadLibrary stub tail
#   - Embedded Rust runtime DLL / .data host surface / LoadLibrary
#   - Full .text section-ddc across peers
param(
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

$WorkDir = Join-Path $Root "scripts\_stage12-selfhost-body-section-ddc"
New-Item -ItemType Directory -Force -Path $WorkDir | Out-Null

$Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
$Ty = Join-Path $Root "yoyo\projects\yoyo.ty"

if (-not (Test-Path $Yoyo) -or -not $SkipBuild) {
    Write-Host "== build verifier (release, serial) =="
    Push-Location (Join-Path $Root "yoyo-rust")
    # Native cargo writes warnings to stderr; with $ErrorActionPreference=Stop that
    # becomes a terminating error under `&`. Temporarily Continue; still fail on exit鈮?.
    $prevEap = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    & cargo build --release -p verifier
    $buildEc = $LASTEXITCODE
    $ErrorActionPreference = $prevEap
    if ($buildEc -ne 0) { throw "verifier build failed (exit $buildEc)" }
    Pop-Location
}
if (-not (Test-Path $Yoyo)) { throw "missing yoyo.exe" }
if (-not (Test-Path $Ty)) { throw "missing $Ty" }

Write-Host ""
Write-Host "== Rust body-ddc (gen1鈮en2 selfhost-body window + H_00 stub pin) =="
& $Yoyo test body-ddc
if ($LASTEXITCODE -ne 0) {
    Write-Host "Stage 12-B: RED (yoyo test body-ddc exit $LASTEXITCODE)"
    exit 1
}

Write-Host ""
Write-Host "== three-peer full yoyo.ty builds =="
$jsOut = Join-Path $WorkDir "M_js.exe"
$rustOut = Join-Path $WorkDir "M_rust.exe"
$asmOut = Join-Path $WorkDir "M_asm.exe"

Write-Host "  JS..."
& node (Join-Path $Root "yoyo-js\src\yoyo.js") $Ty $jsOut
if ($LASTEXITCODE -ne 0 -or -not (Test-Path $jsOut)) {
    Write-Host "Stage 12-B: RED (JS peer build failed)"
    exit 1
}

Write-Host "  Rust..."
& $Yoyo link --target=win32 $Ty $rustOut
if ($LASTEXITCODE -ne 0 -or -not (Test-Path $rustOut)) {
    Write-Host "Stage 12-B: RED (Rust peer build failed)"
    exit 1
}

Write-Host "  Asm..."
& python (Join-Path $Root "yoyo-asm\asm.py") $Ty $asmOut
if ($LASTEXITCODE -ne 0 -or -not (Test-Path $asmOut)) {
    Write-Host "Stage 12-B: RED (asm peer build failed)"
    exit 1
}

Write-Host ""
Write-Host "== three-peer selfhost-body section-ddc (enlarged EQUAL window) =="
$pairs = @(
    @{ A = "JS"; B = "Rust"; PathA = $jsOut; PathB = $rustOut },
    @{ A = "JS"; B = "Asm"; PathA = $jsOut; PathB = $asmOut },
    @{ A = "Rust"; B = "Asm"; PathA = $rustOut; PathB = $asmOut }
)
$allEqual = $true
$minCompared = [int]::MaxValue
$rustStubNz = $null
foreach ($p in $pairs) {
    Write-Host "=== selfhost-body: $($p.A) vs $($p.B) ==="
    $lines = & $Yoyo diff --selfhost-body $p.PathA $p.PathB 2>&1
    $lines | ForEach-Object { Write-Host "  $_" }
    if ($LASTEXITCODE -ne 0) {
        $allEqual = $false
    }
    foreach ($line in $lines) {
        if ("$line" -match 'compared_bytes:\s*(\d+)') {
            $n = [int]$Matches[1]
            if ($n -lt $minCompared) { $minCompared = $n }
        }
        if ("$line" -match 'stub_tail_nonzero a=(\d+) b=(\d+)') {
            if ($p.A -eq "Rust" -or $p.B -eq "Rust") {
                $aNz = [int]$Matches[1]
                $bNz = [int]$Matches[2]
                $cand = if ($p.A -eq "Rust") { $aNz } else { $bNz }
                if ($null -eq $rustStubNz -or $cand -gt $rustStubNz) {
                    $rustStubNz = $cand
                }
            }
        }
    }
}

if (-not $allEqual) {
    Write-Host "Stage 12-B: RED (three-peer selfhost-body section-ddc DIFF)"
    exit 1
}

# Fail-closed: enlarged window must beat the old JS/asm-only 17920-minus-H00 floor.
# Shared window = PE startup (13) + handlers after H_00 (>=17000) 鈬?compared >= 17013.
if ($minCompared -lt 17013) {
    Write-Host "Stage 12-B: RED (compared window $minCompared < 17013 鈥?blind spot not shrunk)"
    exit 1
}

if ($null -eq $rustStubNz -or $rustStubNz -lt 40) {
    Write-Host "Stage 12-B: RED (Rust H_00 LoadLibrary stub missing/too small stub_tail_nonzero=$rustStubNz)"
    exit 1
}
Write-Host "Rust H_00 LoadLibrary stub_tail_nonzero=${rustStubNz}B (selfhost body present; outside three-peer EQUAL window)"

Write-Host ""
Write-Host "== sanity: full .text three-peer may still DIFF (honest) =="
# Expected DIFF writes stderr; do not let Stop + 2>&1 turn that into a gate failure.
$prevEap = $ErrorActionPreference
$ErrorActionPreference = "Continue"
$fullLines = & $Yoyo diff $jsOut $rustOut 2>&1
$fullDiff = $LASTEXITCODE
$ErrorActionPreference = $prevEap
$fullLines | ForEach-Object { Write-Host "  JS vs Rust full .text: $_" }
if ($fullDiff -eq 0) {
    Write-Host "  note: full .text unexpectedly EQUAL (better than baseline; OK)"
} else {
    Write-Host "  note: full .text DIFF as expected (H_00 slot + Rust LoadLibrary stub)"
}

Write-Host ""
Write-Host "Stage 12-B: GREEN 鈥?selfhost body section-ddc"
Write-Host "  three-peer EQUAL window >= $minCompared bytes (startup + post-H_00 handlers)"
Write-Host "  Rust body-ddc + H_00 stub pin: PASS (stub_tail_nonzero=$rustStubNz)"
Write-Host "Still honest DIFF: H_00 entry slot; Rust H_00 LoadLibrary stub; sidecar Rust runtime (no exact embed); LoadLibrary/libdl; full .text peer compare"
exit 0
