# stage-break3.ps1 — AUTO-BREAK3: demote fixture root; A1 load-bearing; #8 dual witness
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null
. (Join-Path $Root "scripts\lib\Assert-YoyoFilePin.ps1")

Write-Host "=== AUTO-BREAK3 ==="
$Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
if (-not (Test-Path -LiteralPath $Yoyo)) { Write-Host "BREAK3 status=RED reason=missing_yoyo"; exit 1 }

$work = Join-Path $Root "scripts\_stage-break3"
if (Test-Path $work) { Remove-Item -Recurse -Force $work }
New-Item -Force -ItemType Directory $work | Out-Null

# --- B0: Gate-I place path is C1 (no Node) — scan ---
$gateI = Join-Path $Root "scripts\stage17-ow-seed-no-rust-sidecar.ps1"
$gi = Get-Content -LiteralPath $gateI -Raw
if ($gi -match '(?m)^\s*&?\s*node\s' -or $gi -match 'place-yoyo-compiler\.js') {
    Write-Host "BREAK3 status=RED reason=gate_i_still_has_node_place"
    exit 1
}
Write-Host "BREAK3 B0_gate_i_node_place=ABSENT"

# --- B1: fixture demoted — product pin is SoT; bootstrap is any bit-identical PE ---
$pinPath = Join-Path $Root "scripts\fixtures\yoyo_place_c1.pin"
$fx = Join-Path $Root "scripts\fixtures\yoyo_compiler.exe"
Assert-YoyoShaPin -Yoyo $Yoyo -File $fx -PinPath $pinPath `
    -Label BREAK3_PRODUCT -RequireTrustRoot c1_product_pin -Doc AUTO-BREAK3.md

$placeOut = Join-Path $work "place"
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $fx
if ($LASTEXITCODE -ne 0) { Write-Host "BREAK3 status=RED reason=place"; exit 1 }
$c1 = Join-Path $placeOut "yoyo_compiler.exe"
Assert-YoyoShaPin -Yoyo $Yoyo -File $c1 -PinPath $pinPath `
    -Label BREAK3_REBUILD -RequireTrustRoot c1_product_pin -Doc AUTO-BREAK3.md

# Second hop: bootstrap from *rebuild* (not the fixture path) — same bits → same product
$place2 = Join-Path $work "place2"
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $place2 -Bootstrap $c1
if ($LASTEXITCODE -ne 0) { Write-Host "BREAK3 status=RED reason=place2_from_rebuild"; exit 1 }
$c2 = Join-Path $place2 "yoyo_compiler.exe"
$h1 = (Get-FileHash $c1 -Algorithm SHA256).Hash.ToLowerInvariant()
$h2 = (Get-FileHash $c2 -Algorithm SHA256).Hash.ToLowerInvariant()
if ($h1 -ne $h2) {
    Write-Host "BREAK3 status=RED reason=rebuild_bootstrap_DIFF"
    exit 1
}
Write-Host "BREAK3 B1_fixture_demote=GREEN hop2_from_rebuild=EQUAL"
Write-Host "BREAK3 B1_note=fixture_file_only_offline_bit_identical_bootstrap;product_pin_SoT"

# --- B1 cold seed witness ---
$cold = Join-Path $work "cold"
New-Item -Force -ItemType Directory $cold | Out-Null
Copy-Item -Force $c1 (Join-Path $cold "yoyo_compiler.exe")
Copy-Item -Force (Join-Path $Root "yoyo\projects\yoyo.tyb") (Join-Path $cold "input.tyb")
if (Test-Path (Join-Path $cold "output.exe")) { Remove-Item -Force (Join-Path $cold "output.exe") }
$ec = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $cold "yoyo_compiler.exe") -WorkingDirectory $cold -TimeoutSec 300
$seed = Join-Path $cold "output.exe"
if ($ec -ne 0 -or -not (Test-Path $seed)) {
    Write-Host ("BREAK3 status=RED reason=cold_failed exit={0}" -f $ec)
    exit 1
}
$seedSha = (& $Yoyo hash $seed).Trim().ToLowerInvariant()
$owPin = @{}
Get-Content (Join-Path $Root "scripts\ow-seed-trust-root-fullbar.pin") | ForEach-Object {
    if ($_ -match '^\s*#' -or $_ -match '^\s*$') { return }
    if ($_ -match '^\s*([^=]+)=(.*)$') { $owPin[$Matches[1].Trim()] = $Matches[2].Trim() }
}
if ($owPin['sha256'].ToLowerInvariant() -ne $seedSha) {
    Write-Host "BREAK3 status=RED reason=seed_pin_DIFF"
    Write-Host ("BREAK3 got={0} want={1}" -f $seedSha.Substring(0,16), $owPin['sha256'].Substring(0,16))
    exit 1
}
Write-Host ("BREAK3 B1_cold_seed=GREEN sha256_prefix={0}" -f $seedSha.Substring(0, 16))

