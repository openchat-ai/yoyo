# stage-llmwrite.ps1 — AUTO-LLMWRITETY/PIPE: .ty lex gate → contract → drift → behavioral
param([switch]$SkipBuild)
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== AUTO-LLMWRITE ==="

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

# --- T: lex authority is gate_lex.ty (YOYO), not Node ---
$GateDir = Join-Path $Work "gate_lex"
New-Item -ItemType Directory -Force -Path $GateDir | Out-Null
$GateTy = Join-Path $Root "scripts\fixtures\anyprog\gate\gate_lex.ty"
$GateExe = Join-Path $GateDir "gate_lex.exe"
& $Yoyo link --target=win32 $GateTy $GateExe | Out-Null
if ($LASTEXITCODE -ne 0) { throw "LLMWRITE status=RED reason=gate_lex_link" }

function Invoke-TyLexGate([string]$srcTy, [int]$wantExit) {
    $in = Join-Path $GateDir "input.tyb"
    Copy-Item -Force -LiteralPath $srcTy -Destination $in
    Push-Location $GateDir
    try {
        & $GateExe
        $code = $LASTEXITCODE
    } finally {
        Pop-Location
    }
    if ($code -ne $wantExit) {
        throw "LLMWRITETY status=RED reason=gate_lex tool=$(Split-Path $srcTy -Leaf) exit=$code want=$wantExit"
    }
}

$goods = Get-ChildItem -LiteralPath $Fix -Filter *.ty |
    Where-Object { -not $_.Name.StartsWith('_poison') } |
    Sort-Object Name
foreach ($g in $goods) {
    Invoke-TyLexGate $g.FullName 0
    Write-Host "LLMWRITETY gate_lex sample=$($g.BaseName) status=GREEN"
}
$lexPoisons = @(
    '_poison_cr.ty',
    '_poison_tab.ty',
    '_poison_bom.ty',
    '_poison_comment.ty',
    '_poison_entry.ty'
)
foreach ($p in $lexPoisons) {
    $path = Join-Path $Fix $p
    if (-not (Test-Path $path)) { throw "LLMWRITETY status=RED reason=lex_poison_missing $p" }
    Invoke-TyLexGate $path 1
    Write-Host "LLMWRITETY gate_lex poison=$p status=RED_as_expected"
}
Write-Host "LLMWRITETY gate_lex status=GREEN doc=AUTO-LLMWRITETY.md"

# grammar/AST/generative still Node (honest residual meta)
& node (Join-Path $Root "yoyo-js\scripts\validate-llmwrite-contract.js")
if ($LASTEXITCODE -ne 0) { Write-Host "LLMWRITE status=RED reason=validate"; exit 1 }

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

Write-Host "LLMWRITE note=lex_authority=gate_lex.ty;grammar_js_residual;not_Thompson"

& (Join-Path $Root "scripts\validate-llmwrite-drift.ps1") -Yoyo $Yoyo -SkipBuild
if ($LASTEXITCODE -ne 0) { Write-Host "LLMWRITE status=RED reason=drift"; exit 1 }

Write-Host "LLMWRITE status=GREEN"
Write-Host "LLMWRITE doc=AUTO-LLMWRITETY.md"
exit 0
