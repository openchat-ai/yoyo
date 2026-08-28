# stage9-gen1-h00-selfhost.ps1 — Stage 9-A: gen1 H_00 pure runtime selfhost
# Proves gen1.exe (PE entry → H_00, no genNrt startup wrapper) produces output.exe exit 0.
# Uses Stage 8-A merged kernel32 IAT in-process for DLL extract + LoadLibrary.
param(
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

$WorkDir = Join-Path $Root "scripts\_stage9-h00"
New-Item -ItemType Directory -Force -Path $WorkDir | Out-Null

$Ty = Join-Path $Root "yoyo\projects\yoyo.ty"
$Tyb = Join-Path $Root "yoyo\projects\yoyo.tyb"
$Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
$RuntimeDllBuilt = Join-Path $Root "yoyo-rust\target\release\yoyo_runtime.dll"

if (-not (Test-Path $Yoyo) -or -not (Test-Path $RuntimeDllBuilt)) {
    if ($SkipBuild) { throw "missing yoyo.exe or yoyo_runtime.dll (and -SkipBuild)" }
    Write-Host "== build yoyo + yoyo-runtime (release) =="
    Push-Location (Join-Path $Root "yoyo-rust")
    cargo build --release -p verifier
    if ($LASTEXITCODE -ne 0) { throw "verifier build failed" }
    cargo build --release -p yoyo-runtime
    if ($LASTEXITCODE -ne 0) { throw "yoyo-runtime build failed" }
    Pop-Location
}

if (-not (Test-Path $Tyb)) {
    python (Join-Path $Root "scripts\ty2tyb.py")
    if (-not (Test-Path $Tyb)) { throw "ty2tyb failed" }
}

$Gen1 = Join-Path $WorkDir "gen1.exe"
$InputTyb = Join-Path $WorkDir "input.tyb"
Copy-Item -Force $Tyb $InputTyb
Copy-Item -Force $Ty (Join-Path $WorkDir "input.ky")

Write-Host "== link gen1 (H_00 runtime path) =="
& $Yoyo link --target=win32 $Ty $Gen1
if ($LASTEXITCODE -ne 0) { throw "gen1 link failed (exit $LASTEXITCODE)" }

Push-Location $WorkDir
try {
    if (Test-Path "output.exe") { Remove-Item "output.exe" }
    Write-Host "== run gen1.exe (zero-arg H_00) =="
    & $Gen1
    $ec = $LASTEXITCODE
    if ($ec -ne 0 -or -not (Test-Path "output.exe")) {
        Write-Host "Stage 9-A: RED (exit=$ec, output.exe missing)"
        exit 1
    }
    Write-Host "Stage 9-A: GREEN (output.exe=$((Get-Item 'output.exe').Length) bytes)"
    exit 0
} finally {
    Pop-Location
}
