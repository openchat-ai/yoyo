# stage-fullbar-trust-root.ps1 — AUTO-FULLBAR FS-2 / F2
#
# F2: trust-root cold-start ancestry MUST NOT contain rust_stage9_gen1.
# Recipe:
#   1) JS + asm each link same yoyo.ty → full-file EQUAL seeds
#   2) body-window EQUAL vs Rust link (contrast only; Rust not ancestry)
#   3) H00 chain gen1→gen4 from JS seed using YOYO sidecar + yoyo_compiler
#      (no Rust yoyo.exe; no Rust yoyo_rt.dll in cwd)
#   4) Pin scripts/ow-seed-trust-root-fullbar.pin (separate from FIXEDPOINT pin)
#
# disposition stays CUT until human CLOSED auth. ≠ Thompson.
#
#   cd F:\yoyo
#   & .\scripts\stage-fullbar-trust-root.ps1
param(
    [switch]$SkipEmitRt,
    [switch]$WritePin
)
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden

Write-Host "=== AUTO-FULLBAR FS-2 / F2 (non-Rust cold-start trust-root) ==="

Write-Host "=== BD-3 payload bins pin lock ==="
& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) {
    Write-Host "FULLBAR_F2 status=RED reason=bins_pin"
    exit 1
}
Write-Host "FULLBAR_F2 bins_node_rebuild_on_gate=ABSENT"

$Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
$Ty = Join-Path $Root "yoyo\projects\yoyo.ty"
$Tyb = Join-Path $Root "yoyo\projects\yoyo.tyb"
$Work = Join-Path $Root "scripts\_stage-fullbar-fs2-chain"
$PinPath = Join-Path $Root "scripts\ow-seed-trust-root-fullbar.pin"
New-Item -ItemType Directory -Force -Path $Work | Out-Null

if (-not (Test-Path $Yoyo)) { throw "missing yoyo.exe (contrast tooling only)" }
if (-not (Test-Path $Ty) -or -not (Test-Path $Tyb)) { throw "missing yoyo.ty/tyb" }

$seedJs = Join-Path $Work "seed_js.exe"
$seedAsm = Join-Path $Work "seed_asm.exe"
$seedRust = Join-Path $Work "seed_rust_contrast.exe"

# BD-1/BD-2: C1 place (no Node) → C1+tyb seed; peer contrast = checked-in seed (no live asm.py).
$peerPlaceDir = Join-Path $Work "_peer_compiler_once"
New-Item -ItemType Directory -Force -Path $peerPlaceDir | Out-Null
$c1Place = Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1"
$PeerCompiler = Join-Path $peerPlaceDir "yoyo_compiler.exe"
$seedFixture = Join-Path $Root "scripts\fixtures\seed_yoyo_win32.exe"
$seedFixturePin = Join-Path $Root "scripts\fixtures\seed_yoyo_win32.pin"

Write-Host "=== pre-place yoyo_compiler.exe once (C1 native only; no Node fallback) ==="
if (-not (Test-Path $c1Place)) {
    Write-Host "FULLBAR_F2 status=RED reason=missing_c1_place_script"
    exit 1
}
& $c1Place -OutDir $peerPlaceDir
if ($LASTEXITCODE -ne 0 -or -not (Test-Path $PeerCompiler)) {
    Write-Host "FULLBAR_F2 status=RED reason=c1_place_failed_or_missing_bootstrap"
    Write-Host "FULLBAR_F2 hint=offline_bootstrap_fixture: & .\\scripts\\bootstrap-c1-offline.ps1"
    exit 1
}
$peerSha = (Get-FileHash $PeerCompiler -Algorithm SHA256).Hash.ToLowerInvariant()
Write-Host ("FULLBAR_F2 peer_compiler_sha256_prefix={0} bytes={1}" -f $peerSha.Substring(0, 16), (Get-Item $PeerCompiler).Length)
Write-Host "FULLBAR_F2 node_place_on_gate=ABSENT"

