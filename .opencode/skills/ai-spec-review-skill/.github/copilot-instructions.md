# Copilot Instructions

## Commands

- Run the standalone helper: `python3 scripts/review_spec.py path/to/spec.md`

## Architecture

This repository is a **skill package**, not a conventional application. Treat the files by role:

- `SKILL.md` is the source of truth for the skill contract: review scope, output schema, review steps, and behavior rules.
- `README.md` is the install and usage entry point.
- `references/*.md` contains the heuristics the skill is expected to apply while reviewing specifications. The skill now spans spec quality, business logic, architecture, performance, security, testing, DevOps/CI/CD, dependencies, standards, UX, documentation, code quality, and maintainability.
- `scripts/review_spec.py` is a lightweight helper for basic automated checks. It is intentionally much narrower than the full skill contract.
- `docs/samples/**` contains zipped sample artifacts, not runtime code.

## Key conventions

- Preserve the skill identity as `ai-spec-review` unless the change is explicitly about renaming or repackaging the skill.
- Treat `SKILL.md` as authoritative whenever behavior, review dimensions, or output shape change.
- Keep the structured output aligned with the top-level sections defined in `SKILL.md`: `summary`, `issues`, `risk_register`, the domain review sections, `test_plan`, `tasks`, and `score`.
- Keep issue metadata aligned with the enums in `SKILL.md`: severity is `low|medium|high|critical`, confidence is `high|medium|low`, and category is `spec|business_logic|architecture|performance|security|testing|devops|dependencies|standards|ux|documentation|code_quality|maintainability`.
- Keep `score` aligned with the full review surface in `SKILL.md`; if a review dimension is added, update the scoring block in the same change.
- Security findings should map to the OWASP Top 10 where relevant.
- Testing guidance is intentionally opinionated: maintain the test-pyramid bias from `references/testing_best_practices.md` and keep risk-based coverage expectations explicit.
- When adding or changing a review dimension, back it with a repository reference file instead of burying all heuristics inside `SKILL.md`.
- Keep the helper script in its current role as a narrow automation utility unless the task explicitly requires expanding it further; it is a preflight subset that emits only `summary`, `risk_register`, and `issues`, not the full review engine.
- Respect the scoring rubric defined in `SKILL.md`; dimension scores are integers 0–10 with a five-band rubric (0–2 Critical, 3–4 Weak, 5–6 Adequate, 7–8 Good, 9–10 Excellent). `score.overall` is a holistic judgment, not a simple average.
