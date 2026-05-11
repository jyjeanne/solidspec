# Regenerate .solidspec/AGENT.md from constitution and current specs
. "$PSScriptRoot\common.ps1"

$root = Get-RepoRoot
$agentFile = Join-Path $root ".solidspec\AGENT.md"
$constitution = Join-Path $root ".solidspec\constitution.md"
$date = Get-Date -Format "yyyy-MM-dd"

# Read project name
$projectName = "unknown"
$tomlPath = Join-Path $root "solidspec.toml"
if (Test-Path $tomlPath) {
    $match = Select-String -Path $tomlPath -Pattern 'name\s*=\s*"([^"]+)"' | Select-Object -First 1
    if ($match) { $projectName = $match.Matches.Groups[1].Value }
}

$content = @"
# SolidSpec Agent Context

**Project**: $projectName
**Updated**: $date

## Constitution Principles

"@

# Append article headers from constitution
if (Test-Path $constitution) {
    Get-Content $constitution | Where-Object { $_ -match '^### Article' } | ForEach-Object {
        $content += "$_`n"
    }
}

$content += @"

## Current Features

"@

# List features with status
Get-ChildItem (Join-Path $root "specs") -Directory | Where-Object { $_.Name -match '^\d{3}-' } | Sort-Object Name | ForEach-Object {
    $feature = $_.Name
    $status = "draft"
    $tasksFile = Join-Path $_.FullName "tasks.md"
    if (Test-Path $tasksFile) {
        $tasksContent = Get-Content $tasksFile -Raw
        $done = ([regex]::Matches($tasksContent, '- \[[xX]\] T')).Count
        $total = ([regex]::Matches($tasksContent, '- \[.\] T')).Count
        if ($total -gt 0 -and $done -eq $total) { $status = "complete" }
        elseif ($done -gt 0) { $status = "in-progress ($done/$total)" }
        else { $status = "planned" }
    }
    $content += "- **${feature}**: $status`n"
}

$content += @"

## Available Commands

- ``/solidspec-specify`` — Create feature specification
- ``/solidspec-clarify`` — Resolve spec ambiguities
- ``/solidspec-plan`` — Generate architecture plan
- ``/solidspec-tasks`` — Generate task breakdown
- ``/solidspec-implement`` — Execute tasks
- ``/solidspec-analyze`` — Validate consistency
- ``/solidspec-checklist`` — Quality validation
"@

Set-Content $agentFile $content
Write-Host "Updated $agentFile"
