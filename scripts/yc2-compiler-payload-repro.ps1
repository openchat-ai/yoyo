# yc2-compiler-payload-repro.ps1 — Y2 via C1 process (AUTO-C1-NATIVE)
#
# C1 (no input.tyb) native-links code.bin+data.bin → output.exe ≡ C1.
# Hop MUST NOT call Node buildCompilerCode / linkPeWin32 / place-yoyo-compiler.
#
#   cd F:\yoyo
#   & .\scripts\yc2-compiler-payload-repro.ps1
param(
    [string]$C1Path = "",
    [string]$OutDir = ""
)
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden

if (-not $C1Path) {
    $candidates = @(
        (Join-Path $Root "scripts\_stage-c1-native\stub-place\yoyo_compiler.exe"),
        (Join-Path $Root "scripts\_stage-c1-native\yoyo_compiler.exe"),
        (Join-Path $Root "scripts\fixtures\yoyo_compiler.exe"),
        (Join-Path $Root "scripts\_stage-hb-fx-peer-compiler\yoyo_compiler.exe"),
        (Join-Path $Root "scripts\_stage-fullbar-fs2-chain\_peer_compiler_once\yoyo_compiler.exe")
    )
    foreach ($c in $candidates) {
        if (Test-Path $c) { $C1Path = $c; break }
    }
}
if (-not $OutDir) {
    $OutDir = Join-Path $Root "scripts\_stage-yc2-repro"
}
New-Item -ItemType Directory -Force -Path $OutDir | Out-Null

$codePath = Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\code.bin"
$dataPath = Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\data.bin"
if (-not $C1Path -or -not (Test-Path $C1Path)) {
    Write-Host "YOYO_CLOSED_Y2 status=RED reason=missing_c1"
    exit 1
}
if (-not (Test-Path $codePath) -or -not (Test-Path $dataPath)) {
    Write-Host "YOYO_CLOSED_Y2 status=RED reason=missing_payload_bins"
    exit 1
}

Write-Host "=== YC-2 C1 process payload→PE repro (no Node on hop) ==="
Write-Host "YOYO_CLOSED_Y2 node_buildCompilerCode=ABSENT"
Write-Host "YOYO_CLOSED_Y2 node_linkPeWin32=ABSENT"
Write-Host "YOYO_CLOSED_Y2 node_place_yoyo_compiler=ABSENT"
Write-Host "YOYO_CLOSED_Y2 emitter=c1_try_place_bins"

$run = Join-Path $OutDir "c1-process"
New-Item -ItemType Directory -Force -Path $run | Out-Null
Copy-Item -Force $C1Path (Join-Path $run "yoyo_compiler.exe")
Copy-Item -Force $codePath (Join-Path $run "code.bin")
Copy-Item -Force $dataPath (Join-Path $run "data.bin")
if (Test-Path (Join-Path $run "input.tyb")) { Remove-Item -Force (Join-Path $run "input.tyb") }
if (Test-Path (Join-Path $run "output.exe")) { Remove-Item -Force (Join-Path $run "output.exe") }

$ec = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $run "yoyo_compiler.exe") -WorkingDirectory $run -TimeoutSec 120
$c2Path = Join-Path $run "output.exe"
if ($ec -ne 0 -or -not (Test-Path $c2Path)) {
    Write-Host ("YOYO_CLOSED_Y2 status=RED reason=c1_process_repro_failed exit={0}" -f $ec)
    exit 1
}

$h1 = (Get-FileHash $C1Path -Algorithm SHA256).Hash.ToLowerInvariant()
$h2 = (Get-FileHash $c2Path -Algorithm SHA256).Hash.ToLowerInvariant()
Write-Host ("YOYO_CLOSED_Y2 c1_sha256_prefix={0} bytes={1}" -f $h1.Substring(0, 16), (Get-Item $C1Path).Length)
Write-Host ("YOYO_CLOSED_Y2 c2_sha256_prefix={0} bytes={1}" -f $h2.Substring(0, 16), (Get-Item $c2Path).Length)
if ($h1 -ne $h2) {
    Write-Host "YOYO_CLOSED_Y2 status=RED reason=c2_NE_c1"
    exit 1
}

Copy-Item -Force $c2Path (Join-Path $OutDir "yoyo_compiler_c2.exe")
Write-Host "YOYO_CLOSED_Y2 compiler_self_repro=EQUAL"
Write-Host "YOYO_CLOSED_Y2 note=c1_native_linkPeWin32;not_Thompson"
Write-Host "YOYO_CLOSED_Y2 doc=AUTO-C1-NATIVE.md"
Write-Host "YOYO_CLOSED_Y2 status=GREEN"
exit 0
