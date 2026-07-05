# SolidSpec common PowerShell functions
# Dot-source from other scripts: . "$PSScriptRoot\common.ps1"

$ErrorActionPreference = 'Stop'

function Get-RepoRoot {
    # Walk up as a string path — Split-Path returns a string, so relying on
    # a .Path property would stop the walk after the first parent step.
    $dir = (Get-Location).Path
    while ($dir) {
        if ((Test-Path (Join-Path $dir "solidspec.toml")) -or (Test-Path (Join-Path $dir ".solidspec"))) {
            return $dir
        }
        $parent = Split-Path $dir -Parent
        if (-not $parent -or $parent -eq $dir) { break }
        $dir = $parent
    }
    throw "Not inside a SolidSpec project"
}

function Get-CurrentBranch {
    # Level 1: env var (full dir name or numeric prefix)
    if ($env:SOLIDSPEC_FEATURE) {
        $root = Get-RepoRoot
        if (Test-Path (Join-Path $root "specs\$env:SOLIDSPEC_FEATURE")) {
            return $env:SOLIDSPEC_FEATURE
        }
        # Bare prefix like "001" — resolve to the full directory name,
        # matching the Rust CLI's resolution behavior.
        return Find-FeatureDir -Prefix $env:SOLIDSPEC_FEATURE
    }

    # Level 2: git branch
    try {
        $branch = git rev-parse --abbrev-ref HEAD 2>$null
        if ($branch -match '^\d{3}-') {
            return $branch
        }
    } catch {}

    # Level 3: latest specs/ dir
    $root = Get-RepoRoot
    $latest = Get-ChildItem "$root\specs" -Directory | Where-Object { $_.Name -match '^\d{3}-' } | Sort-Object Name | Select-Object -Last 1
    if ($latest) {
        return $latest.Name
    }

    throw "No feature found"
}

function Find-FeatureDir {
    param([string]$Prefix)
    $root = Get-RepoRoot
    # Do not name this $matches — that is the automatic variable the -match
    # operator writes to.
    $found = @(Get-ChildItem "$root\specs" -Directory | Where-Object { $_.Name -match "^${Prefix}-" })

    if ($found.Count -eq 0) {
        throw "No feature matching '$Prefix' in specs/"
    }
    return ($found | Sort-Object Name | Select-Object -Last 1).Name
}

function Get-FeaturePaths {
    $root = Get-RepoRoot
    $branch = Get-CurrentBranch
    $featureDir = Join-Path $root "specs\$branch"

    return @{
        RepoRoot     = $root
        Branch       = $branch
        FeatureDir   = $featureDir
        Spec         = Join-Path $featureDir "spec.md"
        Plan         = Join-Path $featureDir "plan.md"
        Tasks        = Join-Path $featureDir "tasks.md"
        Research     = Join-Path $featureDir "research.md"
        DataModel    = Join-Path $featureDir "data-model.md"
        Quickstart   = Join-Path $featureDir "quickstart.md"
        ContractsDir = Join-Path $featureDir "contracts"
    }
}
