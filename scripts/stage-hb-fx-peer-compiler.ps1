# stage-hb-fx-peer-compiler.ps1 — AUTO-LESSHOSTX: C1 place + yoyo hash (Node ABSENT)
#
# Historical HB-FX used Node JS peer place + emit verifies. Plant escape closed.
# This gate now: C1 try_place_bins + payload pin via yoyo hash. ≠ Thompson.
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== AUTO-HB-FX peer compiler place (LESSHOSTX / Node ABSENT) ==="

. (Join-Path $Root "scripts\lib\Assert-YoyoFilePin.ps1")
$Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
if (-not (Test-Path -LiteralPath $Yoyo)) {
    Write-Host "HB_FX status=RED reason=missing_yoyo"
    exit 1
}

try {
    Assert-YoyoPayloadBinsPin -Yoyo $Yoyo `
        -PayloadDir (Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload") `
        -Label HB_FX_BINS -Doc AUTO-TRUER6.md
} catch {
    Write-Host "HB_FX status=RED reason=bins_pin"
    exit 1
}

$work = Join-Path $Root "scripts\_stage-hb-fx-peer-compiler"
if (Test-Path $work) { Remove-Item -Recurse -Force $work }
New-Item -ItemType Directory -Force -Path $work | Out-Null

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $work -Bootstrap $stub
if ($LASTEXITCODE -ne 0 -or -not (Test-Path (Join-Path $work "yoyo_compiler.exe"))) {
    Write-Host "HB_FX status=RED reason=c1_place_failed"
    exit 1
}

$peerOut = Join-Path $work "yoyo_compiler.exe"
$peerSha = (Get-FileHash $peerOut -Algorithm SHA256).Hash.ToLowerInvariant()
Write-Host ("HB_FX peer_sha256_prefix={0} bytes={1}" -f $peerSha.Substring(0, 16), (Get-Item $peerOut).Length)
Write-Host "HB_FX chain_emitter=c1_try_place_bins"
Write-Host "HB_FX rust_place_compiler_on_this_gate=ABSENT"
Write-Host "HB_FX fixture_copy_on_this_gate=ABSENT"
Write-Host "HB_FX node_place=ABSENT"
Write-Host "HB_FX node_emit_verify=ABSENT"
Write-Host "HB_FX note=pin_locked_bins;js_emit_verify_superseded;not_Thompson"
Write-Host "HB_FX doc=AUTO-LESSHOSTX.md"
Write-Host "HB_FX status=GREEN"
exit 0
