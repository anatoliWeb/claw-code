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

function Test-GitRef {
    param([string] $Ref)

    git rev-parse --verify --quiet $Ref *> $null
    return $LASTEXITCODE -eq 0
}

function Invoke-GitLines {
    param(
        [Parameter(ValueFromRemainingArguments = $true)]
        [string[]] $GitArgs
    )

    $output = & git @GitArgs 2>&1
    $exitCode = $LASTEXITCODE

    return @{
        Ok = ($exitCode -eq 0)
        Output = @($output)
        ExitCode = $exitCode
        Args = @($GitArgs)
    }
}

function Get-GitCount {
    param(
        [Parameter(ValueFromRemainingArguments = $true)]
        [string[]] $GitArgs
    )

    $result = Invoke-GitLines @GitArgs
    if (-not $result.Ok) {
        Write-Warning ("Git count command failed ({0}): {1}" -f $result.ExitCode, ($GitArgs -join " "))
        return "unknown"
    }

    $line = @($result.Output | Where-Object { "$_" -match '^\d+$' } | Select-Object -First 1)
    if (-not $line) {
        Write-Warning ("Git count command did not return a numeric line: {0}" -f ($GitArgs -join " "))
        return "unknown"
    }

    return [int]$line[0]
}

function Show-GitLog {
    param(
        [Parameter(ValueFromRemainingArguments = $true)]
        [string[]] $GitArgs
    )

    $result = Invoke-GitLines @GitArgs
    if (-not $result.Ok) {
        Write-Warning ("Git log command failed ({0}): {1}" -f $result.ExitCode, ($GitArgs -join " "))
        return
    }

    foreach ($line in $result.Output) {
        Write-Host $line
    }
}

$repoRoot = Get-RepoRoot
Set-Location $repoRoot

$currentBranch = (git rev-parse --abbrev-ref HEAD).Trim()
$gitDir = (git rev-parse --git-dir).Trim()
if (-not [System.IO.Path]::IsPathRooted($gitDir)) {
    $gitDir = Join-Path $repoRoot $gitDir
}

Write-Host "Fork sync check"
Write-Host "Repository: $repoRoot"
Write-Host "Current branch: $currentBranch"

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
    Write-Host ""
    git status
    Write-Warning "Working tree is dirty. Commit, stash, or otherwise handle changes before checking upstream updates."
    exit 1
}

$remotes = git remote
$hasUpstream = $remotes -contains "upstream"
$hasOrigin = $remotes -contains "origin"

if ($hasUpstream) {
    git fetch upstream
} else {
    Write-Warning "Remote 'upstream' is not configured. Add it before checking upstream/main."
}

if ($hasOrigin) {
    git fetch origin
} else {
    Write-Warning "Remote 'origin' is not configured."
}

$hasMain = Test-GitRef "main"
$hasClawLocal = Test-GitRef "claw-local"
$hasUpstreamMain = Test-GitRef "upstream/main"

if (-not $hasMain) {
    Write-Warning "Local branch 'main' was not found."
    exit 1
}

if (-not $hasClawLocal) {
    Write-Warning "Local branch 'claw-local' was not found. Create it with: git checkout -b claw-local main"
    exit 1
}

Write-Host ""
Write-Host "Git status"
git status

if ($hasUpstreamMain) {
    $upstreamUnique = Get-GitCount "rev-list" "--count" "--cherry-pick" "--left-only" "upstream/main...main"
    $mainUnique = Get-GitCount "rev-list" "--count" "--cherry-pick" "--right-only" "upstream/main...main"
} else {
    $upstreamUnique = "unknown"
    $mainUnique = "unknown"
}

$clawLocalUnique = Get-GitCount "rev-list" "--count" "main..claw-local"
$mainNotInClawLocal = Get-GitCount "rev-list" "--count" "claw-local..main"

Write-Host ""
Write-Host "Branch comparison"
Write-Host "Upstream unique commits vs main: $upstreamUnique"
Write-Host "Local main unique commits vs upstream: $mainUnique"
Write-Host "Commits in claw-local not in main: $clawLocalUnique"
Write-Host "Commits in main not in claw-local: $mainNotInClawLocal"

if ($hasUpstreamMain) {
    Write-Host ""
    Write-Host "Upstream unique list"
    Show-GitLog "log" "--oneline" "--decorate" "--graph" "--left-only" "--cherry-pick" "upstream/main...main" "-20"

    Write-Host ""
    Write-Host "Local main unique list"
    Show-GitLog "log" "--oneline" "--decorate" "--graph" "--right-only" "--cherry-pick" "upstream/main...main" "-20"
}

Write-Host ""
Write-Host "claw-local commits not in main"
Show-GitLog "log" "--oneline" "--decorate" "--graph" "main..claw-local" "-20"

Write-Host ""
Write-Host "main commits not in claw-local"
Show-GitLog "log" "--oneline" "--decorate" "--graph" "claw-local..main" "-20"

Write-Host ""
Write-Host "Next safe steps"
if ($upstreamUnique -ne "unknown" -and $upstreamUnique -gt 0) {
    Write-Host "Upstream updates are available:"
    Write-Host "1. git checkout main"
    Write-Host "2. git merge --ff-only upstream/main"
    Write-Host "3. git push origin main"
    Write-Host "4. git checkout claw-local"
    Write-Host "5. git merge main"
    Write-Host "6. git push"
} else {
    Write-Host "No upstream update action needed."
}

if ($mainNotInClawLocal -ne "unknown" -and $mainNotInClawLocal -gt 0) {
    Write-Host "Run: git checkout claw-local; git merge main"
}

Write-Host ""
Write-Host "This script did not run merge, rebase, push, reset, or clean."
