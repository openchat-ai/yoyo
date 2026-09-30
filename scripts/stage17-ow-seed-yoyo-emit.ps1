# stage17-ow-seed-yoyo-emit.ps1 — OW-SEED Gate G slice: seed emitted by YOYO PE
# (not Rust yoyo.exe) · post-v1.0 path 2
#
# Hop 1: gen4 (YOYO PE) → seed_yoyo ≡ Rust `yoyo link`
# Hop 2: seed_yoyo (YOYO PE) → seed2 ≡ seed_yoyo  (multi-hop, still no Rust on emit)
#
# Honest: trust-root gen4 provenance still chains to Rust stage9 gen1;
# cwd still needs Rust yoyo_rt.dll → OW-SEED remains CUT. Not CLOSED.
#
# Script name stage17-* = post-v1.0 gate id (NOT ROADMAP Stage 17).
#
#   cd F:\yoyo
#   & .\scripts\stage17-ow-seed-yoyo-emit.ps1
param(
    [switch]$SkipStage9
)
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== Post-v1.0: OW-SEED Gate G slice (YOYO PE emit seed · multi-hop) ==="

$Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
$Ty = Join-Path $Root "yoyo\projects\yoyo.ty"
$Tyb = Join-Path $Root "yoyo\projects\yoyo.tyb"
$Stage9Dir = Join-Path $Root "scripts\_stage9-pure-m4"
$Emitter = Join-Path $Stage9Dir "gen4.exe"
$WorkDir = Join-Path $Root "scripts\_stage17-ow-seed-yoyo-emit"
New-Item -ItemType Directory -Force -Path $WorkDir | Out-Null

if (-not (Test-Path $Yoyo)) {
    Write-Host "OW_SEED_EMIT status=RED reason=missing_yoyo_exe"
    exit 1
}
if (-not (Test-Path $Ty) -or -not (Test-Path $Tyb)) {
    Write-Host "OW_SEED_EMIT status=RED reason=missing_yoyo_ty"
    exit 1
}

if (-not (Test-Path $Emitter)) {
    if ($SkipStage9) {
        Write-Host "OW_SEED_EMIT status=RED reason=missing_gen4_and_SkipStage9"
        exit 1
    }
    Write-Host "=== bootstrap emitter: stage9-pure-m4 (produces gen4 YOYO PE) ==="
    & (Join-Path $Root "scripts\stage9-pure-m4.ps1")
    if ($LASTEXITCODE -ne 0 -or -not (Test-Path $Emitter)) {
        Write-Host "OW_SEED_EMIT status=RED reason=stage9_failed"
        exit 1
    }
}

$SeedYoyo = Join-Path $WorkDir "seed_yoyo.exe"
$Seed2 = Join-Path $WorkDir "seed2.exe"
$SeedRust = Join-Path $WorkDir "seed_rust.exe"
$EmitterCopy = Join-Path $WorkDir "emitter_gen4.exe"
Copy-Item -Force $Emitter $EmitterCopy

# H_00 cwd sidecar (same posture as stage9-pure-m4 workdir)
$SidecarSrc = Join-Path $Stage9Dir "yoyo_rt.dll"
if (-not (Test-Path $SidecarSrc)) {
    $SidecarSrc = Join-Path $Root "yoyo-rust\target\release-runtime\yoyo_runtime.dll"
}
if (-not (Test-Path $SidecarSrc)) {
    $SidecarSrc = Join-Path $Root "yoyo-rust\target\release\yoyo_runtime.dll"
}
if (-not (Test-Path $SidecarSrc)) {
    Write-Host "OW_SEED_EMIT status=RED reason=missing_yoyo_rt_sidecar"
    exit 1
}
Copy-Item -Force $SidecarSrc (Join-Path $WorkDir "yoyo_rt.dll")

function Invoke-YoyoPeEmit {
    param([string]$ExePath, [string]$OutPath, [string]$Label)
    Copy-Item -Force $Tyb (Join-Path $WorkDir "input.tyb")
    Copy-Item -Force $Ty (Join-Path $WorkDir "input.ky")
    Push-Location $WorkDir
    try {
        if (Test-Path "output.exe") { Remove-Item "output.exe" -Force }
        Write-Host "=== YOYO PE emit: $Label ==="
        & $ExePath
        $ec = $LASTEXITCODE
        if ($ec -ne 0 -or -not (Test-Path "output.exe")) {
            Write-Host "OW_SEED_EMIT status=RED reason=yoyo_pe_emit_failed label=$Label exit=$ec"
            exit 1
        }
        Copy-Item -Force "output.exe" $OutPath
    } finally {
        Pop-Location
    }
}

