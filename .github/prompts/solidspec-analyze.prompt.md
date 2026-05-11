---
description: "Validate cross-artifact consistency"
agent: "agent"
tools: [read, edit, search, execute]
---

Read the project context from .solidspec/AGENT.md.

Feature ID: $ARGUMENTS
Find the matching directory under specs/.

Validate cross-artifact consistency:
1. Check that plan.md addresses all requirements from spec.md
2. Check that tasks.md covers all planned work
3. Check that tests cover all acceptance scenarios
4. Report any gaps or inconsistencies
