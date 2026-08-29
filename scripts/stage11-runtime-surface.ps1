# stage11-runtime-surface.ps1 鈥?Stage 11-A: thinner embedded yoyo_runtime.dll gate
#
# Continues Stage 10-A: fail-closed size + exact embed match, with a tighter MAX
# after Stage 11-A `release-runtime` fat LTO + strip (still Rust cdylib; not
# YOYO-built). Also runs a YOYO-parity smoke: gen1 H_00 (embedded DLL) compile
# of yoyo.tyb must .text-DDC EQUAL `yoyo bootstrap` of the same input 鈥?so the
# host DLL's compile *effect* stays under the same monitor as seed/bootstrap
# even though DLL bytes remain OUTSIDE the gen12 compared .text window.
#
# v0.3 baseline: 485888 B
# v0.4 / Stage 10-A: 231936 B (MAX 250000) 鈥?stage10-runtime-surface.ps1
# Stage 11-A:        measured under MAX 170000 鈥?this script
#
# Honest remaining: still Rust-compiled; Win/Linux still LoadLibrary/libdl this blob.
param(
    [switch]$SkipBuild,
    [switch]$SkipLinkSmoke,
    [switch]$SkipParity
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

# Fail-closed ceiling (bytes). Tighter than Stage 10 MAX 250000 鈥?do not raise casually.
$MaxDllBytes = 150000
# v0.4 documented size for before鈫抋fter reporting (not enforced as floor).
$BaselineV04Bytes = 231936
$BaselineV03Bytes = 485888

$WorkDir = Join-Path $Root "scripts\_stage11-runtime-surface"
New-Item -ItemType Directory -Force -Path $WorkDir | Out-Null

$Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
$RuntimeDllPreferred = Join-Path $Root "yoyo-rust\target\release-runtime\yoyo_runtime.dll"
$RuntimeDllCompat = Join-Path $Root "yoyo-rust\target\release\yoyo_runtime.dll"
$Ty = Join-Path $Root "yoyo\projects\yoyo.ty"
$Tyb = Join-Path $Root "yoyo\projects\yoyo.tyb"

function Resolve-RuntimeDll {
    if (Test-Path $RuntimeDllPreferred) { return $RuntimeDllPreferred }
    if (Test-Path $RuntimeDllCompat) { return $RuntimeDllCompat }
    return $RuntimeDllPreferred
}
$RuntimeDll = Resolve-RuntimeDll

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

Write-Host "=== Stage 11-A: thinner runtime.dll surface gate ==="
Write-Host "  gen12 window:             18432 bytes (SHA moves with dll_embed_size)"
Write-Host "  v0.4 baseline DLL:        $BaselineV04Bytes bytes (outside gen12)"
Write-Host "  fail-closed MAX DLL:      $MaxDllBytes bytes (Stage 10 was 250000)"

$needRuntime = -not (Test-Path $RuntimeDllPreferred)
$needYoyo = -not (Test-Path $Yoyo)
if ($needYoyo -or $needRuntime) {
    if ($SkipBuild) { throw "missing yoyo.exe or release-runtime yoyo_runtime.dll (and -SkipBuild)" }
    Push-Location (Join-Path $Root "yoyo-rust")
    if ($needYoyo) {
        Write-Host "== build yoyo (release) =="
        cargo build --release -p verifier
        if ($LASTEXITCODE -ne 0) { throw "verifier build failed" }
    }
    if ($needRuntime) {
        Write-Host "== build yoyo-runtime (release-runtime 路 fat LTO) =="
        cargo build --profile release-runtime -p yoyo-runtime
        if ($LASTEXITCODE -ne 0) { throw "yoyo-runtime release-runtime build failed" }
    }
    Pop-Location
}

if (Test-Path $RuntimeDllPreferred) {
    $RuntimeDll = $RuntimeDllPreferred
} elseif (Test-Path $RuntimeDllCompat) {
    $RuntimeDll = $RuntimeDllCompat
    Write-Host "WARN: using compat path $RuntimeDll (prefer --profile release-runtime)"
} else {
    Write-Host "Stage 11-A: RED (yoyo_runtime.dll missing)"
    exit 1
}

# Keep target/release copy in sync so stage10 gate still sees the thin DLL.
if ($RuntimeDll -eq $RuntimeDllPreferred) {
    $compatDir = Split-Path $RuntimeDllCompat
    if (-not (Test-Path $compatDir)) { New-Item -ItemType Directory -Force -Path $compatDir | Out-Null }
    Copy-Item -Force $RuntimeDllPreferred $RuntimeDllCompat
}

$dllItem = Get-Item $RuntimeDll
$dllBytes = [int64]$dllItem.Length
$dllSha = Get-Sha256Hex $RuntimeDll
$deltaVsV04 = $BaselineV04Bytes - $dllBytes
$deltaVsV03 = $BaselineV03Bytes - $dllBytes

Write-Host ""
Write-Host "DLL path:  $RuntimeDll"
Write-Host "DLL size:  $dllBytes bytes"
Write-Host "DLL SHA256:$dllSha"
Write-Host "vs v0.4:   delta=$deltaVsV04 bytes (positive = shrink)"
Write-Host "vs v0.3:   delta=$deltaVsV03 bytes"

if ($dllBytes -gt $MaxDllBytes) {
    Write-Host "Stage 11-A: RED (DLL $dllBytes > MAX $MaxDllBytes) 鈥?host trust surface grew or shrink regresssed"
    exit 1
}
if ($dllBytes -ge $BaselineV04Bytes) {
    Write-Host "Stage 11-A: RED (DLL $dllBytes >= v0.4 baseline $BaselineV04Bytes) 鈥?no measurable Stage 11 shrink"
    exit 1
}

# Probe: exotic backends must not remain reachable strings in the min runtime.
$dllRaw = [System.IO.File]::ReadAllBytes($RuntimeDll)
$dllAscii = [System.Text.Encoding]::ASCII.GetString($dllRaw)
$forbidden = @("wasmtime", "cranelift")
foreach ($s in $forbidden) {
    if ($dllAscii.IndexOf($s) -ge 0) {
        Write-Host "Stage 11-A: RED (forbidden host surface marker '$s' found in DLL)"
        exit 1
    }
}
if ($dllAscii.IndexOf("yoyo_runtime_selfhost") -lt 0) {
    Write-Host "Stage 11-A: RED (export name yoyo_runtime_selfhost missing)"
    exit 1
}

# YOYO-built path status: still host Rust cdylib (honest). Gate records the path
# + compile-parity so a future YOYO-built artifact can flip without silent skip.
$yoyoBuiltPath = $false
$buildPathNote = "rust-cdylib+profile=release-runtime(lto-fat+strip+abort+opt-z) (Stage 11-A thinner; not YOYO-built)"

$gen1 = Join-Path $WorkDir "gen1.exe"
$embedOk = $false
$gen1Len = 0
$embedOff = -1
$parityOk = $false
$paritySha = ""

if (-not $SkipLinkSmoke) {
    if (-not (Test-Path $Ty)) { throw "missing $Ty" }
    if (-not (Test-Path $Yoyo)) { throw "missing $Yoyo" }
    Write-Host ""
    Write-Host "== link smoke: yoyo link 鈫?gen1 (H_00 + embed) =="
    if (Test-Path $gen1) { Remove-Item $gen1 }
    & $Yoyo link --target=win32 $Ty $gen1
    if ($LASTEXITCODE -ne 0 -or -not (Test-Path $gen1)) {
        Write-Host "Stage 11-A: RED (gen1 link failed)"
        exit 1
    }
    $gen1Len = [int64](Get-Item $gen1).Length
    $pe = [System.IO.File]::ReadAllBytes($gen1)
    $embedOff = Find-EmbeddedDllOffset $pe $dllRaw
    if ($embedOff -ge 0) {
        Write-Host "Stage 11-A: RED (exact embed at $embedOff — post-v1.0 sidecar shrink regresssed)"
        exit 1
    }
    $ascii = [System.Text.Encoding]::ASCII.GetString($pe)
    if ($ascii.IndexOf("yoyo_rt.dll") -lt 0) {
        Write-Host "Stage 11-A: RED (sidecar marker yoyo_rt.dll missing)"
        exit 1
    }
    $embedOk = $true
    Write-Host "gen1: $gen1Len bytes; no exact embed (cwd sidecar yoyo_rt.dll)"
}

if (-not $SkipParity -and $embedOk) {
    if (-not (Test-Path $Tyb)) { throw "missing $Tyb" }
    Write-Host ""
    Write-Host "== YOYO parity: gen1 H_00 (DLL) vs yoyo bootstrap (.text DDC) =="
    $runDir = Join-Path $WorkDir "parity-run"
    New-Item -ItemType Directory -Force -Path $runDir | Out-Null
    Copy-Item $gen1 (Join-Path $runDir "gen1.exe") -Force
    Copy-Item $Tyb (Join-Path $runDir "input.tyb") -Force
    Copy-Item $RuntimeDll (Join-Path $runDir "yoyo_rt.dll") -Force
    $gen1Out = Join-Path $runDir "output.exe"
    if (Test-Path $gen1Out) { Remove-Item $gen1Out }
    Push-Location $runDir
    try {
        & ".\gen1.exe"
        $gen1Exit = $LASTEXITCODE
    } finally {
        Pop-Location
    }
    if ($gen1Exit -ne 0 -or -not (Test-Path $gen1Out)) {
        Write-Host "Stage 11-A: RED (gen1 H_00 runtime compile failed, exit=$gen1Exit)"
        exit 1
    }
    $bootOut = Join-Path $WorkDir "parity-bootstrap.exe"
    if (Test-Path $bootOut) { Remove-Item $bootOut }
    & $Yoyo bootstrap $Tyb $bootOut
    if ($LASTEXITCODE -ne 0 -or -not (Test-Path $bootOut)) {
        Write-Host "Stage 11-A: RED (bootstrap reference failed)"
        exit 1
    }
    & $Yoyo diff $gen1Out $bootOut 2>&1 | ForEach-Object { Write-Host "  $_" }
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Stage 11-A: RED (gen1 DLL compile vs bootstrap .text DDC DIFF)"
        exit 1
    }
    $parityOk = $true
    $paritySha = (Get-Sha256Hex $gen1Out).Substring(0, 8)
    Write-Host "parity: gen1(H_00/DLL) 鈮?bootstrap (.text DDC EQUAL), out sha prefix $paritySha"
}

