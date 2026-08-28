# stage13-cross-platform-parity.ps1 — Stage 13-B: Win/Linux parity + honest stub-OS pins
#
# Trust goal: 「绿」must not rest on one hosted platform while the other is blind /
# skipped. Stage 13-B thickens the dual-platform surface under one fail-closed gate:
#   1. stage12-three-peer-io — win32+linux 0x20/0x50/0x51 Rust=JS=asm (prior pin)
#   2. Stub-OS honesty — freebsd/haiku/plan9/serenity pinned as stubs (not invented I/O)
#      - JS/asm: unknown → movabs+store (17B)
#      - Rust freebsd/haiku: ALLOC/LOAD/WRITE 17B movabs+store Rust=JS=asm (no 0F05 invent)
#      - Rust plan9/serenity: NOP+payload / SERE flat stub (honest different shape; pinned)
#      - unknown apple/android → JS/asm G-SM-IO stub
#   3. stage13-link-host WITHOUT -SkipLinux — Win+Linux seed/link contract both green
#   4. stage9-pure-m4 (Win) AND stage10-linux-pure-m4.sh (WSL) — both required
#
# Honest remaining: Plan9/FreeBSD/Haiku/Serenity still stub (not production I/O);
# Rust runtime + LoadLibrary/libdl; full .text peer may DIFF; macOS not a gate.
param(
    [switch]$SkipBuild,
    [switch]$SkipWsl,
    [switch]$SkipPriorPeers
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

$WorkDir = Join-Path $Root "scripts\_stage13-cross-platform-parity"
New-Item -ItemType Directory -Force -Path $WorkDir | Out-Null
$Stamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
$SummaryPath = Join-Path $WorkDir "summary.txt"
$ExitTable = [System.Collections.Generic.List[string]]::new()
$Failed = $false

$Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
$RuntimeDllPreferred = Join-Path $Root "yoyo-rust\target\release-runtime\yoyo_runtime.dll"
$RuntimeDllCompat = Join-Path $Root "yoyo-rust\target\release\yoyo_runtime.dll"

function Write-Summary([string]$Status) {
    $lines = [System.Collections.Generic.List[string]]::new()
    $lines.Add("Stage 13-B cross-platform parity $Stamp") | Out-Null
    $lines.Add("") | Out-Null
    $lines.Add("EXIT TABLE:") | Out-Null
    foreach ($row in $ExitTable) { $lines.Add([string]$row) | Out-Null }
    $lines.Add("") | Out-Null
    $lines.Add($Status) | Out-Null
    $lines.Add("") | Out-Null
    $lines.Add("Trust chain: Win+Linux production I/O + seed/link + pure M4 under one gate;") | Out-Null
    $lines.Add("  stub OS (freebsd/haiku/plan9/serenity) fail-closed pinned — not silently inventing I/O.") | Out-Null
    $lines.Add("Honest remaining: stub OS still stub; Rust runtime + LoadLibrary/libdl; full .text may DIFF.") | Out-Null
    $lines | Set-Content -Path $SummaryPath -Encoding utf8
    foreach ($line in $lines) { Write-Host $line }
}

function Invoke-Gate([string]$Name, [scriptblock]$Body) {
    Write-Host "`n======== $Name ========" -ForegroundColor Cyan
    $sw = [Diagnostics.Stopwatch]::StartNew()
    $code = 0
    try {
        # Do NOT pipe to Out-Host — PS5.1 can lose/poison LASTEXITCODE on native stderr.
        & $Body
        $code = $LASTEXITCODE
        if ($null -eq $code) { $code = 0 }
    } catch {
        Write-Host ("EXCEPTION: {0}" -f $_) -ForegroundColor Red
        $code = 1
        if ($null -ne $LASTEXITCODE -and $LASTEXITCODE -ne 0) { $code = $LASTEXITCODE }
    }
    $sw.Stop()
    $ExitTable.Add(("{0,-55} {1}" -f $Name, $code)) | Out-Null
    Write-Host ("EXIT {0} = {1} ({2}s)" -f $Name, $code, [math]::Round($sw.Elapsed.TotalSeconds, 1)) `
        -ForegroundColor $(if ($code -eq 0) { "Green" } else { "Red" })
    if ($code -ne 0) { $script:Failed = $true }
    return $code
}

function Hex-Bytes([byte[]]$bytes) {
    ($bytes | ForEach-Object { '{0:x2}' -f $_ }) -join ''
}

function Write-Fixture([string]$path, [string]$body) {
    Set-Content -Path $path -Value $body -Encoding ascii
}

Write-Host "=== Stage 13-B: Win/Linux parity + stub-OS honesty ==="
Write-Host "  stamp: $Stamp"
Write-Host "  rule: both hosted platforms required; stub OS pinned (not invented)"

# --- ensure host tools (serial cargo only if needed) ---
$needYoyo = -not (Test-Path $Yoyo)
$needRuntime = -not ((Test-Path $RuntimeDllPreferred) -or (Test-Path $RuntimeDllCompat))
if ($needYoyo -or $needRuntime) {
    if ($SkipBuild) { throw "missing yoyo.exe or yoyo_runtime.dll (and -SkipBuild)" }
    Push-Location (Join-Path $Root "yoyo-rust")
    if ($needYoyo) {
        Write-Host "== build yoyo (release) =="
        cargo build --release -p verifier
        if ($LASTEXITCODE -ne 0) { throw "verifier build failed" }
    }
    if ($needRuntime -and -not (Test-Path $RuntimeDllPreferred) -and -not (Test-Path $RuntimeDllCompat)) {
        Write-Host "== build yoyo-runtime (release-runtime) =="
        cargo build --profile release-runtime -p yoyo-runtime
        if ($LASTEXITCODE -ne 0) { throw "yoyo-runtime build failed" }
    }
    Pop-Location
}

if (-not (Test-Path $Yoyo)) {
    Write-Host "Stage 13-B: RED (yoyo.exe missing)"
    exit 1
}

# --- 1. Prior three-peer Win+Linux (Stage 12-A) ---
if (-not $SkipPriorPeers) {
    $null = Invoke-Gate "stage12-three-peer-io.ps1" {
        & (Join-Path $Root "scripts\stage12-three-peer-io.ps1") -SkipBuild
    }
    if ($Failed) {
        Write-Summary "FAILED"
        Write-Host "Stage 13-B: RED" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "`n======== stage12-three-peer-io SKIPPED (-SkipPriorPeers) ========"
    $ExitTable.Add(("{0,-55} {1}" -f "stage12-three-peer-io.ps1", "SKIP")) | Out-Null
}

# --- 2. Expand stub-OS honesty pins (JS/asm + Rust) ---
$null = Invoke-Gate "stub-OS honesty pins" {
    $stubOs = @("freebsd", "haiku", "plan9", "serenity")
    $AsmDir = Join-Path $Root "yoyo-asm"

    foreach ($os in $stubOs) {
        $jsHex = & node -e @"
const { encodeIoOp, isMovabsStoreStub } = require('./yoyo-js/src/platform/platform-io');
const b = Buffer.from(encodeIoOp(0x20, [0x50, 0x1000], '$os'));
if (!isMovabsStoreStub(b) || b.length !== 17) { console.error('JS $os not stub'); process.exit(2); }
process.stdout.write(b.toString('hex'));
"@
        if ($LASTEXITCODE -ne 0) { throw "JS $os must emit movabs+store stub" }
        $asmHex = & python -c @"
import sys
sys.path.insert(0, r'$AsmDir')
from platform_io import encode_io_op, is_movabs_store_stub
b = encode_io_op(0x20, [0x50, 0x1000], '$os')
assert is_movabs_store_stub(b) and len(b) == 17, '$os not stub'
sys.stdout.write(b.hex())
"@
        if ($LASTEXITCODE -ne 0) { throw "asm $os must emit movabs+store stub" }
        if ($jsHex -ne $asmHex) { throw "stub-OS $os JS≠asm" }
        Write-Host "JS=asm STUB $os ALLOC 17B movabs+store"
    }

    # Rust: freebsd/haiku ALLOC/LOAD/WRITE = G-SM-IO three-peer EQUAL (closes Rust-blind pin).
    $ioFixtures = @(
        @{ Name = "alloc"; Ty = "40 00`r`n  20 50 1000`r`n  FF`r`n"; Op = 0x20; Args = @(0x50, 0x1000) },
        @{ Name = "load";  Ty = "40 00`r`n  50 50 00`r`n  FF`r`n"; Op = 0x50; Args = @(0x50, 0) },
        @{ Name = "write"; Ty = "40 00`r`n  51 50 00 51`r`n  FF`r`n"; Op = 0x51; Args = @(0x50, 0, 0x51) }
    )
    foreach ($os in @("freebsd", "haiku")) {
        foreach ($f in $ioFixtures) {
            $tyPath = Join-Path $WorkDir "$($f.Name)_$os.ty"
            Write-Fixture $tyPath $f.Ty
            $binPath = Join-Path $WorkDir "$($f.Name)_$os.elf"
            Remove-Item $binPath -Force -ErrorAction SilentlyContinue
            $linkOut = & $Yoyo link "--target=$os" $tyPath $binPath 2>&1 | Out-String
            if ($LASTEXITCODE -ne 0 -or -not (Test-Path $binPath)) {
                throw "Rust link --target=$os $($f.Name) failed: $linkOut"
            }
            if ($linkOut -notmatch '(\d+) code bytes') { throw "cannot parse code size for $os $($f.Name)" }
            $codeN = [int]$Matches[1]
            $img = [System.IO.File]::ReadAllBytes($binPath)
            $start = 0x1000 + 13
            $body = $img[$start..($start + $codeN - 2)]
            $hex = Hex-Bytes $body
            if ($body.Length -ne 17) { throw "Rust $os $($f.Name) body want 17B got $($body.Length)" }
            if ($body[0] -ne 0x48 -or $body[1] -ne 0xB8) {
                throw "Rust $os $($f.Name) not movabs+store stub"
            }
            if ($hex -match '0f05') {
                throw "Rust $os $($f.Name) invented syscall 0F05 — stub fork collapsed into Linux-like I/O"
            }
            $argsJson = ConvertTo-Json @($f.Args) -Compress
            $jsHex = & node -e @"
const { encodeIoOp } = require('./yoyo-js/src/platform/platform-io');
process.stdout.write(Buffer.from(encodeIoOp($($f.Op), $argsJson, '$os')).toString('hex'));
"@
            if ($LASTEXITCODE -ne 0) { throw "JS $os $($f.Name) encode failed" }
            $argsCsv = ($f.Args -join ",")
            $asmHex = & python -c @"
import sys
sys.path.insert(0, r'$AsmDir')
from platform_io import encode_io_op
sys.stdout.write(encode_io_op($($f.Op), [$argsCsv], '$os').hex())
"@
            if ($LASTEXITCODE -ne 0) { throw "asm $os $($f.Name) encode failed" }
            if ($jsHex -ne $hex) { throw "Rust $os $($f.Name) ≠ JS (stub peer blind)" }
            if ($asmHex -ne $hex) { throw "Rust $os $($f.Name) ≠ asm (stub peer blind)" }
            Write-Host "STUB-OS EQUAL $($f.Name) $os 17B Rust=JS=asm (no syscall invent)"
        }
    }
    $tyPath = Join-Path $WorkDir "alloc_stubos.ty"
    Write-Fixture $tyPath "40 00`r`n  20 50 1000`r`n  FF`r`n"

    # Rust: plan9 flat = NOP + size LE + slot LE + ret; serenity = SERE hdr + same NOP stub.
    $plan9Path = Join-Path $WorkDir "alloc_plan9.bin"
    Remove-Item $plan9Path -Force -ErrorAction SilentlyContinue
    & $Yoyo link --target=plan9 $tyPath $plan9Path | Out-Host
    if ($LASTEXITCODE -ne 0 -or -not (Test-Path $plan9Path)) { throw "Rust link --target=plan9 failed" }
    $p9 = [System.IO.File]::ReadAllBytes($plan9Path)
    if ($p9.Length -lt 18 -or $p9[0] -ne 0x90) {
        throw "Rust plan9 ALLOC must be NOP-stub flat (got len=$($p9.Length) head=0x$('{0:x2}' -f $p9[0]))"
    }
    $p9HasSyscall = $false
    for ($i = 0; $i -lt $p9.Length - 1; $i++) {
        if ($p9[$i] -eq 0x0F -and $p9[$i + 1] -eq 0x05) { $p9HasSyscall = $true; break }
    }
    if ($p9HasSyscall) { throw "Rust plan9 invented syscall — stub fork collapsed" }
    Write-Host "Rust STUB plan9 flat NOP+payload ($($p9.Length)B) pinned"

    $serPath = Join-Path $WorkDir "alloc_serenity.bin"
    Remove-Item $serPath -Force -ErrorAction SilentlyContinue
    & $Yoyo link --target=serenity $tyPath $serPath | Out-Host
    if ($LASTEXITCODE -ne 0 -or -not (Test-Path $serPath)) { throw "Rust link --target=serenity failed" }
    $se = [System.IO.File]::ReadAllBytes($serPath)
    $sere = [System.Text.Encoding]::ASCII.GetString($se[0..3])
    if ($sere -ne "SERE") { throw "Rust serenity missing SERE header (got '$sere')" }
    # payload after 8-byte header starts with NOP stub
    if ($se.Length -lt 10 -or $se[8] -ne 0x90) {
        throw "Rust serenity ALLOC body must be NOP-stub after SERE hdr"
    }
    Write-Host "Rust STUB serenity SERE+NOP ($($se.Length)B) pinned"

    # Contrast: linux must NOT look like stub (guard against accidental stub regression).
    $linuxPath = Join-Path $WorkDir "alloc_linux_contrast.elf"
    Remove-Item $linuxPath -Force -ErrorAction SilentlyContinue
    $linuxOut = & $Yoyo link --target=linux $tyPath $linuxPath 2>&1 | Out-String
    if ($LASTEXITCODE -ne 0 -or -not (Test-Path $linuxPath)) { throw "Rust link --target=linux contrast failed" }
    if ($linuxOut -notmatch '(\d+) code bytes') { throw "cannot parse linux code size" }
    $linuxCodeN = [int]$Matches[1]
    $linuxImg = [System.IO.File]::ReadAllBytes($linuxPath)
    $linuxBody = $linuxImg[(0x1000 + 13)..(0x1000 + 13 + $linuxCodeN - 2)]
    $linuxHex = Hex-Bytes $linuxBody
    if ($linuxHex -notmatch '0f05') { throw "linux ALLOC contrast missing syscall — production path collapsed to stub?" }
    if ($linuxBody[0] -eq 0x48 -and $linuxBody[1] -eq 0xB8) {
        throw "linux ALLOC looks like movabs stub — Win/Linux production collapsed"
    }
    Write-Host "CONTRAST linux ALLOC has syscall (not stub) OK"

    # Extra unknown names → JS/asm stub (thicken beyond freebsd sample).
    foreach ($unk in @("apple", "android")) {
        $jsHex = & node -e @"
const { encodeIoOp, isMovabsStoreStub } = require('./yoyo-js/src/platform/platform-io');
const b = Buffer.from(encodeIoOp(0x20, [0x50, 0x1000], '$unk'));
if (!isMovabsStoreStub(b) || b.length !== 17) process.exit(2);
process.stdout.write(b.toString('hex'));
"@
        if ($LASTEXITCODE -ne 0) { throw "JS unknown $unk must emit stub" }
        $asmHex = & python -c @"
import sys
sys.path.insert(0, r'$AsmDir')
from platform_io import encode_io_op, is_movabs_store_stub
b = encode_io_op(0x20, [0x50, 0x1000], '$unk')
assert is_movabs_store_stub(b) and len(b) == 17
sys.stdout.write(b.hex())
"@
        if ($LASTEXITCODE -ne 0) { throw "asm unknown $unk must emit stub" }
        if ($jsHex -ne $asmHex) { throw "unknown $unk JS≠asm" }
        Write-Host "UNKNOWN→STUB $unk ALLOC JS=asm"
    }

    Write-Host "stub-OS honesty: freebsd/haiku three-peer EQUAL; plan9/serenity forks pinned; linux contrast OK"
}
if ($Failed) {
    Write-Summary "FAILED"
    Write-Host "Stage 13-B: RED" -ForegroundColor Red
    exit 1
}

# --- 3. Seed/link host both platforms (Stage 13-A keep-green; forbid SkipLinux) ---
$null = Invoke-Gate "stage13-link-host.ps1 (Win+Linux)" {
    & (Join-Path $Root "scripts\stage13-link-host.ps1") -SkipBuild
}
if ($Failed) {
    Write-Summary "FAILED"
    Write-Host "Stage 13-B: RED" -ForegroundColor Red
    exit 1
}

# --- 4. Pure M4 both platforms ---
$null = Invoke-Gate "stage9-pure-m4.ps1 (Win)" {
    & (Join-Path $Root "scripts\stage9-pure-m4.ps1") -SkipBuild
}
if ($Failed) {
    Write-Summary "FAILED"
    Write-Host "Stage 13-B: RED" -ForegroundColor Red
    exit 1
}

if ($SkipWsl) {
    Write-Host "`n======== stage10-linux-pure-m4 SKIPPED (-SkipWsl) ========" -ForegroundColor Yellow
    Write-Host "Stage 13-B: RED (SkipWsl forbidden for graduation — one platform blind)" -ForegroundColor Red
    $ExitTable.Add(("{0,-55} {1}" -f "stage10-linux-pure-m4.sh", "SKIP-FORBIDDEN")) | Out-Null
    Write-Summary "FAILED (SkipWsl = platform-blind)"
    exit 1
}

$null = Invoke-Gate "stage10-linux-pure-m4.sh (WSL)" {
    $wslScript = "/mnt/f/yoyo/scripts/stage10-linux-pure-m4.sh"
    # WSL may emit non-fatal stderr (e.g. systemd user session); keep exit-code truth.
    $prevEap = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    try {
        & wsl -e bash $wslScript
    } finally {
        $ErrorActionPreference = $prevEap
    }
}
if ($Failed) {
    Write-Summary "FAILED"
    Write-Host "Stage 13-B: RED" -ForegroundColor Red
    exit 1
}

Write-Summary "ALL_GREEN"
Write-Host ""
Write-Host "Trust chain: Win+Linux three-peer I/O + seed/link + pure M4 under one fail-closed gate;"
Write-Host "  stub OS (FreeBSD/Haiku/Plan9/Serenity) honesty-pinned — cannot invent production I/O silently."
Write-Host "Still honest forks: stub OS remain stub (not real OS I/O); Rust runtime + LoadLibrary/libdl;"
Write-Host "  full .text peer may DIFF; macOS not required."
Write-Host "Stage 13-B: GREEN (may check [x])"
exit 0
