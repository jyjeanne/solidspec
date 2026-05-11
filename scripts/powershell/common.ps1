# SolidSpec common PowerShell functions
# Dot-source from other scripts: . "$PSScriptRoot\common.ps1"

$ErrorActionPreference = 'Stop'

function Get-RepoRoot {
    $dir = Get-Location
    while ($dir.Path -ne [System.IO.Path]::GetPathRoot($dir.Path)) {
        if ((Test-Path "$dir\solidspec.toml") -or (Test-Path "$dir\.solidspec")) {
            return $dir.Path
        }
        $dir = Split-Path $dir -Parent
    }
    throw "Not inside a SolidSpec project"
}

function Get-CurrentBranch {
    # Level 1: env var
    if ($env:SOLIDSPEC_FEATURE) {
        return $env:SOLIDSPEC_FEATURE
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
    $matches = Get-ChildItem "$root\specs" -Directory | Where-Object { $_.Name -match "^${Prefix}-" }

    if ($matches.Count -eq 0) {
        throw "No feature matching '$Prefix' in specs/"
    }
    return ($matches | Sort-Object Name | Select-Object -Last 1).Name
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
