# stage15-prerun.ps1 — Stage 15-B: one-click serial keep-green pre-run
#
# Graduation-time machine re-verify: serially run v0.9 Stage 15-A + v0.8
# Stage 14 A/B/C umbrella (stage14-v07-regress covers A/B + stage13–9 + WSL).
#
# Hard cargo:
#   wait for zero cargo/rustc → at most ONE release build → nested gates
#   always receive named -SkipBuild (never @("-SkipBuild") splat).
# Fail-closed: any nested non-zero → exit 1. driver.lock if nesting.
param(
    [switch]$SkipBuild
)

$ErrorActionPreference = "Continue"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

$WorkDir = Join-Path $Root "scripts\_stage15-prerun"
New-Item -ItemType Directory -Force -Path $WorkDir | Out-Null
$Stamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
$SummaryPath = Join-Path $WorkDir "summary.txt"
$ExitTable = [System.Collections.Generic.List[string]]::new()
$Failed = $false

$Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
$RuntimeDllPreferred = Join-Path $Root "yoyo-rust\target\release-runtime\yoyo_runtime.dll"
$RuntimeDllCompat = Join-Path $Root "yoyo-rust\target\release\yoyo_runtime.dll"

function Write-Summary([string]$Status) {
    $lines = [System.Collections.Generic.List[string]]::new()
    $lines.Add("Stage 15-B prerun keep-green $Stamp") | Out-Null
    $lines.Add("") | Out-Null
    $lines.Add("EXIT TABLE:") | Out-Null
    foreach ($row in $ExitTable) { $lines.Add([string]$row) | Out-Null }
    $lines.Add("") | Out-Null
    $lines.Add($Status) | Out-Null
    $lines.Add("") | Out-Null
    $lines.Add("Trust chain: graduation-time machine re-verify before Stage 15-C/D.") | Out-Null
    $lines.Add("  stage15-A hole inventory + stage14-v07-regress (A/B + stage13–9 + WSL)") | Out-Null
    $lines.Add("  under one serial gate (one cargo max, then yoyo.exe / -SkipBuild).") | Out-Null
    $lines.Add("Honest unchanged: HOLE_INVENTORY CUT remain; Rust runtime + LoadLibrary;") | Out-Null
    $lines.Add("  full .text may DIFF; Lock pin Decision #25; DDC = detection not proof.") | Out-Null
    $lines | Set-Content -Path $SummaryPath -Encoding utf8
    foreach ($line in $lines) { Write-Host $line }
}