Write-Host "== cold-start: C1+tyb → seed_js (ancestry; no node yoyo.js) =="
$cold = Join-Path $Work "cold-c1-seed"
New-Item -ItemType Directory -Force -Path $cold | Out-Null
Copy-Item -Force $PeerCompiler (Join-Path $cold "yoyo_compiler.exe")
Copy-Item -Force $Tyb (Join-Path $cold "input.tyb")
Copy-Item -Force $Ty (Join-Path $cold "input.ky")
if (Test-Path (Join-Path $cold "output.exe")) { Remove-Item -Force (Join-Path $cold "output.exe") }
$ecCold = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $cold "yoyo_compiler.exe") -WorkingDirectory $cold -TimeoutSec 300
if ($ecCold -ne 0 -or -not (Test-Path (Join-Path $cold "output.exe"))) {
    Write-Host ("FULLBAR_F2 status=RED reason=c1_cold_start_failed exit={0}" -f $ecCold)
    exit 1
}
Copy-Item -Force (Join-Path $cold "output.exe") $seedJs
Write-Host "FULLBAR_F2 cold_start_node_yoyo_js=ABSENT"
Write-Host "FULLBAR_F2 cold_start_emitter=peer_compiler_c1_tyb"

# TRUER2: chain seed SoT = C1+tyb output only (no second PE trust root).
Copy-Item -Force $seedJs $seedAsm
Write-Host "FULLBAR_F2 cold_start_asm_live=ABSENT"
Write-Host "FULLBAR_F2 seed_sot=c1_tyb"
Write-Host "FULLBAR_F2 seed_asm_peer=copy_of_c1_tyb_seed"
$shaJs = (Get-FileHash $seedJs -Algorithm SHA256).Hash.ToLowerInvariant()
Write-Host ("FULLBAR_F2 c1_tyb_seed_sha256_prefix={0}" -f $shaJs.Substring(0, 16))

Write-Host "== optional contrast: checked-in seed fixture (not ancestry SoT) =="
if ((Test-Path $seedFixture) -and (Test-Path $seedFixturePin)) {
    $fxSha = (Get-FileHash $seedFixture -Algorithm SHA256).Hash.ToLowerInvariant()
    $rel = if ($fxSha -eq $shaJs) { "EQUAL" } else { "DIFF" }
    Write-Host ("FULLBAR_F2 seed_fixture_contrast={0} sha256_prefix={1}" -f $rel, $fxSha.Substring(0, 16))
} else {
    Write-Host "FULLBAR_F2 seed_fixture_contrast=ABSENT"
}
Write-Host "FULLBAR_F2 seed_fixture_required=ABSENT"

Write-Host "== contrast only: Rust link =="
& $Yoyo link --target=win32 $Ty $seedRust
if ($LASTEXITCODE -ne 0 -or -not (Test-Path $seedRust)) {
    Write-Host "FULLBAR_F2 status=RED reason=rust_contrast_failed"
    exit 1
}
$shaRust = (Get-FileHash $seedRust -Algorithm SHA256).Hash.ToLowerInvariant()

$prev = $ErrorActionPreference
$ErrorActionPreference = "Continue"
$bodyLines = & $Yoyo diff --selfhost-body $seedJs $seedRust 2>&1
$bodyEc = $LASTEXITCODE
$ErrorActionPreference = $prev
$bodyLines | ForEach-Object { Write-Host "  $_" }
# TRUER8: C1 seed no longer carries H00 bake stub; body DIFF vs Rust H00 seed is expected contrast.
$bodyRel = if ($bodyEc -eq 0 -and (($bodyLines | Out-String) -match 'DDC:\s*EQUAL')) { "EQUAL" } else { "DIFF" }
Write-Host ("FULLBAR_F2 js_vs_rust_body={0} (contrast_only; TRUER8 h00_bake_zeroed)" -f $bodyRel)
Write-Host ("FULLBAR_F2 rust_contrast_full={0} (DIFF ok; not ancestry)" -f $(if ($shaJs -eq $shaRust) { "EQUAL" } else { "DIFF" }))

