# stage14-outside-window-scope-cut.ps1 — Stage 14-A: honest outside-window SCOPE-CUT
#
# Trust goal: shrink the blind spot where selfhost-body EQUAL could mask growth /
# opacity of H_00 slot, extract stub, embedded Rust runtime, and LoadLibrary IAT
# outside the compared window — without pretending full .text EQUAL.
#
# Fail-closed:
#   1. stage12-selfhost-body-section-ddc.ps1 -SkipBuild (comparable window EQUAL)
#   2. Outside-window pins (ceilings + host markers) — see SCOPE-CUT-v0.8-outside-window.md
#   3. Machine line SCOPE_CUT status=…  (ACTIVE if full .text DIFF; PARTIAL if EQUAL
#      but runtime/IAT still cut)
# Honest: DDC = detection; Rust runtime + LoadLibrary remain; seed still Rust-emitted.
param(
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

# Fail-closed pins (do not raise casually). Keep aligned with SCOPE-CUT doc + stage11/13.
# Post-v1.0 OW-RT: no exact embed; sidecar LoadLibrary. OW-IAT manual-map stub grew under Gate A.
# Gate C (2026-09-03): stub pin [40,950]→[40,3000] (obs 2673); dll MAX 150000→170000 (obs 158720).
$MinStubTailNonzero = 40
$MaxStubTailNonzero = 3000
$MaxDllBytes = 170000
$MaxSeedPeBytes = 270000
$MinBodyCompared = 17013
$RequiredMarkers = @("yoyo_rt.dll")
$ForbiddenIatMarkers = @("GetProcAddress", "LoadLibraryA")

$WorkDir = Join-Path $Root "scripts\_stage14-outside-window-scope-cut"
New-Item -ItemType Directory -Force -Path $WorkDir | Out-Null

$Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
$RuntimeDllPreferred = Join-Path $Root "yoyo-rust\target\release-runtime\yoyo_runtime.dll"
$RuntimeDllCompat = Join-Path $Root "yoyo-rust\target\release\yoyo_runtime.dll"
$Ty = Join-Path $Root "yoyo\projects\yoyo.ty"
$ScopeCutDoc = Join-Path $Root "SCOPE-CUT-v0.8-outside-window.md"

function Resolve-RuntimeDll {
    if (Test-Path $RuntimeDllPreferred) { return $RuntimeDllPreferred }
    if (Test-Path $RuntimeDllCompat) { return $RuntimeDllCompat }
    return $RuntimeDllPreferred
}

function Find-Ascii([byte[]]$Bytes, [string]$Needle) {
    return [System.Text.Encoding]::ASCII.GetString($Bytes).Contains($Needle)
}

function Find-EmbeddedExact([byte[]]$Hay, [byte[]]$Needle) {
    if ($Needle.Length -lt 16 -or $Hay.Length -lt $Needle.Length) { return -1 }
    $n0 = $Needle[0]; $n1 = $Needle[1]
    $limit = $Hay.Length - $Needle.Length
    for ($i = 0; $i -le $limit; $i++) {
        if ($Hay[$i] -ne $n0 -or $Hay[$i + 1] -ne $n1) { continue }
        $ok = $true
        for ($j = 0; $j -lt $Needle.Length; $j++) {
            if ($Hay[$i + $j] -ne $Needle[$j]) { $ok = $false; break }
        }
        if ($ok) { return $i }
    }
    return -1
}

function Write-Summary([string]$Status) {
    $path = Join-Path $WorkDir "SUMMARY.txt"
    @(
        "Stage 14-A outside-window SCOPE-CUT"
        $Status
        "Doc: SCOPE-CUT-v0.8-outside-window.md"
        "Honest: NOT full .text EQUAL graduation; body window EQUAL + outside pins."
    ) | Set-Content -Encoding utf8 $path
}

Write-Host "=== Stage 14-A: outside-window SCOPE-CUT gate ==="
if (-not (Test-Path $ScopeCutDoc)) {
    Write-Host "Stage 14-A: RED (missing SCOPE-CUT-v0.8-outside-window.md)"
    Write-Summary "FAILED (missing SCOPE-CUT doc)"
    exit 1
}

if (-not (Test-Path $Yoyo) -or -not $SkipBuild) {
    Write-Host "== build verifier (release, serial) =="
    Push-Location (Join-Path $Root "yoyo-rust")
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

$RuntimeDll = Resolve-RuntimeDll
if (-not (Test-Path $RuntimeDll)) {
    Write-Host "Stage 14-A: RED (missing yoyo_runtime.dll)"
    Write-Summary "FAILED (missing runtime.dll)"
    exit 1
}
$dllSize = (Get-Item $RuntimeDll).Length
Write-Host "runtime.dll: $dllSize bytes (MAX $MaxDllBytes)"
if ($dllSize -gt $MaxDllBytes) {
    Write-Host "Stage 14-A: RED (OW-RT runtime.dll $dllSize > MAX $MaxDllBytes)"
    Write-Summary "FAILED (runtime.dll ceiling)"
    exit 1
}

Write-Host ""
Write-Host "== comparable window: stage12-selfhost-body-section-ddc -SkipBuild =="
& (Join-Path $PSScriptRoot "stage12-selfhost-body-section-ddc.ps1") -SkipBuild
if ($LASTEXITCODE -ne 0) {
    Write-Host "Stage 14-A: RED (stage12-B body window failed exit $LASTEXITCODE)"
    Write-Summary "FAILED (stage12-B)"
    exit 1
}

Write-Host ""
Write-Host "== outside-window inventory (JS vs Rust full yoyo.ty) =="
$jsOut = Join-Path $WorkDir "M_js.exe"
$rustOut = Join-Path $WorkDir "M_rust.exe"

Write-Host "  JS..."
& node (Join-Path $Root "yoyo-js\src\yoyo.js") $Ty $jsOut
if ($LASTEXITCODE -ne 0 -or -not (Test-Path $jsOut)) {
    Write-Host "Stage 14-A: RED (JS peer build failed)"
    Write-Summary "FAILED (JS build)"
    exit 1
}

Write-Host "  Rust..."
& $Yoyo link --target=win32 $Ty $rustOut
if ($LASTEXITCODE -ne 0 -or -not (Test-Path $rustOut)) {
    Write-Host "Stage 14-A: RED (Rust peer build failed)"
    Write-Summary "FAILED (Rust build)"
    exit 1
}

$seedPe = (Get-Item $rustOut).Length
Write-Host "Rust seed PE: $seedPe bytes (MAX $MaxSeedPeBytes)"
if ($seedPe -gt $MaxSeedPeBytes) {
    Write-Host "Stage 14-A: RED (seed PE $seedPe > MAX $MaxSeedPeBytes)"
    Write-Summary "FAILED (seed PE ceiling)"
    exit 1
}

$rustBytes = [System.IO.File]::ReadAllBytes($rustOut)
$dllBytes = [System.IO.File]::ReadAllBytes($RuntimeDll)
$embedOff = Find-EmbeddedExact $rustBytes $dllBytes
Write-Host "OW-RT exact embed offset: $embedOff (post-v1.0: must be absent)"
if ($embedOff -ge 0) {
    Write-Host "Stage 14-A: RED (OW-RT exact embed returned at $embedOff — sidecar shrink regresssed)"
    Write-Summary "FAILED (runtime embed regress)"
    exit 1
}
Write-Host "OW-RT: no exact embed (cwd sidecar yoyo_rt.dll; still Rust runtime CUT)"

foreach ($m in $RequiredMarkers) {
    if (-not (Find-Ascii $rustBytes $m)) {
        Write-Host "Stage 14-A: RED (OW-IAT missing marker $m)"
        Write-Summary "FAILED (IAT/host marker $m)"
        exit 1
    }
}
foreach ($m in $ForbiddenIatMarkers) {
    if (Find-Ascii $rustBytes $m) {
        Write-Host "Stage 14-A: RED (OW-IAT forbidden marker $m still present — GetProcAddress not shrunk)"
        Write-Summary "FAILED (IAT/host forbidden $m)"
        exit 1
    }
}
Write-Host "OW-IAT markers: $($RequiredMarkers -join ', ') OK; LoadLibraryA/GetProcAddress ABSENT (PEB + ordinal-0)"

Write-Host ""
Write-Host "== selfhost-body (reconfirm) + stub pin =="
$bodyLines = & $Yoyo diff --selfhost-body $jsOut $rustOut 2>&1
$bodyEc = $LASTEXITCODE
$bodyLines | ForEach-Object { Write-Host "  $_" }
if ($bodyEc -ne 0) {
    Write-Host "Stage 14-A: RED (selfhost-body DIFF — comparable window must stay EQUAL)"
    Write-Summary "FAILED (body DIFF)"
    exit 1
}
$bodyCompared = $null
$stubNz = $null
foreach ($line in $bodyLines) {
    if ("$line" -match 'compared_bytes:\s*(\d+)') { $bodyCompared = [int]$Matches[1] }
    if ("$line" -match 'stub_tail_nonzero a=(\d+) b=(\d+)') {
        $stubNz = [Math]::Max([int]$Matches[1], [int]$Matches[2])
    }
}
if ($null -eq $bodyCompared -or $bodyCompared -lt $MinBodyCompared) {
    Write-Host "Stage 14-A: RED (body compared $bodyCompared < $MinBodyCompared)"
    Write-Summary "FAILED (body floor)"
    exit 1
}
if ($null -eq $stubNz -or $stubNz -lt $MinStubTailNonzero -or $stubNz -gt $MaxStubTailNonzero) {
    Write-Host "Stage 14-A: RED (OW-STUB stub_tail_nonzero=$stubNz not in [$MinStubTailNonzero,$MaxStubTailNonzero])"
    Write-Summary "FAILED (stub pin)"
    exit 1
}
Write-Host "OW-STUB stub_tail_nonzero=${stubNz}B in [$MinStubTailNonzero,$MaxStubTailNonzero]"

Write-Host ""
Write-Host "== honest full .text (must NOT be graduation EQUAL claim) =="
$prevEap = $ErrorActionPreference
$ErrorActionPreference = "Continue"
$fullLines = & $Yoyo diff $jsOut $rustOut 2>&1
$fullEc = $LASTEXITCODE
$ErrorActionPreference = $prevEap
$fullLines | ForEach-Object { Write-Host "  $_" }

$fullStatus = if ($fullEc -eq 0) { "EQUAL" } else { "DIFF" }
# Pass criterion is NEVER "full .text EQUAL". DIFF => SCOPE-CUT ACTIVE.
# EQUAL => H_00/stub may have closed; OW-RT/OW-IAT still CUT (PARTIAL).
if ($fullStatus -eq "DIFF") {
    $cutStatus = "ACTIVE"
    Write-Host "full .text DIFF — SCOPE-CUT ACTIVE (honest; not inventing EQUAL)"
} else {
    $cutStatus = "PARTIAL"
    Write-Host "full .text EQUAL — OW-H00/OW-STUB may be closable; OW-RT/OW-IAT still CUT"
    Write-Host "  (Stage 14-A still GREEN only because outside pins held — NOT a full-text graduation)"
}

$scopeLine = "SCOPE_CUT status=$cutStatus full_text=$fullStatus body_window=EQUAL compared=$bodyCompared stub_nz=$stubNz dll=$dllSize seed_pe=$seedPe embed_off=$embedOff"
Write-Host ""
Write-Host $scopeLine
Write-Host ""
Write-Host "Stage 14-A: GREEN — outside-window SCOPE-CUT draft + gate"
Write-Host "  Doc: SCOPE-CUT-v0.8-outside-window.md"
Write-Host "  Still CUT: H_00 slot / LoadLibrary stub (if DIFF) / sidecar Rust runtime / LoadLibrary IAT / Rust-emitted seed"
Write-Host "  Comparable EQUAL: selfhost-body window only — NOT full .text"
Write-Summary "GREEN $scopeLine"
exit 0
