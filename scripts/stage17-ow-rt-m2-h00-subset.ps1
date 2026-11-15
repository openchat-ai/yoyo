# stage17-ow-rt-m2-h00-subset.ps1 — S3.526 / M2 Gate I subset
# H_00 seed PE + YOYO in-DLL recompile DLL (no Rust cwd sidecar) + mini RET TYB.
# Expect exit 0 + output.exe. Full Gate I (gen4 + yoyo.tyb) remains RED.
#
#   cd F:\yoyo
#   & .\scripts\stage17-ow-rt-m2-h00-subset.ps1
#
# Honest: mini ≠ full compiler; production default still Rust → OW-RT CUT.
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== Post-v1.0: OW-RT M2 H_00 / Gate I subset (mini) ==="

$Work = Join-Path $Root "scripts\_stage17-ow-rt-m2-h00-subset"
New-Item -ItemType Directory -Force -Path $Work | Out-Null

Push-Location (Join-Path $Root "yoyo-rust")
try {
    $prevEap = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    Write-Host "=== emit-rt-sidecar --m2-h00-subset ==="
    & cargo run -q -p verifier --bin emit-rt-sidecar --no-default-features --features full-backends -- --m2-h00-subset $Work
    $ec = $LASTEXITCODE
    $ErrorActionPreference = $prevEap
} finally {
    Pop-Location
}

$Sidecar = Join-Path $Work "yoyo_rt.dll"
$Out = Join-Path $Work "output.exe"
$Seed = Join-Path $Work "seed_h00.exe"
$hasSidecar = Test-Path $Sidecar
$hasOut = Test-Path $Out
$hasSeed = Test-Path $Seed

Write-Host ("OW_RT_M2_H00_SUBSET exit={0} seed={1} sidecar={2} output={3}" -f $ec, $hasSeed, $hasSidecar, $hasOut)

if ($ec -eq 0 -and $hasOut -and $hasSidecar -and $hasSeed) {
    Write-Host "OW_RT_M2_H00_SUBSET status=GREEN contract=mini_only disposition=CUT"
    Write-Host "OW_RT_M2_H00_SUBSET note=full_Gate_I_gen4_yoyo_tyb_remains_RED; production_default=RUST"
    exit 0
}

Write-Host ("OW_RT_M2_H00_SUBSET status=RED reason=h00_path_or_export exit={0}" -f $ec)
Write-Host "OW_RT_M2_H00_SUBSET note=do_not_fallback_to_host_compose; record root cause (AV/export/IAT)"
exit 1
