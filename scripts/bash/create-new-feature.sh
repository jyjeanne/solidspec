#!/usr/bin/env bash
# Create a new feature branch and directory
source "$(dirname "$0")/common.sh"

description="${1:?Usage: create-new-feature.sh <description>}"
root="$(get_repo_root)"

specs_dir="$root/specs"
mkdir -p "$specs_dir"

# Find next feature number
max_num=0
for dir in "$specs_dir"/[0-9][0-9][0-9]-*/; do
    [ -d "$dir" ] || continue
    # Strip the trailing slash from the glob before taking the basename —
    # ${dir##*/} on "…/001-x/" would yield an empty string and reset numbering.
    num="${dir%/}"
    num="${num##*/}"
    num="${num%%-*}"
    num=$((10#$num))
    if [ "$num" -gt "$max_num" ]; then
        max_num=$num
    fi
done

next_num=$((max_num + 1))
if [ "$next_num" -gt 999 ]; then
    echo "Error: feature number overflow (max 999)" >&2
    exit 1
fi

feature_id=$(printf "%03d" "$next_num")

# Generate short branch name (first 5 words, lowercase, hyphened)
short_name=$(echo "$description" | tr '[:upper:]' '[:lower:]' | tr -cs 'a-z0-9' '-' | sed 's/^-//;s/-$//' | cut -c1-50)
branch_name="${feature_id}-${short_name}"

# Create git branch if in a repo
if git rev-parse --is-inside-work-tree &>/dev/null; then
    git checkout -b "$branch_name" 2>/dev/null || echo "Warning: could not create branch $branch_name"
fi

# Create feature directory
feature_dir="$specs_dir/$branch_name"
mkdir -p "$feature_dir/checklists"
mkdir -p "$feature_dir/contracts"

echo "$branch_name"
