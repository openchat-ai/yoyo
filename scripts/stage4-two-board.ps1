# Stage 4 两板并行 — 创建 PE / ELF git worktree（可重复运行）
# 用法: & F:\yoyo\scripts\stage4-two-board.ps1
# 可选: -Root F:\yoyo -WorktreesBase F:\yoyo-worktrees

param(
    [string]$Root = "F:\yoyo",
    [string]$WorktreesBase = "F:\yoyo-worktrees"
)

$ErrorActionPreference = "Stop"

function Ensure-Branch {
    param([string]$Repo, [string]$Branch, [string]$BaseRef)
    $exists = & git -C $Repo show-ref --verify --quiet "refs/heads/$Branch"
    if ($LASTEXITCODE -ne 0) {
        Write-Host "  create branch $Branch from $BaseRef"
        & git -C $Repo branch $Branch $BaseRef
        if ($LASTEXITCODE -ne 0) { throw "git branch $Branch failed" }
    } else {
        Write-Host "  branch $Branch exists (skip create)"
    }
}

function Ensure-Worktree {
    param(
        [string]$Repo,
        [string]$Path,
        [string]$Branch
    )
    if (Test-Path $Path) {
        $listed = & git -C $Repo worktree list
        if ($listed -match [regex]::Escape($Path.Replace('\', '/'))) {
            Write-Host "  worktree exists: $Path"
            return
        }
        if (Test-Path (Join-Path $Path ".git")) {
            throw "Path exists but not registered as worktree: $Path — resolve manually"
        }
        throw "Path exists but is not a worktree: $Path — resolve manually"
    }
    $parent = Split-Path $Path -Parent
    if (-not (Test-Path $parent)) {
        New-Item -ItemType Directory -Path $parent -Force | Out-Null
    }
    Write-Host "  add worktree: $Path -> $Branch"
    & git -C $Repo worktree add $Path $Branch
    if ($LASTEXITCODE -ne 0) { throw "git worktree add failed for $Path" }
}

Write-Host "Stage 4 two-board worktree setup"
Write-Host "  repo:           $Root"
Write-Host "  worktrees base: $WorktreesBase"

& git -C $Root rev-parse --is-inside-work-tree 2>$null | Out-Null
if ($LASTEXITCODE -ne 0) { throw "Not a git repo: $Root" }

$baseRef = (& git -C $Root rev-parse master).Trim()
if (-not $baseRef) { throw "Cannot resolve master on $Root" }
Write-Host "  base: master @ $baseRef"

$boards = @(
    @{ Name = "PE";  Branch = "stage4/container-pe";  Path = Join-Path $WorktreesBase "stage4-pe" },
    @{ Name = "ELF"; Branch = "stage4/container-elf"; Path = Join-Path $WorktreesBase "stage4-elf" }
)

foreach ($b in $boards) {
    Write-Host ""
    Write-Host "[$($b.Name)] $($b.Branch)"
    Ensure-Branch -Repo $Root -Branch $b.Branch -BaseRef $baseRef
    Ensure-Worktree -Repo $Root -Path $b.Path -Branch $b.Branch
}

Write-Host ""
Write-Host "=== Agent paths (copy to coordinator) ==="
Write-Host "  Board PE:  $($boards[0].Path)  branch $($boards[0].Branch)"
Write-Host "  Board ELF: $($boards[1].Path)  branch $($boards[1].Branch)"
Write-Host ""
Write-Host "Coordinator (master only): F:\yoyo"
Write-Host "Merge when both green: & F:\yoyo\scripts\stage4-two-board-merge.ps1"
Write-Host "Done."
