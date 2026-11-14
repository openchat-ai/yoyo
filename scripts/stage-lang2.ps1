# stage-lang2.ps1 — v1.2 LANG-READABLE (.yoyor → .ty → link → run)
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== LANG2 (v1.2 readable source) ==="

$doc = Join-Path $Root "LANG-READABLE.md"
if (-not (Test-Path $doc)) {
    Write-Host "LANG2 status=RED reason=missing_LANG-READABLE"
    exit 1
}
$docText = Get-Content -LiteralPath $doc -Raw
foreach ($n in @('.yoyor', 'Expand-YoyoReadable', 'raw')) {
    if ($docText -notlike "*$n*") {
        Write-Host ("LANG2 status=RED reason=doc_missing token={0}" -f $n)
        exit 1
    }
}
Write-Host "LANG2 readable_doc=GREEN"

$Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
if (-not (Test-Path $Yoyo)) {
    Write-Host "LANG2 status=RED reason=missing_yoyo"
    exit 1
}

$src = Join-Path $Root "scripts\fixtures\anyprog\hello_hi.yoyor"
$work = Join-Path $Root "scripts\_stage-lang2"
if (Test-Path $work) { Remove-Item -Recurse -Force $work }
New-Item -ItemType Directory -Force -Path $work | Out-Null
$ty = Join-Path $work "hello_hi.ty"
$exe = Join-Path $work "hello_hi.exe"
$stdout = Join-Path $work "stdout.txt"
$stderr = Join-Path $work "stderr.txt"

& (Join-Path $Root "scripts\lib\Expand-YoyoReadable.ps1") -InputPath $src -OutputPath $ty
if ($LASTEXITCODE -ne 0 -and -not (Test-Path $ty)) {
    Write-Host "LANG2 status=RED reason=expand_failed"
    exit 1
}
if (-not (Test-Path $ty)) {
    Write-Host "LANG2 status=RED reason=expand_no_ty"
    exit 1
}
Write-Host "LANG2 expand=GREEN"

& $Yoyo link --target=win32 $ty $exe
if ($LASTEXITCODE -ne 0 -or -not (Test-Path $exe)) {
    Write-Host "LANG2 status=RED reason=link_failed"
    exit 1
}
Write-Host "LANG2 link=GREEN"

$p = Start-Process -FilePath $exe -WorkingDirectory $work -Wait -PassThru -NoNewWindow `
    -RedirectStandardOutput $stdout -RedirectStandardError $stderr
if ($p.ExitCode -ne 0) {
    Write-Host ("LANG2 status=RED reason=run_exit code={0}" -f $p.ExitCode)
    exit 1
}
$text = [IO.File]::ReadAllText($stdout)
if ($text -notlike "*hi*") {
    Write-Host ("LANG2 status=RED reason=stdout_miss got={0}" -f $text)
    exit 1
}
Write-Host "LANG2 run=GREEN stdout_has=hi"

# Poison: missing program header must fail expand
$bad = Join-Path $work "bad.yoyor"
Set-Content -LiteralPath $bad -Value "raw 40 00`n" -Encoding ascii
$badTy = Join-Path $work "bad.ty"
$poisonOk = $false
try {
    & (Join-Path $Root "scripts\lib\Expand-YoyoReadable.ps1") -InputPath $bad -OutputPath $badTy 2>&1 | Out-Null
    if (-not (Test-Path $badTy)) { $poisonOk = $true }
} catch {
    $poisonOk = $true
}
if (-not $poisonOk) {
    Write-Host "LANG2 status=RED reason=poison_expand_accepted"
    exit 1
}
Write-Host "LANG2 poison_expand=RED_OK"

Write-Host "LANG2 note=yoyor_above_hex;no_node;not_full_assembler;not_Thompson"
Write-Host "LANG2 doc=SCOPE-v1.2.md"
Write-Host "LANG2 status=GREEN"
exit 0
