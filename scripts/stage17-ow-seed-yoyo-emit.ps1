# stage17-ow-seed-yoyo-emit.ps1 — OW-SEED Gate G slice: seed emitted by YOYO PE
# (not Rust yoyo.exe) · post-v1.0 path 2
#
# Proves an operational non-Rust *command* can emit seed ≡ Rust `yoyo link`.
# Honest: the YOYO emitter binary's *provenance* still chains to a one-time
# Rust seed (stage9 gen1) → OW-SEED remains CUT. This is path evidence, not CLOSED.
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

Write-Host "=== Post-v1.0: OW-SEED Gate G slice (YOYO PE emit seed) ==="

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

# --- Path A: YOYO PE emits seed (no Rust yoyo.exe on this step) ---
Copy-Item -Force $Tyb (Join-Path $WorkDir "input.tyb")
Copy-Item -Force $Ty (Join-Path $WorkDir "input.ky")
Push-Location $WorkDir
try {
    if (Test-Path "output.exe") { Remove-Item "output.exe" -Force }
    Write-Host "=== YOYO PE emit: emitter_gen4.exe (zero-arg H_00) ==="
    & $EmitterCopy
    $ec = $LASTEXITCODE
    if ($ec -ne 0 -or -not (Test-Path "output.exe")) {
        Write-Host "OW_SEED_EMIT status=RED reason=yoyo_pe_emit_failed exit=$ec"
        exit 1
    }
    Copy-Item -Force "output.exe" $SeedYoyo
} finally {
    Pop-Location
}

# --- Path B: Rust reference (contrast only; not the Gate G emit path) ---
Write-Host "=== Rust contrast: yoyo link → seed_rust.exe ==="
if (Test-Path $SeedRust) { Remove-Item $SeedRust -Force }
& $Yoyo link --target=win32 $Ty $SeedRust
if ($LASTEXITCODE -ne 0 -or -not (Test-Path $SeedRust)) {
    Write-Host "OW_SEED_EMIT status=RED reason=rust_link_failed"
    exit 1
}

# --- Parity ---
Write-Host "=== seed_yoyo vs seed_rust (.text DDC) ==="
& $Yoyo diff $SeedYoyo $SeedRust 2>&1 | ForEach-Object { Write-Host $_ }
$ddc = $LASTEXITCODE

function Get-ShaPrefix([string]$Path, [int]$N = 16) {
    $h = (Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash.ToLowerInvariant()
    return $h.Substring(0, [Math]::Min($N, $h.Length))
}

$emBytes = (Get-Item $EmitterCopy).Length
$emSha = Get-ShaPrefix $EmitterCopy
$syBytes = (Get-Item $SeedYoyo).Length
$sySha = Get-ShaPrefix $SeedYoyo
$srBytes = (Get-Item $SeedRust).Length
$srSha = Get-ShaPrefix $SeedRust
$fileEq = ((Get-FileHash $SeedYoyo -Algorithm SHA256).Hash -eq (Get-FileHash $SeedRust -Algorithm SHA256).Hash)

Write-Host ""
Write-Host ("OW_SEED_EMIT emitter=emitter_gen4.exe kind=YOYO_PE bytes={0} sha256_prefix={1}" -f $emBytes, $emSha)
Write-Host ("OW_SEED_EMIT seed_yoyo=seed_yoyo.exe bytes={0} sha256_prefix={1}" -f $syBytes, $sySha)
Write-Host ("OW_SEED_EMIT seed_rust=seed_rust.exe bytes={0} sha256_prefix={1} (contrast only)" -f $srBytes, $srSha)
Write-Host ("OW_SEED_EMIT parity_ddc={0} full_file_equal={1}" -f $(if ($ddc -eq 0) { "EQUAL" } else { "DIFF" }), $fileEq)
Write-Host "OW_SEED_EMIT rust_yoyo_exe_on_emit_step=ABSENT"
Write-Host "OW_SEED_EMIT provenance=gen4_ancestry_still_from_rust_stage9_gen1"
Write-Host "OW_SEED_EMIT disposition=CUT"
Write-Host "OW_SEED_EMIT note=operational_non_Rust_emit_path; CLOSED_requires_seed_with_no_Rust_in_provenance"
Write-Host "OW_SEED_EMIT doc=SCOPE-CUT-v1.0-ow-seed-observe.md"

if ($ddc -ne 0 -or -not $fileEq) {
    Write-Host "OW_SEED_EMIT status=RED"
    exit 1
}
Write-Host "OW_SEED_EMIT status=GREEN"
exit 0
