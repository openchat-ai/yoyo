# Stage 4 两板并行 — 将 PE + ELF 分支合并进 master（两板都绿后由 coordinator 运行）
# 用法: & F:\yoyo\scripts\stage4-two-board-merge.ps1
# 可选: -Root F:\yoyo -SkipDdc  （仅合并，不跑 ddc）

param(
    [string]$Root = "F:\yoyo",
    [bool]$SkipDdc = $false
)

$ErrorActionPreference = "Stop"

$branches = @(
    @{ Name = "PE";  Branch = "stage4/container-pe" },
    @{ Name = "ELF"; Branch = "stage4/container-elf" }
)

Write-Host "Stage 4 two-board merge -> master"
Write-Host "  repo: $Root"

& git -C $Root rev-parse --is-inside-work-tree 2>$null | Out-Null
if ($LASTEXITCODE -ne 0) { throw "Not a git repo: $Root" }

$current = (& git -C $Root rev-parse --abbrev-ref HEAD).Trim()
if ($current -ne "master") {
    throw "Must be on master (current: $current). Run: git checkout master"
}

$conflicts = @()

foreach ($b in $branches) {
    Write-Host ""
    Write-Host "merge $($b.Branch) ..."
    & git -C $Root merge --no-ff -m "stage4: merge $($b.Branch) (container DDC $($b.Name))" $b.Branch
    if ($LASTEXITCODE -ne 0) {
        $conflicts += $b.Branch
        Write-Host "  CONFLICT on $($b.Branch) — resolve in $Root, then:"
        Write-Host "    git add -A; git commit   # or git merge --abort to undo this merge"
        break
    }
    Write-Host "  OK: $($b.Branch)"
}

if ($conflicts.Count -gt 0) {
    Write-Host ""
    Write-Host "=== Merge blocked ==="
    Write-Host "Conflict branch(es): $($conflicts -join ', ')"
    Write-Host "Typical overlap: shared verifier files touched by both boards."
    Write-Host "Resolve conflicts in master worktree only; do not edit STAGE4_OWNER_CHECKLIST in PE/ELF worktrees."
    exit 1
}

if (-not $SkipDdc) {
    Write-Host ""
    Write-Host "run ddc on master ..."
    Push-Location (Join-Path $Root "yoyo-rust\verifier")
  try {
        & cargo run -- test ddc
        $ddcExit = $LASTEXITCODE
    } finally {
        Pop-Location
    }
    if ($ddcExit -ne 0) {
        Write-Host "  ddc FAILED (exit $ddcExit) — do not check B on checklist"
        exit $ddcExit
    }
    Write-Host "  ddc PASS — coordinator may mark B [x] if container no longer SKIP"
}

Write-Host ""
Write-Host "Merge complete. Next (coordinator on master):"
Write-Host "  1. Confirm container PE+ELF PASS in ddc output"
Write-Host "  2. Update STAGE4_OWNER_CHECKLIST.md B -> [x] if appropriate"
Write-Host "  3. Sync BACKEND_SUPPORT.md if Status changed SKIP -> PASS"
Write-Host "Done."
exit 0
