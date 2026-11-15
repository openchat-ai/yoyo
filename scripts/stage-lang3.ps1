# stage-lang3.ps1 — v1.3 dual extreme: .yoyo max-readable (no hex in source)
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== LANG3 (v1.3 max-readable .yoyo) ==="

foreach ($req in @("SCOPE-v1.3.md", "LANG-SURFACE.md")) {
    $p = Join-Path $Root $req
    if (-not (Test-Path $p)) { Write-Host "LANG3 status=RED reason=missing_$req"; exit 1 }
}
$surf = Get-Content (Join-Path $Root "LANG-SURFACE.md") -Raw
if ($surf -notlike "*最可读*" -and $surf -notlike "*max*readable*" -and $surf -notlike "*.yoyo*") {
    Write-Host "LANG3 status=RED reason=surface_missing_dual_extreme"
    exit 1
}
Write-Host "LANG3 policy_doc=GREEN"

$Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
if (-not (Test-Path $Yoyo)) { Write-Host "LANG3 status=RED reason=missing_yoyo"; exit 1 }

$src = Join-Path $Root "scripts\fixtures\anyprog\hello_hi.yoyo"
$work = Join-Path $Root "scripts\_stage-lang3"
if (Test-Path $work) { Remove-Item -Recurse -Force $work }
New-Item -ItemType Directory -Force -Path $work | Out-Null

# Source must not contain hex leakage
$srcText = Get-Content -LiteralPath $src -Raw
if ($srcText -match '(?im)^\s*(raw|db)\b' -or $srcText -match '(?im)^\s*[0-9A-Fa-f]{2}(\s+[0-9A-Fa-f]{2})+\s*$') {
    Write-Host "LANG3 status=RED reason=source_has_hex_or_raw"
    exit 1
}
Write-Host "LANG3 source_no_hex=GREEN"

$ty = Join-Path $work "hello_hi.ty"
$exe = Join-Path $work "hello_hi.exe"
$stdout = Join-Path $work "stdout.txt"
$stderr = Join-Path $work "stderr.txt"

& (Join-Path $Root "scripts\lib\Expand-YoyoHi.ps1") -InputPath $src -OutputPath $ty
if (-not (Test-Path $ty)) { Write-Host "LANG3 status=RED reason=expand_failed"; exit 1 }
Write-Host "LANG3 expand=GREEN"

& $Yoyo link --target=win32 $ty $exe
if ($LASTEXITCODE -ne 0 -or -not (Test-Path $exe)) { Write-Host "LANG3 status=RED reason=link"; exit 1 }
Write-Host "LANG3 link=GREEN"

$p = Start-Process -FilePath $exe -WorkingDirectory $work -Wait -PassThru -NoNewWindow `
    -RedirectStandardOutput $stdout -RedirectStandardError $stderr
if ($p.ExitCode -ne 0) { Write-Host "LANG3 status=RED reason=run_exit"; exit 1 }
if ([IO.File]::ReadAllText($stdout) -notlike "*hi*") {
    Write-Host "LANG3 status=RED reason=stdout"
    exit 1
}
Write-Host "LANG3 run=GREEN"

# Poison: raw in .yoyo must fail
$bad = Join-Path $work "bad.yoyo"
@"
program bad
target win32
raw 40 00
exit 0
"@ | Set-Content $bad -Encoding ascii
$caught = $false
try {
    & (Join-Path $Root "scripts\lib\Expand-YoyoHi.ps1") -InputPath $bad -OutputPath (Join-Path $work "bad.ty")
} catch { $caught = $true }
if (-not $caught) { Write-Host "LANG3 status=RED reason=raw_not_rejected"; exit 1 }
Write-Host "LANG3 poison_raw=RED_OK"

Write-Host "LANG3 note=dual_extreme;ty_unreadable_or_yoyo_max_readable;middle_raw_banned;not_Thompson"
Write-Host "LANG3 doc=SCOPE-v1.3.md"
Write-Host "LANG3 status=GREEN"
exit 0
