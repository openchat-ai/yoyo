# stage12-v05-regress.ps1 — Stage 12-C: v0.5 regression must not regress
# Fail-closed serial gate (no parallel cargo; & not Start-Process|Out-Null).
# Covers: cargo test all/lock/gen12/fullbody + stage11/10/9 + A/B keep-green.
# Alias: scripts/stage12-regression.ps1
param(
    [switch]$SkipBuild,
    [switch]$SkipWsl
)

$ErrorActionPreference = "Continue"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

$WorkDir = Join-Path $Root "scripts\_stage12-v05-regress"
New-Item -ItemType Directory -Force -Path $WorkDir | Out-Null
$Stamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
$SummaryPath = Join-Path $WorkDir "summary.txt"
$ExitTable = [System.Collections.Generic.List[string]]::new()
$Failed = $false

function Write-Summary([string]$Status) {
    # Build line-by-line (nested ToArray inside @() can collapse to one Write-Host line).
    $lines = [System.Collections.Generic.List[string]]::new()
    $lines.Add("Stage 12-C v0.5 regression $Stamp") | Out-Null
    $lines.Add("") | Out-Null
    $lines.Add("EXIT TABLE:") | Out-Null
    foreach ($row in $ExitTable) { $lines.Add([string]$row) | Out-Null }
    $lines.Add("") | Out-Null
    $lines.Add($Status) | Out-Null
    $lines.Add("") | Out-Null
    $lines.Add("Trust chain: v0.5 stage11/10/9 + fullbody/lock/gen12 + Lock pin stay green after Stage 12 A/B surface expansion.") | Out-Null
    $lines.Add("A/B keep-green: stage12-three-peer-io + stage12-selfhost-body-section-ddc.") | Out-Null
    $lines.Add("Honest unchanged: Rust runtime + LoadLibrary/libdl; full .text peer may DIFF (B window EQUAL).") | Out-Null
    $lines | Set-Content -Path $SummaryPath -Encoding utf8
    foreach ($line in $lines) { Write-Host $line }
}

function Invoke-Gate([string]$Name, [scriptblock]$Body) {
    Write-Host "`n======== $Name ========" -ForegroundColor Cyan
    $sw = [Diagnostics.Stopwatch]::StartNew()
    $code = 0
    try {
        # Do not assign scriptblock stdout into a variable (poisons exit checks).
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

Write-Host "=== Stage 12-C: v0.5 regression gate ==="
Write-Host "  serial cargo + stage11/10/9 + A/B keep-green"
Write-Host "  stamp: $Stamp"

if (-not $SkipBuild) {
    $null = Invoke-Gate "cargo build --release -p verifier" {
        Push-Location (Join-Path $Root "yoyo-rust")
        try {
            cargo build --release -p verifier
        } finally {
            Pop-Location
        }
    }
    if ($Failed) {
        Write-Summary "FAILED"
        Write-Host "Stage 12-C: RED" -ForegroundColor Red
        exit 1
    }
    $null = Invoke-Gate "cargo build --release -p yoyo-runtime" {
        Push-Location (Join-Path $Root "yoyo-rust")
        try {
            cargo build --release -p yoyo-runtime
        } finally {
            Pop-Location
        }
    }
    if ($Failed) {
        Write-Summary "FAILED"
        Write-Host "Stage 12-C: RED" -ForegroundColor Red
        exit 1
    }
}

Push-Location (Join-Path $Root "yoyo-rust\verifier")
foreach ($t in @("all", "lock", "gen12", "fullbody")) {
    $null = Invoke-Gate "cargo run --release -- test $t" {
        cargo run --release -- test $t
    }
    if ($Failed) {
        Pop-Location
        Write-Summary "FAILED"
        Write-Host "Stage 12-C: RED" -ForegroundColor Red
        exit 1
    }
}
Pop-Location

# Prefer -SkipBuild after release binary is warm (gates rebuild otherwise).
$scriptGates = @(
    @{ N = "verify-lock-pin.ps1"; A = @() },
    @{ N = "stage11-runtime-surface.ps1"; A = @("-SkipBuild") },
    @{ N = "stage11-loadlibrary-host.ps1"; A = @("-SkipBuild") },
    @{ N = "stage10-runtime-surface.ps1"; A = @("-SkipBuild") },
    @{ N = "stage10-asm-peer-io.ps1"; A = @("-SkipBuild") },
    @{ N = "stage9-js-peer-io.ps1"; A = @("-SkipBuild") },
    @{ N = "stage9-pure-m4.ps1"; A = @("-SkipBuild") },
    @{ N = "stage12-three-peer-io.ps1"; A = @("-SkipBuild") },
    @{ N = "stage12-selfhost-body-section-ddc.ps1"; A = @("-SkipBuild") }
)

foreach ($g in $scriptGates) {
    $gatePath = Join-Path $Root "scripts\$($g.N)"
    $gateArgs = @($g.A)
    $null = Invoke-Gate $g.N {
        if ($gateArgs.Count -gt 0) {
            & $gatePath @gateArgs
        } else {
            & $gatePath
        }
    }
    if ($Failed) {
        Write-Summary "FAILED"
        Write-Host "Stage 12-C: RED" -ForegroundColor Red
        exit 1
    }
}

if (-not $SkipWsl) {
    $null = Invoke-Gate "stage10-linux-pure-m4.sh (wsl)" {
        & wsl bash /mnt/f/yoyo/scripts/stage10-linux-pure-m4.sh
    }
    if ($Failed) {
        Write-Summary "FAILED"
        Write-Host "Stage 12-C: RED" -ForegroundColor Red
        exit 1
    }
} else {
    $ExitTable.Add(("{0,-55} {1}" -f "stage10-linux-pure-m4.sh (wsl)", "SKIP")) | Out-Null
    Write-Host "SKIP WSL (-SkipWsl)" -ForegroundColor Yellow
}

Write-Summary $(if ($Failed) { "FAILED" } else { "ALL_GREEN" })

if ($Failed) {
    Write-Host "Stage 12-C: RED" -ForegroundColor Red
    exit 1
}
Write-Host "Stage 12-C: GREEN" -ForegroundColor Green
exit 0