# --- B2: A1 load-bearing (6 names present; growth sentinel; drop-any = not attempted empty-disk) ---
$a1Pin = Join-Path $Root "scripts\fixtures\yoyo_a1_imports.pin"
$a1 = @{}
Get-Content $a1Pin | ForEach-Object {
    if ($_ -match '^\s*#' -or $_ -match '^\s*$') { return }
    if ($_ -match '^\s*([^=]+)=(.*)$') { $a1[$Matches[1].Trim()] = $Matches[2].Trim() }
}
$need = @('VirtualAlloc','CreateFileA','ReadFile','WriteFile','CloseHandle','ExitProcess')
$bytes = [IO.File]::ReadAllBytes($c1)
foreach ($n in $need) {
    $enc = [Text.Encoding]::ASCII.GetBytes($n + [char]0)
    $hit = $false
    for ($i = 0; $i -le $bytes.Length - $enc.Length; $i++) {
        $ok = $true
        for ($j = 0; $j -lt $enc.Length; $j++) {
            if ($bytes[$i + $j] -ne $enc[$j]) { $ok = $false; break }
        }
        if ($ok) { $hit = $true; break }
    }
    if (-not $hit) {
        Write-Host ("BREAK3 status=RED reason=a1_missing name={0}" -f $n)
        exit 1
    }
}
if ($a1['func_count'] -ne '6') {
    Write-Host "BREAK3 status=RED reason=a1_count_drift"
    exit 1
}
# Poison: strip one import name face from a copy → must DIFF vs stock (detection)
$poison = Join-Path $work "c1_a1_poison.exe"
Copy-Item -Force $c1 $poison
$pb = [IO.File]::ReadAllBytes($poison)
$target = [Text.Encoding]::ASCII.GetBytes("ExitProcess")
for ($i = 0; $i -le $pb.Length - $target.Length; $i++) {
    $ok = $true
    for ($j = 0; $j -lt $target.Length; $j++) { if ($pb[$i + $j] -ne $target[$j]) { $ok = $false; break } }
    if ($ok) {
        for ($j = 0; $j -lt $target.Length; $j++) { $pb[$i + $j] = [byte][char]'X' }
        break
    }
}
[IO.File]::WriteAllBytes($poison, $pb)
$hp = (Get-FileHash $poison -Algorithm SHA256).Hash
if ($hp -eq (Get-FileHash $c1 -Algorithm SHA256).Hash) {
    Write-Host "BREAK3 status=RED reason=a1_poison_noop"
    exit 1
}
Write-Host "BREAK3 B2_a1_six_present=GREEN"
Write-Host "BREAK3 B2_a1_name_poison=DIFF_OK"
Write-Host "BREAK3 B2_a1_eliminate=HARD_BLOCK note=Win32_face_required;empty_disk_forbidden;not_Thompson"

# --- B3: #8 dual witness (rebuild C1 ≡ pin AND cold seed ≡ OW pin); same-repo collusion still HARD_BLOCK ---
Write-Host "BREAK3 B3_dual_witness=GREEN product_pin+seed_pin"
# Same-commit collusion demo: if both pin texts were attacker-writable, detection cannot prove outsider truth.
Write-Host "BREAK3 B3_colluding_pin=HARD_BLOCK note=same_repo_attacker_can_rewrite_both;needs_human_or_multi_machine;not_Thompson"
Write-Host "BREAK3 B3_same_repo_pin_arms=STOPPED doc=AUTO-TRUST8ACCEPT.md"

# --- closed probe (SkipReal = fullbar path; Gate I sidecar may still hard-block) ---
& (Join-Path $Root "scripts\stage-yoyo-closed.ps1") -SkipReal
if ($LASTEXITCODE -ne 0) {
    Write-Host "BREAK3 status=RED reason=closed_skipreal"
    exit 1
}
Write-Host "BREAK3 closed_skipreal=GREEN"

Write-Host "BREAK3 node_on_gate_i_place=ABSENT"
Write-Host "BREAK3 fixture_role=offline_bit_identical_bootstrap_only"
Write-Host "BREAK3 a1_eliminate=HARD_BLOCK"
Write-Host "BREAK3 trust8_collude=HARD_BLOCK"
Write-Host "BREAK3 note=demoted_fixture;dual_witness;honest_walls;not_Thompson"
Write-Host "BREAK3 doc=AUTO-BREAK3.md"
Write-Host "BREAK3 status=GREEN"
exit 0
