Read the project context from .solidspec/AGENT.md, then launch the APEX implementation workflow for the feature.

The feature ID is: $ARGUMENTS
Find the matching directory under specs/ (e.g. specs/001-feature-name/).

SolidSpec context is in: .solidspec/apex-context.md
(Pre-loaded requirements, architecture plan, and pending tasks.)

APEX workflow (Analyze-Plan-Execute-eXamine):
1. Analyze: Read spec.md, plan.md, and tasks.md — context file has summaries
2. Plan: Create a file-by-file implementation strategy
3. Execute: Implement each task from tasks.md one at a time
- After each task, update tasks.md: change `- [ ]` to `- [x]`
- Tasks marked [P] can be done in parallel
4. Validate: Run type checking and tests; verify acceptance criteria
5. eXamine (optional): Adversarial review for security and quality

If the /apex skill is installed in this agent, invoke it directly:
/apex -a -s implement feature: <feature-slug>

When all tasks are done, run /solidspec-analyze to validate.