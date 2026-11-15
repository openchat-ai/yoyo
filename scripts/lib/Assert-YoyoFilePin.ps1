# Assert-YoyoFilePin.ps1 — pin verify via yoyo hash (no Node crypto)
# Usage:
#   . scripts\lib\Assert-YoyoFilePin.ps1
#   Assert-YoyoShaPin -Yoyo $Yoyo -File $path -PinPath $pin -Label PLACE_STUB
#   Assert-YoyoPayloadBinsPin -Yoyo $Yoyo -PayloadDir $dir -Label HB_FX_BINS

function Read-YoyoPinMap([string]$PinPath) {
    $o = @{}
    Get-Content -LiteralPath $PinPath | ForEach-Object {
        $line = $_.Trim()
        if (-not $line -or $line.StartsWith('#')) { return }
        $i = $line.IndexOf('=')
        if ($i -lt 1) { return }
        $o[$line.Substring(0, $i).Trim()] = $line.Substring($i + 1).Trim()
    }
    return $o
}

function Assert-YoyoShaPin {
    param(
        [Parameter(Mandatory = $true)][string]$Yoyo,
        [Parameter(Mandatory = $true)][string]$File,
        [Parameter(Mandatory = $true)][string]$PinPath,
        [Parameter(Mandatory = $true)][string]$Label,
        [string]$RequireTrustRoot = 'place_stub',
        [string]$Doc = ''
    )
    if (-not (Test-Path -LiteralPath $File) -or -not (Test-Path -LiteralPath $PinPath)) {
        Write-Host "$Label status=RED reason=missing_file_or_pin"
        throw "$Label missing"
    }
    $pin = Read-YoyoPinMap $PinPath
    $bytes = (Get-Item -LiteralPath $File).Length
    $sha = (& $Yoyo hash $File).Trim().ToLowerInvariant()
    Write-Host "$Label bytes=$bytes sha256_prefix=$($sha.Substring(0, [Math]::Min(16, $sha.Length)))"
    if ($RequireTrustRoot) {
        Write-Host "$Label trust_root=$RequireTrustRoot"
        if ($pin['trust_root'] -ne $RequireTrustRoot) {
            Write-Host "$Label status=RED reason=trust_root"
            throw "$Label trust_root"
        }
    }
    $wantBytes = [string]$pin['bytes']
    $wantSha = ([string]$pin['sha256']).ToLowerInvariant()
    if ($wantBytes -ne [string]$bytes -or $wantSha -ne $sha) {
        Write-Host "$Label status=RED reason=pin_DIFF"
        throw "$Label pin_DIFF"
    }
    Write-Host "$Label status=GREEN pin_EQUAL"
    if ($Doc) { Write-Host "$Label doc=$Doc" }
}

function Assert-YoyoPayloadBinsPin {
    param(
        [Parameter(Mandatory = $true)][string]$Yoyo,
        [Parameter(Mandatory = $true)][string]$PayloadDir,
        [string]$Label = 'HB_FX_BINS',
        [string]$Doc = 'AUTO-TRUER6.md'
    )
    $pinPath = Join-Path $PayloadDir 'payload.pin'
    $codePath = Join-Path $PayloadDir 'code.bin'
    $dataPath = Join-Path $PayloadDir 'data.bin'
    if (-not (Test-Path -LiteralPath $pinPath) -or -not (Test-Path -LiteralPath $codePath) -or -not (Test-Path -LiteralPath $dataPath)) {
        Write-Host "$Label status=RED reason=missing_pin_or_bins"
        throw "$Label missing"
    }
    $pin = Read-YoyoPinMap $pinPath
    $codeBytes = (Get-Item -LiteralPath $codePath).Length
    $dataBytes = (Get-Item -LiteralPath $dataPath).Length
    $codeSha = (& $Yoyo hash $codePath).Trim().ToLowerInvariant()
    $dataSha = (& $Yoyo hash $dataPath).Trim().ToLowerInvariant()
    Write-Host "$Label code_bytes=$codeBytes sha256_prefix=$($codeSha.Substring(0, 16))"
    Write-Host "$Label data_bytes=$dataBytes sha256_prefix=$($dataSha.Substring(0, 16))"
    Write-Host "$Label node_rebuild_on_gate=ABSENT"
    Write-Host "$Label policy=pin_locked"
    Write-Host "$Label bins_sot=$($pin['bins_sot'])"
    if ([string]$pin['code_bytes'] -ne [string]$codeBytes -or [string]$pin['data_bytes'] -ne [string]$dataBytes) {
        Write-Host "$Label status=RED reason=size_pin_DIFF"
        throw "$Label size"
    }
    if (([string]$pin['code_sha256']).ToLowerInvariant() -ne $codeSha) {
        Write-Host "$Label status=RED reason=code_sha_pin_DIFF"
        throw "$Label code_sha"
    }
    if (([string]$pin['data_sha256']).ToLowerInvariant() -ne $dataSha) {
        Write-Host "$Label status=RED reason=data_sha_pin_DIFF"
        throw "$Label data_sha"
    }
    if ($pin['bins_gate_policy'] -ne 'pin_locked') {
        Write-Host "$Label status=RED reason=bins_gate_policy_not_pin_locked"
        throw "$Label policy"
    }
    $sotOk = ($pin['bins_sot'] -eq 'fixture_pe') -or ($pin['bins_sot'] -eq 'pin_locked_content')
    if (-not $sotOk) {
        Write-Host "$Label status=RED reason=bins_sot_invalid"
        throw "$Label sot"
    }
    if ($pin['sidecar_face_in_data'] -eq 'ABSENT' -or $pin['sidecar_name_face'] -eq 'ABSENT') {
        $data = [IO.File]::ReadAllBytes($dataPath)
        $s1 = [Text.Encoding]::ASCII.GetBytes('yoyo_rt.dll')
        $s2 = [Text.Encoding]::ASCII.GetBytes('yoyo_no_rt.')
        function IndexOfBytes([byte[]]$hay, [byte[]]$needle) {
            for ($i = 0; $i -le $hay.Length - $needle.Length; $i++) {
                $ok = $true
                for ($j = 0; $j -lt $needle.Length; $j++) {
                    if ($hay[$i + $j] -ne $needle[$j]) { $ok = $false; break }
                }
                if ($ok) { return $i }
            }
            return -1
        }
        if ((IndexOfBytes $data $s1) -ge 0 -or (IndexOfBytes $data $s2) -ge 0) {
            Write-Host "$Label status=RED reason=sidecar_name_face_still_in_data"
            throw "$Label sidecar"
        }
        Write-Host "$Label sidecar_face_in_data=ABSENT"
        Write-Host "$Label sidecar_name_face=ABSENT"
    }
    Write-Host "$Label status=GREEN pin_EQUAL"
    Write-Host "$Label doc=$Doc"
}
