# stage14-v07-regress.ps1 — Stage 14-C: v0.7 regression must not regress
# Fail-closed serial gate (no parallel cargo; & not Start-Process|Out-Null).
# Covers: stage13-* + stage12-* + stage11-* + stage10-* + stage9-pure-m4
#   + yoyo.exe test all/lock/gen12/fullbody + Stage 14 A/B gates.
# Hard cargo: wait for zero cargo/rustc → at most ONE release build →
#   then -SkipBuild / direct yoyo.exe (no overlapping cargo).
param(
    [switch]$SkipBuild,
    [switch]$SkipWsl
)

$ErrorActionPreference = "Continue"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

$WorkDir = Join-Path $Root "scripts\_stage14-v07-regress"
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
    $lines.Add("Stage 14-C v0.7 regression $Stamp") | Out-Null
    $lines.Add("") | Out-Null
    $lines.Add("EXIT TABLE:") | Out-Null
    foreach ($row in $ExitTable) { $lines.Add([string]$row) | Out-Null }
    $lines.Add("") | Out-Null
    $lines.Add($Status) | Out-Null
    $lines.Add("") | Out-Null
    $lines.Add("Trust chain: Stage 14 A/B expansion must not drop v0.7 baseline.") | Out-Null
    $lines.Add("  stage13/12/11/10/9 + fullbody/lock/gen12 + Lock pin + Stage 14 A/B") | Out-Null
    $lines.Add("  stay green under one serial gate (one cargo max, then yoyo.exe / -SkipBuild).") | Out-Null
    $lines.Add("Honest unchanged: Rust runtime + LoadLibrary/libdl; full .text peer may DIFF;") | Out-Null
    $lines.Add("  SCOPE-CUT ACTIVE; seed still Rust-emitted; Lock pin Decision #25.") | Out-Null
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
    Write-Host "Stage 14-C: RED" -ForegroundColor Red
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
            Write-Host "Stage 14-C: RED (timeout waiting for cargo/rustc)" -ForegroundColor Red
            $ExitTable.Add(("{0,-55} {1}" -f "wait cargo/rustc", "TIMEOUT")) | Out-Null
            Fail-Out
        }
        $ids = ($procs | ForEach-Object { "{0}:{1}" -f $_.ProcessName, $_.Id }) -join ", "
        Write-Host ("  waiting ({0})..." -f $ids)
        Start-Sleep -Seconds 5
    }
}

Write-Host "=== Stage 14-C: v0.7 regression gate ==="
Write-Host "  serial: wait cargo → one release build → yoyo.exe test* → gates -SkipBuild"
Write-Host "  stamp: $Stamp"

# Fail-closed if another stage14-v07-regress still owns shared _stage* workdirs.
$LockPath = Join-Path $WorkDir "driver.lock"
$script:LockPath = $LockPath
if (Test-Path $LockPath) {
    $lockAgeMin = ((Get-Date) - (Get-Item -LiteralPath $LockPath).LastWriteTime).TotalMinutes
    if ($lockAgeMin -lt 120) {
        $lockBody = (Get-Content -LiteralPath $LockPath -Raw -ErrorAction SilentlyContinue)
        Write-Host ("Stage 14-C: RED (concurrent driver.lock age={0:N1}m — {1})" -f $lockAgeMin, $lockBody.Trim()) -ForegroundColor Red
        $ExitTable.Add(("{0,-55} {1}" -f "driver.lock", "BUSY")) | Out-Null
        Write-Summary "FAILED (concurrent regress — workdir race)"
        exit 1
    }
    Remove-Item -LiteralPath $LockPath -Force -ErrorAction SilentlyContinue
}
("{0}`npid={1}" -f $Stamp, $PID) | Set-Content -LiteralPath $LockPath -Encoding ascii

if ($SkipWsl) {
    Write-Host "Stage 14-C: RED (SkipWsl forbidden for graduation — platform-blind)" -ForegroundColor Red
    $ExitTable.Add(("{0,-55} {1}" -f "SkipWsl", "FORBIDDEN")) | Out-Null
    Write-Summary "FAILED (SkipWsl = platform-blind)"
    Remove-Item -LiteralPath $LockPath -Force -ErrorAction SilentlyContinue
    exit 1
}

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
        Write-Host "Stage 14-C: RED (yoyo.exe missing and -SkipBuild)" -ForegroundColor Red
        Fail-Out
    }
    $ExitTable.Add(("{0,-55} {1}" -f "cargo build --release (one)", "SKIP (-SkipBuild)")) | Out-Null
}

if (-not (Test-Path $Yoyo)) {
    Write-Host "Stage 14-C: RED (yoyo.exe missing after build)" -ForegroundColor Red
    Fail-Out
}

# Direct yoyo.exe tests — no cargo after this point.
foreach ($t in @("all", "lock", "gen12", "fullbody")) {
    $null = Invoke-Gate "yoyo.exe test $t" {
        & $Yoyo test $t
    }
    if ($Failed) { Fail-Out }
}

# PS5.1: array splat @("-SkipBuild") does NOT bind [switch]$SkipBuild (stays $false).
# Always pass the named switch token, or hashtable @{ SkipBuild = $true }.
$scriptGates = @(
    "verify-lock-pin.ps1",
    "stage13-link-host.ps1",
    "stage13-cross-platform-parity.ps1",
    "stage12-three-peer-io.ps1",
    "stage12-selfhost-body-section-ddc.ps1",
    "stage11-runtime-surface.ps1",
    "stage11-loadlibrary-host.ps1",
    "stage10-runtime-surface.ps1",
    "stage10-asm-peer-io.ps1",
    "stage9-pure-m4.ps1",
    "stage14-outside-window-scope-cut.ps1",
    "stage14-lock-harden.ps1"
)

try {
    foreach ($name in $scriptGates) {
        $gatePath = Join-Path $Root "scripts\$name"
        $null = Invoke-Gate $name {
            & $gatePath -SkipBuild
        }
        if ($Failed) { Fail-Out }
    }

    $null = Invoke-Gate "stage10-linux-pure-m4.sh (wsl)" {
        # WSL may emit non-fatal stderr; keep exit-code truth under Continue.
        $prevEap = $ErrorActionPreference
        $ErrorActionPreference = "Continue"
        try {
            & wsl -e bash /mnt/f/yoyo/scripts/stage10-linux-pure-m4.sh
        } finally {
            $ErrorActionPreference = $prevEap
        }
    }
    if ($Failed) { Fail-Out }

    Write-Summary $(if ($Failed) { "FAILED" } else { "ALL_GREEN" })

    if ($Failed) {
        Write-Host "Stage 14-C: RED" -ForegroundColor Red
        exit 1
    }
    Write-Host "Stage 14-C: GREEN" -ForegroundColor Green
    exit 0
} finally {
    Remove-Item -LiteralPath $LockPath -Force -ErrorAction SilentlyContinue
}
