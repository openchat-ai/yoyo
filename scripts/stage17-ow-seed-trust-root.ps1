# stage17-ow-seed-trust-root.ps1 — OW-SEED Gate H: formalize YOYO PE trust-root
# (post-v1.0 path 2)
#
# Fail-closed: on-disk gen4.exe must match scripts/ow-seed-trust-root.pin.
# Honest: pin accepts one Rust-ancestry bootstrap root → OW-SEED remains CUT.
# Not CLOSED. Script name stage17-* = post-v1.0 gate id (NOT ROADMAP Stage 17).
#
#   cd F:\yoyo
#   & .\scripts\stage17-ow-seed-trust-root.ps1
#   & .\scripts\stage17-ow-seed-trust-root.ps1 -SkipStage9
param(
    [switch]$SkipStage9
)
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== Post-v1.0: OW-SEED Gate H (trust-root pin) ==="

$PinPath = Join-Path $Root "scripts\ow-seed-trust-root.pin"
$Stage9Dir = Join-Path $Root "scripts\_stage9-pure-m4"
$Emitter = Join-Path $Stage9Dir "gen4.exe"

if (-not (Test-Path $PinPath)) {
    Write-Host "OW_SEED_TRUST_ROOT status=RED reason=missing_pin"
    exit 1
}

$pin = @{}
Get-Content -LiteralPath $PinPath | ForEach-Object {
    if ($_ -match '^\s*#' -or $_ -match '^\s*$') { return }
    if ($_ -match '^\s*([^=]+)=(.*)$') {
        $pin[$Matches[1].Trim()] = $Matches[2].Trim()
    }
}
foreach ($k in @('bytes', 'sha256', 'sha256_prefix', 'disposition')) {
    if (-not $pin.ContainsKey($k) -or [string]::IsNullOrWhiteSpace($pin[$k])) {
        Write-Host "OW_SEED_TRUST_ROOT status=RED reason=pin_missing_field field=$k"
        exit 1
    }
}
if ($pin['disposition'] -ne 'CUT') {
    Write-Host "OW_SEED_TRUST_ROOT status=RED reason=pin_disposition_must_be_CUT got=$($pin['disposition'])"
    exit 1
}

if (-not (Test-Path $Emitter)) {
    if ($SkipStage9) {
        Write-Host "OW_SEED_TRUST_ROOT status=RED reason=missing_gen4_and_SkipStage9"
        exit 1
    }
    Write-Host "=== materialize trust-root via stage9-pure-m4 ==="
    & (Join-Path $Root "scripts\stage9-pure-m4.ps1")
    if ($LASTEXITCODE -ne 0 -or -not (Test-Path $Emitter)) {
        Write-Host "OW_SEED_TRUST_ROOT status=RED reason=stage9_failed"
        exit 1
    }
}

$bytes = (Get-Item -LiteralPath $Emitter).Length
$sha = (Get-FileHash -LiteralPath $Emitter -Algorithm SHA256).Hash.ToLowerInvariant()
$prefix = $sha.Substring(0, [Math]::Min(16, $sha.Length))
$expectBytes = [int64]$pin['bytes']
$expectSha = $pin['sha256'].ToLowerInvariant()
$expectPrefix = $pin['sha256_prefix'].ToLowerInvariant()

Write-Host ("OW_SEED_TRUST_ROOT pin=$PinPath")
Write-Host ("OW_SEED_TRUST_ROOT path={0}" -f $Emitter)
Write-Host ("OW_SEED_TRUST_ROOT bytes={0} expect={1}" -f $bytes, $expectBytes)
Write-Host ("OW_SEED_TRUST_ROOT sha256_prefix={0} expect={1}" -f $prefix, $expectPrefix)
Write-Host ("OW_SEED_TRUST_ROOT sha256={0}" -f $sha)
Write-Host ("OW_SEED_TRUST_ROOT provenance={0}" -f $pin['provenance'])
Write-Host "OW_SEED_TRUST_ROOT disposition=CUT"
Write-Host "OW_SEED_TRUST_ROOT note=pinned_YOYO_PE_root; ancestry_still_includes_Rust_gen1; NOT_CLOSED"

if ($bytes -ne $expectBytes) {
    Write-Host "OW_SEED_TRUST_ROOT status=RED reason=bytes_mismatch"
    exit 1
}
if ($sha -ne $expectSha) {
    Write-Host "OW_SEED_TRUST_ROOT status=RED reason=sha256_mismatch"
    exit 1
}
if (-not $sha.StartsWith($expectPrefix)) {
    Write-Host "OW_SEED_TRUST_ROOT status=RED reason=sha256_prefix_mismatch"
    exit 1
}

Write-Host "OW_SEED_TRUST_ROOT status=GREEN"
exit 0
