# place-yoyo-compiler-c1.ps1 — place via C1 try_place_bins (no Node link/emit)
#
#   & .\scripts\place-yoyo-compiler-c1.ps1 -OutDir <outdir> [-Bootstrap <c1.exe>]
#   [-CodeBin path] [-DataBin path] [-ForceDerive]
#
# AUTO-TRUER20: if payload bins are absent (and no overrides), auto-derive from
# YOYO plant tyb + seeds assemble into OutDir/_derived and place from those.
param(
    [Parameter(Mandatory = $true)][string]$OutDir,
    [string]$Bootstrap = "",
    [string]$CodeBin = "",
    [string]$DataBin = "",
    [switch]$ForceDerive
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

$defaultCode = Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\code.bin"
$defaultData = Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\data.bin"
$codePath = if ($CodeBin) { $CodeBin } else { $defaultCode }
$dataPath = if ($DataBin) { $DataBin } else { $defaultData }
$placeInput = "payload_bins"
$derivedUsed = $false

if (-not $Bootstrap -or -not (Test-Path $Bootstrap)) {
    Write-Host "HB_FX_PLACE emitter=c1_try_place_bins"
    Write-Host "HB_FX_PLACE status=ABSENT"
    Write-Host "HB_FX_PLACE reason=missing_bootstrap_c1"
    exit 2
}

$needDerive = $ForceDerive -or (-not $CodeBin -and -not $DataBin -and (-not (Test-Path $codePath) -or -not (Test-Path $dataPath)))
if ($needDerive) {
    # AUTO-LESSHOSTX: refuse Node auto-derive on place gate. Bins must be pin-present.
    Write-Host "HB_FX_PLACE auto_derive=REFUSED"
    Write-Host "HB_FX_PLACE node_derive=ABSENT"
    Write-Host "HB_FX_PLACE status=RED reason=bins_required_no_node_derive"
    Write-Host "HB_FX_PLACE hint=restore pin-locked code.bin/data.bin; doc=AUTO-LESSHOSTX.md"
    exit 1
}

if (-not (Test-Path $codePath) -or -not (Test-Path $dataPath)) {
    Write-Host "HB_FX_PLACE status=ABSENT reason=missing_payload_bins"
    exit 2
}
if ($CodeBin) {
    Write-Host ("HB_FX_PLACE code_bin_override={0}" -f $CodeBin)
}
if ($DataBin) {
    Write-Host ("HB_FX_PLACE data_bin_override={0}" -f $DataBin)
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
Write-Output "HB_FX_PLACE emitter=c1_try_place_bins"
Write-Output "HB_FX_PLACE payload_origin=derived_bins_native_link_by_c1"
Write-Output ("HB_FX_PLACE place_bins_input={0}" -f $placeInput)
Write-Output ("HB_FX_PLACE auto_derive_used={0}" -f ($(if ($derivedUsed) { "1" } else { "0" })))
Write-Output ("HB_FX_PLACE code_bytes={0} data_bytes={1} pe_bytes={2}" -f (Get-Item $codePath).Length, (Get-Item $dataPath).Length, (Get-Item $dest).Length)
Write-Output ("HB_FX_PLACE out={0}" -f $dest)
Write-Output ("HB_FX_PLACE sha256_prefix={0}" -f $sha.Substring(0, 16))
Write-Output "HB_FX_PLACE node_linkPeWin32=ABSENT"
Write-Output "HB_FX_PLACE node_buildCompilerCode=ABSENT"
Write-Output "HB_FX_PLACE status=GREEN"
Write-Output "HB_FX_PLACE doc=AUTO-C1-NATIVE.md"
exit 0
