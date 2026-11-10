# stage-truer5-semantic.ps1 — AUTO-TRUER5 X3: controlled tyb bitflip → C1 seed hash DIFF
#
#   cd F:\yoyo
#   & .\scripts\stage-truer5-semantic.ps1
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden | Out-Null

Write-Host "=== AUTO-TRUER5 X3 YOYO source semantic gate ==="

$stub = Join-Path $Root "scripts\fixtures\yoyo_place_stub.exe"
$placeOut = Join-Path $Root "scripts\_stage-truer5\sem-place"
if (Test-Path $placeOut) { Remove-Item -Recurse -Force $placeOut }
& (Join-Path $Root "scripts\place-yoyo-compiler-c1.ps1") -OutDir $placeOut -Bootstrap $stub
if ($LASTEXITCODE -ne 0) {
    Write-Host "TRUER5_SEM status=RED reason=place"
    exit 1
}
$c1 = Join-Path $placeOut "yoyo_compiler.exe"
$tyb = Join-Path $Root "yoyo\projects\yoyo.tyb"
if (-not (Test-Path $tyb)) {
    Write-Host "TRUER5_SEM status=RED reason=missing_yoyo_tyb"
    exit 1
}

function Invoke-Cold([string]$Dir, [string]$TybPath) {
    if (Test-Path $Dir) { Remove-Item -Recurse -Force $Dir }
    New-Item -Force -ItemType Directory $Dir | Out-Null
    Copy-Item -Force $c1 (Join-Path $Dir "yoyo_compiler.exe")
    Copy-Item -Force $TybPath (Join-Path $Dir "input.tyb")
    $ec = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $Dir "yoyo_compiler.exe") -WorkingDirectory $Dir -TimeoutSec 300
    $out = Join-Path $Dir "output.exe"
    if (-not (Test-Path $out)) {
        return @{ Ec = $ec; Sha = "MISSING"; Bytes = 0 }
    }
    $sha = (Get-FileHash $out -Algorithm SHA256).Hash
    return @{ Ec = $ec; Sha = $sha; Bytes = (Get-Item $out).Length }
}

$baseDir = Join-Path $Root "scripts\_stage-truer5\sem-base"
$base = Invoke-Cold $baseDir $tyb
Write-Host ("TRUER5_SEM base exit={0} bytes={1} sha256_prefix={2}" -f $base.Ec, $base.Bytes, $base.Sha.Substring(0, [Math]::Min(16, $base.Sha.Length)))
if ($base.Ec -ne 0 -or $base.Sha -eq "MISSING") {
    Write-Host "TRUER5_SEM status=RED reason=base_compile"
    exit 1
}

# Controlled mutation: XOR one byte at live body offset (OPENLOOP H_00 grew tyb; 2048 now fatal).
$mutTyb = Join-Path $Root "scripts\_stage-truer5\sem-mut.tyb"
$bytes = [IO.File]::ReadAllBytes($tyb)
$off = 4000
if ($bytes.Length -le $off) {
    Write-Host "TRUER5_SEM status=RED reason=tyb_too_small"
    exit 1
}
$bytes[$off] = $bytes[$off] -bxor 0x01
[IO.File]::WriteAllBytes($mutTyb, $bytes)

$mutDir = Join-Path $Root "scripts\_stage-truer5\sem-mut"
$mut = Invoke-Cold $mutDir $mutTyb
Write-Host ("TRUER5_SEM mut exit={0} bytes={1} sha256_prefix={2} flip_off={3}" -f $mut.Ec, $mut.Bytes, $mut.Sha.Substring(0, [Math]::Min(16, $mut.Sha.Length)), $off)
if ($mut.Ec -ne 0 -or $mut.Sha -eq "MISSING") {
    Write-Host "TRUER5_SEM status=RED reason=mut_compile"
    exit 1
}
if ($mut.Sha -eq $base.Sha) {
    Write-Host "TRUER5_SEM status=RED reason=hash_EQUAL_mutation_ignored"
    exit 1
}

Write-Host "TRUER5_SEM source_driven=PRESENT"
Write-Host "TRUER5_SEM input=input.tyb"
Write-Host "TRUER5_SEM mutation=xor1_off_4000"
Write-Host "TRUER5_SEM status=GREEN"
Write-Host "TRUER5_SEM doc=AUTO-TRUER5.md"
Write-Host "TRUER5_SEM note=tyb_flip_changes_seed;not_Thompson;not_full_YOYO_rewrite_plant"
exit 0
