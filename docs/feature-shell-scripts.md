# Feature Specification: Shell Helper Scripts

**Feature**: Cross-platform shell scripts for SolidSpec project operations
**Status**: Draft
**Priority**: P2

## Overview

SolidSpec commands reference shell scripts in their YAML frontmatter (`scripts.sh` and `scripts.ps` keys). These scripts are embedded in the SolidSpec binary and copied to `.solidspec/scripts/` on `init` and `upgrade`. They provide helper functions that AI agents can call during SDD workflows.

## Scripts

### 1. `common.sh` / `common.ps1` — Shared utility functions

- `get_repo_root()` — Find project root by walking up to `solidspec.toml`
- `get_current_branch()` — Get current git branch name, fallback to `SOLIDSPEC_FEATURE` env
- `find_feature_dir()` — Find feature directory by numeric prefix in `specs/`
- `get_feature_paths()` — Output all feature paths as shell variables

### 2. `check-prerequisites.sh` / `check-prerequisites.ps1`

- Verify git is installed
- Verify solidspec binary is accessible
- Check `.solidspec/` structure exists
- Report status

### 3. `create-new-feature.sh` / `create-new-feature.ps1`

- Determine next feature number
- Create git branch
- Create feature directory under `specs/`

### 4. `setup-plan.sh` / `setup-plan.ps1`

- Create plan supporting files (research.md, data-model.md, contracts/)
- Called by the `plan` command frontmatter

### 5. `update-agent-context.sh` / `update-agent-context.ps1`

- Regenerate `.solidspec/AGENT.md` from constitution + current specs

## Implementation

- Scripts are embedded in the binary via `include_str!`
- Copied to `.solidspec/scripts/{bash,powershell}/` on `init` and `upgrade`
- `upgrade` always overwrites scripts (they are not user-customizable)

## Acceptance Criteria

- `solidspec init` copies all scripts to `.solidspec/scripts/`
- `solidspec upgrade` refreshes scripts
- Scripts are executable (Unix: shebang + chmod in future)
- Tests verify all scripts are embedded and non-empty
