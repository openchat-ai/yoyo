#!/usr/bin/env pwsh
<#
.SYNOPSIS
  yoyo-asm golden verification (3rd DDC peer).
  Builds and runs yoyo-asm inside WSL, compares INC/DEC raw bytes
  against golden hex. Exit 0 on all pass.
#>
$ErrorActionPreference = 'Stop'
Write-Host "== yoyo-asm (WSL) =="

$scriptBlock = @'
cd /mnt/f/yoyo/yoyo-asm
make yoyo-asm 2>&1
./yoyo-asm > /tmp/yoyo-asm-out.bin
INC_HEX=$(xxd -p -l 18 /tmp/yoyo-asm-out.bin | tr -d '\n')
DEC_HEX=$(xxd -p -s 18 -l 18 /tmp/yoyo-asm-out.bin | tr -d '\n')
echo "INC=$INC_HEX"
echo "DEC=$DEC_HEX"
'@

$result = wsl -e bash -c $scriptBlock

$incLine = $result | Select-String '^INC='
$decLine = $result | Select-String '^DEC='
if (-not $incLine -or -not $decLine) {
    Write-Host "FAIL: asm output parse error"
    Write-Host $result
    exit 1
}
$incHex = ($incLine -replace '^INC=','').Trim()
$decHex = ($decLine -replace '^DEC=','').Trim()

$incExpected = '498b878002000048ffc049898780020000c3'
$decExpected = '498b878002000048ffc849898780020000c3'

$pass = 0
if ($incHex -eq $incExpected) {
    Write-Host "  INC S[0x50] = $incHex  PASS"
    $pass++
} else {
    Write-Host "  INC FAIL: got $incHex  want $incExpected"
}
if ($decHex -eq $decExpected) {
    Write-Host "  DEC S[0x50] = $decHex  PASS"
    $pass++
} else {
    Write-Host "  DEC FAIL: got $decHex  want $decExpected"
}

Write-Host "---"
if ($pass -eq 2) {
    Write-Host "yoyo-asm: ALL PASS"
    exit 0
} else {
    Write-Host "yoyo-asm: FAIL"
    exit 1
}
