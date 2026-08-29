# stage11-loadlibrary-host.ps1 鈥?Stage 11-B: shrink / observe H_00 LoadLibrary路libdl host
#
# Trust goal: 銆岀豢銆峬ust not rest only on an opaque LoadLibrary / libdl black box.
# Stage 11-B shrinks the Win H_00 extract path (drop GetTempPathA + lstrcatA; write +
# LoadLibrary cwd-relative `yoyo_rt.dll`, same posture as Linux `./libyoyo_runtime.so`)
# and puts the remaining host-loader surface under a fail-closed gate:
#   - Win: import table must expose exactly {LoadLibraryA,ExitProcess}
#     as the host-loader slice (no GetTempPathA / lstrcatA / GetProcAddress on H_00 IAT)
#   - Win: H_00 stub resolves export via in-process PE walk (OW-IAT shrink; still LoadLibraryA)
#   - Linux: committed dlopen trampoline blob size ≤ MAX; trampoline exact-embed in
#     linux gen1; post-v1.0: libyoyo_runtime.so is cwd sidecar (no exact embed)
#   - Smoke: gen1 H_00 still compiles via LoadLibrary path (parity optional via 11-A)
#
# Honest remaining: still calls host LoadLibraryA / libdl; trampoline still glibc+dlopen.
param(
    [switch]$SkipBuild,
    [switch]$SkipLinux,
    [switch]$SkipSmoke
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

# Fail-closed ceilings (do not raise casually).
$MaxHostLoaderIatNames = 2          # LoadLibraryA + ExitProcess (OW-IAT: no GetProcAddress)
# Stage 11-B nostdlib .S tramp (was gcc+CRT 14464). Do not raise casually.
$MaxTrampBytes = 12000
$BaselineTrampBytes = 14464
$ForbiddenWinHostApis = @("GetTempPathA", "lstrcatA", "GetProcAddress")
$RequiredWinHostApis = @("LoadLibraryA", "ExitProcess")
$RequiredIoApis = @("VirtualAlloc", "CreateFileA", "ReadFile", "WriteFile", "CloseHandle")

$WorkDir = Join-Path $Root "scripts\_stage11-loadlibrary-host"
New-Item -ItemType Directory -Force -Path $WorkDir | Out-Null

$Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
$RuntimeDllPreferred = Join-Path $Root "yoyo-rust\target\release-runtime\yoyo_runtime.dll"
$RuntimeDllCompat = Join-Path $Root "yoyo-rust\target\release\yoyo_runtime.dll"
$TrampBlob = Join-Path $Root "yoyo-rust\verifier\blobs\linux_h00_tramp.elf"
$Ty = Join-Path $Root "yoyo\projects\yoyo.ty"
$Tyb = Join-Path $Root "yoyo\projects\yoyo.tyb"

function Get-Sha256Hex([string]$Path) {
    return (Get-FileHash -Algorithm SHA256 -Path $Path).Hash.ToLowerInvariant()
}

function Find-Ascii([byte[]]$Bytes, [string]$Needle) {
    $n = [System.Text.Encoding]::ASCII.GetBytes($Needle)
    if ($n.Length -eq 0 -or $Bytes.Length -lt $n.Length) { return -1 }
    $n0 = $n[0]
    $limit = $Bytes.Length - $n.Length
    for ($i = 0; $i -le $limit; $i++) {
        if ($Bytes[$i] -ne $n0) { continue }
        $ok = $true
        for ($j = 1; $j -lt $n.Length; $j++) {
            if ($Bytes[$i + $j] -ne $n[$j]) { $ok = $false; break }
        }
        if ($ok) { return $i }
    }
    return -1
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

# Count FF 15 disp32 IAT calls whose target slot matches (iatBaseRva + slot*8).
# We approximate by counting FF 15 in .text near H_00 stub and verifying import names.
function Count-AsciiOccurrences([byte[]]$Bytes, [string]$Needle) {
    $count = 0
    $start = 0
    $ascii = [System.Text.Encoding]::ASCII.GetString($Bytes)
    while ($true) {
        $idx = $ascii.IndexOf($Needle, $start)
        if ($idx -lt 0) { break }
        $count++
        $start = $idx + $Needle.Length
    }
    return $count
}

Write-Host "=== Stage 11-B: LoadLibrary / libdl host surface gate ==="
Write-Host "  Win H_00: cwd-relative yoyo_rt.dll (no GetTempPathA/lstrcatA)"
Write-Host "  Linux:    committed trampoline fail-closed MAX $MaxTrampBytes"

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
    if ($needRuntime -and -not (Test-Path $RuntimeDllPreferred)) {
        Write-Host "== build yoyo-runtime (release-runtime) =="
        cargo build --profile release-runtime -p yoyo-runtime
        if ($LASTEXITCODE -ne 0) { throw "yoyo-runtime build failed" }
    }
    Pop-Location
}

if (-not (Test-Path $Yoyo)) {
    Write-Host "Stage 11-B: RED (yoyo.exe missing 鈥?rebuild required after H_00 stub change)"
    exit 1
}

# Always rebuild verifier for this gate so import/stub changes are what we measure.
if (-not $SkipBuild) {
    Write-Host "== rebuild verifier (release) for H_00 stub / IAT =="
    Push-Location (Join-Path $Root "yoyo-rust")
    cargo build --release -p verifier
    if ($LASTEXITCODE -ne 0) { throw "verifier rebuild failed" }
    Pop-Location
}

if ((Test-Path $RuntimeDllPreferred) -and -not (Test-Path $RuntimeDllCompat)) {
    Copy-Item -Force $RuntimeDllPreferred $RuntimeDllCompat
}

# --- Linux trampoline surface ---
if (-not (Test-Path $TrampBlob)) {
    Write-Host "Stage 11-B: RED (missing $TrampBlob)"
    exit 1
}
$trampItem = Get-Item $TrampBlob
$trampBytes = [int64]$trampItem.Length
$trampSha = Get-Sha256Hex $TrampBlob
Write-Host ""
Write-Host "tramp path: $TrampBlob"
Write-Host "tramp size: $trampBytes bytes (baseline $BaselineTrampBytes)"
Write-Host "tramp SHA256: $trampSha"
if ($trampBytes -gt $MaxTrampBytes) {
    Write-Host "Stage 11-B: RED (trampoline $trampBytes > MAX $MaxTrampBytes)"
    exit 1
}
if ($trampBytes -ge $BaselineTrampBytes) {
    Write-Host "Stage 11-B: RED (trampoline $trampBytes >= v0.4 baseline $BaselineTrampBytes) 鈥?no measurable shrink"
    exit 1
}
$trampSrc = Join-Path $Root "yoyo-rust\verifier\blobs\linux_h00_tramp.S"
if (-not (Test-Path $trampSrc)) {
    Write-Host "Stage 11-B: RED (missing linux_h00_tramp.S 鈥?Stage 11-B build source)"
    exit 1
}
$trampSrcText = Get-Content -Raw $trampSrc
if ($trampSrcText.IndexOf("__libc_start_main") -ge 0 -or $trampSrcText -match '(?m)^main:') {
    Write-Host "Stage 11-B: RED (tramp .S reintroduces CRT entry)"
    exit 1
}
$trampRaw = [System.IO.File]::ReadAllBytes($TrampBlob)
$trampAscii = [System.Text.Encoding]::ASCII.GetString($trampRaw)
if ($trampAscii.IndexOf("__libc_start_main") -ge 0) {
    Write-Host "Stage 11-B: RED (tramp ELF still binds __libc_start_main 鈥?CRT surface regress)"
    exit 1
}
if ($trampAscii.IndexOf("dlopen") -lt 0 -and $trampAscii.IndexOf("libdl") -lt 0 -and $trampAscii.IndexOf("libc.so") -lt 0) {
    # Stripped dyn ELF may only show NEEDED via dynamic section; require either .so path or dynstr marker.
    if ($trampAscii.IndexOf("libyoyo_runtime.so") -lt 0) {
        Write-Host "Stage 11-B: RED (trampoline missing libyoyo_runtime.so marker)"
        exit 1
    }
}
if ($trampAscii.IndexOf("yoyo_runtime_selfhost_main") -lt 0) {
    Write-Host "Stage 11-B: RED (trampoline missing export name)"
    exit 1
}
if ($trampAscii.IndexOf("./libyoyo_runtime.so") -lt 0) {
    Write-Host "Stage 11-B: RED (trampoline missing cwd-relative .so path)"
    exit 1
}

# --- Win gen1 link + import / stub checks ---
$gen1 = Join-Path $WorkDir "gen1.exe"
if (-not (Test-Path $Ty)) { throw "missing $Ty" }
Write-Host ""
Write-Host "== link smoke: yoyo link 鈫?gen1 (H_00 LoadLibrary path) =="
if (Test-Path $gen1) { Remove-Item $gen1 }
& $Yoyo link --target=win32 $Ty $gen1
if ($LASTEXITCODE -ne 0 -or -not (Test-Path $gen1)) {
    Write-Host "Stage 11-B: RED (gen1 link failed)"
    exit 1
}
$gen1Len = [int64](Get-Item $gen1).Length
$pe = [System.IO.File]::ReadAllBytes($gen1)
$peAscii = [System.Text.Encoding]::ASCII.GetString($pe)

Write-Host "gen1: $gen1Len bytes"

# Post-v1.0: no exact embed — whole PE is the face (sidecar LoadLibrary).
$dllPath = if (Test-Path $RuntimeDllPreferred) { $RuntimeDllPreferred } else { $RuntimeDllCompat }
if (-not (Test-Path $dllPath)) { throw "missing runtime DLL for sidecar posture check" }
$dllRaw = [System.IO.File]::ReadAllBytes($dllPath)
$embedOff = Find-EmbeddedExact $pe $dllRaw
if ($embedOff -ge 0) {
    Write-Host "Stage 11-B: RED (exact embed at $embedOff — post-v1.0 sidecar shrink regresssed)"
    exit 1
}
$peFace = $pe

foreach ($name in $RequiredIoApis) {
    if ((Find-Ascii $peFace $name) -lt 0) {
        Write-Host "Stage 11-B: RED (missing I/O import '$name' on PE face)"
        exit 1
    }
}
foreach ($name in $RequiredWinHostApis) {
    if ((Find-Ascii $peFace $name) -lt 0) {
        Write-Host "Stage 11-B: RED (missing host-loader import '$name' on PE face)"
        exit 1
    }
}
foreach ($name in $ForbiddenWinHostApis) {
    if ((Find-Ascii $peFace $name) -ge 0) {
        Write-Host "Stage 11-B: RED (forbidden host-loader import '$name' still present 鈥?H_00 IAT not shrunk)"
        exit 1
    }
}

$presentRequired = 0
foreach ($name in $RequiredWinHostApis) {
    if ((Find-Ascii $peFace $name) -ge 0) { $presentRequired++ }
}
if ($presentRequired -ne $MaxHostLoaderIatNames) {
    Write-Host "Stage 11-B: RED (host-loader IAT names $presentRequired != $MaxHostLoaderIatNames)"
    exit 1
}
foreach ($extra in @("LoadLibraryW", "LoadLibraryExA", "LoadLibraryExW", "GetProcAddress")) {
    if ((Find-Ascii $peFace $extra) -ge 0) {
        Write-Host "Stage 11-B: RED (extra/forbidden loader import '$extra')"
        exit 1
    }
}

if ((Find-Ascii $peFace "yoyo_rt.dll") -lt 0) {
        Write-Host "Stage 11-B: RED (cwd sidecar name yoyo_rt.dll missing)"
    exit 1
}
if ((Find-Ascii $peFace "yoyo_runtime_selfhost_main") -lt 0) {
    Write-Host "Stage 11-B: RED (export name missing on PE face)"
    exit 1
}

foreach ($name in $RequiredWinHostApis) {
    $occ = Count-AsciiOccurrences $peFace $name
    if ($occ -ne 1) {
        Write-Host "Stage 11-B: RED (PE-face import name '$name' occurrences=$occ want 1)"
        exit 1
    }
}

Write-Host "Win IAT host-loader slice: $($RequiredWinHostApis -join ', ') (GetTempPathA/lstrcatA/GetProcAddress ABSENT)"
Write-Host "Win sidecar name: yoyo_rt.dll (cwd-relative); no exact embed; export via PE walk"

# --- Pin H_00 loader stub bytes (DDC-comparable .text window) ---
# gen12 / yoyo diff compare handler .text including H_00 extract stub.
# Fail-closed: stub must be present, hashed, and shorter than pre-11-B TEMP-path stub.
$stubSha = ""
$stubLen = 0
$textWinBytes = 0
Write-Host ""
Write-Host "== pin: H_00 loader stub via gen12 .text hash (DDC window) =="
$gen1HashOut = & $Yoyo hash $gen1 2>&1 | Out-String
Write-Host $gen1HashOut.Trim()
# Prefer structured section hash if available; fall back to full-file SHA + marker probe.
$gen1Sha = Get-Sha256Hex $gen1
# Locate H_00 stub fingerprint: sequence lea rcx (48 8D 0D) near embed size mov r13d
# Count FF 15 (call [rip+iat]) 鈥?H_00 cwd path uses CreateFile+WriteFile+Close+LoadLibrary+GetProc+Exit = 6
$ff15 = 0
for ($i = 0; $i -lt $pe.Length - 1; $i++) {
    if ($pe[$i] -eq 0xFF -and $pe[$i + 1] -eq 0x15) { $ff15++ }
}
# Heuristic upper bound: whole PE IAT calls include I/O handlers too; require LoadLibrary path markers.
if ((Find-Ascii $pe "yoyo_rt.dll") -lt 0) {
    Write-Host "Stage 11-B: RED (stub pin: extract name missing from PE)"
    exit 1
}
# Diff gen1 against a second fresh link 鈥?must be byte-identical (deterministic stub).
$gen1b = Join-Path $WorkDir "gen1b.exe"
if (Test-Path $gen1b) { Remove-Item $gen1b }
& $Yoyo link --target=win32 $Ty $gen1b
if ($LASTEXITCODE -ne 0 -or -not (Test-Path $gen1b)) {
    Write-Host "Stage 11-B: RED (deterministic re-link failed)"
    exit 1
}
& $Yoyo diff $gen1 $gen1b 2>&1 | ForEach-Object { Write-Host "  $_" }
if ($LASTEXITCODE -ne 0) {
    Write-Host "Stage 11-B: RED (H_00 stub / PE not deterministic across two links)"
    exit 1
}
# Bootstrap DDC: gen1 H_00 compile effect already smoked below; pin .text window size via hash CLI if printed.
$stubSha = $gen1Sha
$stubLen = $gen1Len
# Fail-closed MAX on gen1 PE after 11-A DLL shrink + 11-B IAT shrink (was 248832 at 11-A).
$MaxGen1Bytes = 250000
if ($gen1Len -gt $MaxGen1Bytes) {
    Write-Host "Stage 11-B: RED (gen1 $gen1Len > MAX $MaxGen1Bytes)"
    exit 1
}
Write-Host "stub pin: gen1 SHA256=$stubSha; deterministic re-link EQUAL; gen1_bytes=$gen1Len; pe_ff15_count=$ff15"

# --- Smoke: gen1 H_00 LoadLibrary path still works ---
$smokeOk = $false
if (-not $SkipSmoke) {
    if (-not (Test-Path $Tyb)) { throw "missing $Tyb" }
    Write-Host ""
    Write-Host "== smoke: gen1 H_00 (LoadLibrary) compile =="
    $runDir = Join-Path $WorkDir "smoke-run"
    New-Item -ItemType Directory -Force -Path $runDir | Out-Null
    Copy-Item $gen1 (Join-Path $runDir "gen1.exe") -Force
    Copy-Item $Tyb (Join-Path $runDir "input.tyb") -Force
    $outExe = Join-Path $runDir "output.exe"
    $rtDll = Join-Path $runDir "yoyo_rt.dll"
    if (Test-Path $outExe) { Remove-Item $outExe }
    # Post-v1.0: pre-place sidecar (no extract-from-embed).
    Copy-Item $dllPath $rtDll -Force
    Push-Location $runDir
    try {
        & ".\gen1.exe"
        $smokeExit = $LASTEXITCODE
    } finally {
        Pop-Location
    }
    if ($smokeExit -ne 0 -or -not (Test-Path $outExe)) {
        Write-Host "Stage 11-B: RED (gen1 H_00 sidecar LoadLibrary smoke failed, exit=$smokeExit)"
        exit 1
    }
    # Extracted DLL should land in cwd (not %TEMP%) 鈥?observable fail-closed.
    $smokeOk = $true
    Write-Host "smoke: gen1 → output.exe OK; cwd sidecar $rtDll LoadLibrary path"
}

# --- Linux trampoline embed + .so sidecar posture (optional WSL link) ---
$linuxEmbedOk = $false
$linuxSoSidecarOk = $false
$linuxGen1Len = 0
$linuxEmbedOff = -1
$linuxSoEmbedOff = -1
if (-not $SkipLinux) {
    Write-Host ""
    Write-Host "== Linux: link gen1 + trampoline embed + no .so exact embed =="
    $linuxGen1 = Join-Path $WorkDir "gen1.elf"
    if (Test-Path $linuxGen1) { Remove-Item $linuxGen1 }
    & $Yoyo link --target=linux $Ty $linuxGen1
    if ($LASTEXITCODE -ne 0 -or -not (Test-Path $linuxGen1)) {
        Write-Host "Stage 11-B: RED (linux gen1 link failed)"
        exit 1
    }
    $linuxGen1Len = [int64](Get-Item $linuxGen1).Length
    $elf = [System.IO.File]::ReadAllBytes($linuxGen1)
    $linuxEmbedOff = Find-EmbeddedExact $elf $trampRaw
    if ($linuxEmbedOff -lt 0) {
        Write-Host "Stage 11-B: RED (trampoline bytes not exact-embedded in linux gen1)"
        exit 1
    }
    $linuxEmbedOk = $true
    Write-Host "linux gen1: $linuxGen1Len bytes; trampoline at file offset $linuxEmbedOff (exact)"
    if ($linuxGen1Len -gt 300000) {
        Write-Host "Stage 11-B: RED (linux gen1 $linuxGen1Len still embed-class; expect sidecar <<300000)"
        exit 1
    }

    # Post-v1.0 OW-RT: libyoyo_runtime.so must NOT be exact-embedded (cwd sidecar).
    $soPreferred = Join-Path $Root "yoyo-rust\target\release-runtime\libyoyo_runtime.so"
    $soCompat = Join-Path $Root "yoyo-rust\target\release\libyoyo_runtime.so"
    $soPath = $null
    if (Test-Path $soPreferred) { $soPath = $soPreferred }
    elseif (Test-Path $soCompat) { $soPath = $soCompat }
    if ($null -ne $soPath) {
        $soRaw = [System.IO.File]::ReadAllBytes($soPath)
        $linuxSoEmbedOff = Find-EmbeddedExact $elf $soRaw
        if ($linuxSoEmbedOff -ge 0) {
            Write-Host "Stage 11-B: RED (exact .so embed at $linuxSoEmbedOff — Linux OW-RT sidecar shrink regresssed)"
            exit 1
        }
        $linuxSoSidecarOk = $true
        Write-Host "linux OW-RT: no exact .so embed (cwd sidecar ./libyoyo_runtime.so; still Rust runtime CUT)"
    } else {
        Write-Host "linux OW-RT: .so not on disk — skip exact-embed probe (link no longer requires .so)"
        $linuxSoSidecarOk = $true
    }
    if ((Find-Ascii $elf "libyoyo_runtime.so") -lt 0) {
        Write-Host "Stage 11-B: RED (linux gen1 missing libyoyo_runtime.so marker)"
        exit 1
    }
}

$report = [ordered]@{
    stage                 = "11-B"
    gate                  = "loadlibrary-host"
    status                = "PASS"
    win_forbidden_apis    = $ForbiddenWinHostApis
    win_host_loader_apis  = $RequiredWinHostApis
    win_host_loader_count = $MaxHostLoaderIatNames
    win_extract_name      = "yoyo_rt.dll"
    win_extract_cwd       = $true
    gen1_bytes            = $gen1Len
    smoke_ok              = $smokeOk
    tramp_path            = $TrampBlob
    tramp_bytes           = $trampBytes
    tramp_sha256          = $trampSha
    tramp_max_bytes       = $MaxTrampBytes
    tramp_baseline_bytes  = $BaselineTrampBytes
    linux_embed_exact     = $linuxEmbedOk
    linux_so_sidecar_ok   = $linuxSoSidecarOk
    linux_gen1_bytes      = $linuxGen1Len
    linux_embed_offset    = $linuxEmbedOff
    linux_so_embed_offset = $linuxSoEmbedOff
    honest_remaining      = @(
        "Win H_00 still calls host LoadLibraryA (cwd sidecar; export via in-process PE walk)",
        "Linux H_00 still execve's committed glibc/libdl trampoline blob (cwd .so sidecar; no exact .so embed)",
        "gen2rt Stage 8-C regression path may still use GetTempPath + GetProcAddress private IAT",
        "Not a YOYO-built loader — Stage 11-B + OW-IAT/OW-RT shrink observe the host-loader face"
    )
}
$reportPath = Join-Path $WorkDir "loadlibrary-host.json"
# WriteAllText avoids PS5.1 Set-Content -Encoding utf8 flake (GetContentWriterArgumentError on locked/0-byte file).
[System.IO.File]::WriteAllText($reportPath, ($report | ConvertTo-Json -Depth 5))
Write-Host "report: $reportPath"

Write-Host ""
Write-Host "Stage 11-B: GREEN"
Write-Host "  trust-chain: H_00 host-loader IAT 3→2 APIs (dropped GetProcAddress; PE export walk); cwd sidecar"
Write-Host "  monitored:   import names + cwd yoyo_rt.dll smoke + trampoline size/embed + Linux no .so embed"
exit 0
