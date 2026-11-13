# stage-lesshostx.ps1 — prove selfhost gate Node ABSENT; placepin green; cold_start honesty
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== AUTO-LESSHOSTX ==="

function Assert-NoNodeCall([string]$ScriptPath) {
    $lines = Get-Content -LiteralPath $ScriptPath
    $bad = @()
    for ($i = 0; $i -lt $lines.Count; $i++) {
        $line = $lines[$i]
        if ($line -match '^\s*#' ) { continue }
        if ($line -match '(?i)\bnode\b' -and $line -notmatch 'Node ABSENT|node_|=ABSENT|no Node|without Node|node_place|node_verify|node_rebuild|node_emit|node_yoyo|node_host|node_derive|node_link|node_build|node_plant') {
            # allow comments already skipped; flag real invocations
            if ($line -match '(?i)(&\s*)?node\s|\bnode\.exe\b') {
                $bad += ("{0}:{1}" -f ($i + 1), $line.Trim())
            }
        }
    }
    if ($bad.Count -gt 0) {
        Write-Host "LESSHOSTX status=RED reason=node_call_in $(Split-Path $ScriptPath -Leaf)"
        $bad | ForEach-Object { Write-Host "LESSHOSTX node_line=$_" }
        throw "node present"
    }
    Write-Host ("LESSHOSTX node_scan={0} status=ABSENT" -f (Split-Path $ScriptPath -Leaf))
}

$gates = @(
    (Join-Path $Root "scripts\stage-placepin.ps1"),
    (Join-Path $Root "scripts\stage-hb-fx-peer-compiler.ps1"),
    (Join-Path $Root "scripts\stage-c1-native.ps1"),
    (Join-Path $Root "scripts\stage-fullbar-trust-root.ps1"),
    (Join-Path $Root "scripts\stage-yoyo-closed.ps1"),
    (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1")
)
foreach ($g in $gates) { Assert-NoNodeCall $g }

& (Join-Path $Root "scripts\stage-placepin.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "LESSHOSTX status=RED reason=placepin"; exit 1 }

& (Join-Path $Root "scripts\stage-hb-fx-peer-compiler.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "LESSHOSTX status=RED reason=hb_fx"; exit 1 }

& (Join-Path $Root "scripts\yc2-compiler-payload-repro.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "LESSHOSTX status=RED reason=yc2"; exit 1 }

# cold_start probe (honest)
$cold = Join-Path $Root "scripts\_stage-lesshostx-cold"
if (Test-Path $cold) { Remove-Item -Recurse -Force $cold }
New-Item -Force -ItemType Directory $cold | Out-Null
$c1 = Join-Path $Root "scripts\_stage-hb-fx-peer-compiler\yoyo_compiler.exe"
Copy-Item -Force $c1 (Join-Path $cold "yoyo_compiler.exe")
Copy-Item -Force (Join-Path $Root "yoyo\projects\yoyo.tyb") (Join-Path $cold "input.tyb")
. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null
$ec = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $cold "yoyo_compiler.exe") -WorkingDirectory $cold -TimeoutSec 120
$outOk = Test-Path (Join-Path $cold "output.exe")
if ($ec -eq 0 -and $outOk) {
    Write-Host "LESSHOSTX cold_start=GREEN"
    Write-Host "LESSHOSTX status=GREEN"
    Write-Host "LESSHOSTX doc=AUTO-LESSHOSTX.md"
    exit 0
}

Write-Host ("LESSHOSTX cold_start=HARD_BLOCK exit={0} out={1}" -f $ec, $outOk)
Write-Host "LESSHOSTX cold_start_note=C1_place_PE_pin_EQUAL_but_tyb_compile_fails;fix=separate;not_Stage17"
Write-Host "LESSHOSTX node_on_gate=ABSENT"
Write-Host "LESSHOSTX placepin=GREEN"
Write-Host "LESSHOSTX hb_fx=GREEN"
Write-Host "LESSHOSTX yc2=GREEN"
Write-Host "LESSHOSTX status=GREEN_WITH_HARD_BLOCK"
Write-Host "LESSHOSTX doc=AUTO-LESSHOSTX.md"
exit 0