# --- Hop 1: trust-root gen4 → seed_yoyo (no Rust yoyo.exe) ---
Invoke-YoyoPeEmit -ExePath $EmitterCopy -OutPath $SeedYoyo -Label "hop1 emitter_gen4.exe → seed_yoyo.exe"

# --- Hop 2: seed_yoyo → seed2 (still no Rust yoyo.exe) ---
Invoke-YoyoPeEmit -ExePath $SeedYoyo -OutPath $Seed2 -Label "hop2 seed_yoyo.exe → seed2.exe"

# --- Rust contrast (verification tooling only; not an emit hop) ---
Write-Host "=== Rust contrast: yoyo link → seed_rust.exe ==="
if (Test-Path $SeedRust) { Remove-Item $SeedRust -Force }
& $Yoyo link --target=win32 $Ty $SeedRust
if ($LASTEXITCODE -ne 0 -or -not (Test-Path $SeedRust)) {
    Write-Host "OW_SEED_EMIT status=RED reason=rust_link_failed"
    exit 1
}

Write-Host "=== seed_yoyo vs seed_rust (.text DDC) ==="
& $Yoyo diff $SeedYoyo $SeedRust 2>&1 | ForEach-Object { Write-Host $_ }
$ddcRust = $LASTEXITCODE

Write-Host "=== seed2 vs seed_yoyo (.text DDC) ==="
& $Yoyo diff $Seed2 $SeedYoyo 2>&1 | ForEach-Object { Write-Host $_ }
$ddcHop2 = $LASTEXITCODE

function Get-ShaPrefix([string]$Path, [int]$N = 16) {
    $h = (Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash.ToLowerInvariant()
    return $h.Substring(0, [Math]::Min($N, $h.Length))
}

$emBytes = (Get-Item $EmitterCopy).Length
$emSha = Get-ShaPrefix $EmitterCopy
$syBytes = (Get-Item $SeedYoyo).Length
$sySha = Get-ShaPrefix $SeedYoyo
$s2Bytes = (Get-Item $Seed2).Length
$s2Sha = Get-ShaPrefix $Seed2
$srBytes = (Get-Item $SeedRust).Length
$srSha = Get-ShaPrefix $SeedRust
$h1 = (Get-FileHash $SeedYoyo -Algorithm SHA256).Hash
$h2 = (Get-FileHash $Seed2 -Algorithm SHA256).Hash
$hr = (Get-FileHash $SeedRust -Algorithm SHA256).Hash
$fileEqAll = ($h1 -eq $h2) -and ($h1 -eq $hr)

Write-Host ""
Write-Host ("OW_SEED_EMIT hops=2")
Write-Host ("OW_SEED_EMIT hop1_emitter=emitter_gen4.exe kind=YOYO_PE bytes={0} sha256_prefix={1}" -f $emBytes, $emSha)
Write-Host ("OW_SEED_EMIT hop1_out=seed_yoyo.exe bytes={0} sha256_prefix={1}" -f $syBytes, $sySha)
Write-Host ("OW_SEED_EMIT hop2_emitter=seed_yoyo.exe kind=YOYO_PE")
Write-Host ("OW_SEED_EMIT hop2_out=seed2.exe bytes={0} sha256_prefix={1}" -f $s2Bytes, $s2Sha)
Write-Host ("OW_SEED_EMIT seed_rust=seed_rust.exe bytes={0} sha256_prefix={1} (contrast only)" -f $srBytes, $srSha)
Write-Host ("OW_SEED_EMIT parity_hop1_vs_rust={0} parity_hop2_vs_hop1={1} full_file_equal={2}" -f `
    $(if ($ddcRust -eq 0) { "EQUAL" } else { "DIFF" }), `
    $(if ($ddcHop2 -eq 0) { "EQUAL" } else { "DIFF" }), `
    $fileEqAll)
Write-Host "OW_SEED_EMIT rust_yoyo_exe_on_emit_hops=ABSENT"
Write-Host "OW_SEED_EMIT rust_sidecar_cwd=PRESENT"
Write-Host "OW_SEED_EMIT provenance=trust_root_gen4_ancestry_still_from_rust_stage9_gen1"
Write-Host "OW_SEED_EMIT disposition=CUT"
Write-Host "OW_SEED_EMIT note=multi_hop_YOYO_emit; CLOSED_requires_no_Rust_in_trust_root_or_sidecar"
Write-Host "OW_SEED_EMIT doc=SCOPE-CUT-v1.0-ow-seed-observe.md"

if ($ddcRust -ne 0 -or $ddcHop2 -ne 0 -or -not $fileEqAll) {
    Write-Host "OW_SEED_EMIT status=RED"
    exit 1
}
Write-Host "OW_SEED_EMIT status=GREEN"
exit 0
