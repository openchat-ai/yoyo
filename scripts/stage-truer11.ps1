# stage-truer11.ps1 — AUTO-TRUER11 E2E (YOYO compact + code lineage)
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-TRUER11 E2E ==="

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER11 status=RED reason=place_stub_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER11 status=RED reason=bins_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-no-sidecar-face.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER11 status=RED reason=sidecar_name_face"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-h00-bake-zero.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER11 status=RED reason=h00_clean"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-prefab-bins-sot.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER11 status=RED reason=prefab_sot"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-compact-code.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER11 status=RED reason=yoyo_compact_code"; exit 1 }

$dataBytes = (Get-Item (Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\data.bin")).Length
if ($dataBytes -ne 42608) {
    Write-Host ("TRUER11 status=RED reason=data_bytes got={0} want=42608" -f $dataBytes)
    exit 1
}

& (Join-Path $Root "scripts\stage-truer5-semantic.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER11 status=RED reason=semantic"; exit 1 }

& (Join-Path $Root "scripts\stage-c1-native.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER11 status=RED reason=c1_native"; exit 1 }

# C1 × yoyo-compact-ret.tyb → output must be MZ and .text contain C3 (YOYO FF)
$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$placeOut = Join-Path $Root "scripts\_stage-truer11\place"
if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER11 status=RED reason=place_compact"; exit 1 }
$c1 = Join-Path $placeOut "yoyo_compiler.exe"
$cold = Join-Path $Root "scripts\_stage-truer11\cold-compact"
if (Test-Path $cold) { Remove-Item -Recurse -Force $cold }
New-Item -Force -ItemType Directory $cold | Out-Null
Copy-Item -Force $c1 (Join-Path $cold "yoyo_compiler.exe")
Copy-Item -Force (Join-Path $Root "yoyo\projects\yoyo-compact-ret.tyb") (Join-Path $cold "input.tyb")
$ec = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $cold "yoyo_compiler.exe") -WorkingDirectory $cold -TimeoutSec 120
$out = Join-Path $cold "output.exe"
if ($ec -ne 0 -or -not (Test-Path $out)) {
    Write-Host ("TRUER11 status=RED reason=compact_ret_compile exit={0}" -f $ec)
    exit 1
}
$outBytes = [IO.File]::ReadAllBytes($out)
if ($outBytes.Length -lt 2 -or $outBytes[0] -ne 0x4D -or $outBytes[1] -ne 0x5A) {
    Write-Host "TRUER11 status=RED reason=compact_ret_not_mz"
    exit 1
}
# .text must include C3 (ret from YOYO FF)
$eLfanew = [BitConverter]::ToInt32($outBytes, 0x3c)
$sizeOpt = [BitConverter]::ToUInt16($outBytes, $eLfanew + 20)
$sect = $eLfanew + 24 + $sizeOpt
$textRp = [BitConverter]::ToInt32($outBytes, $sect + 20)
$textRs = [BitConverter]::ToInt32($outBytes, $sect + 16)
$hasC3 = $false
for ($i = $textRp; $i -lt ($textRp + $textRs) -and $i -lt $outBytes.Length; $i++) {
    if ($outBytes[$i] -eq 0xC3) { $hasC3 = $true; break }
}
if (-not $hasC3) {
    Write-Host "TRUER11 status=RED reason=compact_ret_text_no_c3"
    exit 1
}
Write-Host ("TRUER11 compact_ret_pe_bytes={0} text_has_c3=1 exit={1}" -f $outBytes.Length, $ec)

Write-Host "TRUER11 trust_root=place_stub"
Write-Host "TRUER11 compact_sot=yoyo_ff_tyb"
Write-Host "TRUER11 code_lineage=c1_pe_text_plus13"
Write-Host "TRUER11 prefab_sot=bins_minus_compact"
Write-Host "TRUER11 data_bytes=42608"
Write-Host "TRUER11 bins_total=58204"
Write-Host "TRUER11 compact_derived=9728"
Write-Host "TRUER11 doc=AUTO-TRUER11.md"
Write-Host "TRUER11 status=GREEN"
Write-Host "TRUER11 note=compact_yoyo;code_lineage_pinned;plant_still_prefab;not_Thompson"
exit 0
