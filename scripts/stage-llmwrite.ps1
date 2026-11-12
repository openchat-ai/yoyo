# stage-llmwrite.ps1 — AUTO-LLMWRITEPIPE: contract + drift/dual + behavioral
param([switch]$SkipBuild)
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== AUTO-LLMWRITE ==="

& node (Join-Path $Root "yoyo-js\scripts\validate-llmwrite-contract.js")
if ($LASTEXITCODE -ne 0) { Write-Host "LLMWRITE status=RED reason=validate"; exit 1 }

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

function Run-Stdout([string]$name, [string]$wantContains) {
    $dir = Join-Path $Work $name
    New-Item -ItemType Directory -Force -Path $dir | Out-Null
    $ty = Join-Path $Fix "$name.ty"
    $exe = Join-Path $dir "$name.exe"
    & $Yoyo link --target=win32 $ty $exe | Out-Null
    if ($LASTEXITCODE -ne 0) { throw "LLMWRITE link failed $name" }
    $stdoutPath = Join-Path $dir "stdout.txt"
    if (Test-Path $stdoutPath) { Remove-Item -Force $stdoutPath }
    $p = Start-Process -FilePath $exe -WorkingDirectory $dir -Wait -PassThru -NoNewWindow `
        -RedirectStandardOutput $stdoutPath -RedirectStandardError (Join-Path $dir "stderr.txt")
    if ($p.ExitCode -ne 0) { throw "LLMWRITE status=RED reason=exit tool=$name code=$($p.ExitCode)" }
    $text = [IO.File]::ReadAllText($stdoutPath)
    if ($text -notlike "*$wantContains*") {
        throw "LLMWRITE status=RED reason=stdout_miss tool=$name want=$wantContains got=$text"
    }
    Write-Host "LLMWRITE run=$name status=GREEN"
}

function Run-Tick {
    $name = "tick"
    $dir = Join-Path $Work $name
    New-Item -ItemType Directory -Force -Path $dir | Out-Null
    $ty = Join-Path $Fix "$name.ty"
    $exe = Join-Path $dir "$name.exe"
    $out = Join-Path $dir "output.exe"
    if (Test-Path $out) { Remove-Item -Force $out }
    & $Yoyo link --target=win32 $ty $exe | Out-Null
    if ($LASTEXITCODE -ne 0) { throw "LLMWRITE link failed $name" }
    Push-Location $dir
    try { & $exe; $code = $LASTEXITCODE } finally { Pop-Location }
    if ($code -ne 0) { throw "LLMWRITE status=RED reason=exit tool=$name code=$code" }
    $got = [IO.File]::ReadAllBytes($out)
    if ($got.Length -ne 8) { throw "LLMWRITE status=RED reason=tick_len" }
    if (($got | Where-Object { $_ -ne 0 }).Count -eq 0) { throw "LLMWRITE status=RED reason=tick_all_zero" }
    Write-Host "LLMWRITE run=tick status=GREEN"
}

$hello = [Text.Encoding]::ASCII.GetBytes("hello")
Run-File "cat" $hello $hello
Run-File "head" $hello ([Text.Encoding]::ASCII.GetBytes("hel"))
Run-File "zero" $null ([byte[]](0, 0, 0, 0))
Run-Stdout "hello_console" "hi"
Run-Stdout "cmdline" "yoyo"
Run-Tick
Run-Stdout "thread" "ok"

Write-Host "LLMWRITE note=contract_v5_pipe;surface_9_in;socket_OPEN;not_Thompson"

& (Join-Path $Root "scripts\validate-llmwrite-drift.ps1") -Yoyo $Yoyo -SkipBuild
if ($LASTEXITCODE -ne 0) { Write-Host "LLMWRITE status=RED reason=drift"; exit 1 }

Write-Host "LLMWRITE status=GREEN"
Write-Host "LLMWRITE doc=AUTO-LLMWRITEPIPE.md"
exit 0
