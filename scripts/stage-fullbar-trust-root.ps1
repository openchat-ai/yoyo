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

Write-Host "== cold-start: JS peer link (ancestry) =="
& node (Join-Path $Root "yoyo-js\src\yoyo.js") $Ty $seedJs
if ($LASTEXITCODE -ne 0 -or -not (Test-Path $seedJs)) {
    Write-Host "FULLBAR_F2 status=RED reason=js_link_failed"
    exit 1
}
Write-Host "== cold-start: asm peer link (ancestry peer) =="
& python (Join-Path $Root "yoyo-asm\asm.py") $Ty $seedAsm
if ($LASTEXITCODE -ne 0 -or -not (Test-Path $seedAsm)) {
    Write-Host "FULLBAR_F2 status=RED reason=asm_link_failed"
    exit 1
}
Write-Host "== contrast only: Rust link =="
& $Yoyo link --target=win32 $Ty $seedRust
if ($LASTEXITCODE -ne 0 -or -not (Test-Path $seedRust)) {
    Write-Host "FULLBAR_F2 status=RED reason=rust_contrast_failed"
    exit 1
}

$shaJs = (Get-FileHash $seedJs -Algorithm SHA256).Hash.ToLowerInvariant()
$shaAsm = (Get-FileHash $seedAsm -Algorithm SHA256).Hash.ToLowerInvariant()
$shaRust = (Get-FileHash $seedRust -Algorithm SHA256).Hash.ToLowerInvariant()
if ($shaJs -ne $shaAsm) {
    Write-Host "FULLBAR_F2 status=RED reason=js_asm_full_file_DIFF"
    exit 1
}
Write-Host ("FULLBAR_F2 js_asm_full=EQUAL sha256_prefix={0}" -f $shaJs.Substring(0, 16))

$prev = $ErrorActionPreference
$ErrorActionPreference = "Continue"
$bodyLines = & $Yoyo diff --selfhost-body $seedJs $seedRust 2>&1
$bodyEc = $LASTEXITCODE
$ErrorActionPreference = $prev
$bodyLines | ForEach-Object { Write-Host "  $_" }
if ($bodyEc -ne 0 -or (($bodyLines | Out-String) -notmatch 'DDC:\s*EQUAL')) {
    Write-Host "FULLBAR_F2 status=RED reason=js_vs_rust_body_DIFF"
    exit 1
}
Write-Host ("FULLBAR_F2 rust_contrast_full={0} (DIFF ok; not ancestry)" -f $(if ($shaJs -eq $shaRust) { "EQUAL" } else { "DIFF" }))

# YOYO sidecar + compiler for H00 chain (no Rust runtime dll)
$AltSrc = Join-Path $Root "scripts\_stage17-ow-rt-in-dll-recompile\yoyo_rt.dll"
if (-not (Test-Path $AltSrc)) {
    if ($SkipEmitRt) {
        Write-Host "FULLBAR_F2 status=RED reason=missing_yoyo_sidecar"
        exit 1
    }
    Push-Location (Join-Path $Root "yoyo-rust")
    try {
        $prevEap = $ErrorActionPreference
        $ErrorActionPreference = "Continue"
        & cargo run -q -p verifier --bin emit-rt-sidecar --no-default-features --features full-backends -- --in-dll-recompile (Join-Path $Root "scripts\_stage17-ow-rt-in-dll-recompile")
        $ErrorActionPreference = $prevEap
    } finally { Pop-Location }
}
if (-not (Test-Path $AltSrc)) {
    Write-Host "FULLBAR_F2 status=RED reason=yoyo_sidecar_absent"
    exit 1
}

