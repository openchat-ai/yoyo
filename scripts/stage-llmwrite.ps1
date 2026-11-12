# stage-llmwrite.ps1 — AUTO-LLMWRITE: unambiguous LLM write contract + file samples
param([switch]$SkipBuild)
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== AUTO-LLMWRITE ==="

& node (Join-Path $Root "yoyo-js\scripts\validate-llmwrite-contract.js")
if ($LASTEXITCODE -ne 0) { Write-Host "LLMWRITE status=RED reason=validate"; exit 1 }

# Behavioral: contract llm samples == cat/head/zero shapes
$Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
if (-not (Test-Path $Yoyo)) {
    if ($SkipBuild) { throw "missing yoyo.exe" }
    Push-Location (Join-Path $Root "yoyo-rust")
    cargo build --release -p verifier
    if ($LASTEXITCODE -ne 0) { throw "verifier build failed" }
    Pop-Location
}

$Fix = Join-Path $Root "scripts\fixtures\anyprog\llm"
$Work = Join-Path $Root "scripts\_stage-llmwrite"
New-Item -ItemType Directory -Force -Path $Work | Out-Null

function Run-File([string]$name, [byte[]]$inputBytes, [byte[]]$wantOut) {
    $dir = Join-Path $Work $name
    New-Item -ItemType Directory -Force -Path $dir | Out-Null
    $ty = Join-Path $Fix "$name.ty"
    $exe = Join-Path $dir "$name.exe"
    $in = Join-Path $dir "input.tyb"
    $out = Join-Path $dir "output.exe"
    if (Test-Path $out) { Remove-Item -Force $out }
    if ($null -ne $inputBytes) { [IO.File]::WriteAllBytes($in, $inputBytes) }
    elseif (Test-Path $in) { Remove-Item -Force $in }
    & $Yoyo link --target=win32 $ty $exe | Out-Null
    if ($LASTEXITCODE -ne 0) { throw "LLMWRITE link failed $name" }
    Push-Location $dir
    try { & $exe; $code = $LASTEXITCODE } finally { Pop-Location }
    if ($code -ne 0) { throw "LLMWRITE status=RED reason=exit tool=$name code=$code" }
    $got = [IO.File]::ReadAllBytes($out)
    if ($got.Length -ne $wantOut.Length) { throw "LLMWRITE status=RED reason=len $name" }
    for ($i = 0; $i -lt $wantOut.Length; $i++) {
        if ($got[$i] -ne $wantOut[$i]) { throw "LLMWRITE status=RED reason=bytes $name" }
    }
    Write-Host "LLMWRITE run=$name status=GREEN"
}

$hello = [Text.Encoding]::ASCII.GetBytes("hello")
Run-File "cat" $hello $hello
Run-File "head" $hello ([Text.Encoding]::ASCII.GetBytes("hel"))
Run-File "zero" $null ([byte[]](0, 0, 0, 0))

Write-Host "LLMWRITE note=contract_v4_drift;surface_coverage;human_deferred;not_Thompson"

# Extreme-env drift: rebuild determinism + artifact pins + bitflip detect
& (Join-Path $Root "scripts\validate-llmwrite-drift.ps1") -Yoyo $Yoyo -SkipBuild
if ($LASTEXITCODE -ne 0) { Write-Host "LLMWRITE status=RED reason=drift"; exit 1 }

Write-Host "LLMWRITE status=GREEN"
Write-Host "LLMWRITE doc=AUTO-LLMWRITEDRIFT.md"
exit 0
