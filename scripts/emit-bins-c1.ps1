# emit-bins-c1.ps1 — C1 process emits code.bin+data.bin from self PE (AUTO-YOYO-BINS)
# No Node rebuild-compiler-payload / buildCompilerCode on this hop.
#
#   cd F:\yoyo
#   & .\scripts\emit-bins-c1.ps1 [-OutDir <dir>] [-Bootstrap <c1.exe>]
param(
    [string]$OutDir = "",
    [string]$Bootstrap = ""
)
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root
. (Join-Path $Root "scripts\lib\unattended-ui-harden.ps1")
Enable-YoyoUnattendedUiHarden

if (-not $OutDir) {
    $OutDir = Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload"
}
if (-not $Bootstrap) {
    $candidates = @(
        (Join-Path $Root "scripts\fixtures\yoyo_compiler.exe"),
        (Join-Path $Root "scripts\_stage-c1-native\yoyo_compiler.exe")
    )
    foreach ($c in $candidates) {
        if (Test-Path $c) { $Bootstrap = $c; break }
    }
}
if (-not $Bootstrap -or -not (Test-Path $Bootstrap)) {
    Write-Host "YOYO_BINS status=RED reason=missing_c1"
    exit 1
}

$work = Join-Path $Root "scripts\_stage-yoyo-bins\emit-work"
New-Item -ItemType Directory -Force -Path $work | Out-Null
Copy-Item -Force $Bootstrap (Join-Path $work "yoyo_compiler.exe")
# Sentinel triggers try_emit_bins; remove inputs that would take other paths.
"" | Set-Content -Encoding ascii (Join-Path $work "emit.bins")
if (Test-Path (Join-Path $work "input.tyb")) { Remove-Item -Force (Join-Path $work "input.tyb") }
if (Test-Path (Join-Path $work "code.bin")) { Remove-Item -Force (Join-Path $work "code.bin") }
if (Test-Path (Join-Path $work "data.bin")) { Remove-Item -Force (Join-Path $work "data.bin") }
if (Test-Path (Join-Path $work "output.exe")) { Remove-Item -Force (Join-Path $work "output.exe") }

Write-Host "=== YB C1 emit bins from self PE (no Node rebuild) ==="
Write-Host "YOYO_BINS node_rebuild=ABSENT"
Write-Host "YOYO_BINS node_buildCompilerCode=ABSENT"
$ec = Invoke-YoyoUnattendedNativeExe -FilePath (Join-Path $work "yoyo_compiler.exe") -WorkingDirectory $work -TimeoutSec 120
$codeOut = Join-Path $work "code.bin"
$dataOut = Join-Path $work "data.bin"
if ($ec -ne 0 -or -not (Test-Path $codeOut) -or -not (Test-Path $dataOut)) {
    Write-Host ("YOYO_BINS status=RED reason=c1_emit_failed exit={0}" -f $ec)
    exit 1
}

New-Item -ItemType Directory -Force -Path $OutDir | Out-Null
Copy-Item -Force $codeOut (Join-Path $OutDir "code.bin")
Copy-Item -Force $dataOut (Join-Path $OutDir "data.bin")

$codeSha = (Get-FileHash $codeOut -Algorithm SHA256).Hash.ToLowerInvariant()
$dataSha = (Get-FileHash $dataOut -Algorithm SHA256).Hash.ToLowerInvariant()
Write-Host ("YOYO_BINS emitter=c1_extract_self_pe")
Write-Host ("YOYO_BINS code_bytes={0} sha256_prefix={1}" -f (Get-Item $codeOut).Length, $codeSha.Substring(0, 16))
Write-Host ("YOYO_BINS data_bytes={0} sha256_prefix={1}" -f (Get-Item $dataOut).Length, $dataSha.Substring(0, 16))
Write-Host ("YOYO_BINS out={0}" -f $OutDir)
Write-Host "YOYO_BINS doc=AUTO-YOYO-BINS.md"
Write-Host "YOYO_BINS status=GREEN"
exit 0