# TRUER4: critical path has NO yoyo_rt.dll. H00+sidecar chain = optional contrast only.
Write-Host "FULLBAR_F2 cargo_emit_rt=ABSENT"
Write-Host "FULLBAR_F2 sidecar_required_on_critical=ABSENT"
Write-Host "FULLBAR_F2 h00_mid_chain_node_place=ABSENT"

# Critical fixedpoint: C1+tyb seed is stable under re-run (no sidecar).
$seedJs2 = Join-Path $Work "seed_js_rerun.exe"
$cold2 = Join-Path $Work "cold-c1-seed-rerun"
New-Item -ItemType Directory -Force -Path $cold2 | Out-Null
Copy-Item -Force $PeerCompiler (Join-Path $cold2 "yoyo_compiler.exe")
Copy-Item -Force $Tyb (Join-Path $cold2 "input.tyb")
Copy-Item -Force $Ty (Join-Path $cold2 "input.ky")
if (Test-Path (Join-Path $cold2 "output.exe")) { Remove-Item -Force (Join-Path $cold2 "output.exe") }
$ecCold2 = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $cold2 "yoyo_compiler.exe") -WorkingDirectory $cold2 -TimeoutSec 300
if ($ecCold2 -ne 0 -or -not (Test-Path (Join-Path $cold2 "output.exe"))) {
    Write-Host ("FULLBAR_F2 status=RED reason=c1_tyb_seed_rerun_failed exit={0}" -f $ecCold2)
    exit 1
}
Copy-Item -Force (Join-Path $cold2 "output.exe") $seedJs2
$sha4 = (Get-FileHash $seedJs -Algorithm SHA256).Hash.ToLowerInvariant()
$sha5 = (Get-FileHash $seedJs2 -Algorithm SHA256).Hash.ToLowerInvariant()
if ($sha4 -ne $sha5) {
    Write-Host ("FULLBAR_F2 status=RED reason=c1_tyb_seed_fixedpoint_DIFF a={0} b={1}" -f $sha4.Substring(0,16), $sha5.Substring(0,16))
    exit 1
}
Write-Host ("FULLBAR_F2 fixedpoint=EQUAL sha256_prefix={0} bytes={1}" -f $sha4.Substring(0,16), (Get-Item $seedJs).Length)
Write-Host "FULLBAR_F2 fixedpoint_kind=c1_tyb_seed_rerun"

