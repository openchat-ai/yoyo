# stage15-hole-inventory.ps1 鈥?Stage 15-A: hole inventory CLOSED|CUT
#
# Trust goal: turn v0.8 OW-* / RELEASE remaining surfaces from a lump
# SCOPE-CUT into a per-hole machine disposition (CLOSED or CUT with pins).
# Must NOT pretend full .text EQUAL. Must NOT mark CLOSED without evidence.
#
# Fail-closed:
#   1. stage14-outside-window-scope-cut.ps1 -SkipBuild (no-regress Stage 14-A)
#   2. Per-hole HOLE id=... disposition=CLOSED|CUT evidence=...
#   3. HOLE_INVENTORY status=... summary line
# Honest: DDC = detection; Rust runtime + LoadLibrary + Rust seed remain CUT.
# Post-v1.0 OW-SEED: pin emitter identity + seed sha256_prefix + path=h00 (still CUT).
param(
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

# Post-v1.0 OW-RT sidecar + OW-IAT manual-map wire-up (PR #8): stub ~905B; DLL ≤150000.
$MinStubTailNonzero = 40
$MaxStubTailNonzero = 2300
$MaxDllBytes = 150000
$MaxSeedPeBytes = 270000
$MinBodyCompared = 17013

$WorkDir = Join-Path $Root "scripts\_stage15-hole-inventory"
New-Item -ItemType Directory -Force -Path $WorkDir | Out-Null

$Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
$RuntimeDllPreferred = Join-Path $Root "yoyo-rust\target\release-runtime\yoyo_runtime.dll"
$RuntimeDllCompat = Join-Path $Root "yoyo-rust\target\release\yoyo_runtime.dll"
$Ty = Join-Path $Root "yoyo\projects\yoyo.ty"
$ScopeCutDoc = Join-Path $Root "SCOPE-CUT-v0.9-hole-inventory.md"
$Stage13Parity = Join-Path $Root "scripts\stage13-cross-platform-parity.ps1"
$Stamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"

$HoleRows = [System.Collections.Generic.List[string]]::new()
$ClosedCount = 0
$CutCount = 0

function Resolve-RuntimeDll {
    if (Test-Path $RuntimeDllPreferred) { return $RuntimeDllPreferred }
    if (Test-Path $RuntimeDllCompat) { return $RuntimeDllCompat }
    return $RuntimeDllPreferred
}

function Find-Ascii([byte[]]$Bytes, [string]$Needle) {
    return [System.Text.Encoding]::ASCII.GetString($Bytes).Contains($Needle)
}

function Find-EmbeddedExact([byte[]]$Hay, [byte[]]$Needle) {
    if ($Needle.Length -lt 16 -or $Hay.Length -lt $Needle.Length) { return -1 }
    $n0 = $Needle[0]; $n1 = $Needle[1]
    $limit = $Hay.Length - $Needle.Length
    for ($i = 0; $i -le $limit; $i++) {
        if ($Hay[$i] -ne $n0 -or $Hay[$i + 1] -ne $n1) { continue }
        $ok = $true
        for ($j = 0; $j -lt $Needle.Length; $j++) {
            if ($Hay[$i + $j] -ne $Needle[$j]) { $ok = $false; break }
        }
        if ($ok) { return $i }
    }
    return -1
}

function Get-PeTextSection([byte[]]$Pe) {
    if ($Pe.Length -lt 0x200 -or $Pe[0] -ne 0x4D -or $Pe[1] -ne 0x5A) { return $Pe }
    $lfanew = [BitConverter]::ToInt32($Pe, 0x3C)
    if ($lfanew + 0x180 -gt $Pe.Length) { return $Pe }
    $soh = [BitConverter]::ToInt16($Pe, $lfanew + 0x14)
    $sec = $lfanew + 0x18 + $soh
    if ($sec + 40 -gt $Pe.Length) { return $Pe }
    $vs = [BitConverter]::ToInt32($Pe, $sec + 8)
    $rawSz = [BitConverter]::ToInt32($Pe, $sec + 16)
    $rawPtr = [BitConverter]::ToInt32($Pe, $sec + 20)
    $n = [Math]::Min($vs, [Math]::Min($rawSz, $Pe.Length - $rawPtr))
    if ($n -le 0) { return @() }
    $text = New-Object byte[] $n
    [Array]::Copy($Pe, $rawPtr, $text, 0, $n)
    return ,$text
}

function Add-Hole([string]$Id, [string]$Disposition, [string]$Evidence) {
    if ($Disposition -ne "CLOSED" -and $Disposition -ne "CUT") {
        throw ("invalid disposition for {0} : {1}" -f $Id, $Disposition)
    }
    $line = "HOLE id={0} disposition={1} evidence={2}" -f $Id, $Disposition, $Evidence
    $HoleRows.Add($line) | Out-Null
    Write-Host $line
    if ($Disposition -eq "CLOSED") { $script:ClosedCount++ } else { $script:CutCount++ }
}

function Write-Inventory([string]$StatusLine) {
    $path = Join-Path $WorkDir "INVENTORY.txt"
    $lines = [System.Collections.Generic.List[string]]::new()
    $lines.Add("Stage 15-A hole inventory $Stamp") | Out-Null
    $lines.Add("Doc: SCOPE-CUT-v0.9-hole-inventory.md") | Out-Null
    $lines.Add("") | Out-Null
    foreach ($row in $HoleRows) { $lines.Add($row) | Out-Null }
    $lines.Add("") | Out-Null
    $lines.Add($StatusLine) | Out-Null
    $lines.Add("Honest: NOT full .text EQUAL graduation; CLOSED only with fail-closed evidence.") | Out-Null
    $lines | Set-Content -Encoding utf8 $path
}

function Fail-Out([string]$Why) {
    Write-Host ("Stage 15-A: RED ({0})" -f $Why) -ForegroundColor Red
    Write-Inventory ("FAILED ({0})" -f $Why)
    if ($script:LockPath -and (Test-Path -LiteralPath $script:LockPath)) {
        Remove-Item -LiteralPath $script:LockPath -Force -ErrorAction SilentlyContinue
    }
    exit 1
}

Write-Host "=== Stage 15-A: hole inventory CLOSED|CUT ==="
Write-Host ("  stamp: {0}" -f $Stamp)
Write-Host "  rule: per-hole disposition; never pretend full .text EQUAL"

if (-not (Test-Path $ScopeCutDoc)) {
    Fail-Out "missing SCOPE-CUT-v0.9-hole-inventory.md"
}
if (-not (Test-Path $Stage13Parity)) {
    Fail-Out "missing stage13-cross-platform-parity.ps1 (REL-STUBOS pin source)"
}

# Concurrent guard when nesting Stage 14-A / shared workdirs.
$LockPath = Join-Path $WorkDir "driver.lock"
$script:LockPath = $LockPath
if (Test-Path $LockPath) {
    $lockAgeMin = ((Get-Date) - (Get-Item -LiteralPath $LockPath).LastWriteTime).TotalMinutes
    if ($lockAgeMin -lt 120) {
        $lockBody = (Get-Content -LiteralPath $LockPath -Raw -ErrorAction SilentlyContinue)
        Fail-Out ("concurrent driver.lock age={0:N1}m -- {1}" -f $lockAgeMin, $lockBody.Trim())
    }
    Remove-Item -LiteralPath $LockPath -Force -ErrorAction SilentlyContinue
}
("{0}`npid={1}" -f $Stamp, $PID) | Set-Content -LiteralPath $LockPath -Encoding ascii

# Wait for zero cargo before optional build / nested gates.
$deadline = (Get-Date).AddMinutes(45)
Write-Host "== wait for zero cargo/rustc =="
while ($true) {
    $procs = @(Get-Process -Name cargo, rustc -ErrorAction SilentlyContinue)
    if ($procs.Count -eq 0) { break }
    if ((Get-Date) -gt $deadline) {
        Fail-Out "cargo/rustc still running after wait"
    }
    Write-Host ("  waiting on {0} cargo/rustc..." -f $procs.Count)
    Start-Sleep -Seconds 5
}

if (-not (Test-Path $Yoyo) -or -not $SkipBuild) {
    Write-Host "== one cargo: build --release -p verifier =="
    Push-Location (Join-Path $Root "yoyo-rust")
    $prevEap = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    & cargo build --release -p verifier
    $buildEc = $LASTEXITCODE
    $ErrorActionPreference = $prevEap
    Pop-Location
    if ($buildEc -ne 0) { Fail-Out ("verifier build failed (exit {0})" -f $buildEc) }
} else {
    Write-Host "== cargo build SKIP (SkipBuild; yoyo.exe present) =="
}
if (-not (Test-Path $Yoyo)) { Fail-Out "missing yoyo.exe" }
if (-not (Test-Path $Ty)) { Fail-Out "missing yoyo.ty" }

$RuntimeDll = Resolve-RuntimeDll
if (-not (Test-Path $RuntimeDll)) { Fail-Out "missing yoyo_runtime.dll" }
$dllSize = (Get-Item $RuntimeDll).Length
Write-Host ("runtime.dll: {0} bytes (MAX {1})" -f $dllSize, $MaxDllBytes)
if ($dllSize -gt $MaxDllBytes) {
    Fail-Out ("OW-RT runtime.dll {0} exceeds MAX {1}" -f $dllSize, $MaxDllBytes)
}

Write-Host ""
Write-Host "== no-regress Stage 14-A: stage14-outside-window-scope-cut SkipBuild =="
# PS5.1: named switch only -- never @("-SkipBuild") array splat.
& (Join-Path $PSScriptRoot "stage14-outside-window-scope-cut.ps1") -SkipBuild
if ($LASTEXITCODE -ne 0) {
    Fail-Out ("stage14-A outside-window SCOPE-CUT failed exit {0}" -f $LASTEXITCODE)
}

Write-Host ""
Write-Host "== Stage 15-A inventory peers (JS vs Rust full yoyo.ty) =="
$jsOut = Join-Path $WorkDir "M_js.exe"
$rustOut = Join-Path $WorkDir "M_rust.exe"

Write-Host "  JS..."
& node (Join-Path $Root "yoyo-js\src\yoyo.js") $Ty $jsOut
if ($LASTEXITCODE -ne 0 -or -not (Test-Path $jsOut)) { Fail-Out "JS peer build failed" }

Write-Host "  Rust..."
$prevEapLink = $ErrorActionPreference
$ErrorActionPreference = "Continue"
$linkOut = & $Yoyo link --target=win32 $Ty $rustOut 2>&1 | Out-String
$linkEc = $LASTEXITCODE
$ErrorActionPreference = $prevEapLink
Write-Host $linkOut.TrimEnd()
if ($linkEc -ne 0 -or -not (Test-Path $rustOut)) { Fail-Out "Rust peer build failed" }

$seedPe = (Get-Item $rustOut).Length
Write-Host ("Rust seed PE: {0} bytes (MAX {1})" -f $seedPe, $MaxSeedPeBytes)
if ($seedPe -gt $MaxSeedPeBytes) {
    Fail-Out ("seed PE {0} exceeds MAX {1}" -f $seedPe, $MaxSeedPeBytes)
}

# Post-v1.0 OW-SEED: fail-closed pin emitter identity + seed hash (still CUT — Rust emits).
$emitterName = [System.IO.Path]::GetFileName($Yoyo)
if ($emitterName -ne "yoyo.exe") {
    Fail-Out ("OW-SEED emitter basename {0} != yoyo.exe" -f $emitterName)
}
$emitterBytes = (Get-Item -LiteralPath $Yoyo).Length
$emitterSha = (Get-FileHash -LiteralPath $Yoyo -Algorithm SHA256).Hash.ToLowerInvariant()
$emitterShaPrefix = $emitterSha.Substring(0, [Math]::Min(16, $emitterSha.Length))
$seedSha = (Get-FileHash -LiteralPath $rustOut -Algorithm SHA256).Hash.ToLowerInvariant()
$seedShaPrefix = $seedSha.Substring(0, [Math]::Min(16, $seedSha.Length))
if ($linkOut -notmatch 'SEED_HOST cmd=link target=win32 path=h00\b') {
    Fail-Out "OW-SEED missing SEED_HOST cmd=link target=win32 path=h00"
}
if ($linkOut -notmatch 'SEED_HOST cmd=link target=win32 path=h00 bytes=(\d+) dll_embed=\S+ sha256_prefix=([0-9a-fA-F]+)') {
    Fail-Out "OW-SEED SEED_HOST line missing bytes/sha256_prefix fields"
}
$obsSeedBytes = [int64]$Matches[1]
$obsSeedShaPrefix = $Matches[2].ToLowerInvariant()
if ($obsSeedBytes -ne $seedPe) {
    Fail-Out ("OW-SEED SEED_HOST bytes={0} != on-disk seed_pe={1}" -f $obsSeedBytes, $seedPe)
}
if (-not $seedSha.StartsWith($obsSeedShaPrefix)) {
    Fail-Out ("OW-SEED SEED_HOST sha256_prefix={0} != file sha {1}..." -f $obsSeedShaPrefix, $seedShaPrefix)
}
Write-Host ("OW-SEED emitter={0} bytes={1} sha256_prefix={2}" -f $emitterName, $emitterBytes, $emitterShaPrefix)
Write-Host ("OW-SEED seed_pe={0} sha256_prefix={1} path=h00 (SEED_HOST matched)" -f $seedPe, $seedShaPrefix)

$rustBytes = [System.IO.File]::ReadAllBytes($rustOut)
$dllBytes = [System.IO.File]::ReadAllBytes($RuntimeDll)
$embedOff = Find-EmbeddedExact $rustBytes $dllBytes
Write-Host ("OW-RT exact embed offset: {0}" -f $embedOff)

$hasLoadLibrary = Find-Ascii $rustBytes "LoadLibraryA"
$hasYoyoRt = Find-Ascii $rustBytes "yoyo_rt.dll"
# Deeper OW-IAT: LoadLibraryA ASCII/IAT absent; sidecar marker still required while CUT.
if ($hasLoadLibrary) {
    Fail-Out "OW-IAT LoadLibraryA still on seed PE (deeper shrink requires PEB resolve, no IAT/ASCII)"
}
if (-not $hasYoyoRt) {
    Fail-Out "OW-IAT yoyo_rt.dll marker missing (sidecar host-load face)"
}

Write-Host ""
Write-Host "== body window + stub pin =="
$bodyLines = & $Yoyo diff --selfhost-body $jsOut $rustOut 2>&1
$bodyEc = $LASTEXITCODE
$bodyLines | ForEach-Object { Write-Host ("  {0}" -f $_) }
if ($bodyEc -ne 0) { Fail-Out "selfhost-body DIFF -- comparable window must stay EQUAL" }

$bodyCompared = $null
$stubNz = $null
foreach ($line in $bodyLines) {
    if ("$line" -match 'compared_bytes:\s*(\d+)') { $bodyCompared = [int]$Matches[1] }
    if ("$line" -match 'stub_tail_nonzero a=(\d+) b=(\d+)') {
        $stubNz = [Math]::Max([int]$Matches[1], [int]$Matches[2])
    }
}
if ($null -eq $bodyCompared -or $bodyCompared -lt $MinBodyCompared) {
    Fail-Out ("body compared {0} below floor {1}" -f $bodyCompared, $MinBodyCompared)
}
if ($null -eq $stubNz) { Fail-Out "stub_tail_nonzero not parsed" }

Write-Host ""
Write-Host "== honest full .text =="
$prevEap = $ErrorActionPreference
$ErrorActionPreference = "Continue"
$fullLines = & $Yoyo diff $jsOut $rustOut 2>&1
$fullEc = $LASTEXITCODE
$ErrorActionPreference = $prevEap
$fullLines | ForEach-Object { Write-Host ("  {0}" -f $_) }
$fullStatus = if ($fullEc -eq 0) { "EQUAL" } else { "DIFF" }

# OW-H00 slot pin: PE startup 13B + H_00 slot 18B must match JS↔Rust (JMP+NOP aligned).
$H00StartupLen = 13
$H00SlotLen = 18
$jsBytes = [System.IO.File]::ReadAllBytes($jsOut)
$jsText = Get-PeTextSection $jsBytes
$rustText = Get-PeTextSection $rustBytes
if ($jsText.Length -lt ($H00StartupLen + $H00SlotLen) -or $rustText.Length -lt ($H00StartupLen + $H00SlotLen)) {
    Fail-Out "H_00 slot pin: .text too short for startup+slot"
}
$jsSlot = $jsText[$H00StartupLen..($H00StartupLen + $H00SlotLen - 1)]
$rustSlot = $rustText[$H00StartupLen..($H00StartupLen + $H00SlotLen - 1)]
$h00SlotAligned = ($jsSlot.Length -eq $rustSlot.Length)
if ($h00SlotAligned) {
    for ($i = 0; $i -lt $jsSlot.Length; $i++) {
        if ($jsSlot[$i] -ne $rustSlot[$i]) { $h00SlotAligned = $false; break }
    }
}
if (-not $h00SlotAligned) {
    Fail-Out "OW-H00 H_00 entry slot JS/Rust mismatch (expected JMP+NOP 18B aligned)"
}
if ($rustSlot[0] -ne 0xE9) {
    Fail-Out ("OW-H00 Rust H_00 slot missing JMP opcode (got 0x{0:X2})" -f $rustSlot[0])
}
Write-Host ("H_00 entry slot: ALIGNED 18B JMP+NOP (first byte 0x{0:X2})" -f $rustSlot[0])

# REL-STUBOS: stage13 gate source still pins stub OS honesty (not inventing I/O).
$stage13Text = Get-Content -LiteralPath $Stage13Parity -Raw
$stubOsNeedles = @("freebsd", "haiku", "plan9", "serenity", "stub-OS")
foreach ($n in $stubOsNeedles) {
    if ($stage13Text -notmatch [regex]::Escape($n)) {
        Fail-Out ("REL-STUBOS pin missing in stage13 script: {0}" -f $n)
    }
}

Write-Host ""
Write-Host "== per-hole disposition (fail-closed CLOSED) =="

# OW-H00: CLOSED only if full .text EQUAL (slot aligned alone is NOT CLOSED).
if ($fullStatus -eq "EQUAL") {
    Add-Hole "OW-H00" "CLOSED" ("full_text=EQUAL;body_window=EQUAL;compared={0}" -f $bodyCompared)
} else {
    Add-Hole "OW-H00" "CUT" ("full_text=DIFF;H00_slot_18B_ALIGNED;stub_still_DIFF;compared={0}" -f $bodyCompared)
}

# OW-STUB: CLOSED only if stub span gone.
if ($stubNz -eq 0) {
    Add-Hole "OW-STUB" "CLOSED" "stub_tail_nonzero=0"
} elseif ($stubNz -ge $MinStubTailNonzero -and $stubNz -le $MaxStubTailNonzero) {
    Add-Hole "OW-STUB" "CUT" ("stub_tail_nonzero={0};pin={1}..{2}" -f $stubNz, $MinStubTailNonzero, $MaxStubTailNonzero)
} else {
    Fail-Out ("OW-STUB stub_tail_nonzero={0} outside pin and not CLOSED" -f $stubNz)
}

# OW-RT: CLOSED only if no exact embed AND no Rust sidecar surface (yoyo_rt.dll gone).
# Post-v1.0: no-embed alone is NOT CLOSED (sidecar Rust runtime remains CUT).
if ($embedOff -ge 0) {
    Fail-Out ("OW-RT exact embed regress at offset {0} (post-v1.0 requires sidecar-only)" -f $embedOff)
} elseif (-not $hasYoyoRt) {
    Add-Hole "OW-RT" "CLOSED" ("no_exact_embed;no_yoyo_rt_sidecar;dll={0}" -f $dllSize)
} else {
    Add-Hole "OW-RT" "CUT" ("sidecar_rust_runtime;no_exact_embed;dll={0};max={1};still_Rust_runtime" -f $dllSize, $MaxDllBytes)
}

# OW-IAT: CLOSED only if no host DLL load face (no yoyo_rt.dll). LoadLibraryA ASCII absent
# alone is NOT CLOSED (PEB-resolved host LoadLibrary still CUT).
$hasGetProc = Find-Ascii $rustBytes "GetProcAddress"
if ($hasLoadLibrary -or $hasGetProc) {
    Fail-Out "OW-IAT LoadLibraryA/GetProcAddress still on seed PE (post-v1.0 requires PEB + no IAT loader names)"
}
if (-not $hasYoyoRt) {
    Add-Hole "OW-IAT" "CLOSED" "no_yoyo_rt_sidecar;no_IAT_LoadLibraryA"
} else {
    Add-Hole "OW-IAT" "CUT" ("no_IAT_LoadLibraryA;manual_map_wired;kernel32_IO;yoyo_rt.dll_sidecar;still_host_trusted;hasYoyoRt={0}" -f $hasYoyoRt)
}

# OW-SEED: still CUT — Rust yoyo.exe emits seed. Post-v1.0 pins emitter + seed hash + path=h00.
# CLOSED only with a non-Rust emitter evidence path (not invented here).
Add-Hole "OW-SEED" "CUT" ("emitter=Rust_yoyo.exe;emitter_bytes={0};emitter_sha256_prefix={1};seed_pe={2};seed_sha256_prefix={3};path=h00;max={4}" -f `
    $emitterBytes, $emitterShaPrefix, $seedPe, $seedShaPrefix, $MaxSeedPeBytes)

# REL-FULLTEXT: never CLOSED as graduation claim.
if ($fullStatus -eq "DIFF") {
    Add-Hole "REL-FULLTEXT" "CUT" "full_text=DIFF;not_graduation_EQUAL"
} else {
    Add-Hole "REL-FULLTEXT" "CUT" "full_text=EQUAL_observed;PARTIAL_only;OW-RT_IAT_still_cut;not_graduation"
}

# REL-STUBOS: still stub -- CUT.
Add-Hole "REL-STUBOS" "CUT" "stage13_stub_OS_pins=present;not_production_IO"

$expectedIds = @("OW-H00", "OW-STUB", "OW-RT", "OW-IAT", "OW-SEED", "REL-FULLTEXT", "REL-STUBOS")
if ($HoleRows.Count -ne $expectedIds.Count) {
    Fail-Out ("hole row count {0} != {1}" -f $HoleRows.Count, $expectedIds.Count)
}
foreach ($id in $expectedIds) {
    $found = $false
    foreach ($row in $HoleRows) {
        if ($row -match ("HOLE id={0} disposition=" -f [regex]::Escape($id))) { $found = $true; break }
    }
    if (-not $found) { Fail-Out ("missing HOLE row for {0}" -f $id) }
}

# Inventory status: ACTIVE while any CUT remains; CLOSED_ALL only if zero CUT (future).
if ($CutCount -gt 0) {
    $invStatus = "ACTIVE"
} else {
    $invStatus = "CLOSED_ALL"
}
$partialNote = ""
if ($fullStatus -eq "EQUAL" -and $CutCount -gt 0) { $partialNote = " overlay=PARTIAL" }

$threePeerFull = $fullStatus
$statusLine = ("HOLE_INVENTORY status={0} full_text={1} three_peer_full={2} body_window=EQUAL compared={3} stub_nz={4} dll={5} seed_pe={6} seed_sha={7} emitter_sha={8} emitter_bytes={9} embed_off={10} closed={11} cut={12}{13}" -f `
    $invStatus, $fullStatus, $threePeerFull, $bodyCompared, $stubNz, $dllSize, $seedPe, $seedShaPrefix, $emitterShaPrefix, $emitterBytes, $embedOff, $ClosedCount, $CutCount, $partialNote)
Write-Host ""
Write-Host $statusLine
Write-Host ""
Write-Host "Stage 15-A: GREEN -- hole inventory CLOSED|CUT enumerated"
Write-Host "  Doc: SCOPE-CUT-v0.9-hole-inventory.md"
Write-Host ("  CLOSED={0} CUT={1} -- comparable EQUAL: selfhost-body only" -f $ClosedCount, $CutCount)
Write-Inventory $statusLine

Remove-Item -LiteralPath $LockPath -Force -ErrorAction SilentlyContinue
exit 0
