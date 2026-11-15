# Expand-YoyoHi.ps1 — maximal-readable .yoyo → .ty (NO raw/hex). No Node.
param(
    [Parameter(Mandatory = $true)][string]$InputPath,
    [Parameter(Mandatory = $true)][string]$OutputPath
)
$ErrorActionPreference = "Stop"
if (-not (Test-Path -LiteralPath $InputPath)) {
    throw "Expand-YoyoHi missing input: $InputPath"
}

# pe_link embed_string_table_r15 slots (STR_TABLE_OFF=0x10000, ENTRY=64)
$PrintTable = @{
    "hi`n" = @{ Off = 0x100C0; Len = 3 }
    "ok`n" = @{ Off = 0x10100; Len = 3 }
}

function Emit-PrintExitTy([string]$Program, [string]$Text, [int]$ExitCode) {
    if (-not $PrintTable.ContainsKey($Text)) {
        $keys = ($PrintTable.Keys | ForEach-Object { $_.Replace("`n", "\n") }) -join ", "
        throw "Expand-YoyoHi print string not in table. allowed: $keys"
    }
    $meta = $PrintTable[$Text]
    $off = $meta.Off
    $len = $meta.Len
    $b0 = $off -band 0xFF
    $b1 = ($off -shr 8) -band 0xFF
    $b2 = ($off -shr 16) -band 0xFF
    $b3 = ($off -shr 24) -band 0xFF
    $lb0 = $len -band 0xFF
    $lb1 = ($len -shr 8) -band 0xFF
    $lb2 = ($len -shr 16) -band 0xFF
    $lb3 = ($len -shr 24) -band 0xFF
    $eb0 = $ExitCode -band 0xFF
    # ExitProcess path uses xor ecx,ecx for 0; non-zero not supported in 1.3 template
    if ($ExitCode -ne 0) {
        throw "Expand-YoyoHi exit only supports 0 in v1.3"
    }

    $lines = New-Object System.Collections.Generic.List[string]
    $lines.Add("; expanded from .yoyo program=$Program by Expand-YoyoHi.ps1")
    $lines.Add("; MAX_READABLE: print + exit — no raw hex in source")
    $lines.Add("40 00")
    foreach ($h in @(
            "A0 48", "A0 83", "A0 EC", "A0 48", "A0 B9", "A0 F5", "A0 FF", "A0 FF", "A0 FF",
            "A0 41", "A0 FF", "A0 97", "A0 30", "A0 00", "A0 00", "A0 00",
            "A0 48", "A0 89", "A0 C1", "A0 49", "A0 8D", "A0 97",
            ("A0 {0:X2}" -f $b0), ("A0 {0:X2}" -f $b1), ("A0 {0:X2}" -f $b2), ("A0 {0:X2}" -f $b3),
            "A0 41", "A0 B8",
            ("A0 {0:X2}" -f $lb0), ("A0 {0:X2}" -f $lb1), ("A0 {0:X2}" -f $lb2), ("A0 {0:X2}" -f $lb3),
            "A0 4C", "A0 8D", "A0 4C", "A0 24", "A0 20",
            "A0 48", "A0 C7", "A0 44", "A0 24", "A0 20", "A0 00", "A0 00", "A0 00", "A0 00",
            "A0 41", "A0 FF", "A0 97", "A0 18", "A0 00", "A0 00", "A0 00",
            "A0 48", "A0 83", "A0 C4", "A0 48", "A0 31", "A0 C9",
            "A0 48", "A0 83", "A0 EC", "A0 20",
            "A0 41", "A0 FF", "A0 97", "A0 28", "A0 00", "A0 00", "A0 00"
        )) {
        $lines.Add($h)
    }
    return $lines
}

$program = $null
$target = $null
$printText = $null
$exitCode = $null
$lineNo = 0

Get-Content -LiteralPath $InputPath | ForEach-Object {
    $lineNo++
    $raw = $_
    $line = $raw.Trim()
    if ($line.Length -eq 0) { return }
    if ($line.StartsWith(";;") -or $line.StartsWith("#") -or $line.StartsWith(";")) { return }

    # Reject middle/unreadable leakage
    if ($line -match '(?i)^(raw|db)\b') {
        throw "Expand-YoyoHi FORBIDDEN raw/db at line $lineNo (middle tier banned; use .ty for hex)"
    }
    if ($line -match '^[0-9A-Fa-f]{2}(\s+[0-9A-Fa-f]{2})+$') {
        throw "Expand-YoyoHi FORBIDDEN hex at line $lineNo (use pure .ty for unreadable tier)"
    }

    if ($line -match '^(?i)program\s+(\S+)\s*$') { $program = $Matches[1]; return }
    if ($line -match '^(?i)target\s+(\S+)\s*$') { $target = $Matches[1].ToLowerInvariant(); return }
    if ($line -match '^(?i)print\s+"(.*)"\s*$') {
        $esc = $Matches[1]
        $esc = $esc.Replace("\n", "`n").Replace("\t", "`t")
        $printText = $esc
        return
    }
    if ($line -match '^(?i)exit\s+(\d+)\s*$') { $exitCode = [int]$Matches[1]; return }
    throw "Expand-YoyoHi unknown syntax at line $lineNo : $raw"
}

if (-not $program) { throw "Expand-YoyoHi missing program" }
if ($target -ne "win32") { throw "Expand-YoyoHi target must be win32 in v1.3" }
if ($null -eq $printText) { throw "Expand-YoyoHi missing print" }
if ($null -eq $exitCode) { throw "Expand-YoyoHi missing exit" }

$outLines = Emit-PrintExitTy -Program $program -Text $printText -ExitCode $exitCode
$dir = Split-Path -Parent $OutputPath
if ($dir -and -not (Test-Path $dir)) { New-Item -ItemType Directory -Force -Path $dir | Out-Null }
Set-Content -LiteralPath $OutputPath -Value $outLines -Encoding ascii
Write-Output "YOYO_HI_EXPAND program=$program target=$target print_len=$($printText.Length) out=$OutputPath"