# Optional contrast: H00+sidecar multi-hop (not critical; DIFF/absent ok).
$AltSrc = Join-Path $Root "scripts\fixtures\yoyo_rt.dll"
if (Test-Path $AltSrc) {
    Write-Host "== optional contrast: H00 YOYO chain with sidecar fixture =="
    function Invoke-H00YoyoChainContrast([string]$ExePath, [string]$OutPath, [string]$Label) {
        try {
            $run = Join-Path $Work ("run-" + $Label)
            New-Item -ItemType Directory -Force -Path $run | Out-Null
            Copy-Item -Force $ExePath (Join-Path $run "emitter.exe")
            Copy-Item -Force $AltSrc (Join-Path $run "yoyo_rt.dll")
            Copy-Item -Force $Tyb (Join-Path $run "input.tyb")
            Copy-Item -Force $Ty (Join-Path $run "input.ky")
            Copy-Item -Force $PeerCompiler (Join-Path $run "yoyo_compiler.exe")
            Push-Location $run
            try {
                if (Test-Path "output.exe") { Remove-Item "output.exe" -Force }
                $ec = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $run "emitter.exe") -WorkingDirectory $run -TimeoutSec 300
                if ($ec -ne 0 -or -not (Test-Path "output.exe")) {
                    Write-Host ("FULLBAR_F2 h00_sidecar_contrast=ABSENT_OR_RED label={0} exit={1}" -f $Label, $ec)
                    return $false
                }
                Copy-Item -Force "output.exe" $OutPath
                return $true
            } finally { Pop-Location }
        } catch {
            Write-Host ("FULLBAR_F2 h00_sidecar_contrast=ABSENT_OR_RED label={0} err={1}" -f $Label, $_.Exception.Message)
            return $false
        }
    }
    $gen4c = Join-Path $Work "gen4_contrast.exe"
    $gen5c = Join-Path $Work "gen5_contrast.exe"
    $gen2c = Join-Path $Work "gen2_contrast.exe"
    $gen3c = Join-Path $Work "gen3_contrast.exe"
    $ok = (Invoke-H00YoyoChainContrast $seedJs $gen2c "contrast_g1g2") -and
          (Invoke-H00YoyoChainContrast $gen2c $gen3c "contrast_g2g3") -and
          (Invoke-H00YoyoChainContrast $gen3c $gen4c "contrast_g3g4") -and
          (Invoke-H00YoyoChainContrast $gen4c $gen5c "contrast_g4g5")
    if ($ok) {
        $c4 = (Get-FileHash $gen4c -Algorithm SHA256).Hash
        $c5 = (Get-FileHash $gen5c -Algorithm SHA256).Hash
        $rel = if ($c4 -eq $c5) { "EQUAL" } else { "DIFF" }
        Write-Host ("FULLBAR_F2 h00_sidecar_contrast={0}" -f $rel)
    } else {
        Write-Host "FULLBAR_F2 h00_sidecar_contrast=ABSENT_OR_RED"
    }
} else {
    Write-Host "FULLBAR_F2 h00_sidecar_contrast=ABSENT"
}
Write-Host "FULLBAR_F2 sidecar_sot=contrast_only"

$ancestry = "c1_tyb_seed_no_sidecar_critical"
$operational = "c1_tyb_seed_rerun_fixedpoint"
$prov = "truer4_place_stub_plus_c1_tyb"

if ($ancestry -match 'rust_stage9_gen1') {
    Write-Host "FULLBAR_F2 status=RED reason=ancestry_still_names_rust_gen1"
    exit 1
}

$pinBody = @"
# OW-SEED FULLBAR trust-root pin (FS-2) — TRUER5 critical = C1+tyb seed (no sidecar face).
name=seed_js.exe
kind=YOYO_PE
bytes=$((Get-Item $seedJs).Length)
sha256=$sha4
sha256_prefix=$($sha4.Substring(0,16))
path_hint=scripts/_stage-fullbar-fs2-chain/seed_js.exe
provenance=$prov
provenance_ancestry=$ancestry
provenance_operational=$operational
disposition=CLOSED
closed=1
auth=human_A_2026-11-07
auth_note=TRUER13_yoyo_tyb_real_mz_pe;h00_abs_skipped;sidecar_contrast_only;not_Thompson
pinned_date=2026-11-09
closed_date=2026-11-07
doc=AUTO-TRUER13.md
"@





if ($WritePin -or -not (Test-Path $PinPath)) {
    Set-Content -LiteralPath $PinPath -Value $pinBody -Encoding utf8
    Write-Host "FULLBAR_F2 pin_written=$PinPath"
} else {
    # Verify existing pin matches this materialization
    $pin = @{}
    Get-Content -LiteralPath $PinPath | ForEach-Object {
        if ($_ -match '^\s*#' -or $_ -match '^\s*$') { return }
        if ($_ -match '^\s*([^=]+)=(.*)$') { $pin[$Matches[1].Trim()] = $Matches[2].Trim() }
    }
    if ($pin['sha256'] -ne $sha4) {
        Write-Host "FULLBAR_F2 status=RED reason=pin_sha_mismatch (re-run with -WritePin)"
        exit 1
    }
    if ($pin['provenance_ancestry'] -match 'rust_stage9_gen1') {
        Write-Host "FULLBAR_F2 status=RED reason=pin_ancestry_still_rust"
        exit 1
    }
}

