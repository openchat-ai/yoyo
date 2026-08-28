# stage10-asm-peer-io.ps1 — Stage 10-C: Python asm peer platform I/O aligned with Rust Win32
# Fail-closed: asm production emit for 0x20/0x50/0x51 must NOT be movabs+store stub,
# and must byte-equal Rust --target=win32 handler bodies on a minimal fixture.
param(
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

$WorkDir = Join-Path $Root "scripts\_stage10-asm-peer-io"
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

Write-Host "== Stage 10-C: v0.3 JS peer gate still green =="
& (Join-Path $Root "scripts\stage9-js-peer-io.ps1") -SkipBuild
if ($LASTEXITCODE -ne 0) { throw "stage9-js-peer-io failed (exit $LASTEXITCODE)" }

$fixtures = @(
    @{ Name = "alloc"; Ty = "40 00`r`n  20 50 1000`r`n  FF`r`n"; Op = "0x20"; Args = @(0x50, 0x1000) },
    @{ Name = "load";  Ty = "40 00`r`n  50 50 00`r`n  FF`r`n"; Op = "0x50"; Args = @(0x50, 0) },
    @{ Name = "write"; Ty = "40 00`r`n  51 50 00 51`r`n  FF`r`n"; Op = "0x51"; Args = @(0x50, 0, 0x51) }
)

$AsmDir = Join-Path $Root "yoyo-asm"
$Py = "python"

Write-Host "== Stage 10-C: asm win32 emit vs Rust win32 .text body =="
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

    $argsCsv = ($f.Args -join ",")
    $asmHex = & $Py -c @"
import sys
sys.path.insert(0, r'$AsmDir')
from platform_io import encode_io_op, is_movabs_store_stub
op = $($f.Op)
args = [$argsCsv]
b = encode_io_op(op, args, 'win32')
if is_movabs_store_stub(b):
    sys.stderr.write('STUB\n')
    sys.exit(2)
sys.stdout.write(b.hex())
"@
    if ($LASTEXITCODE -eq 2) { throw "asm $($f.Name) still movabs+store stub (blind zone)" }
    if ($LASTEXITCODE -ne 0) { throw "asm emit $($f.Name) failed" }
    if ($asmHex -ne $rustHex) {
        Write-Host "DIFF $($f.Name)"
        Write-Host "  asm  $asmHex"
        Write-Host "  Rust $rustHex"
        throw "Stage 10-C: $($f.Name) asm≠Rust win32 I/O bytes"
    }
    Write-Host "EQUAL $($f.Name) ($($rustBody.Length)B)"
}

Write-Host "== Stage 10-C: asm linux ALLOC not stub + has syscall =="
$linuxHex = & $Py -c @"
import sys
sys.path.insert(0, r'$AsmDir')
from platform_io import encode_io_op, is_movabs_store_stub
b = encode_io_op(0x20, [0x50, 0x1000], 'linux')
if is_movabs_store_stub(b):
    sys.stderr.write('STUB\n')
    sys.exit(2)
if b'\x0f\x05' not in b:
    sys.stderr.write('NO_SYSCALL\n')
    sys.exit(3)
sys.stdout.write(b.hex())
"@
if ($LASTEXITCODE -eq 2) { throw "asm linux ALLOC still movabs+store stub" }
if ($LASTEXITCODE -eq 3) { throw "asm linux ALLOC missing syscall 0F 05" }
if ($LASTEXITCODE -ne 0) { throw "asm linux ALLOC emit failed" }
Write-Host "linux ALLOC ok ($([int]($linuxHex.Length / 2))B)"

Write-Host "== Stage 10-C: stub path still movabs+store (G-SM-IO contract) =="
$stubOk = & $Py -c @"
import sys
sys.path.insert(0, r'$AsmDir')
from platform_io import encode_io_op, is_movabs_store_stub
b = encode_io_op(0x20, [0x50, 0x1000], 'stub')
assert is_movabs_store_stub(b), 'stub must be movabs+store'
assert len(b) == 17, f'stub ALLOC want 17B got {len(b)}'
print('ok')
"@
if ($LASTEXITCODE -ne 0) { throw "asm stub contract failed" }

Write-Host "Trust chain: asm platform_io.py win32 0x20/0x50/0x51 byte-equal Rust platform_io; linux syscall path peer-checked; stub remains for G-SM-IO."
Write-Host "Still divergent: full yoyo.ty section-ddc may differ (H_00 / IAT width / embedded runtime)."
Write-Host "Stage 10-C: GREEN"
exit 0
