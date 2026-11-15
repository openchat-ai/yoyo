# stage17-ow-rt-gate-i-gen4.ps1 — S3.531 Gate I gen4 + yoyo.tyb oracle e2e
# trust-root gen4.exe + YOYO in-DLL (oracle+yoyo.tyb) + input.tyb=yoyo.tyb
# Expect exit 0 + output.exe. Oracle != live full compile. OW-RT CUT.
#
#   cd F:\yoyo
#   & .\scripts\stage17-ow-rt-gate-i-gen4.ps1
#
# Honest: GREEN here = gen4 driver served oracle bake; not CLOSED / not M4 flip.
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== Post-v1.0: OW-RT Gate I gen4 + yoyo.tyb oracle (S3.531) ==="

$Work = Join-Path $Root "scripts\_stage17-ow-rt-gate-i-gen4"
New-Item -ItemType Directory -Force -Path $Work | Out-Null

$Gen4 = Join-Path $Root "scripts\_stage9-pure-m4\gen4.exe"
if (-not (Test-Path $Gen4)) {
    Write-Host "OW_RT_GATE_I_GEN4 status=RED reason=missing_trust_root_gen4"
    exit 1
}

Push-Location (Join-Path $Root "yoyo-rust")
try {
    $prevEap = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    Write-Host "=== emit-rt-sidecar --gate-i-gen4-yoyo-tyb ==="
    & cargo run -q -p verifier --bin emit-rt-sidecar --no-default-features --features full-backends -- --gate-i-gen4-yoyo-tyb $Work
    $ec = $LASTEXITCODE
    $ErrorActionPreference = $prevEap
} finally {
    Pop-Location
}

$Sidecar = Join-Path $Work "yoyo_rt.dll"
$Out = Join-Path $Work "output.exe"
$Emitter = Join-Path $Work "emitter_gen4.exe"
$hasSidecar = Test-Path $Sidecar
$hasOut = Test-Path $Out
$hasEmitter = Test-Path $Emitter

Write-Host ("OW_RT_GATE_I_GEN4 exit={0} emitter={1} sidecar={2} output={3}" -f $ec, $hasEmitter, $hasSidecar, $hasOut)

if ($ec -eq 0 -and $hasOut -and $hasSidecar -and $hasEmitter) {
    Write-Host "OW_RT_GATE_I_GEN4 status=GREEN contract=oracle_bake_via_gen4 disposition=CUT"
    Write-Host "OW_RT_GATE_I_GEN4 note=oracle_ne_live_compile; production_default=RUST; not_CLOSED"
    exit 0
}

Write-Host ("OW_RT_GATE_I_GEN4 status=RED reason=gen4_path_or_export exit={0}" -f $ec)
Write-Host "OW_RT_GATE_I_GEN4 note=do_not_fallback_to_host_compose; record root cause"
exit 1
