---
description: "Review spec quality with preflight heuristics"
tools: [read, edit, search, execute]
argument-hint: "Feature ID (e.g. 001-feature-name)"
---

Read the project context from .solidspec/AGENT.md.

Feature ID: $ARGUMENTS
Find the matching directory under specs/.

Perform a comprehensive spec quality review:
1. Check for placeholder text and incomplete sections
2. Validate requirement quality and testability
3. Check cross-artifact consistency (spec → plan → tasks)
4. Assess security, performance, and maintainability concerns
5. Write findings to $ARGUMENTS/review-report.md
