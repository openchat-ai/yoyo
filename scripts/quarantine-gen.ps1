# quarantine-gen.ps1 — Gen quarantine (C3 Q0–Q9), fail-closed
# Usage:
#   .\scripts\quarantine-gen.ps1 -Gen gen4 -Root F:\yoyo
#   .\scripts\quarantine-gen.ps1 -Gen gen4 -Root F:\yoyo -SkipCachePurge
# Exit 0 = machine checks passed; exit ≠ 0 = FAIL-CLOSED (do not advertise LOCKED / C-ddc)

[CmdletBinding()]
param(
  [Parameter(Mandatory = $false)]
  [string]$Gen = $env:YOYO_GEN,

  [Parameter(Mandatory = $false)]
  [string]$Root = $env:YOYO_ROOT,

  [switch]$SkipCachePurge,
  [switch]$AllowMainBranch,
  [switch]$PurgeCaches
)

$ErrorActionPreference = "Stop"
$FailCount = 0

function Fail([string]$Msg) {
  Write-Host "FAIL: $Msg" -ForegroundColor Red
  $script:FailCount++
}

function Ok([string]$Msg) {
  Write-Host "OK:   $Msg" -ForegroundColor Green
}

function Warn([string]$Msg) {
  Write-Host "WARN: $Msg" -ForegroundColor Yellow
}

Write-Host "== quarantine-gen (fail-closed) =="

# --- Q0: pin gen + root ---
if ([string]::IsNullOrWhiteSpace($Gen)) {
  Fail "Q0: YOYO_GEN / -Gen not set (e.g. gen4)"
} else {
  Ok "Q0: YOYO_GEN=$Gen"
  $env:YOYO_GEN = $Gen
}

if ([string]::IsNullOrWhiteSpace($Root)) {
  $Root = (Resolve-Path (Split-Path -Parent $PSScriptRoot)).Path
  Warn "Q0: -Root / YOYO_ROOT unset; defaulting to repo parent of scripts: $Root"
}

try {
  $Root = (Resolve-Path -LiteralPath $Root).Path
} catch {
  Fail "Q0: Root does not exist: $Root"
  Write-Host "quarantine-gen: FAILED ($FailCount)" -ForegroundColor Red
  exit 1
}

$env:YOYO_ROOT = $Root
$cwd = (Get-Location).Path
if (-not ($cwd.StartsWith($Root, [System.StringComparison]::OrdinalIgnoreCase))) {
  Fail "Q0: cwd '$cwd' is not under YOYO_ROOT '$Root' (Set-Location first)"
} else {
  Ok "Q0: cwd under YOYO_ROOT"
}

Set-Location -LiteralPath $Root

# --- Q1: PATH slice (prior-gen name heuristics) ---
$pathEntries = ($env:Path -split ';' | Where-Object { $_ })
$priorHits = @()
foreach ($p in $pathEntries) {
  if ($p -match '(?i)yoyo-gen\d|gen[0-9]+.*(bin|install)|\\yoyo-gen2\\') {
    # Allow if path is inside current root
    if (-not $p.StartsWith($Root, [System.StringComparison]::OrdinalIgnoreCase)) {
      $priorHits += $p
    }
  }
}
if ($priorHits.Count -gt 0) {
  Fail ("Q1: PATH contains prior-gen prefixes:`n  " + ($priorHits -join "`n  "))
} else {
  Ok "Q1: no obvious prior-gen PATH prefixes"
}

# Soft check: yoyo.exe outside root
$yoyoCmd = Get-Command yoyo -ErrorAction SilentlyContinue
if ($yoyoCmd) {
  $src = $yoyoCmd.Source
  if (-not $src.StartsWith($Root, [System.StringComparison]::OrdinalIgnoreCase)) {
    Fail "Q1: 'yoyo' resolves outside YOYO_ROOT: $src"
  } else {
    Ok "Q1: yoyo Source under root"
  }
} else {
  Ok "Q1: no global 'yoyo' on PATH (ok if using tree-local bins)"
}

# --- Q2: build caches ---
$cacheDirs = @(
  "yoyo-rust\target",
  "yoyo-asm\target",
  "yoyo-js\node_modules",
  "node_modules",
  ".cargo-cache"
)
$presentCaches = @()
foreach ($rel in $cacheDirs) {
  $full = Join-Path $Root $rel
  if (Test-Path -LiteralPath $full) { $presentCaches += $rel }
}

if ($PurgeCaches -and -not $SkipCachePurge) {
  foreach ($rel in $presentCaches) {
    $full = Join-Path $Root $rel
    Write-Host "Q2: purging $rel"
    Remove-Item -LiteralPath $full -Recurse -Force
  }
  $presentCaches = @()
  Ok "Q2: caches purged"
} elseif ($presentCaches.Count -gt 0) {
  if ($SkipCachePurge) {
    Warn ("Q2: caches present (not purged): " + ($presentCaches -join ", ") + " — rebuilds may mix objects; pass -PurgeCaches for clean-room")
  } else {
    Fail ("Q2: build caches present (pass -PurgeCaches to remove, or -SkipCachePurge to acknowledge): " + ($presentCaches -join ", "))
  }
} else {
  Ok "Q2: no listed build caches present"
}

# --- Q3: PE inventory ---
$ledgerName = "docs\PE_LEDGER_$Gen.txt"
$ledgerPath = Join-Path $Root $ledgerName
$binaries = @()
if (Test-Path -LiteralPath $Root) {
  $binaries = Get-ChildItem -LiteralPath $Root -Recurse -File -ErrorAction SilentlyContinue |
    Where-Object {
      $_.Extension -match '^\.(exe|dll|node)$' -and
      $_.FullName -notmatch '\\(\.git)\\'
    }
}

