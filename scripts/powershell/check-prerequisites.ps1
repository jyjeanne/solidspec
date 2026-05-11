# SolidSpec prerequisites check
. "$PSScriptRoot\common.ps1"

Write-Host "SolidSpec Prerequisites Check"
Write-Host "=============================="

$errors = 0

# Check git
try {
    $gitVersion = git --version 2>$null
    Write-Host "[OK] git: $gitVersion"
} catch {
    Write-Host "[!!] git: not found"
    $errors++
}

# Check project structure
try {
    $root = Get-RepoRoot
    Write-Host "[OK] Project root: $root"

    if (Test-Path "$root\.solidspec\constitution.md") {
        Write-Host "[OK] Constitution file present"
    } else {
        Write-Host "[!!] Constitution file missing"
        $errors++
    }

    if (Test-Path "$root\solidspec.toml") {
        Write-Host "[OK] solidspec.toml found"
    } else {
        Write-Host "[!!] solidspec.toml missing"
        $errors++
    }
} catch {
    Write-Host "[!!] Not inside a SolidSpec project"
    $errors++
}

Write-Host ""
if ($errors -eq 0) {
    Write-Host "All checks passed."
} else {
    Write-Host "$errors issue(s) found."
    exit 1
}
