# stage16-scope-cut-finalize.ps1 — Stage 16-A: v1.0 FINAL SCOPE-CUT
#
# Trust goal: promote v0.9 HOLE_INVENTORY into v1.0 FINAL disposition.
# Each OW-* / REL-* is CLOSED (fail-closed evidence via nested stage15) or CUT
# (pinned into SCOPE-CUT-v1.0-hole-inventory.md). Must NOT fake EQUAL/CLOSED.
#
# Fail-closed:
#   1. stage15-hole-inventory.ps1 -SkipBuild (no-regress Stage 15-A)
#   2. SCOPE-CUT-v1.0-hole-inventory.md present + FINAL + seven IDs
#   3. Per-hole FINAL_HOLE id=... disposition=CLOSED|CUT evidence=...
#   4. HOLE_INVENTORY_V10 status=FINAL summary line
# Honest: DDC = detection; Rust runtime + LoadLibrary + Rust seed remain CUT.
param(
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

$WorkDir = Join-Path $Root "scripts\_stage16-scope-cut-finalize"
New-Item -ItemType Directory -Force -Path $WorkDir | Out-Null

$Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
$ScopeCutDoc = Join-Path $Root "SCOPE-CUT-v1.0-hole-inventory.md"
$Stage15InventoryTxt = Join-Path $Root "scripts\_stage15-hole-inventory\INVENTORY.txt"
$Stamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"

$ExpectedIds = @("OW-H00", "OW-STUB", "OW-RT", "OW-IAT", "OW-SEED", "REL-FULLTEXT", "REL-STUBOS")
$FinalRows = [System.Collections.Generic.List[string]]::new()
$ClosedCount = 0
$CutCount = 0

function Write-Final([string]$StatusLine) {
    $path = Join-Path $WorkDir "FINAL.txt"
    $lines = [System.Collections.Generic.List[string]]::new()
    $lines.Add("Stage 16-A SCOPE-CUT FINAL $Stamp") | Out-Null
    $lines.Add("Doc: SCOPE-CUT-v1.0-hole-inventory.md") | Out-Null
    $lines.Add("") | Out-Null
    foreach ($row in $FinalRows) { $lines.Add($row) | Out-Null }
    $lines.Add("") | Out-Null
    $lines.Add($StatusLine) | Out-Null
    $lines.Add("Honest: NOT full .text EQUAL graduation; CLOSED only with fail-closed evidence.") | Out-Null
    $lines | Set-Content -Encoding utf8 $path
}

function Fail-Out([string]$Why) {
    Write-Host ("Stage 16-A: RED ({0})" -f $Why) -ForegroundColor Red
    Write-Final ("FAILED ({0})" -f $Why)
    if ($script:LockPath -and (Test-Path -LiteralPath $script:LockPath)) {
        Remove-Item -LiteralPath $script:LockPath -Force -ErrorAction SilentlyContinue
    }
    exit 1
}

Write-Host "=== Stage 16-A: v1.0 FINAL SCOPE-CUT (CLOSED|CUT) ==="
Write-Host ("  stamp: {0}" -f $Stamp)
Write-Host "  rule: promote v0.9 inventory; never pretend full .text EQUAL / fake CLOSED"

if (-not (Test-Path $ScopeCutDoc)) {
    Fail-Out "missing SCOPE-CUT-v1.0-hole-inventory.md"
}

$docText = Get-Content -LiteralPath $ScopeCutDoc -Raw
if ($docText -notmatch 'Status:\s*FINAL') {
    Fail-Out "SCOPE-CUT-v1.0 doc missing Status: FINAL"
}
foreach ($id in $ExpectedIds) {
    if ($docText -notmatch [regex]::Escape("**$id**")) {
        Fail-Out ("SCOPE-CUT-v1.0 doc missing hole id {0}" -f $id)
    }
}

# Concurrent guard when nesting Stage 15-A / shared workdirs.
$LockPath = Join-Path $WorkDir "driver.lock"
$script:LockPath = $LockPath
if (Test-Path $LockPath) {
    $lockAgeMin = ((Get-Date) - (Get-Item -LiteralPath $LockPath).LastWriteTime).TotalMinutes
    if ($lockAgeMin -lt 120) {
        $lockBody = (Get-Content -LiteralPath $LockPath -Raw -ErrorAction SilentlyContinue)
        Fail-Out ("concurrent driver.lock age={0:N1}m -- {1}" -f $lockAgeMin, $lockBody.Trim())
    }
    Remove-Item -LiteralPath $LockPath -Force -ErrorAction SilentlyContinue
}
("{0}`npid={1}" -f $Stamp, $PID) | Set-Content -LiteralPath $LockPath -Encoding ascii

# Wait for zero cargo before optional build / nested gates.
$deadline = (Get-Date).AddMinutes(45)
Write-Host "== wait for zero cargo/rustc =="
while ($true) {
    $procs = @(Get-Process -Name cargo, rustc -ErrorAction SilentlyContinue)
    if ($procs.Count -eq 0) { break }
    if ((Get-Date) -gt $deadline) {
        Fail-Out "cargo/rustc still running after wait"
    }
    Write-Host ("  waiting on {0} cargo/rustc..." -f $procs.Count)
    Start-Sleep -Seconds 5
}

if (-not (Test-Path $Yoyo) -or -not $SkipBuild) {
    Write-Host "== one cargo: build --release -p verifier =="
    Push-Location (Join-Path $Root "yoyo-rust")
    $prevEap = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    & cargo build --release -p verifier
    $buildEc = $LASTEXITCODE
    $ErrorActionPreference = $prevEap
    Pop-Location
    if ($buildEc -ne 0) { Fail-Out ("verifier build failed (exit {0})" -f $buildEc) }
} else {
    Write-Host "== cargo build SKIP (SkipBuild; yoyo.exe present) =="
}
if (-not (Test-Path $Yoyo)) { Fail-Out "missing yoyo.exe" }

Write-Host ""
Write-Host "== no-regress Stage 15-A: stage15-hole-inventory SkipBuild =="
# Nest in a child powershell so parent $WorkDir/$LockPath script-scope cannot
# collide with stage15's driver.lock (same-process & can share visible vars).
# PS5.1: named -SkipBuild after -File -- never @("-SkipBuild") array splat.
# After our optional one cargo, always nest with -SkipBuild (zero parallel cargo).
$stage15Gate = Join-Path $PSScriptRoot "stage15-hole-inventory.ps1"
& powershell.exe -NoProfile -ExecutionPolicy Bypass -File $stage15Gate -SkipBuild
if ($LASTEXITCODE -ne 0) {
    Fail-Out ("stage15-A hole-inventory failed exit {0}" -f $LASTEXITCODE)
}

if (-not (Test-Path $Stage15InventoryTxt)) {
    Fail-Out "missing stage15 INVENTORY.txt after nest"
}

Write-Host ""
Write-Host "== promote stage15 rows → FINAL_HOLE + doc cross-check =="
$invLines = Get-Content -LiteralPath $Stage15InventoryTxt
$statusFrom15 = $null
foreach ($line in $invLines) {
    if ($line -match '^HOLE id=(\S+) disposition=(CLOSED|CUT) evidence=(.+)$') {
        $id = $Matches[1]
        $disp = $Matches[2]
        $ev = $Matches[3]
        $final = "FINAL_HOLE id={0} disposition={1} evidence={2}" -f $id, $disp, $ev
        $FinalRows.Add($final) | Out-Null
        Write-Host $final
        if ($disp -eq "CLOSED") { $ClosedCount++ } else { $CutCount++ }

        # Doc must not claim CLOSED when measured CUT (anti-fake).
        $idEsc = [regex]::Escape($id)
        $docClosed = ($docText -match ("\*\*{0}\*\*[^\r\n]*\*\*CLOSED\*\*" -f $idEsc))
        $docCut = ($docText -match ("\*\*{0}\*\*[^\r\n]*\*\*CUT\*\*" -f $idEsc))
        if (-not $docClosed -and -not $docCut) {
            Fail-Out ("SCOPE-CUT-v1.0 doc missing disposition marker for {0}" -f $id)
        }
        if ($disp -eq "CUT" -and $docClosed -and -not $docCut) {
            Fail-Out ("doc claims CLOSED for {0} but measured CUT" -f $id)
        }
        if ($disp -eq "CLOSED" -and $docCut -and -not $docClosed) {
            Fail-Out ("doc claims CUT for {0} but measured CLOSED — update SCOPE-CUT-v1.0" -f $id)
        }
    }
    if ($line -match '^HOLE_INVENTORY status=') {
        $statusFrom15 = $line
    }
}

if ($FinalRows.Count -ne $ExpectedIds.Count) {
    Fail-Out ("FINAL_HOLE row count {0} != {1}" -f $FinalRows.Count, $ExpectedIds.Count)
}
foreach ($id in $ExpectedIds) {
    $found = $false
    foreach ($row in $FinalRows) {
        if ($row -match ("FINAL_HOLE id={0} disposition=" -f [regex]::Escape($id))) { $found = $true; break }
    }
    if (-not $found) { Fail-Out ("missing FINAL_HOLE row for {0}" -f $id) }
}
if (-not $statusFrom15) {
    Fail-Out "stage15 INVENTORY missing HOLE_INVENTORY status line"
}

# Parse monitors from stage15 summary for v10 line.
$fullText = "UNKNOWN"
$bodyCompared = "?"
$stubNz = "?"
$dllSize = "?"
$seedPe = "?"
$embedOff = "?"
if ($statusFrom15 -match 'full_text=(\S+)') { $fullText = $Matches[1] }
if ($statusFrom15 -match 'compared=(\d+)') { $bodyCompared = $Matches[1] }
if ($statusFrom15 -match 'stub_nz=(\d+)') { $stubNz = $Matches[1] }
if ($statusFrom15 -match 'dll=(\d+)') { $dllSize = $Matches[1] }
if ($statusFrom15 -match 'seed_pe=(\d+)') { $seedPe = $Matches[1] }
if ($statusFrom15 -match 'embed_off=(-?\d+)') { $embedOff = $Matches[1] }

# Honesty: full DIFF must not be marketed as EQUAL; REL-FULLTEXT must stay CUT.
$relFull = ($FinalRows | Where-Object { $_ -match 'FINAL_HOLE id=REL-FULLTEXT' } | Select-Object -First 1)
if ($relFull -match 'disposition=CLOSED') {
    Fail-Out "REL-FULLTEXT must never be CLOSED as graduation"
}
if ($fullText -eq "DIFF") {
    $owH00 = ($FinalRows | Where-Object { $_ -match 'FINAL_HOLE id=OW-H00' } | Select-Object -First 1)
    if ($owH00 -match 'disposition=CLOSED') {
        Fail-Out "OW-H00 CLOSED while full_text=DIFF (fake CLOSED)"
    }
}

# v1.0-A expected baseline: all CUT (closed=0 cut=7) is OK and honest.
# FINAL status means the table is released for 1.0 — not that holes are closed.
$statusLine = ("HOLE_INVENTORY_V10 status=FINAL full_text={0} body_window=EQUAL compared={1} stub_nz={2} dll={3} seed_pe={4} embed_off={5} closed={6} cut={7} upstream={8}" -f `
    $fullText, $bodyCompared, $stubNz, $dllSize, $seedPe, $embedOff, $ClosedCount, $CutCount, ($statusFrom15 -replace '^HOLE_INVENTORY ', ''))

Write-Host ""
Write-Host $statusLine
Write-Host ""
Write-Host "Stage 16-A: GREEN -- v1.0 SCOPE-CUT FINAL (CLOSED|CUT)"
Write-Host "  Doc: SCOPE-CUT-v1.0-hole-inventory.md"
Write-Host ("  CLOSED={0} CUT={1} -- comparable EQUAL: selfhost-body only" -f $ClosedCount, $CutCount)
Write-Final $statusLine

Remove-Item -LiteralPath $LockPath -Force -ErrorAction SilentlyContinue
exit 0