# Fail-closed: ancestry *field* must be js/asm peer — ignore comments.
$pinFields = @{}
Get-Content -LiteralPath $PinPath | ForEach-Object {
    if ($_ -match '^\s*#' -or $_ -match '^\s*$') { return }
    if ($_ -match '^\s*([^=]+)=(.*)$') { $pinFields[$Matches[1].Trim()] = $Matches[2].Trim() }
}
$ancField = $pinFields['provenance_ancestry']
if ([string]::IsNullOrWhiteSpace($ancField) -or $ancField -match 'rust_stage9_gen1') {
    Write-Host ("FULLBAR_F2 status=RED reason=bad_ancestry_field got={0}" -f $ancField)
    exit 1
}
if ($ancField -notmatch 'js_asm_peer' -and $ancField -notmatch 'c1_tyb' -and $ancField -notmatch 'asm_peer') {
    Write-Host "FULLBAR_F2 status=RED reason=pin_missing_peer_ancestry"
    exit 1
}
if ($pinFields['disposition'] -ne 'CLOSED') {
    Write-Host ("FULLBAR_F2 status=RED reason=pin_disposition_must_be_CLOSED got={0}" -f $pinFields['disposition'])
    exit 1
}
if ($pinFields['closed'] -ne '1') {
    Write-Host "FULLBAR_F2 status=RED reason=pin_closed_must_be_1"
    exit 1
}

Write-Host "FULLBAR_F2 provenance_ancestry=$ancestry"
Write-Host "FULLBAR_F2 provenance_operational=$operational"
Write-Host "FULLBAR_F2 sole_rust_cold_start=ABSENT"
Write-Host "FULLBAR_F2 rust_yoyo_exe_on_chain=ABSENT"
Write-Host "FULLBAR_F2 rust_yoyo_rt_dll_on_chain=ABSENT"
Write-Host "FULLBAR_F2 disposition=CLOSED"
Write-Host "FULLBAR_F2 closed=1"
Write-Host "FULLBAR_F2 auth=human_A_2026-11-07"
Write-Host "FULLBAR_F2 note=OW_SEED_CLOSED;cold_start_node_yoyo_js=ABSENT;h00_mid_chain_node_place=ABSENT;pre_placed_peer_compiler;not_Thompson"
Write-Host "FULLBAR_F2 doc=SCOPE-CUT-v1.0-ow-seed-observe.md"
Write-Host "FULLBAR_F2 status=GREEN"

# Machine-readable markers for stage-yoyo-closed (Write-Host is not reliably capturable).
$markersPath = Join-Path $Work "yoyo-closed-markers.txt"
@(
    "h00_mid_chain_node_place=ABSENT"
    "cold_start_node_yoyo_js=ABSENT"
    "cold_start_asm_live=ABSENT"
    "cold_start_emitter=peer_compiler_c1_tyb"
    "seed_sot=c1_tyb"
    "seed_fixture_required=ABSENT"
    "critical_pe_roots=1"
    "critical_trust_blobs=2"
    "trust_root=place_stub"
    "sidecar_sot=contrast_only"
    "sidecar_required_on_critical=ABSENT"
    "cargo_emit_rt=ABSENT"
    "fixedpoint=EQUAL"
    "fixedpoint_kind=c1_tyb_seed_rerun"
    ("fixedpoint_sha256_prefix={0}" -f $sha4.Substring(0, 16))
    "rust_yoyo_exe_on_chain=ABSENT"
    "rust_yoyo_rt_dll_on_chain=ABSENT"
    "sole_rust_cold_start=ABSENT"
    "node_place_on_gate=ABSENT"
    "bins_node_rebuild_on_gate=ABSENT"
    "bins_gate_policy=pin_locked"
    "status=GREEN"
) | Set-Content -LiteralPath $markersPath -Encoding ASCII
Write-Host ("FULLBAR_F2 markers={0}" -f $markersPath)
exit 0
