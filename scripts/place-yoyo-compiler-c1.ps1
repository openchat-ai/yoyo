# place-yoyo-compiler-c1.ps1 — place via C1 try_place_bins (no Node link/emit)
#
#   & .\scripts\place-yoyo-compiler-c1.ps1 <outdir> [-Bootstrap <c1.exe>]
param(
    [Parameter(Mandatory = $true)][string]$OutDir,
    [string]$Bootstrap = "",
    [string]$CodeBin = "",
    [string]$DataBin = ""
)
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root
. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden

if (-not $Bootstrap) {
    $candidates = @(
        (Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"),
        (Join-Path $Root "scripts\fixtures\yoyo_compiler.exe"),
        (Join-Path $Root "scripts\_stage-c1-native\yoyo_compiler.exe"),
        (Join-Path $Root "scripts\_stage-hb-fx-peer-compiler\yoyo_compiler.exe"),
        (Join-Path $Root "scripts\_stage-fullbar-fs2-chain\_peer_compiler_once\yoyo_compiler.exe")
    )
    foreach ($c in $candidates) {
        if (Test-Path $c) { $Bootstrap = $c; break }
    }
}
$codePath = if ($CodeBin) { $CodeBin } else { Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\code.bin" }
$dataPath = if ($DataBin) { $DataBin } else { Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\data.bin" }
if (-not $Bootstrap -or -not (Test-Path $Bootstrap)) {
    Write-Host "HB_FX_PLACE emitter=c1_try_place_bins"
    Write-Host "HB_FX_PLACE status=ABSENT"
    Write-Host "HB_FX_PLACE reason=missing_bootstrap_c1"
    exit 2
}
if (-not (Test-Path $codePath) -or -not (Test-Path $dataPath)) {
    Write-Host "HB_FX_PLACE status=ABSENT reason=missing_payload_bins"
    exit 2
}
if ($CodeBin) {
    Write-Host ("HB_FX_PLACE code_bin_override={0}" -f $CodeBin)
}

New-Item -ItemType Directory -Force -Path $OutDir | Out-Null
$work = Join-Path $OutDir "_c1_place_work"
New-Item -ItemType Directory -Force -Path $work | Out-Null
Copy-Item -Force $Bootstrap (Join-Path $work "yoyo_compiler.exe")
Copy-Item -Force $codePath (Join-Path $work "code.bin")
Copy-Item -Force $dataPath (Join-Path $work "data.bin")
if (Test-Path (Join-Path $work "input.tyb")) { Remove-Item -Force (Join-Path $work "input.tyb") }
if (Test-Path (Join-Path $work "output.exe")) { Remove-Item -Force (Join-Path $work "output.exe") }

Write-Host "=== C1 place (try_place_bins; no Node link) ==="
$ec = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $work "yoyo_compiler.exe") -WorkingDirectory $work -TimeoutSec 120
$outPe = Join-Path $work "output.exe"
if ($ec -ne 0 -or -not (Test-Path $outPe)) {
    Write-Host ("HB_FX_PLACE status=RED reason=c1_place_failed exit={0}" -f $ec)
    exit 1
}
$dest = Join-Path $OutDir "yoyo_compiler.exe"
Copy-Item -Force $outPe $dest
$sha = (Get-FileHash $dest -Algorithm SHA256).Hash.ToLowerInvariant()
Write-Host "HB_FX_PLACE emitter=c1_try_place_bins"
Write-Host "HB_FX_PLACE payload_origin=derived_bins_native_link_by_c1"
Write-Host ("HB_FX_PLACE code_bytes={0} data_bytes={1} pe_bytes={2}" -f (Get-Item $codePath).Length, (Get-Item $dataPath).Length, (Get-Item $dest).Length)
Write-Host ("HB_FX_PLACE out={0}" -f $dest)
Write-Host ("HB_FX_PLACE sha256_prefix={0}" -f $sha.Substring(0, 16))
Write-Host "HB_FX_PLACE node_linkPeWin32=ABSENT"
Write-Host "HB_FX_PLACE node_buildCompilerCode=ABSENT"
Write-Host "HB_FX_PLACE status=GREEN"
Write-Host "HB_FX_PLACE doc=AUTO-C1-NATIVE.md"
exit 0
