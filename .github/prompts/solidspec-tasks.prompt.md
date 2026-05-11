---
description: "Generate a story-driven task breakdown from the plan"
agent: "agent"
tools: [read, edit, search, execute]
---

Read the project context from .solidspec/AGENT.md.

Feature ID: $ARGUMENTS
Find the matching directory under specs/.
Read spec.md and plan.md.

Fill in tasks.md with concrete, actionable tasks:
1. Define specific tasks with clear deliverables
2. Organize by phases (Setup → Foundational → User Stories → Polish)
3. Mark parallel-safe tasks with [P]
4. Link tasks to user stories with [US1], [US2], etc.
5. Replace all placeholder text with real content
