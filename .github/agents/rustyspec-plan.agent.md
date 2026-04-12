---
description: "Generate an architecture plan from a specification"
tools: [read, edit, search, execute]
argument-hint: "Feature ID (e.g. 001-feature-name)"
---

Read the project context from .rustyspec/AGENT.md.

Feature ID: $ARGUMENTS
Find the matching directory under specs/ and read spec.md for requirements.

Fill in the planning documents with real content:
1. plan.md — Architecture decisions, tech stack, project structure, constitution check
2. research.md — Technology investigation findings
3. data-model.md — Entity definitions and relationships
4. contracts/api.md — API contracts if applicable
5. quickstart.md — Key validation scenarios

Complete the Constitution Check in plan.md.
Fill all [NEEDS CLARIFICATION] and [To be filled] sections with concrete content.
