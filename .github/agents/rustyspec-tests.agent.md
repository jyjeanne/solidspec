---
description: "Generate test scaffolds from acceptance scenarios"
tools: [read, edit, search, execute]
argument-hint: "Feature ID (e.g. 001-feature-name)"
---

Read the project context from .rustyspec/AGENT.md.

Feature ID: $ARGUMENTS
Find the matching directory under specs/.
Read spec.md for acceptance scenarios.

Review and enhance test scaffolds in the feature's tests/ directory:
1. Add concrete test implementations for each Given/When/Then scenario
2. Replace placeholder text with real test assertions
3. Add edge case tests based on the spec
4. Ensure tests are runnable with the project's test framework
