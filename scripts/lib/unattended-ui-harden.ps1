# unattended-ui-harden.ps1 — 离席跑 genN / smoke 时尽量不弹窗，失败靠 exit code / dump
#
# 覆盖：WER 崩溃框、关键错误框（本进程 SetErrorMode）、可选杀掉 WerFault UI。
# 不覆盖：UAC、应用主动 MessageBox、杀软框。
#
#   . F:\yoyo\scripts\lib\unattended-ui-harden.ps1
#   Enable-YoyoUnattendedUiHarden
#   $ec = Invoke-YoyoUnattendedNativeExe -FilePath .\gen1.exe -WorkingDirectory $pwd
#
# Fail-closed 语义不变：崩了仍非 0；只是不等人点「关闭程序」。

Set-StrictMode -Version Latest

$script:YoyoUnattendedUiApplied = $false
$script:YoyoUnattendedWerWatch = $null
$script:YoyoUnattendedWerPrev = $null

function Enable-YoyoUnattendedUiHarden {
    param(
        [switch]$SkipWerRegistry,
        [switch]$SkipWerWatchdog,
        [switch]$SkipErrorMode
    )
    if ($script:YoyoUnattendedUiApplied) {
        Write-Host "YOYO_UNATTENDED_UI already=1"
        return
    }

    $env:YOYO_UNATTENDED_UI = "1"
    $env:YOYO_NO_UI = "1"

    if (-not $SkipErrorMode) {
        try {
            if (-not ("YoyoUnattendedUi" -as [type])) {
                Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public static class YoyoUnattendedUi {
    public const uint SEM_FAILCRITICALERRORS = 0x0001;
    public const uint SEM_NOGPFAULTERRORBOX = 0x0002;
    public const uint SEM_NOOPENFILEERRORBOX = 0x8000;
    [DllImport("kernel32.dll")] public static extern uint SetErrorMode(uint uMode);
    [DllImport("kernel32.dll")] public static extern bool SetThreadErrorMode(uint dwNewMode, out uint lpOldMode);
}
"@
            }
            $mode = [YoyoUnattendedUi]::SEM_FAILCRITICALERRORS -bor
                    [YoyoUnattendedUi]::SEM_NOGPFAULTERRORBOX -bor
                    [YoyoUnattendedUi]::SEM_NOOPENFILEERRORBOX
            [void][YoyoUnattendedUi]::SetErrorMode($mode)
            $old = [uint32]0
            [void][YoyoUnattendedUi]::SetThreadErrorMode($mode, [ref]$old)
            Write-Host "YOYO_UNATTENDED_UI error_mode=0x$('{0:X}' -f $mode)"
        } catch {
            Write-Host "YOYO_UNATTENDED_UI error_mode=SKIP reason=$($_.Exception.Message)"
        }
    }

    if (-not $SkipWerRegistry) {
        try {
            $werKey = "HKCU:\Software\Microsoft\Windows\Windows Error Reporting"
            if (-not (Test-Path $werKey)) {
                New-Item -Path $werKey -Force | Out-Null
            }
            $prev = $null
            try { $prev = (Get-ItemProperty -Path $werKey -Name DontShowUI -ErrorAction Stop).DontShowUI } catch { }
            $script:YoyoUnattendedWerPrev = $prev
            New-ItemProperty -Path $werKey -Name DontShowUI -PropertyType DWord -Value 1 -Force | Out-Null
            Write-Host "YOYO_UNATTENDED_UI wer_DontShowUI=1 (prev=$prev)"
        } catch {
            Write-Host "YOYO_UNATTENDED_UI wer_registry=SKIP reason=$($_.Exception.Message)"
        }
    }

    if (-not $SkipWerWatchdog) {
        try {
            $script:YoyoUnattendedWerWatch = Start-Job -ScriptBlock {
                $names = @("WerFault", "WerFaultSecure", "DWWIN", "dwwin", "vsjitdebugger", "vsjitdebuggerue")
                while ($true) {
                    foreach ($n in $names) {
                        Get-Process -Name $n -ErrorAction SilentlyContinue |
                            Stop-Process -Force -ErrorAction SilentlyContinue
                    }
                    Start-Sleep -Milliseconds 400
                }
            }
            Write-Host "YOYO_UNATTENDED_UI wer_watchdog=ON job=$($script:YoyoUnattendedWerWatch.Id)"
        } catch {
            Write-Host "YOYO_UNATTENDED_UI wer_watchdog=SKIP reason=$($_.Exception.Message)"
        }
    }

    $script:YoyoUnattendedUiApplied = $true
    Write-Host "YOYO_UNATTENDED_UI status=ON"
}

function Disable-YoyoUnattendedUiHarden {
    param([switch]$RestoreWer)
    if ($script:YoyoUnattendedWerWatch) {
        try {
            Stop-Job $script:YoyoUnattendedWerWatch -ErrorAction SilentlyContinue
            Remove-Job $script:YoyoUnattendedWerWatch -Force -ErrorAction SilentlyContinue
        } catch { }
        $script:YoyoUnattendedWerWatch = $null
    }
    if ($RestoreWer -and $null -ne $script:YoyoUnattendedWerPrev) {
        try {
            $werKey = "HKCU:\Software\Microsoft\Windows\Windows Error Reporting"
            New-ItemProperty -Path $werKey -Name DontShowUI -PropertyType DWord -Value ([int]$script:YoyoUnattendedWerPrev) -Force | Out-Null
        } catch { }
    }
    $script:YoyoUnattendedUiApplied = $false
    Write-Host "YOYO_UNATTENDED_UI status=OFF"
}

function Invoke-YoyoUnattendedNativeExe {
    param(
        [Parameter(Mandatory = $true)][string]$FilePath,
        [string]$WorkingDirectory = "",
        [int]$TimeoutSec = 120,
        [string[]]$ArgumentList = $null
    )
    Enable-YoyoUnattendedUiHarden
    if (-not (Test-Path -LiteralPath $FilePath)) {
        throw "Invoke-YoyoUnattendedNativeExe missing: $FilePath"
    }
    $wd = $WorkingDirectory
    if ([string]::IsNullOrWhiteSpace($wd)) {
        $wd = Split-Path -Parent (Resolve-Path -LiteralPath $FilePath)
    }

    # Start-Process rejects -ArgumentList @() / $null under StrictMode — omit when empty.
    $startParams = @{
        FilePath         = $FilePath
        WorkingDirectory = $wd
        PassThru         = $true
        WindowStyle      = "Hidden"
        ErrorAction      = "Stop"
    }
    if ($null -ne $ArgumentList -and $ArgumentList.Count -gt 0) {
        $startParams["ArgumentList"] = $ArgumentList
    }
    $p = Start-Process @startParams

    $ok = $p.WaitForExit($TimeoutSec * 1000)
    if (-not $ok) {
        try { $p.Kill() } catch { }
        Write-Host ("YOYO_UNATTENDED_UI timeout_kill after={0}s path={1}" -f $TimeoutSec, $FilePath)
        return -1
    }
    return [int]$p.ExitCode
}
