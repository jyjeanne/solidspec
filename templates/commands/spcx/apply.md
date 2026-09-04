Read the project context from .solidspec/AGENT.md.

Feature: $ARGUMENTS (auto-detected from the current git branch or latest spec
if left empty).

1. Run: `solidspec status $ARGUMENTS`
   Confirms tasks.md exists and shows which artifacts are ready.
2. Read the feature's tasks.md, spec.md, and plan.md.
3. Execute each task in tasks.md in order, respecting phase dependencies.
   Tasks marked `[P]` can be done in parallel.
4. After completing each task, update tasks.md: change `- [ ]` to `- [x]`
   for that task.
5. Do not create files outside the feature directory.

Next: /spcx:finalise to validate, review, and get a SHIP/HOLD decision.
