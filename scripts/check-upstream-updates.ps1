$ErrorActionPreference = "Stop"

function Get-RepoRoot {
    $root = git rev-parse --show-toplevel 2>$null
    if (-not $root) {
        Write-Error "Not inside a Git repository."
    }
    return $root.Trim()
}

function Test-GitStateFile {
    param(
        [string] $GitDir,
        [string] $Name
    )

    return Test-Path -LiteralPath (Join-Path $GitDir $Name)
}

$repoRoot = Get-RepoRoot
Set-Location $repoRoot

$gitDir = (git rev-parse --git-dir).Trim()
if (-not [System.IO.Path]::IsPathRooted($gitDir)) {
    $gitDir = Join-Path $repoRoot $gitDir
}

$stateChecks = @(
    @{ Name = "rebase"; Path = "rebase-merge" },
    @{ Name = "rebase"; Path = "rebase-apply" },
    @{ Name = "merge"; Path = "MERGE_HEAD" },
    @{ Name = "cherry-pick"; Path = "CHERRY_PICK_HEAD" },
    @{ Name = "bisect"; Path = "BISECT_LOG" }
)

foreach ($check in $stateChecks) {
    if (Test-GitStateFile -GitDir $gitDir -Name $check.Path) {
        Write-Warning "Active git $($check.Name) state detected. Resolve it before checking upstream updates."
        exit 1
    }
}

$shortStatus = git status --porcelain
if ($shortStatus) {
    git status
    Write-Warning "Working tree is dirty. Commit, stash, or otherwise handle changes before checking upstream updates."
    exit 1
}

git fetch upstream
git fetch origin

Write-Host ""
Write-Host "Git status"
git status

$upstreamUnique = (git rev-list --count --cherry-pick --left-only upstream/main...main).Trim()
$localUnique = (git rev-list --count --cherry-pick --right-only upstream/main...main).Trim()

Write-Host ""
Write-Host "Upstream unique commits: $upstreamUnique"
Write-Host "Local unique commits: $localUnique"

Write-Host ""
Write-Host "Upstream unique list"
git log --oneline --decorate --graph --left-only --cherry-pick upstream/main...main -20

Write-Host ""
Write-Host "Local unique list"
git log --oneline --decorate --graph --right-only --cherry-pick upstream/main...main -20

Write-Host ""
if ([int]$upstreamUnique -gt 0) {
    Write-Host "Updates are available. Review docs/fork-sync-workflow.md before rebasing."
} else {
    Write-Host "No unique upstream updates to apply."
}
