# Create a new feature branch and directory
param([Parameter(Mandatory)][string]$Description)

. "$PSScriptRoot\common.ps1"

$root = Get-RepoRoot
$specsDir = Join-Path $root "specs"
New-Item -ItemType Directory -Path $specsDir -Force | Out-Null

# Find next feature number
$maxNum = 0
Get-ChildItem $specsDir -Directory | Where-Object { $_.Name -match '^\d{3}-' } | ForEach-Object {
    $num = [int]($_.Name.Substring(0, 3))
    if ($num -gt $maxNum) { $maxNum = $num }
}

$nextNum = $maxNum + 1
if ($nextNum -gt 999) {
    throw "Feature number overflow (max 999)"
}

$featureId = $nextNum.ToString("D3")
# Truncate based on the transformed slug's own length — the replaces can make
# it shorter than $Description, and Substring past the end throws.
$slug = ($Description.ToLower() -replace '[^a-z0-9]', '-' -replace '-+', '-').Trim('-')
$shortName = $slug.Substring(0, [Math]::Min(50, $slug.Length)).TrimEnd('-')
$branchName = "$featureId-$shortName"

# Create git branch if in a repo
try {
    git checkout -b $branchName 2>$null
} catch {
    Write-Host "Warning: could not create branch $branchName"
}

# Create feature directory
$featureDir = Join-Path $specsDir $branchName
New-Item -ItemType Directory -Path "$featureDir\checklists" -Force | Out-Null
New-Item -ItemType Directory -Path "$featureDir\contracts" -Force | Out-Null

Write-Output $branchName
