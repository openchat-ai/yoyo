# stage14-lock-harden.ps1 — Stage 14-B: Lock pin / Relock discipline harden
#
# Trust goal: thicken observable Lock discipline so the pin cannot silently
# drift. Unchanged source -> nail Decision #25. Source change -> RED with
# RELOCK_REQUIRED (no auto-relock; Relock + Decision note required).
#
# Fail-closed:
#   1. yoyo.ty SHA-256 == yoyo.ty.lock.sha256
#   2. lock.sha256 == Decision #25 authoritative pin (until a formal Relock)
#   3. lock.note names Decision #25; previous_sha256 chain present
#   4. verify-lock-pin (node + test lock)
#   5. stage14-outside-window-scope-cut -SkipBuild (do not regress A)
# Honest: does not Relock; does not claim Thompson-proof.
param(
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

# Decision #25 authoritative pin (PROMPT / yoyo.ty.lock). Update only after
# formal Relock + new Decision note — never silent rewrite.
$Decision25Pin = "0275802d2b4459e6ece0801a73af3e988d203c6a34dacfa382f2e48fe8ad43cb"
$Decision25Prev = "af5300941cfecdef1a8d4f3733239846bf9a99087b39c76cf9c645fe380e9725"
$DecisionLabel = 'Decision #25'

$Ty = Join-Path $Root "yoyo\projects\yoyo.ty"
$LockPath = Join-Path $Root "yoyo\tests\yoyo.ty.lock"
$Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
$WorkDir = Join-Path $Root "scripts\_stage14-lock-harden"
New-Item -ItemType Directory -Force -Path $WorkDir | Out-Null

function Write-Summary([string]$Status) {
    $path = Join-Path $WorkDir "SUMMARY.txt"
    @(
        "Stage 14-B Lock harden"
        $Status
        ("Authoritative: {0} pin {1}..." -f $DecisionLabel, $Decision25Pin.Substring(0, 16))
        "Honest: no auto-Relock; pin drift => RELOCK_REQUIRED + Decision note."
    ) | Set-Content -Encoding utf8 $path
}

function Get-Sha256File([string]$Path) {
    $hasher = [System.Security.Cryptography.SHA256]::Create()
    try {
        $fs = [System.IO.File]::OpenRead($Path)
        try {
            $hash = $hasher.ComputeHash($fs)
            return ([System.BitConverter]::ToString($hash) -replace '-', '').ToLowerInvariant()
        } finally {
            $fs.Dispose()
        }
    } finally {
        $hasher.Dispose()
    }
}

Write-Host "=== Stage 14-B: Lock pin / Relock discipline harden ==="

if (-not (Test-Path $Ty)) {
    Write-Host "Stage 14-B: RED (missing yoyo.ty)"
    Write-Summary "FAILED (missing yoyo.ty)"
    exit 1
}
if (-not (Test-Path $LockPath)) {
    Write-Host "Stage 14-B: RED (missing yoyo.ty.lock)"
    Write-Summary "FAILED (missing lock)"
    exit 1
}

# Optional one serial release build if binary missing (cargo hard rule: serial only).
if (-not (Test-Path $Yoyo)) {
    if ($SkipBuild) {
        Write-Host "Stage 14-B: RED (yoyo.exe missing and -SkipBuild)"
        Write-Summary "FAILED (missing yoyo.exe)"
        exit 1
    }
    Write-Host '== build verifier (release, serial) =='
    Push-Location (Join-Path $Root "yoyo-rust")
    $prevEap = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    & cargo build --release -p verifier
    $buildEc = $LASTEXITCODE
    $ErrorActionPreference = $prevEap
    Pop-Location
    if ($buildEc -ne 0) { throw "verifier build failed (exit $buildEc)" }
}
if (-not (Test-Path $Yoyo)) { throw "missing yoyo.exe" }

Write-Host ""
Write-Host ('== pin inventory (ty vs lock vs {0}) ==' -f $DecisionLabel)
$actual = Get-Sha256File $Ty
$lockRaw = Get-Content -Raw -Encoding utf8 $LockPath
$lock = $lockRaw | ConvertFrom-Json
$expected = [string]$lock.sha256
$prev = [string]$lock.previous_sha256
$note = [string]$lock.note
$signer = [string]$lock.signer

Write-Host ("  ty.sha256:     {0}..." -f $actual.Substring(0, 16))
Write-Host ("  lock.sha256:   {0}..." -f $expected.Substring(0, 16))
Write-Host ("  {0}:  {1}..." -f $DecisionLabel, $Decision25Pin.Substring(0, 16))
if ($prev.Length -ge 16) {
    Write-Host ("  lock.prev:     {0}..." -f $prev.Substring(0, 16))
} else {
    Write-Host ("  lock.prev:     {0}" -f $prev)
}
Write-Host ("  lock.signer:   {0}" -f $signer)
Write-Host ("  lock.note:     {0}" -f $note)

if ($actual -ne $expected) {
    Write-Host "Stage 14-B: RED (RELOCK_REQUIRED - yoyo.ty drifted from lock)"
    Write-Host ("  expected: {0}" -f $expected)
    Write-Host ("  actual:   {0}" -f $actual)
    Write-Host "  Discipline: formal Relock + Decision note; do NOT silent rewrite lock."
    Write-Summary "FAILED RELOCK_REQUIRED ty!=lock"
    exit 1
}

if ($expected -ne $Decision25Pin) {
    Write-Host ("Stage 14-B: RED (pin != {0} - Relock occurred or lock tampered)" -f $DecisionLabel)
    Write-Host ("  lock: {0}" -f $expected)
    Write-Host ("  want: {0}" -f $Decision25Pin)
    Write-Host "  Discipline: if intentional Relock, add Decision note and update this gate pin."
    Write-Summary ("FAILED pin!={0}" -f $DecisionLabel)
    exit 1
}

if ($prev -ne $Decision25Prev) {
    Write-Host ("Stage 14-B: RED (previous_sha256 chain broken vs {0})" -f $DecisionLabel)
    Write-Host ("  lock.prev: {0}" -f $prev)
    Write-Host ("  expected:  {0}" -f $Decision25Prev)
    Write-Summary "FAILED previous_sha256"
    exit 1
}

if ($note -notmatch 'Decision\s*#\s*25') {
    Write-Host ("Stage 14-B: RED (lock.note missing {0} label)" -f $DecisionLabel)
    Write-Summary "FAILED note label"
    exit 1
}

if ([string]::IsNullOrWhiteSpace($signer)) {
    Write-Host "Stage 14-B: RED (lock.signer empty)"
    Write-Summary "FAILED signer"
    exit 1
}

Write-Host ("PINNED {0} - source/pin unchanged; no Relock" -f $DecisionLabel)

Write-Host ""
Write-Host "== verify-lock-pin -SkipBuild =="
& (Join-Path $PSScriptRoot "verify-lock-pin.ps1") -SkipBuild
if ($LASTEXITCODE -ne 0) {
    Write-Host ("Stage 14-B: RED (verify-lock-pin exit {0})" -f $LASTEXITCODE)
    Write-Summary "FAILED verify-lock-pin"
    exit 1
}

Write-Host ""
Write-Host "== no-regress A: stage14-outside-window-scope-cut -SkipBuild =="
& (Join-Path $PSScriptRoot "stage14-outside-window-scope-cut.ps1") -SkipBuild
if ($LASTEXITCODE -ne 0) {
    Write-Host ("Stage 14-B: RED (stage14-A scope-cut exit {0})" -f $LASTEXITCODE)
    Write-Summary "FAILED stage14-A regress"
    exit 1
}

Write-Host ""
Write-Host "== spot stage13-link-host -SkipBuild (v0.7 baseline) =="
& (Join-Path $PSScriptRoot "stage13-link-host.ps1") -SkipBuild
if ($LASTEXITCODE -ne 0) {
    Write-Host ("Stage 14-B: RED (stage13-link-host exit {0})" -f $LASTEXITCODE)
    Write-Summary "FAILED stage13 regress"
    exit 1
}

$pinShort = $Decision25Pin.Substring(0, 16)
$hardenLine = "LOCK_HARDEN status=PINNED decision=25 pin=${pinShort}... relock=NO ty_eq_lock=YES"
Write-Host ""
Write-Host $hardenLine
Write-Host ""
Write-Host "Stage 14-B: GREEN - Lock pin / Relock discipline hardened"
Write-Host ("  Trust: {0} nailed; drift => RELOCK_REQUIRED (no auto-relock)" -f $DecisionLabel)
Write-Host "  Still honest: DDC=detection; Rust runtime / LoadLibrary / seed host remain"
Write-Summary "GREEN $hardenLine"
exit 0
