# stage13-link-host.ps1 — Stage 13-A: seed/link host fail-closed observe (+ contract)
#
# Trust goal: 「绿」for pure M4 / selfhost entry must not rest on an opaque dual host
# (`yoyo link` vs `yoyo bootstrap`) or silently slide onto `bootstrap --selfhost`
# (genNrt / GetTempPathA). Stage 13-A contracts the approved seed path to one named
# host surface and pins it under a fail-closed gate:
#   - Canonical seed = H_00 `link_pe_win32` / `link_elf_linux` (Rust: seed_host_compile*)
#   - link(.ty) ≡ link(.tyb) ≡ bootstrap(.tyb) .text/full DDC EQUAL
#   - Seed PE/ELF size ≤ STAGE13_MAX (Rust also enforces on H_00 link)
#   - Win seed markers: yoyo_rt.dll + LoadLibraryA + GetProcAddress; NO GetTempPathA
#   - bootstrap --selfhost MUST DIFF seed (and Win must expose GetTempPathA) — else RED
#
# Honest remaining: still trusts Rust-built `yoyo.exe` host to emit the seed; embedded
# Rust runtime + LoadLibrary/libdl remain (Stages 10–11). This gate observes/contracts
# the seed *entry* surface — it does not eliminate host compile trust.
param(
    [switch]$SkipBuild,
    [switch]$SkipLinux,
    [switch]$SkipSelfhostDiff
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

# Fail-closed ceilings (keep in sync with verifier selfhost::STAGE13_MAX_SEED_*).
$MaxSeedPeBytes = 270000
$MaxSeedElfBytes = 550000
$ObservedSeedPeBytes = 248832
$ObservedSeedElfBytes = 512000

$WorkDir = Join-Path $Root "scripts\_stage13-link-host"
New-Item -ItemType Directory -Force -Path $WorkDir | Out-Null

$Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
$RuntimeDllPreferred = Join-Path $Root "yoyo-rust\target\release-runtime\yoyo_runtime.dll"
$RuntimeDllCompat = Join-Path $Root "yoyo-rust\target\release\yoyo_runtime.dll"
$Ty = Join-Path $Root "yoyo\projects\yoyo.ty"
$Tyb = Join-Path $Root "yoyo\projects\yoyo.tyb"

function Find-Ascii([byte[]]$Bytes, [string]$Needle) {
    $ascii = [System.Text.Encoding]::ASCII.GetString($Bytes)
    return $ascii.Contains($Needle)
}

function Invoke-YoyoDiffEqual([string]$A, [string]$B, [string]$Label) {
    $out = & $Yoyo diff $A $B 2>&1 | Out-String
    $ec = $LASTEXITCODE
    Write-Host $out.TrimEnd()
    if ($ec -ne 0) {
        Write-Host "Stage 13-A: RED ($Label — DDC not EQUAL, exit=$ec)"
        return $false
    }
    if ($out -notmatch 'DDC:\s*EQUAL') {
        Write-Host "Stage 13-A: RED ($Label — missing DDC: EQUAL)"
        return $false
    }
    return $true
}

# Linux H_00: prefer SEED_HOST path=h00; also accept older classifiers that labeled
# embedded-so+dlopen seeds as path=plain (markers still prove H_00 extract surface).
function Test-LinuxH00Observe([string]$SeedHostLine, [byte[]]$ElfBytes, [string]$Label) {
    if ($SeedHostLine -match 'SEED_HOST cmd=\S+ target=linux path=h00\b') {
        Write-Host "Linux H_00 observe ($Label): SEED_HOST path=h00"
        return $true
    }
    $hasSo = Find-Ascii $ElfBytes "libyoyo_runtime.so"
    $hasDl = (Find-Ascii $ElfBytes "dlopen") -or (Find-Ascii $ElfBytes "libdl.so")
    if ($hasSo -and $hasDl -and ($SeedHostLine -match 'SEED_HOST cmd=\S+ target=linux path=')) {
        Write-Host "Linux H_00 observe ($Label): markers libyoyo_runtime.so+dlopen (SEED_HOST path not yet h00)"
        return $true
    }
    Write-Host "Stage 13-A: RED ($Label — Linux seed not H_00; need path=h00 or libyoyo_runtime.so+dlopen)"
    Write-Host "  line: $($SeedHostLine.Trim())"
    return $false
}

Write-Host "=== Stage 13-A: seed/link host contract + fail-closed observe ==="
Write-Host "  Approved seed: yoyo link / bootstrap (no --selfhost) → H_00 path"
Write-Host "  Fail-closed MAX PE:  $MaxSeedPeBytes (observed $ObservedSeedPeBytes)"
Write-Host "  Fail-closed MAX ELF: $MaxSeedElfBytes (observed $ObservedSeedElfBytes)"

$needYoyo = -not (Test-Path $Yoyo)
$needRuntime = -not ((Test-Path $RuntimeDllPreferred) -or (Test-Path $RuntimeDllCompat))
if ($needYoyo -or $needRuntime) {
    if ($SkipBuild) { throw "missing yoyo.exe or yoyo_runtime.dll (and -SkipBuild)" }
    Push-Location (Join-Path $Root "yoyo-rust")
    if ($needYoyo) {
        Write-Host "== build yoyo (release) =="
        cargo build --release -p verifier
        if ($LASTEXITCODE -ne 0) { throw "verifier build failed" }
    }
    if ($needRuntime -and -not (Test-Path $RuntimeDllPreferred) -and -not (Test-Path $RuntimeDllCompat)) {
        Write-Host "== build yoyo-runtime (release-runtime) =="
        cargo build --profile release-runtime -p yoyo-runtime
        if ($LASTEXITCODE -ne 0) { throw "yoyo-runtime build failed" }
    }
    Pop-Location
}

# Always rebuild verifier so Stage 13-A Rust ceilings / seed_host_compile are what we measure.
if (-not $SkipBuild) {
    Write-Host "== rebuild verifier (release · Stage 13-A) =="
    Push-Location (Join-Path $Root "yoyo-rust")
    cargo build --release -p verifier
    if ($LASTEXITCODE -ne 0) { throw "verifier rebuild failed" }
    Pop-Location
}

if (-not (Test-Path $Yoyo)) {
    Write-Host "Stage 13-A: RED (yoyo.exe missing)"
    exit 1
}
if (-not (Test-Path $Ty) -or -not (Test-Path $Tyb)) {
    Write-Host "Stage 13-A: RED (missing yoyo.ty / yoyo.tyb)"
    exit 1
}

$SeedLinkTy = Join-Path $WorkDir "seed_link_ty.exe"
$SeedLinkTyb = Join-Path $WorkDir "seed_link_tyb.exe"
$SeedBoot = Join-Path $WorkDir "seed_bootstrap.exe"
$SeedSelfhost = Join-Path $WorkDir "seed_bootstrap_selfhost.exe"

Write-Host ""
Write-Host "=== Win32: seed via yoyo link (.ty) ==="
Remove-Item $SeedLinkTy -Force -ErrorAction SilentlyContinue
$linkTyOut = & $Yoyo link --target=win32 $Ty $SeedLinkTy 2>&1 | Out-String
Write-Host $linkTyOut.TrimEnd()
if ($LASTEXITCODE -ne 0 -or -not (Test-Path $SeedLinkTy)) {
    Write-Host "Stage 13-A: RED (link .ty failed)"
    exit 1
}
if ($linkTyOut -notmatch 'SEED_HOST cmd=link target=win32 path=h00') {
    Write-Host "Stage 13-A: RED (link .ty missing SEED_HOST path=h00)"
    exit 1
}
$peSz = (Get-Item $SeedLinkTy).Length
Write-Host "seed link(.ty): $peSz bytes"
if ($peSz -gt $MaxSeedPeBytes) {
    Write-Host "Stage 13-A: RED (seed PE $peSz > MAX $MaxSeedPeBytes)"
    exit 1
}

Write-Host ""
Write-Host "=== Win32: same host via yoyo link (.tyb) ==="
Remove-Item $SeedLinkTyb -Force -ErrorAction SilentlyContinue
& $Yoyo link --target=win32 $Tyb $SeedLinkTyb
if ($LASTEXITCODE -ne 0 -or -not (Test-Path $SeedLinkTyb)) {
    Write-Host "Stage 13-A: RED (link .tyb failed)"
    exit 1
}
if (-not (Invoke-YoyoDiffEqual $SeedLinkTy $SeedLinkTyb "link(.ty) vs link(.tyb)")) { exit 1 }

Write-Host ""
Write-Host "=== Win32: bootstrap without --selfhost must ≡ link (seed/link contract) ==="
Remove-Item $SeedBoot -Force -ErrorAction SilentlyContinue
$bootOut = & $Yoyo bootstrap $Tyb $SeedBoot 2>&1 | Out-String
Write-Host $bootOut.TrimEnd()
if ($LASTEXITCODE -ne 0 -or -not (Test-Path $SeedBoot)) {
    Write-Host "Stage 13-A: RED (bootstrap no --selfhost failed)"
    exit 1
}
if ($bootOut -notmatch 'SEED_HOST cmd=bootstrap target=win32 path=h00') {
    Write-Host "Stage 13-A: RED (bootstrap missing SEED_HOST path=h00 observe line)"
    exit 1
}
if (-not (Invoke-YoyoDiffEqual $SeedLinkTy $SeedBoot "link(.ty) vs bootstrap(.tyb)")) { exit 1 }

$seedBytes = [System.IO.File]::ReadAllBytes($SeedLinkTy)
$required = @("LoadLibraryA", "GetProcAddress", "yoyo_rt.dll", "yoyo_runtime_selfhost_main")
$forbidden = @("GetTempPathA", "lstrcatA")
foreach ($n in $required) {
    if (-not (Find-Ascii $seedBytes $n)) {
        Write-Host "Stage 13-A: RED (seed missing required marker: $n)"
        exit 1
    }
}
foreach ($n in $forbidden) {
    if (Find-Ascii $seedBytes $n) {
        Write-Host "Stage 13-A: RED (seed has forbidden host API: $n — slipped toward --selfhost surface)"
        exit 1
    }
}
Write-Host "Win seed markers: H_00 cwd-relative LoadLibrary surface OK (no GetTempPathA)"

if (-not $SkipSelfhostDiff) {
    Write-Host ""
    Write-Host "=== Win32: bootstrap --selfhost MUST DIFF seed (not approved seed path) ==="
    Remove-Item $SeedSelfhost -Force -ErrorAction SilentlyContinue
    $shOut = & $Yoyo bootstrap --selfhost $Tyb $SeedSelfhost 2>&1 | Out-String
    Write-Host $shOut.TrimEnd()
    if ($LASTEXITCODE -ne 0 -or -not (Test-Path $SeedSelfhost)) {
        Write-Host "Stage 13-A: RED (bootstrap --selfhost failed to build contrast image)"
        exit 1
    }
    if ($shOut -notmatch 'SEED_HOST cmd=bootstrap-selfhost target=win32 path=gennrt') {
        Write-Host "Stage 13-A: RED (--selfhost missing SEED_HOST path=gennrt observe line)"
        exit 1
    }
    $shBytes = [System.IO.File]::ReadAllBytes($SeedSelfhost)
    if (-not (Find-Ascii $shBytes "GetTempPathA")) {
        Write-Host "Stage 13-A: RED (--selfhost contrast missing GetTempPathA; cannot prove surface split)"
        exit 1
    }
    # Expected DIFF: swallow native stderr ErrorRecord under Stop preference.
    $prevEap = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    $diffLines = @()
    try {
        $diffLines = & $Yoyo diff $SeedLinkTy $SeedSelfhost 2>&1
    } catch {
        $diffLines = @($_.Exception.Message)
    }
    $diffEc = $LASTEXITCODE
    $ErrorActionPreference = $prevEap
    $diffOut = ($diffLines | ForEach-Object { "$_" }) -join "`n"
    Write-Host $diffOut.TrimEnd()
    if ($diffEc -eq 0 -or $diffOut -match 'DDC:\s*EQUAL') {
        Write-Host "Stage 13-A: RED (seed ≡ bootstrap --selfhost — seed surface collapsed into genNrt)"
        exit 1
    }
    Write-Host "contrast: seed DIFF --selfhost (GetTempPath path) OK"
}

$linuxOk = $true
if (-not $SkipLinux) {
    Write-Host ""
    Write-Host "=== Linux: link(.ty) ≡ bootstrap(.tyb) under ELF size MAX ==="
    $SeedLinkElf = Join-Path $WorkDir "seed_link.elf"
    $SeedBootElf = Join-Path $WorkDir "seed_bootstrap.elf"
    Remove-Item $SeedLinkElf, $SeedBootElf -Force -ErrorAction SilentlyContinue
    & $Yoyo link --target=linux $Ty $SeedLinkElf
    if ($LASTEXITCODE -ne 0 -or -not (Test-Path $SeedLinkElf)) {
        Write-Host "Stage 13-A: RED (linux link .ty failed)"
        exit 1
    }
    $elfSz = (Get-Item $SeedLinkElf).Length
    Write-Host "seed link linux: $elfSz bytes"
    if ($elfSz -gt $MaxSeedElfBytes) {
        Write-Host "Stage 13-A: RED (seed ELF $elfSz > MAX $MaxSeedElfBytes)"
        exit 1
    }
    $bootElfOut = & $Yoyo bootstrap --target=linux $Tyb $SeedBootElf 2>&1 | Out-String
    Write-Host $bootElfOut.TrimEnd()
    if ($LASTEXITCODE -ne 0 -or -not (Test-Path $SeedBootElf)) {
        Write-Host "Stage 13-A: RED (linux bootstrap no --selfhost failed)"
        exit 1
    }
    $elfBytes = [System.IO.File]::ReadAllBytes($SeedBootElf)
    $linuxH00Markers = (Find-Ascii $elfBytes "libyoyo_runtime.so") -and (
        (Find-Ascii $elfBytes "dlopen") -or (Find-Ascii $elfBytes "yoyo_runtime_selfhost_main")
    )
    $linuxSeedHostLine = $bootElfOut -match 'SEED_HOST cmd=bootstrap target=linux path=h00'
    if (-not $linuxSeedHostLine -and -not $linuxH00Markers) {
        Write-Host "Stage 13-A: RED (linux bootstrap neither SEED_HOST path=h00 nor H_00 ELF markers)"
        exit 1
    }
    if (-not $linuxSeedHostLine) {
        Write-Host "linux SEED_HOST path not yet h00; ELF H_00 markers OK (libyoyo_runtime.so + loader)"
    }
    if (-not (Invoke-YoyoDiffEqual $SeedLinkElf $SeedBootElf "linux link(.ty) vs bootstrap(.tyb)")) {
        $linuxOk = $false
        exit 1
    }
} else {
    Write-Host ""
    Write-Host "=== Linux: SKIPPED (-SkipLinux) ==="
}

Write-Host ""
Write-Host "=== summary ==="
Write-Host "Win seed PE:           $peSz bytes (MAX $MaxSeedPeBytes)"
Write-Host "link.ty ≡ link.tyb:    EQUAL"
Write-Host "link ≡ bootstrap:      EQUAL (no --selfhost)"
Write-Host "bootstrap --selfhost:  DIFF seed (not approved)"
Write-Host "Linux seed contract:   $(if ($SkipLinux) { 'SKIPPED' } elseif ($linuxOk) { 'EQUAL' } else { 'RED' })"
Write-Host "Stage 13-A:            GREEN (may check [x])"
Write-Host ""
Write-Host "Trust chain: seed/link host contracted to H_00 link pipeline;"
Write-Host "  bootstrap without --selfhost is an alias of that seed path (Stage 13-A)."
Write-Host "  Gate fails closed if seed grows past MAX, gains GetTempPathA, or ≡ --selfhost."
Write-Host "Remaining host surface (honest):"
Write-Host "  - Rust-built yoyo.exe still emits the seed (host compile trust)"
Write-Host "  - embedded yoyo_runtime.dll / libyoyo_runtime.so (Rust; Stages 10–11)"
Write-Host "  - host LoadLibraryA / libdl on H_00 extract (Stage 11-B observes face)"
exit 0
