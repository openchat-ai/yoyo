# stage-anyprog.ps1 — AUTO-ANYPROG tier-1: three Win32 tools via rust link
param([switch]$SkipBuild)
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== AUTO-ANYPROG ==="

$Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
if (-not (Test-Path $Yoyo)) {
    if ($SkipBuild) { throw "missing yoyo.exe" }
    Push-Location (Join-Path $Root "yoyo-rust")
    cargo build --release -p verifier
    if ($LASTEXITCODE -ne 0) { throw "verifier build failed" }
    Pop-Location
}

$Fix = Join-Path $Root "scripts\fixtures\anyprog"
$Work = Join-Path $Root "scripts\_stage-anyprog"
New-Item -ItemType Directory -Force -Path $Work | Out-Null

function Run-Tool([string]$name, [string]$tyName, [byte[]]$inputBytes, [byte[]]$wantOut) {
    $dir = Join-Path $Work $name
    New-Item -ItemType Directory -Force -Path $dir | Out-Null
    $ty = Join-Path $Fix $tyName
    $exe = Join-Path $dir "$name.exe"
    $in = Join-Path $dir "input.tyb"
    $out = Join-Path $dir "output.exe"
    if (Test-Path $out) { Remove-Item -Force $out }
    if ($null -ne $inputBytes) {
        [IO.File]::WriteAllBytes($in, $inputBytes)
    } elseif (Test-Path $in) {
        Remove-Item -Force $in
    }

    $linkOut = & $Yoyo link --target=win32 $ty $exe 2>&1 | Out-String
    if ($LASTEXITCODE -ne 0) {
        Write-Host $linkOut
        throw "ANYPROG link failed tool=$name"
    }

    Push-Location $dir
    try {
        & $exe
        $code = $LASTEXITCODE
    } finally {
        Pop-Location
    }
    if ($code -ne 0) {
        throw "ANYPROG status=RED reason=exit tool=$name code=$code"
    }
    if (-not (Test-Path $out)) {
        throw "ANYPROG status=RED reason=no_output tool=$name"
    }
    $got = [IO.File]::ReadAllBytes($out)
    if ($got.Length -ne $wantOut.Length) {
        throw "ANYPROG status=RED reason=len tool=$name want=$($wantOut.Length) got=$($got.Length)"
    }
    for ($i = 0; $i -lt $wantOut.Length; $i++) {
        if ($got[$i] -ne $wantOut[$i]) {
            throw "ANYPROG status=RED reason=bytes tool=$name at=$i"
        }
    }
    Write-Host "ANYPROG tool=$name status=GREEN bytes=$($got.Length)"
}

$hello = [Text.Encoding]::ASCII.GetBytes("hello")
Run-Tool "cat" "cat.ty" $hello $hello
Run-Tool "head" "head.ty" $hello ([Text.Encoding]::ASCII.GetBytes("hel"))
Run-Tool "zero" "zero.ty" $null ([byte[]](0, 0, 0, 0))

if (-not (Test-Path (Join-Path $Fix "API-HONEST.md"))) {
    throw "ANYPROG status=RED reason=api_honest_missing"
}

Write-Host "ANYPROG tools=cat,head,zero"
Write-Host "ANYPROG note=console_via_output_file;A1_unchanged;not_Thompson"
Write-Host "ANYPROG status=GREEN"
Write-Host "ANYPROG doc=AUTO-ANYPROG.md"
exit 0
