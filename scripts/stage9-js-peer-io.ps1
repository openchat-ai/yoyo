# stage9-js-peer-io.ps1 — Stage 9-B: JS peer platform I/O aligned with Rust Win32
# Fail-closed: JS production emit for 0x20/0x50/0x51 must NOT be movabs+store stub,
# and must byte-equal Rust --target=win32 handler bodies on a minimal fixture.
param(
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

$WorkDir = Join-Path $Root "scripts\_stage9-peer-io"
New-Item -ItemType Directory -Force -Path $WorkDir | Out-Null

$Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
if (-not (Test-Path $Yoyo)) {
    if ($SkipBuild) { throw "missing yoyo.exe (and -SkipBuild)" }
    Write-Host "== build verifier (release) =="
    Push-Location (Join-Path $Root "yoyo-rust")
    cargo build --release -p verifier
    if ($LASTEXITCODE -ne 0) { throw "verifier build failed" }
    Pop-Location
}

function Hex-Bytes([byte[]]$bytes) {
    ($bytes | ForEach-Object { '{0:x2}' -f $_ }) -join ''
}

function Write-Fixture([string]$path, [string]$body) {
    Set-Content -Path $path -Value $body -Encoding ascii
}

Write-Host "== Stage 9-B: JS golden peer I/O =="
& node (Join-Path $Root "yoyo-js\scripts\golden.js") 2>&1 | Select-Object -Last 8
if ($LASTEXITCODE -ne 0) { throw "golden.js failed (exit $LASTEXITCODE)" }

$fixtures = @(
    @{ Name = "alloc"; Ty = "40 00`r`n  20 50 1000`r`n  FF`r`n"; Op = "0x20"; Args = @(0x50, 0x1000) },
    @{ Name = "load";  Ty = "40 00`r`n  50 50 00`r`n  FF`r`n"; Op = "0x50"; Args = @(0x50, 0) },
    @{ Name = "write"; Ty = "40 00`r`n  51 50 00 51`r`n  FF`r`n"; Op = "0x51"; Args = @(0x50, 0, 0x51) }
)

Write-Host "== Stage 9-B: JS win32 emit vs Rust win32 .text body =="
foreach ($f in $fixtures) {
    $tyPath = Join-Path $WorkDir "$($f.Name).ty"
    $exePath = Join-Path $WorkDir "$($f.Name)_rust.exe"
    Write-Fixture $tyPath $f.Ty

    $linkOut = & $Yoyo link --target=win32 $tyPath $exePath 2>&1 | Out-String
    if ($LASTEXITCODE -ne 0) { throw "Rust link $($f.Name) failed: $linkOut" }
    if ($linkOut -notmatch '(\d+) code bytes') { throw "cannot parse code size for $($f.Name): $linkOut" }
    $codeN = [int]$Matches[1]
    $pe = [System.IO.File]::ReadAllBytes($exePath)
    $start = 0x400 + 13
    $rustBody = $pe[$start..($start + $codeN - 2)]
    $rustHex = Hex-Bytes $rustBody

    $argsJson = ($f.Args | ConvertTo-Json -Compress)
    $jsHex = & node -e @"
const { setEmitPlatform, encodeOp } = require('./yoyo-js/src/platform/encode-x64');
const { isMovabsStoreStub } = require('./yoyo-js/src/platform/platform-io');
setEmitPlatform('win32');
const op = $($f.Op);
const args = $argsJson;
const b = Buffer.from(encodeOp(op, args, false));
if (isMovabsStoreStub(b)) { console.error('STUB'); process.exit(2); }
process.stdout.write(b.toString('hex'));
"@
    if ($LASTEXITCODE -eq 2) { throw "JS $($f.Name) still movabs+store stub (blind zone)" }
    if ($LASTEXITCODE -ne 0) { throw "JS emit $($f.Name) failed" }
    if ($jsHex -ne $rustHex) {
        Write-Host "DIFF $($f.Name)"
        Write-Host "  JS   $jsHex"
        Write-Host "  Rust $rustHex"
        throw "Stage 9-B: $($f.Name) JS≠Rust win32 I/O bytes"
    }
    Write-Host "EQUAL $($f.Name) ($($rustBody.Length)B)"
}

Write-Host "Trust chain: JS PE path setEmitPlatform(win32); 0x20/0x50/0x51 byte-equal Rust platform_io; stub remains for G-SM-IO."
Write-Host "Still divergent: Python asm peer still movabs+store; full yoyo.ty section-ddc may differ (H_00 / IAT width)."
Write-Host "Stage 9-B: GREEN"
exit 0
