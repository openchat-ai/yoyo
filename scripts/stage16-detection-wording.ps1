# stage16-detection-wording.ps1 — Stage 16-B: detection wording / RELEASE boundary
#
# Trust goal: nail outward detection-only banlist + CUT list into RELEASE draft.
# Fail-closed on Thompson-proof / fully closed / fake EQUAL claims without
# negation context; require SCOPE-CUT-v1.0 inventory cited.
#
# Fail-closed:
#   1. stage16-scope-cut-finalize.ps1 -SkipBuild (no-regress Stage 16-A)
#   2. DETECTION-BANLIST-v1.0.md present + ACTIVE + BAN id= lines
#   3. RELEASE-v1.0.md DRAFT cites banlist + SCOPE-CUT + seven CUT IDs
#   4. Banned affirmative claims absent outside Forbidden/negation lines
#   5. DETECTION_WORDING status=DRAFT summary line
# Honest: DDC = detection; closed=0 cut=7 remains; not a publish/tag gate.
param(
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

$WorkDir = Join-Path $Root "scripts\_stage16-detection-wording"
New-Item -ItemType Directory -Force -Path $WorkDir | Out-Null

$BanlistDoc = Join-Path $Root "DETECTION-BANLIST-v1.0.md"
$ReleaseDoc = Join-Path $Root "RELEASE-v1.0.md"
$ScopeCutDoc = Join-Path $Root "SCOPE-CUT-v1.0-hole-inventory.md"
$Stamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"

$ExpectedIds = @("OW-H00", "OW-STUB", "OW-RT", "OW-IAT", "OW-SEED", "REL-FULLTEXT", "REL-STUBOS")
$ExpectedBanIds = @("THOMPSON", "FULLY_CLOSED", "FAKE_EQUAL", "FAKE_PROOF", "YOYO_RUNTIME_DONE", "IAT_GONE", "SEED_HOST_GONE")
$BanRows = [System.Collections.Generic.List[string]]::new()
$script:LockPath = $null

function Write-Final([string]$StatusLine) {
    $path = Join-Path $WorkDir "WORDING.txt"
    $lines = [System.Collections.Generic.List[string]]::new()
    $lines.Add("Stage 16-B DETECTION WORDING $Stamp") | Out-Null
    $lines.Add("Banlist: DETECTION-BANLIST-v1.0.md") | Out-Null
    $lines.Add("Release: RELEASE-v1.0.md") | Out-Null
    $lines.Add("") | Out-Null
    foreach ($row in $BanRows) { $lines.Add($row) | Out-Null }
    $lines.Add("") | Out-Null
    $lines.Add($StatusLine) | Out-Null
    $lines.Add("Honest: detection bar only; NOT Thompson-proof; CUT list remains.") | Out-Null
    $lines | Set-Content -Encoding utf8 $path
}

function Fail-Out([string]$Why) {
    Write-Host ("Stage 16-B: RED ({0})" -f $Why) -ForegroundColor Red
    Write-Final ("FAILED ({0})" -f $Why)
    if ($script:LockPath -and (Test-Path -LiteralPath $script:LockPath)) {
        Remove-Item -LiteralPath $script:LockPath -Force -ErrorAction SilentlyContinue
    }
    exit 1
}

function Test-NegationContext([string]$Line) {
    # Lines that list or forbid the claim are allowed to contain the banned tokens.
    return [bool]($Line -match '(?i)forbidden|misleading|must not|do not|do \*{0,2}not\*{0,2}|不得|禁止|ban id=|banlist|❌|never claim|not ship|not publish|not claim|not\*{0,2}\s*thompson|不是\s*Thompson|仍是 detection|detection[- ]only|not a |not an |without claiming|contracting false|implies:|why out|not graduation|remain\s+\*{0,2}CUT\*{0,2}|still \*{0,2}CUT\*{0,2}|honest boundary|not YOYO-built|not production|\*{0,2}not\*{0,2}\s+Thompson')
}

function Test-ForbiddenSectionTitle([string]$Line) {
    if ($Line -notmatch '^#{1,3}\s+') { return $false }
    return [bool]($Line -match '(?i)forbidden|misleading|must not|what must not|still out|禁止宣称|禁词|banlist|explicitly not')
}

Write-Host "=== Stage 16-B: detection wording / RELEASE boundary (DRAFT) ==="
Write-Host ("  stamp: {0}" -f $Stamp)
Write-Host "  rule: banlist + CUT cite in RELEASE; no bare Thompson-proof / fully closed / fake EQUAL"

if (-not (Test-Path $BanlistDoc)) { Fail-Out "missing DETECTION-BANLIST-v1.0.md" }
if (-not (Test-Path $ReleaseDoc)) { Fail-Out "missing RELEASE-v1.0.md" }
if (-not (Test-Path $ScopeCutDoc)) { Fail-Out "missing SCOPE-CUT-v1.0-hole-inventory.md" }

$banText = Get-Content -LiteralPath $BanlistDoc -Raw
$relText = Get-Content -LiteralPath $ReleaseDoc -Raw

# Allow markdown bold around Status value: Status:** ACTIVE / Status: DRAFT
if ($banText -notmatch 'Status:\s*\**\s*ACTIVE') {
    Fail-Out "DETECTION-BANLIST-v1.0 missing Status: ACTIVE"
}
# Stage 16-B ships DRAFT; Stage 16-D graduates to Status: graduated (same honesty gates).
if ($relText -notmatch 'Status:\s*\**\s*(DRAFT|graduated)') {
    Fail-Out "RELEASE-v1.0.md must be Status: DRAFT (B) or graduated (D)"
}
$releaseGraduated = [bool]($relText -match 'Status:\s*\**\s*graduated')
if ($relText -notmatch 'DETECTION-BANLIST-v1\.0\.md') {
    Fail-Out "RELEASE-v1.0.md must cite DETECTION-BANLIST-v1.0.md"
}
if ($relText -notmatch 'SCOPE-CUT-v1\.0-hole-inventory\.md') {
    Fail-Out "RELEASE-v1.0.md must cite SCOPE-CUT-v1.0-hole-inventory.md"
}
if ($relText -notmatch '(?i)detection') {
    Fail-Out "RELEASE-v1.0.md missing detection wording"
}
if ($relText -notmatch '(?i)not\*{0,2}\s*Thompson|Thompson\*{0,2}[^\n]{0,40}not|不是\s*Thompson') {
    Fail-Out "RELEASE-v1.0.md must deny Thompson-proof (detection != proof)"
}
if ($relText -notmatch 'closed=0') {
    Fail-Out "RELEASE-v1.0.md must state closed=0 (honest; no fake full close)"
}
if ($relText -notmatch 'cut=7') {
    Fail-Out "RELEASE-v1.0.md must state cut=7"
}
if ($relText -notmatch '(?i)HOLE_INVENTORY_V10') {
    Fail-Out "RELEASE-v1.0.md must cite HOLE_INVENTORY_V10"
}
if ($relText -notmatch 'status=FINAL') {
    Fail-Out "RELEASE-v1.0.md must cite inventory status=FINAL"
}

foreach ($id in $ExpectedIds) {
    $idEsc = [regex]::Escape($id)
    if ($relText -notmatch ("\*\*{0}\*\*[^\r\n]*\*\*CUT\*\*" -f $idEsc)) {
        Fail-Out ("RELEASE-v1.0.md missing CUT disposition for {0}" -f $id)
    }
}

# Concurrent guard.
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

# Wait for zero cargo (serial cargo rule); -SkipBuild never starts cargo itself.
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
# Doc/wording gate: never starts cargo; -SkipBuild kept for serial Stage harness parity.
Write-Host "== cargo build SKIP (detection-wording is doc gate; named -SkipBuild honored) =="

Write-Host ""
Write-Host "== no-regress Stage 16-A: stage16-scope-cut-finalize SkipBuild =="
$stage16A = Join-Path $PSScriptRoot "stage16-scope-cut-finalize.ps1"
# PS5.1: named -SkipBuild after -File — never @("-SkipBuild") array splat.
& powershell.exe -NoProfile -ExecutionPolicy Bypass -File $stage16A -SkipBuild
if ($LASTEXITCODE -ne 0) {
    Fail-Out ("stage16-A scope-cut-finalize failed exit {0}" -f $LASTEXITCODE)
}

Write-Host ""
Write-Host "== parse BAN lines + scan RELEASE for bare claims =="
$banLines = Get-Content -LiteralPath $BanlistDoc
$parsedBans = [System.Collections.Generic.List[object]]::new()
foreach ($line in $banLines) {
    if ($line -match '^BAN id=(\S+)\s+pattern=(.+)\s*$') {
        $bid = $Matches[1]
        $pat = $Matches[2].Trim()
        $parsedBans.Add([pscustomobject]@{ Id = $bid; Pattern = $pat }) | Out-Null
        $row = "BAN_CHECK id={0} pattern_ok=1" -f $bid
        $BanRows.Add($row) | Out-Null
        Write-Host $row
    }
}
if ($parsedBans.Count -lt $ExpectedBanIds.Count) {
    Fail-Out ("BAN id count {0} < expected {1}" -f $parsedBans.Count, $ExpectedBanIds.Count)
}
foreach ($bid in $ExpectedBanIds) {
    $found = $false
    foreach ($b in $parsedBans) {
        if ($b.Id -eq $bid) { $found = $true; break }
    }
    if (-not $found) { Fail-Out ("missing BAN id={0} in DETECTION-BANLIST-v1.0.md" -f $bid) }
}

# Scan RELEASE line-by-line: banned hits require negation context OR Forbidden/OUT section.
$relLines = Get-Content -LiteralPath $ReleaseDoc
$bareHits = 0
$inForbiddenSection = $false
foreach ($b in $parsedBans) {
    $rx = $null
    try {
        $rx = [regex]::new($b.Pattern)
    } catch {
        Fail-Out ("invalid BAN pattern id={0}: {1}" -f $b.Id, $_.Exception.Message)
    }
    $lineNo = 0
    $inForbiddenSection = $false
    foreach ($rl in $relLines) {
        $lineNo++
        if ($rl -match '^#{1,3}\s+') {
            $inForbiddenSection = (Test-ForbiddenSectionTitle $rl)
        }
        if (-not $rx.IsMatch($rl)) { continue }
        if ($inForbiddenSection -or (Test-NegationContext $rl)) {
            Write-Host ("  OK ban={0} line={1} (negation/forbidden context)" -f $b.Id, $lineNo)
            continue
        }
        $bareHits++
        Write-Host ("  BARE claim ban={0} line={1}: {2}" -f $b.Id, $lineNo, $rl.Trim()) -ForegroundColor Yellow
        Fail-Out ("RELEASE bare banned claim id={0} line={1}" -f $b.Id, $lineNo)
    }
}

# Soft wire: RELEASE must mention Thompson-proof forbid + fully closed forbid + EQUAL boundary.
if ($relText -notmatch '(?i)Thompson-proof') {
    Fail-Out "RELEASE must mention Thompson-proof in forbidden claims section"
}
if ($relText -notmatch '(?i)fully closed|洞已全关') {
    Fail-Out "RELEASE must mention fully closed / 洞已全关 as forbidden"
}
if ($relText -notmatch '(?i)three[- ]peer EQUAL|full \.text') {
    Fail-Out "RELEASE must address full .text / three-peer EQUAL boundary"
}

$relLabel = if ($releaseGraduated) { "GRADUATED" } else { "DRAFT" }
$statusLine = ("DETECTION_WORDING status={0} banlist=ACTIVE release={0} cuts_cited=7 closed=0 cut=7 bare_hits={1} nested_stage16a=0 inventory=SCOPE-CUT-v1.0-hole-inventory.md" -f $relLabel, $bareHits)

Write-Host ""
Write-Host $statusLine
Write-Host ""
Write-Host ("Stage 16-B: GREEN -- detection wording {0} + RELEASE boundary" -f $relLabel)
Write-Host "  Banlist: DETECTION-BANLIST-v1.0.md"
Write-Host ("  Release: RELEASE-v1.0.md ({0})" -f $relLabel)
Write-Host "  Honest: NOT Thompson-proof; NOT fully closed; CUT list remains"
Write-Final $statusLine

Remove-Item -LiteralPath $LockPath -Force -ErrorAction SilentlyContinue
exit 0
