#!/usr/bin/env bash
# SolidSpec common shell functions
# Sourced by other scripts: source "$(dirname "$0")/common.sh"

set -euo pipefail

# Find the project root by walking up to solidspec.toml
get_repo_root() {
    local dir="$PWD"
    while [ "$dir" != "/" ]; do
        if [ -f "$dir/solidspec.toml" ] || [ -d "$dir/.solidspec" ]; then
            echo "$dir"
            return 0
        fi
        dir="$(dirname "$dir")"
    done
    echo "Error: not inside a SolidSpec project" >&2
    return 1
}

# Get the current feature branch or fallback
get_current_branch() {
    # Level 1: SOLIDSPEC_FEATURE env var
    if [ -n "${SOLIDSPEC_FEATURE:-}" ]; then
        echo "$SOLIDSPEC_FEATURE"
        return 0
    fi

    # Level 2: git branch
    if command -v git &>/dev/null && git rev-parse --is-inside-work-tree &>/dev/null; then
        local branch
        branch="$(git rev-parse --abbrev-ref HEAD 2>/dev/null || true)"
        if [[ "$branch" =~ ^[0-9]{3}- ]]; then
            echo "$branch"
            return 0
        fi
    fi

    # Level 3: latest specs/ directory
    local root
    root="$(get_repo_root)" || return 1
    local latest
    latest="$(ls -d "$root"/specs/[0-9][0-9][0-9]-* 2>/dev/null | sort | tail -1 || true)"
    if [ -n "$latest" ]; then
        basename "$latest"
        return 0
    fi

    echo "Error: no feature found" >&2
    return 1
}

# Find a feature directory by numeric prefix
find_feature_dir() {
    local prefix="${1:?Usage: find_feature_dir <prefix>}"
    local root
    root="$(get_repo_root)" || return 1

    local matches
    matches="$(ls -d "$root"/specs/"${prefix}"-* 2>/dev/null || true)"
    local count
    count="$(echo "$matches" | grep -c . || true)"

    if [ "$count" -eq 0 ]; then
        echo "Error: no feature matching '$prefix' in specs/" >&2
        return 1
    elif [ "$count" -eq 1 ]; then
        basename "$matches"
    else
        basename "$(echo "$matches" | sort | tail -1)"
    fi
}

# Output all feature paths as shell variables
get_feature_paths() {
    local root
    root="$(get_repo_root)" || return 1
    local branch
    branch="$(get_current_branch)" || return 1
    local feature_dir="$root/specs/$branch"

    echo "REPO_ROOT=\"$root\""
    echo "CURRENT_BRANCH=\"$branch\""
    echo "HAS_GIT=$(git rev-parse --is-inside-work-tree 2>/dev/null && echo true || echo false)"
    echo "FEATURE_DIR=\"$feature_dir\""
    echo "FEATURE_SPEC=\"$feature_dir/spec.md\""
    echo "IMPL_PLAN=\"$feature_dir/plan.md\""
    echo "TASKS=\"$feature_dir/tasks.md\""
    echo "RESEARCH=\"$feature_dir/research.md\""
    echo "DATA_MODEL=\"$feature_dir/data-model.md\""
    echo "QUICKSTART=\"$feature_dir/quickstart.md\""
    echo "CONTRACTS_DIR=\"$feature_dir/contracts\""
}

# Escape string for safe JSON embedding
json_escape() {
    local str="${1:-}"
    str="${str//\\/\\\\}"
    str="${str//\"/\\\"}"
    str="${str//$'\n'/\\n}"
    str="${str//$'\t'/\\t}"
    str="${str//$'\r'/\\r}"
    echo "$str"
}