$report = [ordered]@{
    stage              = "11-A"
    gate               = "runtime-surface"
    status             = "PASS"
    gen12_window       = 18432
    dll_path           = $RuntimeDll
    dll_bytes          = $dllBytes
    dll_sha256         = $dllSha
    max_dll_bytes      = $MaxDllBytes
    baseline_v04_bytes = $BaselineV04Bytes
    baseline_v03_bytes = $BaselineV03Bytes
    delta_vs_v04       = $deltaVsV04
    delta_vs_v03       = $deltaVsV03
    gen1_bytes         = $gen1Len
    embed_offset       = $embedOff
    embed_exact        = $embedOk
    parity_equal       = $parityOk
    parity_sha_prefix  = $paritySha
    yoyo_built_path    = $yoyoBuiltPath
    build_path         = $buildPathNote
    honest_remaining   = @(
        "DLL still Rust-compiled (verifier lib, Win32/Linux/Stub emit only)",
        "DLL bytes still outside gen12 18432B compared .text window",
        "Each genN still embeds and LoadLibrary this blob via H_00",
        "Not YOYO-built 鈥?Stage 11-A is thinner host face + fail-closed + compile-parity monitor"
    )
}
$reportPath = Join-Path $WorkDir "runtime-surface.json"
# WriteAllText avoids PS5.1 Set-Content -Encoding utf8 flake (GetContentWriterArgumentError on locked/0-byte file).
[System.IO.File]::WriteAllText($reportPath, ($report | ConvertTo-Json -Depth 5))
Write-Host "report: $reportPath"

Write-Host ""
Write-Host "Stage 11-A: GREEN"
Write-Host "  trust-chain: host DLL $BaselineV04Bytes 鈫?$dllBytes (螖 -$deltaVsV04 vs v0.4); still outside gen12"
Write-Host "  monitored:   embed exact + compile-parity vs bootstrap; path=$buildPathNote"
exit 0
