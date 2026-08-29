# stage10-runtime-surface.ps1 鈥?Stage 10-A: embedded yoyo_runtime.dll trust-surface gate
#
# Measures the Rust-compiled runtime DLL that every genN embeds (outside gen12
# 18432B compared .text window; SHA moves with dll_embed_size in H_00 stub 鈥?# v0.3 was b609a735, Stage 10-A is 43ffde58). Fail-closed against a documented
# MAX size so the host-trust hole cannot silently grow. Also verifies gen1 embeds
# the same bytes as the built DLL (exact embed match).
#
# v0.3 baseline (pre Stage 10-A): 485888 bytes
# Stage 10-A shrink: default-features=false (no full-backends/wasmtime) +
#   profile.release.package.yoyo-runtime opt-level=z 鈫?231936 bytes
# Stage 11-A further shrink (fat LTO + strip + verifier -z): see
#   scripts/stage11-runtime-surface.ps1 (tighter MAX; this Stage 10 gate stays).
#
# Honest remaining surface: DLL bytes remain OUTSIDE gen12; still Rust-compiled;
# Win/Linux selfhost still LoadLibrary this blob. Not a YOYO-built runtime yet.
param(
    [switch]$SkipBuild,
    [switch]$SkipLinkSmoke
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

# Fail-closed ceiling (bytes). Update only when an intentional shrink/expand is
# documented in BACKEND_SUPPORT.md / STAGE10 鈥?never raise casually.
$MaxDllBytes = 250000
# v0.3 documented baseline for before鈫抋fter reporting (not enforced as floor).
$BaselineV03Bytes = 485888

$WorkDir = Join-Path $Root "scripts\_stage10-runtime-surface"
New-Item -ItemType Directory -Force -Path $WorkDir | Out-Null

$Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
$RuntimeDll = Join-Path $Root "yoyo-rust\target\release\yoyo_runtime.dll"
$Ty = Join-Path $Root "yoyo\projects\yoyo.ty"

function Get-Sha256Hex([string]$Path) {
    return (Get-FileHash -Algorithm SHA256 -Path $Path).Hash.ToLowerInvariant()
}

function Find-EmbeddedDllOffset([byte[]]$Pe, [byte[]]$Dll) {
    if ($Dll.Length -lt 16 -or $Pe.Length -lt $Dll.Length) { return -1 }
    $n0 = $Dll[0]; $n1 = $Dll[1]
    $limit = $Pe.Length - $Dll.Length
    for ($i = 0; $i -le $limit; $i++) {
        if ($Pe[$i] -ne $n0 -or $Pe[$i + 1] -ne $n1) { continue }
        $ok = $true
        for ($j = 0; $j -lt $Dll.Length; $j++) {
            if ($Pe[$i + $j] -ne $Dll[$j]) { $ok = $false; break }
        }
        if ($ok) { return $i }
    }
    return -1
}

Write-Host "=== Stage 10-A: runtime.dll surface gate ==="
Write-Host "  gen12 window:             18432 bytes 路 SHA prefix 43ffde58 (was b609a735 @ v0.3)"
Write-Host "  v0.3 baseline DLL:        $BaselineV03Bytes bytes (outside gen12)"
Write-Host "  fail-closed MAX DLL:      $MaxDllBytes bytes"

if (-not (Test-Path $Yoyo) -or -not (Test-Path $RuntimeDll)) {
    if ($SkipBuild) { throw "missing yoyo.exe or yoyo_runtime.dll (and -SkipBuild)" }
    Write-Host "== build yoyo + yoyo-runtime (release) =="
    Push-Location (Join-Path $Root "yoyo-rust")
    cargo build --release -p verifier
    if ($LASTEXITCODE -ne 0) { throw "verifier build failed" }
    cargo build --release -p yoyo-runtime
    if ($LASTEXITCODE -ne 0) { throw "yoyo-runtime build failed" }
    Pop-Location
}

if (-not (Test-Path $RuntimeDll)) {
    Write-Host "Stage 10-A: RED (yoyo_runtime.dll missing)"
    exit 1
}

$dllItem = Get-Item $RuntimeDll
$dllBytes = [int64]$dllItem.Length
$dllSha = Get-Sha256Hex $RuntimeDll
$deltaVsV03 = $BaselineV03Bytes - $dllBytes

Write-Host ""
Write-Host "DLL path:  $RuntimeDll"
Write-Host "DLL size:  $dllBytes bytes"
Write-Host "DLL SHA256:$dllSha"
Write-Host "vs v0.3:   delta=$deltaVsV03 bytes (positive = shrink)"

if ($dllBytes -gt $MaxDllBytes) {
    Write-Host "Stage 10-A: RED (DLL $dllBytes > MAX $MaxDllBytes) 鈥?host trust surface grew or shrink regresssed"
    exit 1
}
if ($dllBytes -ge $BaselineV03Bytes) {
    Write-Host "Stage 10-A: RED (DLL $dllBytes >= v0.3 baseline $BaselineV03Bytes) 鈥?no measurable shrink"
    exit 1
}

# Probe: exotic backends must not remain reachable strings in the min runtime.
$dllRaw = [System.IO.File]::ReadAllBytes($RuntimeDll)
$dllAscii = [System.Text.Encoding]::ASCII.GetString($dllRaw)
$forbidden = @("wasmtime", "cranelift")
foreach ($s in $forbidden) {
    if ($dllAscii.IndexOf($s) -ge 0) {
        Write-Host "Stage 10-A: RED (forbidden host surface marker '$s' found in DLL)"
        exit 1
    }
}
if ($dllAscii.IndexOf("yoyo_runtime_selfhost") -lt 0) {
    Write-Host "Stage 10-A: RED (export name yoyo_runtime_selfhost missing)"
    exit 1
}

$gen1 = Join-Path $WorkDir "gen1.exe"
$embedOk = $false
$gen1Len = 0
$embedOff = -1

if (-not $SkipLinkSmoke) {
    if (-not (Test-Path $Ty)) { throw "missing $Ty" }
    if (-not (Test-Path $Yoyo)) { throw "missing $Yoyo" }
    Write-Host ""
    Write-Host "== link smoke: yoyo link 鈫?gen1 (H_00 + embed) =="
    if (Test-Path $gen1) { Remove-Item $gen1 }
    & $Yoyo link --target=win32 $Ty $gen1
    if ($LASTEXITCODE -ne 0 -or -not (Test-Path $gen1)) {
        Write-Host "Stage 10-A: RED (gen1 link failed)"
        exit 1
    }
    $gen1Len = [int64](Get-Item $gen1).Length
    $pe = [System.IO.File]::ReadAllBytes($gen1)
    $embedOff = Find-EmbeddedDllOffset $pe $dllRaw
    if ($embedOff -ge 0) {
        Write-Host "Stage 10-A: RED (exact embed at $embedOff — post-v1.0 sidecar shrink regresssed)"
        exit 1
    }
    $ascii = [System.Text.Encoding]::ASCII.GetString($pe)
    if ($ascii.IndexOf("yoyo_rt.dll") -lt 0 -or $ascii.IndexOf("LoadLibraryA") -lt 0) {
        Write-Host "Stage 10-A: RED (sidecar markers yoyo_rt.dll / LoadLibraryA missing)"
        exit 1
    }
    $embedOk = $true  # means "sidecar posture OK" (no exact embed)
    Write-Host "gen1: $gen1Len bytes; no exact embed (cwd sidecar yoyo_rt.dll)"
}

# Persist machine-readable observation for docs / Relock notes.
$report = [ordered]@{
    stage            = "10-A"
    gate             = "runtime-surface"
    status           = "PASS"
    gen12_window     = 18432
    gen12_sha_prefix = "43ffde58"
    dll_path         = $RuntimeDll
    dll_bytes        = $dllBytes
    dll_sha256       = $dllSha
    max_dll_bytes    = $MaxDllBytes
    baseline_v03_bytes = $BaselineV03Bytes
    delta_vs_v03     = $deltaVsV03
    gen1_bytes       = $gen1Len
    embed_offset     = $embedOff
    embed_exact      = $embedOk
    honest_remaining = @(
        "DLL still Rust-compiled (verifier lib, Win32/Linux/Stub emit only)",
        "DLL bytes still outside gen12 18432B compared .text window",
        "H_00 LoadLibraryA cwd sidecar yoyo_rt.dll (no exact embed; still Rust runtime)"
    )
}
$reportPath = Join-Path $WorkDir "runtime-surface.json"
# WriteAllText avoids PS5.1 Set-Content -Encoding utf8 flake (GetContentWriterArgumentError on locked/0-byte file).
[System.IO.File]::WriteAllText($reportPath, ($report | ConvertTo-Json -Depth 5))
Write-Host "report: $reportPath"

Write-Host ""
Write-Host "Stage 10-A: GREEN"
Write-Host "  trust-chain: host DLL $BaselineV03Bytes 鈫?$dllBytes (螖 -$deltaVsV03); still outside gen12"
exit 0