function Invoke-Gate([string]$Name, [scriptblock]$Body) {
    Write-Host "`n======== $Name ========" -ForegroundColor Cyan
    $sw = [Diagnostics.Stopwatch]::StartNew()
    $code = 0
    try {
        # Do NOT pipe to Out-Host — PS5.1 can lose/poison LASTEXITCODE on native stderr.
        & $Body
        $code = $LASTEXITCODE
        if ($null -eq $code) { $code = 0 }
    } catch {
        Write-Host ("EXCEPTION: {0}" -f $_) -ForegroundColor Red
        $code = 1
        if ($null -ne $LASTEXITCODE -and $LASTEXITCODE -ne 0) { $code = $LASTEXITCODE }
    }
    $sw.Stop()
    $ExitTable.Add(("{0,-55} {1}" -f $Name, $code)) | Out-Null
    Write-Host ("EXIT {0} = {1} ({2}s)" -f $Name, $code, [math]::Round($sw.Elapsed.TotalSeconds, 1)) `
        -ForegroundColor $(if ($code -eq 0) { "Green" } else { "Red" })
    if ($code -ne 0) { $script:Failed = $true }
    return $code
}

function Fail-Out {
    Write-Summary "FAILED"
    Write-Host "Stage 15-B: RED" -ForegroundColor Red
    if ($script:LockPath -and (Test-Path -LiteralPath $script:LockPath)) {
        Remove-Item -LiteralPath $script:LockPath -Force -ErrorAction SilentlyContinue
    }
    exit 1
}

function Wait-ZeroCargo {
    $deadline = (Get-Date).AddMinutes(45)
    Write-Host "== wait for zero cargo/rustc =="
    while ($true) {
        $procs = @(Get-Process -Name cargo, rustc -ErrorAction SilentlyContinue)
        if ($procs.Count -eq 0) {
            Write-Host "  cargo/rustc: clear"
            $ExitTable.Add(("{0,-55} {1}" -f "wait cargo/rustc", "0")) | Out-Null
            return
        }
        if ((Get-Date) -ge $deadline) {
            Write-Host "Stage 15-B: RED (timeout waiting for cargo/rustc)" -ForegroundColor Red
            $ExitTable.Add(("{0,-55} {1}" -f "wait cargo/rustc", "TIMEOUT")) | Out-Null
            Fail-Out
        }
        $ids = ($procs | ForEach-Object { "{0}:{1}" -f $_.ProcessName, $_.Id }) -join ", "
        Write-Host ("  waiting ({0})..." -f $ids)
        Start-Sleep -Seconds 5
    }
}

Write-Host "=== Stage 15-B: prerun keep-green ==="
Write-Host "  serial: wait cargo → one release build → stage15-A → stage14-v07-regress -SkipBuild"
Write-Host "  stamp: $Stamp"

# Fail-closed if another stage15-prerun still owns shared workdir nesting.
$LockPath = Join-Path $WorkDir "driver.lock"
$script:LockPath = $LockPath
if (Test-Path $LockPath) {
    $lockAgeMin = ((Get-Date) - (Get-Item -LiteralPath $LockPath).LastWriteTime).TotalMinutes
    if ($lockAgeMin -lt 120) {
        $lockBody = (Get-Content -LiteralPath $LockPath -Raw -ErrorAction SilentlyContinue)
        Write-Host ("Stage 15-B: RED (concurrent driver.lock age={0:N1}m — {1})" -f $lockAgeMin, $lockBody.Trim()) -ForegroundColor Red
        $ExitTable.Add(("{0,-55} {1}" -f "driver.lock", "BUSY")) | Out-Null
        Write-Summary "FAILED (concurrent prerun — workdir race)"
        exit 1
    }
    Remove-Item -LiteralPath $LockPath -Force -ErrorAction SilentlyContinue
}
("{0}`npid={1}" -f $Stamp, $PID) | Set-Content -LiteralPath $LockPath -Encoding ascii

Wait-ZeroCargo

# --- at most one release build (verifier; runtime packages only if missing) ---
if (-not $SkipBuild) {
    $needRuntime = -not ((Test-Path $RuntimeDllPreferred) -or (Test-Path $RuntimeDllCompat))
    $cargoArgs = @("build", "--release", "-p", "verifier")
    if ($needRuntime) {
        $cargoArgs += @("-p", "yoyo-runtime")
        Write-Host "== one cargo: build --release -p verifier -p yoyo-runtime =="
    } else {
        Write-Host "== one cargo: build --release -p verifier (runtime present) =="
        $ExitTable.Add(("{0,-55} {1}" -f "cargo build yoyo-runtime", "SKIP (present)")) | Out-Null
    }
    $null = Invoke-Gate "cargo build --release (one)" {
        Push-Location (Join-Path $Root "yoyo-rust")
        try {
            & cargo @cargoArgs
        } finally {
            Pop-Location
        }
    }
    if ($Failed) { Fail-Out }
} else {
    if (-not (Test-Path $Yoyo)) {
        Write-Host "Stage 15-B: RED (yoyo.exe missing and -SkipBuild)" -ForegroundColor Red
        Fail-Out
    }
    $ExitTable.Add(("{0,-55} {1}" -f "cargo build --release (one)", "SKIP (-SkipBuild)")) | Out-Null
}

if (-not (Test-Path $Yoyo)) {
    Write-Host "Stage 15-B: RED (yoyo.exe missing after build)" -ForegroundColor Red
    Fail-Out
}

# PS5.1: array splat @("-SkipBuild") does NOT bind [switch]$SkipBuild (stays $false).
# Always pass the named switch token after the optional parent build.
# Order: Stage 15-A first (v0.9 hole inventory), then Stage 14-C umbrella
# (v07-regress = A/B + stage13–9 + fullbody/lock/gen12 + WSL).
$scriptGates = @(
    "stage15-hole-inventory.ps1",
    "stage14-v07-regress.ps1"
)

try {
    foreach ($name in $scriptGates) {
        $gatePath = Join-Path $Root "scripts\$name"
        if (-not (Test-Path $gatePath)) {
            Write-Host ("Stage 15-B: RED (missing gate {0})" -f $name) -ForegroundColor Red
            $ExitTable.Add(("{0,-55} {1}" -f $name, "MISSING")) | Out-Null
            Fail-Out
        }
        $null = Invoke-Gate $name {
            & $gatePath -SkipBuild
        }
        if ($Failed) { Fail-Out }
    }

    Write-Summary $(if ($Failed) { "FAILED" } else { "ALL_GREEN" })

    if ($Failed) {
        Write-Host "Stage 15-B: RED" -ForegroundColor Red
        exit 1
    }
    Write-Host "Stage 15-B: GREEN" -ForegroundColor Green
    exit 0
} finally {
    Remove-Item -LiteralPath $LockPath -Force -ErrorAction SilentlyContinue
}
