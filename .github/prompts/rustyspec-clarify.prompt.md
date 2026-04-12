---
description: "Resolve ambiguities in a specification"
agent: "agent"
tools: [read, edit, search, execute]
---

Read the project context from .rustyspec/AGENT.md.

Feature ID: $ARGUMENTS
Find the matching directory under specs/.

Read spec.md and find all [NEEDS CLARIFICATION] markers.
For each marker:
1. Identify the ambiguity
2. Propose a resolution based on best practices
3. Update spec.md with the resolution
4. Remove the [NEEDS CLARIFICATION] marker