if ($binaries.Count -eq 0) {
  Ok "Q3: no .exe/.dll/.node under root"
} elseif (Test-Path -LiteralPath $ledgerPath) {
  $ledgerText = Get-Content -LiteralPath $ledgerPath -Raw
  $missing = @()
  foreach ($b in $binaries) {
    $hash = (Get-FileHash -LiteralPath $b.FullName -Algorithm SHA256).Hash
    $rel = $b.FullName.Substring($Root.Length).TrimStart('\', '/')
    if ($ledgerText -notmatch [regex]::Escape($hash)) {
      $missing += "$rel ($hash)"
    }
  }
  if ($missing.Count -gt 0) {
    Fail ("Q3: binaries not in ${ledgerName}:`n  " + ($missing -join "`n  "))
  } else {
    Ok "Q3: all binaries hashed in $ledgerName"
  }
} else {
  Fail "Q3: found $($binaries.Count) binary(ies) but missing ledger $ledgerName — create ledger or delete untracked PEs"
}

# --- Q4: lock present (same-gen hygiene; do not invent green) ---
$lockPath = Join-Path $Root "yoyo\tests\yoyo.ty.lock"
if (-not (Test-Path -LiteralPath $lockPath)) {
  Fail "Q4: missing yoyo\tests\yoyo.ty.lock"
} else {
  Ok "Q4: lock file present"
}

$verifyScript = Join-Path $Root "scripts\verify-yoyo-ty.mjs"
if (Test-Path -LiteralPath $verifyScript) {
  Push-Location $Root
  try {
    & node $verifyScript
    if ($LASTEXITCODE -ne 0) {
      Fail "Q4: verify-yoyo-ty.mjs exit $LASTEXITCODE (honest red — do not hand-edit hashes)"
    } else {
      Ok "Q4: verify-yoyo-ty.mjs exit 0"
    }
  } catch {
    Fail "Q4: verify-yoyo-ty.mjs failed to run: $_"
  } finally {
    Pop-Location
  }
} else {
  Warn "Q4: scripts\verify-yoyo-ty.mjs missing — skip run"
}

# --- Q5: worktree hint ---
$gitOk = $false
try {
  & git -C $Root rev-parse --is-inside-work-tree 2>$null | Out-Null
  if ($LASTEXITCODE -eq 0) { $gitOk = $true }
} catch { }

if ($gitOk) {
  $wt = & git -C $Root worktree list 2>$null
  $wtLines = @($wt | Where-Object { $_ })
  if ($wtLines.Count -gt 1) {
    Warn "Q5: multiple worktrees listed — ensure others are READONLY-ARCHIVE:`n  $($wtLines -join "`n  ")"
  } else {
    Ok "Q5: single worktree (or only one listed)"
  }

  # --- Q6: branch naming ---
  $branch = (& git -C $Root rev-parse --abbrev-ref HEAD).Trim()
  $genNum = $null
  if ($Gen -match '(?i)^gen(\d+)$') { $genNum = $Matches[1] }

  $branchOk = $false
  if ($branch -eq "HEAD") {
    Warn "Q6: detached HEAD — declare gen explicitly in agent session"
    $branchOk = $true
  } elseif ($AllowMainBranch -and ($branch -eq "main" -or $branch -eq "master")) {
    Warn "Q6: on $branch with -AllowMainBranch (pointer/README only; no new bootstrap ads)"
    $branchOk = $true
  } elseif ($genNum -and ($branch -like "gen$genNum/*" -or $branch -like "archive/gen*/*")) {
    $branchOk = $true
  } elseif ($branch -match '(?i)^gen\d+/') {
    $branchOk = $true
  }

  if ($branchOk) {
    Ok "Q6: branch '$branch' acceptable for Gen=$Gen"
  } else {
    Fail "Q6: branch '$branch' must match gen{N}/* or archive/gen*/* (or pass -AllowMainBranch for pointer-only main)"
  }

  # --- Q7: remotes ---
  $remotes = @(& git -C $Root remote 2>$null)
  if ($remotes.Count -eq 0) {
    Warn "Q7: no remotes (ok for orphan clean-room)"
  } elseif ($remotes.Count -eq 1) {
    Ok "Q7: single remote '$($remotes[0])'"
  } else {
    Fail ("Q7: multiple remotes (want one origin): " + ($remotes -join ", "))
  }

  # --- Q8: declaration reminder (env already set) ---
  $short = (& git -C $Root rev-parse --short HEAD).Trim()
  Ok "Q8: declare in agent first message — YOYO_ROOT=$Root YOYO_GEN=$Gen HEAD=$short PROMPT-v3.md only"
} else {
  Warn "Q5–Q8: not a git work tree — skipped branch/remote checks"
}

# --- Q9: RESTORE discipline (heuristic scan of reflog / recent messages not automated) ---
Ok "Q9: RESTORE rule is process — any checkout/blob from prior gen needs PR title RESTORE-FROM-GENN + SHA table (see PROMPT-v3.md N.7 / quarantine script)"

Write-Host ""
if ($FailCount -gt 0) {
  Write-Host "quarantine-gen: FAILED ($FailCount check(s)) — MUST NOT advertise LOCKED / C-ddc" -ForegroundColor Red
  exit 1
}

Write-Host "quarantine-gen: PASSED (complete human Q0–Q9 checklist per PROMPT-v3.md N.7)" -ForegroundColor Green
exit 0
