Read the project context from .solidspec/AGENT.md.

Feature description: $ARGUMENTS

This is the full "start a feature" flow for the default (spec-driven) schema
— spec → clarify → plan → tasks → tests — run in one pass instead of
running each phase's command separately.

1. Run: `solidspec pipeline --new "$ARGUMENTS" --to tests --no-agent --auto`
   This scaffolds spec.md, plan.md (+ research.md/data-model.md/contracts/api.md/quickstart.md),
   tasks.md, and tests/ with placeholder content under the new specs/NNN-slug/
   directory it creates and prints. Note that directory name — you need it
   for every step below.
2. Fill in spec.md: replace [Brief Title], write Given/When/Then user
   stories, define FR-### functional requirements, key entities, measurable
   success criteria, and edge cases. Technology-agnostic — WHAT, not HOW.
3. Resolve every [NEEDS CLARIFICATION] marker left in spec.md: propose a
   concrete resolution and update the text in place.
4. Fill in plan.md, research.md, data-model.md, contracts/api.md, and
   quickstart.md with real architecture decisions, tech stack, entities, and
   a completed Constitution Check.
5. Fill in tasks.md: concrete tasks organized Setup → Foundational → User
   Stories → Polish, `[P]` for parallel-safe tasks, `[US#]` linking tasks to
   user stories.
6. Fill in the test scaffolds under tests/: real assertions for each
   Given/When/Then scenario, plus edge cases from the spec.

Next: /spcx:apply to implement, or /spcx:explore to discuss the plan first.
