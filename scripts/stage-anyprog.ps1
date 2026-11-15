# stage-anyprog.ps1 — AUTO-ANYPROG: tier-1 file tools + tier-2 console/argv/tick
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

function Link-App([string]$name, [string]$tyName) {
    $dir = Join-Path $Work $name
    New-Item -ItemType Directory -Force -Path $dir | Out-Null
    $ty = Join-Path $Fix $tyName
    $exe = Join-Path $dir "$name.exe"
    $linkOut = & $Yoyo link --target=win32 $ty $exe 2>&1 | Out-String
    if ($LASTEXITCODE -ne 0) {
        Write-Host $linkOut
        throw "ANYPROG link failed tool=$name"
    }
    return $dir, $exe
}

function Run-FileTool([string]$name, [string]$tyName, [byte[]]$inputBytes, [byte[]]$wantOut) {
    $dir, $exe = Link-App $name $tyName
    $in = Join-Path $dir "input.tyb"
    $out = Join-Path $dir "output.exe"
    if (Test-Path $out) { Remove-Item -Force $out }
    if ($null -ne $inputBytes) {
        [IO.File]::WriteAllBytes($in, $inputBytes)
    } elseif (Test-Path $in) {
        Remove-Item -Force $in
    }
    Push-Location $dir
    try {
        & $exe
        $code = $LASTEXITCODE
    } finally {
        Pop-Location
    }
    if ($code -ne 0) { throw "ANYPROG status=RED reason=exit tool=$name code=$code" }
    if (-not (Test-Path $out)) { throw "ANYPROG status=RED reason=no_output tool=$name" }
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

function Run-StdoutTool([string]$name, [string]$tyName, [string]$wantContains) {
    $dir, $exe = Link-App $name $tyName
    $stdoutPath = Join-Path $dir "stdout.txt"
    if (Test-Path $stdoutPath) { Remove-Item -Force $stdoutPath }
    $p = Start-Process -FilePath $exe -WorkingDirectory $dir -Wait -PassThru -NoNewWindow `
        -RedirectStandardOutput $stdoutPath -RedirectStandardError (Join-Path $dir "stderr.txt")
    if ($p.ExitCode -ne 0) {
        throw "ANYPROG status=RED reason=exit tool=$name code=$($p.ExitCode)"
    }
    if (-not (Test-Path $stdoutPath)) {
        throw "ANYPROG status=RED reason=no_stdout tool=$name"
    }
    $text = [IO.File]::ReadAllText($stdoutPath)
    if ($text -notlike "*$wantContains*") {
        throw "ANYPROG status=RED reason=stdout_miss tool=$name want=$wantContains got=$text"
    }
    Write-Host "ANYPROG tool=$name status=GREEN stdout_len=$($text.Length)"
}

function Run-TickTool {
    $name = "tick"
    $dir, $exe = Link-App $name "tick.ty"
    $out = Join-Path $dir "output.exe"
    if (Test-Path $out) { Remove-Item -Force $out }
    Push-Location $dir
    try {
        & $exe
        $code = $LASTEXITCODE
    } finally {
        Pop-Location
    }
    if ($code -ne 0) { throw "ANYPROG status=RED reason=exit tool=$name code=$code" }
    if (-not (Test-Path $out)) { throw "ANYPROG status=RED reason=no_output tool=$name" }
    $got = [IO.File]::ReadAllBytes($out)
    if ($got.Length -ne 8) {
        throw "ANYPROG status=RED reason=tick_len got=$($got.Length)"
    }
    $allZero = ($got | Where-Object { $_ -ne 0 }).Count -eq 0
    if ($allZero) {
        throw "ANYPROG status=RED reason=tick_all_zero"
    }
    Write-Host ("ANYPROG tool=tick status=GREEN tick64_prefix={0:x2}{1:x2}" -f $got[0], $got[1])
}

# Tier-1
$hello = [Text.Encoding]::ASCII.GetBytes("hello")
Run-FileTool "cat" "cat.ty" $hello $hello
Run-FileTool "head" "head.ty" $hello ([Text.Encoding]::ASCII.GetBytes("hel"))
Run-FileTool "zero" "zero.ty" $null ([byte[]](0, 0, 0, 0))

# Tier-2
Run-StdoutTool "hello_console" "hello_console.ty" "hi"
# First 16 cmdline bytes are like `"F:\yoyo\scripts` — assert path fragment, not .exe suffix.
Run-StdoutTool "cmdline" "cmdline.ty" "yoyo"
Run-TickTool
Run-StdoutTool "thread" "thread.ty" "ok"
# socket.ty kept as fixture; Winsock OPEN (WSAStartup host AV) — see API-HONEST.md

if (-not (Test-Path (Join-Path $Fix "API-HONEST.md"))) {
    throw "ANYPROG status=RED reason=api_honest_missing"
}

Write-Host "ANYPROG tools=cat,head,zero,hello_console,cmdline,tick,thread"
Write-Host "ANYPROG note=app_iat_11;cold_start_a1_6;winsock_OPEN;not_Thompson"
Write-Host "ANYPROG status=GREEN"
Write-Host "ANYPROG doc=AUTO-ANYPROG.md"
exit 0
