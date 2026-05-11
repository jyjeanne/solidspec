---
description: "Create a new feature specification"
agent: "agent"
tools: [read, edit, search, execute]
---

Read the project context from .solidspec/AGENT.md.

Feature ID: $ARGUMENTS
Find the matching directory under specs/ (e.g. specs/001-feature-name/).

Fill in the feature's spec.md with real content:
1. Replace [Brief Title] with a descriptive story title
2. Write user stories with clear Given/When/Then acceptance scenarios
3. Define functional requirements (FR-001, FR-002, etc.)
4. Identify key entities and their relationships
5. Define measurable success criteria
6. List edge cases

Keep requirements technology-agnostic. Focus on WHAT, not HOW.
Only edit the existing spec.md — do not create new files.
