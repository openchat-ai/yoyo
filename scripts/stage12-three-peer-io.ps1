# stage12-three-peer-io.ps1 — Stage 12-A: Rust / JS / asm production I/O contract
# Fail-closed three-peer gate:
#   - stage10-asm-peer-io (+ embedded stage9-js-peer-io) must stay green
#   - win32 0x20/0x50/0x51: Rust PE body == JS encodeIoOp == asm encode_io_op
#   - linux 0x20/0x50/0x51: Rust ELF body == JS == asm (closes stage10 linux LOAD/WRITE blind zone)
#   - stub remains deterministic movabs+store (G-SM-IO); unknown platform → stub
param(
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

$WorkDir = Join-Path $Root "scripts\_stage12-three-peer-io"
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

function Get-JsHex([int]$op, [object[]]$opArgs, [string]$platform) {
    # Force JSON array even for single-element lists (ConvertTo-Json scalarizes).
    $argsJson = ConvertTo-Json @($opArgs) -Compress
    $hex = & node -e @"
const { encodeIoOp, isMovabsStoreStub } = require('./yoyo-js/src/platform/platform-io');
const b = Buffer.from(encodeIoOp($op, $argsJson, '$platform'));
if ('$platform' !== 'stub' && isMovabsStoreStub(b)) {
  console.error('STUB');
  process.exit(2);
}
process.stdout.write(b.toString('hex'));
"@
    if ($LASTEXITCODE -eq 2) { throw "JS $platform op=0x$('{0:x2}' -f $op) still movabs+store stub" }
    if ($LASTEXITCODE -ne 0) { throw "JS encode failed platform=$platform op=0x$('{0:x2}' -f $op)" }
    return $hex
}

function Get-AsmHex([int]$op, [object[]]$opArgs, [string]$platform) {
    $AsmDir = Join-Path $Root "yoyo-asm"
    $argsCsv = ($opArgs -join ",")
    $hex = & python -c @"
import sys
sys.path.insert(0, r'$AsmDir')
from platform_io import encode_io_op, is_movabs_store_stub
b = encode_io_op($op, [$argsCsv], '$platform')
if '$platform' != 'stub' and is_movabs_store_stub(b):
    sys.stderr.write('STUB\n')
    sys.exit(2)
sys.stdout.write(b.hex())
"@
    if ($LASTEXITCODE -eq 2) { throw "asm $platform op=0x$('{0:x2}' -f $op) still movabs+store stub" }
    if ($LASTEXITCODE -ne 0) { throw "asm encode failed platform=$platform op=0x$('{0:x2}' -f $op)" }
    return $hex
}

function Extract-RustBody([string]$target, [string]$tyPath, [string]$outPath) {
    $linkOut = & $Yoyo link "--target=$target" $tyPath $outPath 2>&1 | Out-String
    if ($LASTEXITCODE -ne 0) { throw "Rust link --target=$target failed: $linkOut" }
    if ($linkOut -notmatch '(\d+) code bytes') { throw "cannot parse code size: $linkOut" }
    $codeN = [int]$Matches[1]
    $img = [System.IO.File]::ReadAllBytes($outPath)
    if ($target -eq "win32") {
        $start = 0x400 + 13
    } elseif ($target -eq "linux") {
        $start = 0x1000 + 13
    } else {
        throw "unsupported extract target $target"
    }
    # Inclusive PowerShell range yields codeN-1 handler bytes (excludes trailing end marker).
    return ,($img[$start..($start + $codeN - 2)])
}

$fixtures = @(
    @{ Name = "alloc"; Ty = "40 00`r`n  20 50 1000`r`n  FF`r`n"; Op = 0x20; Args = @(0x50, 0x1000) },
    @{ Name = "load";  Ty = "40 00`r`n  50 50 00`r`n  FF`r`n"; Op = 0x50; Args = @(0x50, 0) },
    @{ Name = "write"; Ty = "40 00`r`n  51 50 00 51`r`n  FF`r`n"; Op = 0x51; Args = @(0x50, 0, 0x51) }
)

Write-Host "== Stage 12-A: prior peer gates (stage10 embeds stage9) =="
& (Join-Path $Root "scripts\stage10-asm-peer-io.ps1") -SkipBuild
if ($LASTEXITCODE -ne 0) { throw "stage10-asm-peer-io failed (exit $LASTEXITCODE)" }

foreach ($plat in @("win32", "linux")) {
    Write-Host "== Stage 12-A: three-peer $plat 0x20/0x50/0x51 =="
    foreach ($f in $fixtures) {
        $tyPath = Join-Path $WorkDir "$($f.Name)_$plat.ty"
        $binPath = Join-Path $WorkDir "$($f.Name)_rust_$plat"
        if ($plat -eq "win32") { $binPath += ".exe" }
        Write-Fixture $tyPath $f.Ty

        $rustBody = Extract-RustBody $plat $tyPath $binPath
        $rustHex = Hex-Bytes $rustBody
        $jsHex = Get-JsHex $f.Op $f.Args $plat
        $asmHex = Get-AsmHex $f.Op $f.Args $plat

        if ($jsHex -ne $rustHex) {
            Write-Host "DIFF $($f.Name) $plat JS≠Rust"
            Write-Host "  JS   $jsHex"
            Write-Host "  Rust $rustHex"
            throw "Stage 12-A: $($f.Name) $plat JS≠Rust"
        }
        if ($asmHex -ne $rustHex) {
            Write-Host "DIFF $($f.Name) $plat asm≠Rust"
            Write-Host "  asm  $asmHex"
            Write-Host "  Rust $rustHex"
            throw "Stage 12-A: $($f.Name) $plat asm≠Rust"
        }
        if ($jsHex -ne $asmHex) {
            throw "Stage 12-A: $($f.Name) $plat JS≠asm (should be unreachable after Rust pin)"
        }
        if ($plat -eq "linux" -and ($rustHex -notmatch '0f05')) {
            throw "Stage 12-A: $($f.Name) linux missing syscall 0F 05"
        }
        Write-Host "EQUAL $($f.Name) $plat ($($rustBody.Length)B) Rust=JS=asm"
    }
}

Write-Host "== Stage 12-A: stub contract (G-SM-IO) + unknown→stub fail-closed =="
foreach ($f in $fixtures) {
    $jsStub = Get-JsHex $f.Op $f.Args "stub"
    $asmStub = Get-AsmHex $f.Op $f.Args "stub"
    if ($jsStub -ne $asmStub) { throw "Stage 12-A: stub $($f.Name) JS≠asm" }
    if ($jsStub.Length -ne 34) { throw "Stage 12-A: stub $($f.Name) want 17B got $($jsStub.Length / 2)B" }
    if (-not $jsStub.StartsWith("48b8")) { throw "Stage 12-A: stub $($f.Name) not movabs+store" }
    Write-Host "STUB $($f.Name) 17B movabs+store JS=asm"
}

# Unknown platform must remain stub (honest fork — not silently inventing OS I/O).
$jsUnk = & node -e @"
const { encodeIoOp, isMovabsStoreStub } = require('./yoyo-js/src/platform/platform-io');
const b = Buffer.from(encodeIoOp(0x20, [0x50, 0x1000], 'freebsd'));
if (!isMovabsStoreStub(b) || b.length !== 17) process.exit(2);
process.stdout.write(b.toString('hex'));
"@
if ($LASTEXITCODE -ne 0) { throw "JS unknown platform must emit stub" }
$asmUnk = & python -c @"
import sys
sys.path.insert(0, r'$(Join-Path $Root 'yoyo-asm')')
from platform_io import encode_io_op, is_movabs_store_stub
b = encode_io_op(0x20, [0x50, 0x1000], 'freebsd')
assert is_movabs_store_stub(b) and len(b) == 17
sys.stdout.write(b.hex())
"@
if ($LASTEXITCODE -ne 0) { throw "asm unknown platform must emit stub" }
if ($jsUnk -ne $asmUnk) { throw "unknown-platform stub JS≠asm" }
Write-Host "UNKNOWN→STUB freebsd ALLOC JS=asm (honest fork pinned)"

Write-Host "Trust chain: Rust/JS/asm win32+linux 0x20/0x50/0x51 byte-equal; stub G-SM-IO; unknown OS → stub."
Write-Host "Still honest forks: Plan9/FreeBSD/Haiku/Serenity production I/O; full yoyo.ty section-ddc; embedded Rust runtime + LoadLibrary/libdl host."
Write-Host "Stage 12-A: GREEN"
exit 0
