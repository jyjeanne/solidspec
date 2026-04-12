---
description: "Execute tasks from the task breakdown"
agent: "agent"
tools: [read, edit, search, execute]
---

Read the project context from .rustyspec/AGENT.md, then implement the feature.

The feature ID is: $ARGUMENTS
Find the matching directory under specs/ (e.g. specs/001-feature-name/).

Steps:
1. Read the feature's tasks.md for the task list
2. Read the feature's spec.md for requirements and acceptance criteria
3. Read the feature's plan.md for architecture decisions
4. Execute each task in order, respecting phase dependencies
5. Tasks marked [P] can be done in parallel
6. After completing each task, update tasks.md: change `- [ ]` to `- [x]` for that task
7. When all tasks are done, run /rustyspec-analyze to validate
