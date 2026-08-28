# stage5-win-selfhost.ps1 — Windows M1→M2→M3 self-host chain monitor
# M1→M2 interim: yoyo bootstrap (Rust host compiler, not runtime selfhost in gen1.exe)
# M2→M3: gen2rt.exe single-file (runtime embedded in PE; extract to %TEMP% at run) → compile input → output.exe
param(
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

$WorkDir = Join-Path $Root "scripts\_stage5-win"
New-Item -ItemType Directory -Force -Path $WorkDir | Out-Null

$Ty = Join-Path $Root "yoyo\projects\yoyo.ty"
$Tyb = Join-Path $Root "yoyo\projects\yoyo.tyb"
$Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
$RuntimeDllBuilt = Join-Path $Root "yoyo-rust\target\release\yoyo_runtime.dll"

if (-not (Test-Path $Yoyo) -or -not (Test-Path $RuntimeDllBuilt)) {
    Write-Host "== build yoyo + yoyo-runtime (release) =="
    Push-Location (Join-Path $Root "yoyo-rust")
    cargo build --release -p verifier
    if ($LASTEXITCODE -ne 0) { throw "verifier build failed" }
    cargo build --release -p yoyo-runtime
    if ($LASTEXITCODE -ne 0) { throw "yoyo-runtime build failed" }
    Pop-Location
}

if (-not (Test-Path $Tyb)) {
    Write-Host "== ty2tyb =="
    python (Join-Path $Root "scripts\ty2tyb.py")
    if (-not (Test-Path $Tyb)) { throw "ty2tyb failed: missing $Tyb" }
}

$Gen1 = Join-Path $WorkDir "gen1.exe"
$Gen2 = Join-Path $WorkDir "gen2.exe"
$Gen3 = Join-Path $WorkDir "gen3.exe"
$InputTyb = Join-Path $WorkDir "input.tyb"
$InputKy = Join-Path $WorkDir "input.ky"
$RuntimeDll = Join-Path $WorkDir "yoyo_runtime.dll"

Copy-Item -Force $Tyb $InputTyb
Copy-Item -Force $Ty $InputKy

if (-not $SkipBuild) {
    Write-Host "== M0: yoyo link (gen1 reference) =="
    & $Yoyo link --target=win32 $Ty $Gen1
    if ($LASTEXITCODE -ne 0) { throw "gen1 link failed (exit $LASTEXITCODE)" }
}

$m1m2Green = $false
$m2m3Green = $false

Write-Host ""
Write-Host "=== M1→M2: bootstrap input.tyb → gen2.exe (interim) ==="
Push-Location $WorkDir
try {
    if (Test-Path $Gen2) { Remove-Item $Gen2 }
    & $Yoyo bootstrap $InputTyb $Gen2
    if ($LASTEXITCODE -eq 0 -and (Test-Path $Gen2)) {
        $m1m2Green = $true
        Write-Host "M1→M2 bootstrap: GREEN (gen2=$((Get-Item $Gen2).Length) bytes)"
        if (Test-Path $Gen1) {
            & $Yoyo diff $Gen1 $Gen2 2>&1 | ForEach-Object { Write-Host $_ }
            if ($LASTEXITCODE -eq 0) {
                Write-Host "gen1 ≡ gen2 (.text DDC): EQUAL"
            } else {
                Write-Host "gen1 ≡ gen2 (.text DDC): DIFF (expected until emit parity)"
            }
        }
    } else {
        Write-Host "M1→M2 bootstrap: RED"
    }
} finally {
    Pop-Location
}

Write-Host ""
Write-Host "=== M1→M2: gen1.exe runtime (Stage 9-A H_00 pure path — no genNrt entry wrapper) ==="
Push-Location $WorkDir
$gen1RuntimeGreen = $false
try {
    if (Test-Path "output.exe") { Remove-Item "output.exe" }
    if (Test-Path $Gen1) {
        & $Gen1
        $ec = $LASTEXITCODE
        if ((Test-Path "output.exe") -and $ec -eq 0) {
            $gen1RuntimeGreen = $true
            Write-Host "gen1 runtime selfhost: GREEN (output.exe=$((Get-Item 'output.exe').Length) bytes, H_00 entry)"
        } else {
            Write-Host "gen1 runtime selfhost: RED (exit=$ec, no output.exe)"
        }
    } else {
        Write-Host "gen1 runtime selfhost: SKIP (no gen1.exe)"
    }
} finally {
    Pop-Location
}

Write-Host ""
Write-Host "=== M2→M3: gen2rt embedded startup compiles input → gen3 (no AV, no sidecar) ==="
Push-Location $WorkDir
try {
    if (Test-Path $Gen3) { Remove-Item $Gen3 }
    if (Test-Path "output.exe") { Remove-Item "output.exe" }
    $Gen2rt = Join-Path $WorkDir "gen2rt.exe"
    if (Test-Path $Gen2rt) { Remove-Item $Gen2rt }
    if (Test-Path $RuntimeDll) { Remove-Item $RuntimeDll }
    Write-Host "building gen2rt via bootstrap --selfhost (single-file, runtime embedded in PE)..."
    & $Yoyo bootstrap --selfhost $InputTyb $Gen2rt
    if ($LASTEXITCODE -ne 0 -or -not (Test-Path $Gen2rt)) {
        Write-Host "M2→M3: RED (bootstrap --selfhost failed)"
    } elseif (Test-Path $RuntimeDll) {
        Write-Host "M2→M3: RED (unexpected yoyo_runtime.dll sidecar in workdir)"
    } else {
        & $Gen2rt
        $ec = $LASTEXITCODE
        if ($ec -eq 0xC0000005) {
            Write-Host "M2→M3: RED (STATUS_ACCESS_VIOLATION 0xC0000005)"
        } elseif ((Test-Path "output.exe") -and $ec -eq 0) {
            Copy-Item -Force "output.exe" $Gen3
            $m2m3Green = $true
            Write-Host "M2→M3: GREEN (gen3=$((Get-Item $Gen3).Length) bytes, embedded startup, no sidecar)"
        } else {
            Write-Host "M2→M3: RED (exit=$ec, no output.exe)"
        }
    }
} finally {
    Pop-Location
}

Write-Host ""
Write-Host "=== summary ==="
Write-Host "M1→M2 bootstrap: $(if ($m1m2Green) { 'GREEN' } else { 'RED' })"
Write-Host "gen1 H_00 runtime: $(if ($gen1RuntimeGreen) { 'GREEN' } else { 'RED' })"
Write-Host "M2→M3 runtime:   $(if ($m2m3Green) { 'GREEN' } else { 'RED' })"
Write-Host "Stage 5 checkbox:  $(if ($m2m3Green) { 'may check [x]' } else { 'keep [ ] — partial bootstrap only' })"
Write-Host "Stage 9-A (H_00):  $(if ($gen1RuntimeGreen) { 'may check [x]' } else { 'keep [ ]' })"

if ($m2m3Green) { exit 0 } else { exit 1 }
