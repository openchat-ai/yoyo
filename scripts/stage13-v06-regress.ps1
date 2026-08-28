# stage13-v06-regress.ps1 — Stage 13-C: v0.6 regression must not regress
# Fail-closed serial gate (no parallel cargo; & not Start-Process|Out-Null).
# Covers: stage13 A+B keep-green + stage12-v05-regress
#   (stage12/11/10/9 + cargo test all/lock/gen12/fullbody + lock pin).
# One release build, then -SkipBuild on nested gates.
param(
    [switch]$SkipBuild,
    [switch]$SkipWsl
)

$ErrorActionPreference = "Continue"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

$WorkDir = Join-Path $Root "scripts\_stage13-v06-regress"
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
    $lines.Add("Stage 13-C v0.6 regression $Stamp") | Out-Null
    $lines.Add("") | Out-Null
    $lines.Add("EXIT TABLE:") | Out-Null
    foreach ($row in $ExitTable) { $lines.Add([string]$row) | Out-Null }
    $lines.Add("") | Out-Null
    $lines.Add($Status) | Out-Null
    $lines.Add("") | Out-Null
    $lines.Add("Trust chain: Stage 13 A/B expansion must not drop v0.6 baseline.") | Out-Null
    $lines.Add("  stage13-link-host + stage13-cross-platform-parity + stage12-v05-regress") | Out-Null
    $lines.Add("  (stage12/11/10/9 + fullbody/lock/gen12 + Lock pin) stay green under one serial gate.") | Out-Null
    $lines.Add("Honest unchanged: Rust runtime + LoadLibrary/libdl; full .text peer may DIFF;") | Out-Null
    $lines.Add("  stub OS still stub; seed still Rust-emitted (A observe, not eliminate).") | Out-Null
    $lines | Set-Content -Path $SummaryPath -Encoding utf8
    foreach ($line in $lines) { Write-Host $line }
}

function Invoke-Gate([string]$Name, [scriptblock]$Body) {
    Write-Host "`n======== $Name ========" -ForegroundColor Cyan
    $sw = [Diagnostics.Stopwatch]::StartNew()
    $code = 0
    try {
        & $Body | Out-Host
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
    Write-Host "Stage 13-C: RED" -ForegroundColor Red
    exit 1
}

Write-Host "=== Stage 13-C: v0.6 regression gate ==="
Write-Host "  serial: one release build → stage13 A+B → stage12-v05-regress (-SkipBuild)"
Write-Host "  stamp: $Stamp"

if ($SkipWsl) {
    Write-Host "Stage 13-C: RED (SkipWsl forbidden for graduation — platform-blind)" -ForegroundColor Red
    $ExitTable.Add(("{0,-55} {1}" -f "SkipWsl", "FORBIDDEN")) | Out-Null
    Write-Summary "FAILED (SkipWsl = platform-blind)"
    exit 1
}

# --- at most one release build ---
if (-not $SkipBuild) {
    $null = Invoke-Gate "cargo build --release -p verifier" {
        Push-Location (Join-Path $Root "yoyo-rust")
        try {
            cargo build --release -p verifier
        } finally {
            Pop-Location
        }
    }
    if ($Failed) { Fail-Out }

    $needRuntime = -not ((Test-Path $RuntimeDllPreferred) -or (Test-Path $RuntimeDllCompat))
    if ($needRuntime) {
        $null = Invoke-Gate "cargo build --release -p yoyo-runtime" {
            Push-Location (Join-Path $Root "yoyo-rust")
            try {
                cargo build --release -p yoyo-runtime
            } finally {
                Pop-Location
            }
        }
        if ($Failed) { Fail-Out }
    } else {
        $ExitTable.Add(("{0,-55} {1}" -f "cargo build yoyo-runtime", "SKIP (present)")) | Out-Null
    }
} else {
    if (-not (Test-Path $Yoyo)) {
        Write-Host "Stage 13-C: RED (yoyo.exe missing and -SkipBuild)" -ForegroundColor Red
        Fail-Out
    }
}

if (-not (Test-Path $Yoyo)) {
    Write-Host "Stage 13-C: RED (yoyo.exe missing after build)" -ForegroundColor Red
    Fail-Out
}

# Prefer -SkipBuild after release binary is warm (nested gates rebuild otherwise).
$scriptGates = @(
    @{ N = "stage13-link-host.ps1"; A = @("-SkipBuild") },
    @{ N = "stage13-cross-platform-parity.ps1"; A = @("-SkipBuild") },
    @{ N = "stage12-v05-regress.ps1"; A = @("-SkipBuild") }
)

foreach ($g in $scriptGates) {
    $gatePath = Join-Path $Root "scripts\$($g.N)"
    $gateArgs = @($g.A)
    $null = Invoke-Gate $g.N {
        & $gatePath @gateArgs
    }
    if ($Failed) { Fail-Out }
}

Write-Summary $(if ($Failed) { "FAILED" } else { "ALL_GREEN" })

if ($Failed) {
    Write-Host "Stage 13-C: RED" -ForegroundColor Red
    exit 1
}
Write-Host "Stage 13-C: GREEN" -ForegroundColor Green
exit 0
