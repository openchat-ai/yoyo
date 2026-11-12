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

# --- pin authority: yoyo hash (not Node crypto) ---
$ContractJson = Join-Path $Root "scripts\fixtures\anyprog\model-contract.json"
$ContractPin = Join-Path $Root "scripts\fixtures\anyprog\model-contract.sha256"
$gotPin = (& $Yoyo hash $ContractJson).Trim().ToLowerInvariant()
$wantPin = (Get-Content -LiteralPath $ContractPin -Raw).Trim().Split()[0].ToLowerInvariant()
if ($gotPin -ne $wantPin) {
    throw "LLMWRITEGRAM status=RED reason=yoyo_hash_pin_DIFF want=$wantPin got=$gotPin"
}
Write-Host "LLMWRITEGRAM yoyo_hash pin=EQUAL sha256_prefix=$($gotPin.Substring(0,16)) status=GREEN"

$Contract = Get-Content -LiteralPath $ContractJson -Raw | ConvertFrom-Json

# --- ty gates: lex + file_grammar ---
$GateRoot = Join-Path $Root "scripts\fixtures\anyprog\gate"
function Invoke-TyGate([string]$gateName, [string]$srcTy, [int]$wantExit) {
    $dir = Join-Path $Work $gateName
    New-Item -ItemType Directory -Force -Path $dir | Out-Null
    $exe = Join-Path $dir "$gateName.exe"
    if (-not (Test-Path $exe)) {
        & $Yoyo link --target=win32 (Join-Path $GateRoot "$gateName.ty") $exe | Out-Null
        if ($LASTEXITCODE -ne 0) { throw "LLMWRITE status=RED reason=${gateName}_link" }
    }
    Copy-Item -Force -LiteralPath $srcTy -Destination (Join-Path $dir "input.tyb")
    Push-Location $dir
    try { & $exe; $code = $LASTEXITCODE } finally { Pop-Location }
    if ($code -ne $wantExit) {
        throw "LLMWRITEGRAM status=RED reason=$gateName tool=$(Split-Path $srcTy -Leaf) exit=$code want=$wantExit"
    }
}

$goods = Get-ChildItem -LiteralPath $Fix -Filter *.ty |
    Where-Object { -not $_.Name.StartsWith('_poison') } |
    Sort-Object Name
foreach ($g in $goods) {
    Invoke-TyGate "gate_lex" $g.FullName 0
    Write-Host "LLMWRITETY gate_lex sample=$($g.BaseName) status=GREEN"
    $kind = [string]$Contract.program_kinds.($g.BaseName)
    if ($kind -eq 'file_grammar') {
        Invoke-TyGate "gate_grammar" $g.FullName 0
        Write-Host "LLMWRITEGRAM gate_grammar sample=$($g.BaseName) status=GREEN"
    }
}
$lexPoisons = @('_poison_cr.ty','_poison_tab.ty','_poison_bom.ty','_poison_comment.ty','_poison_entry.ty')
foreach ($p in $lexPoisons) {
    Invoke-TyGate "gate_lex" (Join-Path $Fix $p) 1
    Write-Host "LLMWRITETY gate_lex poison=$p status=RED_as_expected"
}
$gramPoisons = @('_poison_shape.ty','_poison_df_size.ty','_poison_df_buf.ty','_poison_op.ty','_poison_ret.ty','_poison_raw.ty')
foreach ($p in $gramPoisons) {
    Invoke-TyGate "gate_grammar" (Join-Path $Fix $p) 1
    Write-Host "LLMWRITEGRAM gate_grammar poison=$p status=RED_as_expected"
}
Write-Host "LLMWRITEGRAM ty_gates status=GREEN doc=AUTO-LLMWRITEGRAM.md"

# AST/generative residual Node (honest)
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

Write-Host "LLMWRITE note=lex+grammar=.ty;pin=yoyo_hash;AST_js_residual;not_Thompson"

& (Join-Path $Root "scripts\validate-llmwrite-drift.ps1") -Yoyo $Yoyo -SkipBuild
if ($LASTEXITCODE -ne 0) { Write-Host "LLMWRITE status=RED reason=drift"; exit 1 }

Write-Host "LLMWRITE status=GREEN"
Write-Host "LLMWRITE doc=AUTO-LLMWRITEGRAM.md"
exit 0
