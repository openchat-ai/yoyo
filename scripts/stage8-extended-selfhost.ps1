# stage8-extended-selfhost.ps1 — Windows M2→M3→M4 self-host chain (Stage 8-C)
# M2→M3: gen2rt embedded startup → compile input → gen3 (same as stage5)
# M3→M4: gen3rt embedded startup → compile input → gen4
# gen4 parity vs gen3_direct (yoyo bootstrap reference) via .text section-ddc
param(
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

$WorkDir = Join-Path $Root "scripts\_stage8-win"
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

$Gen2 = Join-Path $WorkDir "gen2.exe"
$Gen3 = Join-Path $WorkDir "gen3.exe"
$Gen3Direct = Join-Path $WorkDir "gen3_direct.exe"
$Gen4 = Join-Path $WorkDir "gen4.exe"
$InputTyb = Join-Path $WorkDir "input.tyb"
$InputKy = Join-Path $WorkDir "input.ky"
$RuntimeDll = Join-Path $WorkDir "yoyo_runtime.dll"

Copy-Item -Force $Tyb $InputTyb
Copy-Item -Force $Ty $InputKy

$m2m3Green = $false
$m3m4Green = $false
$parityEqual = $false
$trustSha = ""

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
Write-Host "=== M3→M4: gen3rt embedded startup compiles input → gen4 (no AV, no sidecar) ==="
Push-Location $WorkDir
try {
    if (Test-Path $Gen4) { Remove-Item $Gen4 }
    if (Test-Path "output.exe") { Remove-Item "output.exe" }
    $Gen3rt = Join-Path $WorkDir "gen3rt.exe"
    if (Test-Path $Gen3rt) { Remove-Item $Gen3rt }
    if (Test-Path $RuntimeDll) { Remove-Item $RuntimeDll }

    if (-not $m2m3Green) {
        Write-Host "M3→M4: SKIP (M2→M3 not green)"
    } else {
        Write-Host "building gen3_direct reference via bootstrap..."
        if (Test-Path $Gen3Direct) { Remove-Item $Gen3Direct }
        & $Yoyo bootstrap $InputTyb $Gen3Direct
        if ($LASTEXITCODE -ne 0 -or -not (Test-Path $Gen3Direct)) {
            Write-Host "M3→M4: RED (gen3_direct bootstrap failed)"
        } else {
            Write-Host "gen3_direct: $((Get-Item $Gen3Direct).Length) bytes"

            Write-Host "building gen3rt via bootstrap --selfhost..."
            & $Yoyo bootstrap --selfhost $InputTyb $Gen3rt
            if ($LASTEXITCODE -ne 0 -or -not (Test-Path $Gen3rt)) {
                Write-Host "M3→M4: RED (bootstrap --selfhost gen3rt failed)"
            } elseif (Test-Path $RuntimeDll) {
                Write-Host "M3→M4: RED (unexpected yoyo_runtime.dll sidecar in workdir)"
            } else {
                & $Gen3rt
                $ec = $LASTEXITCODE
                if ($ec -eq 0xC0000005) {
                    Write-Host "M3→M4: RED (STATUS_ACCESS_VIOLATION 0xC0000005)"
                } elseif ((Test-Path "output.exe") -and $ec -eq 0) {
                    Copy-Item -Force "output.exe" $Gen4
                    $m3m4Green = $true
                    Write-Host "M3→M4: GREEN (gen4=$((Get-Item $Gen4).Length) bytes, embedded startup, no sidecar)"

                    Write-Host ""
                    Write-Host "=== trust chain: gen4 vs gen3_direct (.text section-ddc) ==="
                    & $Yoyo diff $Gen4 $Gen3Direct 2>&1 | ForEach-Object { Write-Host $_ }
                    if ($LASTEXITCODE -eq 0) {
                        $parityEqual = $true
                        $trustSha = (Get-FileHash -Algorithm SHA256 -Path $Gen4).Hash.Substring(0, 8).ToLower()
                        Write-Host "gen4 ≡ gen3_direct (.text DDC): EQUAL (sha256 prefix $trustSha)"
                    } else {
                        Write-Host "gen4 ≡ gen3_direct (.text DDC): DIFF"
                        & $Yoyo diff $Gen3 $Gen4 2>&1 | ForEach-Object { Write-Host "  gen3 vs gen4: $_" }
                    }

                    Write-Host ""
                    Write-Host "=== gen12 window: gen2 vs gen3 vs gen4 (.text DDC) ==="
                    if (Test-Path $Gen2) {
                        & $Yoyo diff $Gen2 $Gen3 2>&1 | ForEach-Object { Write-Host "  gen2 vs gen3: $_" }
                    }
                    & $Yoyo diff $Gen3 $Gen4 2>&1 | ForEach-Object { Write-Host "  gen3 vs gen4: $_" }
                } else {
                    Write-Host "M3→M4: RED (exit=$ec, no output.exe)"
                }
            }
        }
    }
} finally {
    Pop-Location
}

Write-Host ""
Write-Host "=== summary ==="
Write-Host "M2→M3 runtime:   $(if ($m2m3Green) { 'GREEN' } else { 'RED' })"
Write-Host "M3→M4 runtime:   $(if ($m3m4Green) { 'GREEN' } else { 'RED' })"
Write-Host "gen4 DDC parity: $(if ($parityEqual) { "EQUAL (sha256 prefix $trustSha)" } else { 'DIFF or N/A' })"
Write-Host "Stage 8-C:       $(if ($m2m3Green -and $m3m4Green -and $parityEqual) { 'may check [x]' } else { 'keep [ ] — chain incomplete or DDC mismatch' })"
Write-Host ""
Write-Host "Trust chain: M4 chain uses same gen12/section-ddc gates as stage5/fullbody."
Write-Host "  gen3_direct reference = yoyo bootstrap input.tyb (788-handler full body)"
Write-Host "  gen4 = gen3rt runtime output (second embedded selfhost generation)"
if ($trustSha) {
    Write-Host "  gen4 .text SHA256 prefix: $trustSha"
}

if ($m2m3Green -and $m3m4Green -and $parityEqual) { exit 0 } else { exit 1 }