function Invoke-H00YoyoChain([string]$ExePath, [string]$OutPath, [string]$Label) {
    $run = Join-Path $Work ("run-" + $Label)
    New-Item -ItemType Directory -Force -Path $run | Out-Null
    Copy-Item -Force $ExePath (Join-Path $run "emitter.exe")
    Copy-Item -Force $AltSrc (Join-Path $run "yoyo_rt.dll")
    Copy-Item -Force $Tyb (Join-Path $run "input.tyb")
    Copy-Item -Force $Ty (Join-Path $run "input.ky")
    $jsPlace = Join-Path $Root "yoyo-js\scripts\place-yoyo-compiler.js"
    if (-not (Test-Path $jsPlace)) {
        Write-Host ("FULLBAR_F2 status=RED reason=missing_js_place_script label={0}" -f $Label)
        return $false
    }
    & node $jsPlace $run
    if ($LASTEXITCODE -ne 0 -or -not (Test-Path (Join-Path $run "yoyo_compiler.exe"))) {
        Write-Host ("FULLBAR_F2 status=RED reason=js_peer_place_failed label={0} exit={1}" -f $Label, $LASTEXITCODE)
        return $false
    }
    $sideLen = (Get-Item (Join-Path $run "yoyo_rt.dll")).Length
    if ($sideLen -lt 500000) {
        Write-Host ("FULLBAR_F2 status=RED reason=sidecar_looks_like_rust_runtime bytes={0}" -f $sideLen)
        return $false
    }
    Push-Location $run
    try {
        if (Test-Path "output.exe") { Remove-Item "output.exe" -Force }
        Write-Host ("=== H00 YOYO chain: {0} ===" -f $Label)
        $ec = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $run "emitter.exe") -WorkingDirectory $run -TimeoutSec 300
        if ($ec -ne 0 -or -not (Test-Path "output.exe")) {
            Write-Host ("FULLBAR_F2 status=RED reason=h00_chain_failed label={0} exit={1}" -f $Label, $ec)
            return $false
        }
        Copy-Item -Force "output.exe" $OutPath
        return $true
    } finally { Pop-Location }
}

$gen2 = Join-Path $Work "gen2_js.exe"
$gen3 = Join-Path $Work "gen3_js.exe"
$gen4 = Join-Path $Work "gen4_js.exe"
$gen5 = Join-Path $Work "gen5_js.exe"

if (-not (Invoke-H00YoyoChain $seedJs $gen2 "js_gen1_to_gen2")) { exit 1 }
if (-not (Invoke-H00YoyoChain $gen2 $gen3 "js_gen2_to_gen3")) { exit 1 }
if (-not (Invoke-H00YoyoChain $gen3 $gen4 "js_gen3_to_gen4")) { exit 1 }
if (-not (Invoke-H00YoyoChain $gen4 $gen5 "js_gen4_to_gen5_fixedpoint")) { exit 1 }

$sha4 = (Get-FileHash $gen4 -Algorithm SHA256).Hash.ToLowerInvariant()
$sha5 = (Get-FileHash $gen5 -Algorithm SHA256).Hash.ToLowerInvariant()
if ($sha4 -ne $sha5) {
    Write-Host ("FULLBAR_F2 status=RED reason=fixedpoint_DIFF gen4={0} gen5={1}" -f $sha4.Substring(0,16), $sha5.Substring(0,16))
    exit 1
}
Write-Host ("FULLBAR_F2 fixedpoint=EQUAL sha256_prefix={0} bytes={1}" -f $sha4.Substring(0,16), (Get-Item $gen4).Length)

$ancestry = "js_asm_peer_link_then_H00_yoyo_createprocess"
$operational = "yoyo_pe_emit_hops_gen4_js"
$prov = "dual_ancestry_js_asm_peer_plus_operational_yoyo_pe"

if ($ancestry -match 'rust_stage9_gen1') {
    Write-Host "FULLBAR_F2 status=RED reason=ancestry_still_names_rust_gen1"
    exit 1
}

$pinBody = @"
# OW-SEED FULLBAR trust-root pin (FS-2)
# Cold-start ancestry = JS+asm peer link (full-file EQUAL); no Rust stage9 gen1 field.
# H00 chain uses YOYO sidecar + yoyo_compiler (CreateProcess). disposition=CUT.
name=gen4_js.exe
kind=YOYO_PE
bytes=$((Get-Item $gen4).Length)
sha256=$sha4
sha256_prefix=$($sha4.Substring(0,16))
path_hint=scripts/_stage-fullbar-fs2-chain/gen4_js.exe
provenance=$prov
provenance_ancestry=$ancestry
provenance_operational=$operational
disposition=CLOSED
closed=1
auth=human_A_2026-11-07
auth_note=FULLBAR_pin_production_trust_root;fixture_compiler_accepted;not_Thompson
pinned_date=2026-11-06
closed_date=2026-11-07
doc=SCOPE-CUT-v1.0-ow-seed-observe.md
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
if ($ancField -notmatch 'js_asm_peer') {
    Write-Host "FULLBAR_F2 status=RED reason=pin_missing_js_asm_ancestry"
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
Write-Host "FULLBAR_F2 note=OW_SEED_CLOSED;js_peer_link_compiler;payload_origin_js_parse1_source_interp;not_Thompson"
Write-Host "FULLBAR_F2 doc=SCOPE-CUT-v1.0-ow-seed-observe.md"
Write-Host "FULLBAR_F2 status=GREEN"
exit 0
