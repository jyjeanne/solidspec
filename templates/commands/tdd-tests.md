Read the project context from .solidspec/AGENT.md, then execute the TDD RED phase using VERTICAL SLICES — not horizontal batching.

The feature ID is: $ARGUMENTS
Find the matching directory under specs/ (e.g. specs/001-feature-name/).
Read tdd-red-report.md — it contains your interface design template.

STEP 1 — INTERFACE DESIGN (before writing any test code):
Read spec.md and plan.md. For each public API that tests will call:
- Accept external dependencies as parameters (do not create them internally)
- Prefer functions that return results over functions that produce side effects
- Keep interfaces small: fewer public methods = fewer tests needed
Identify MOCK BOUNDARIES — mock ONLY:
- External APIs and HTTP clients
- Databases (prefer a real test DB over mocks when practical)
- Clocks, random sources, file I/O
DO NOT mock your own modules or internal collaborators — those are implementation details.
If framework auto-detection fails (no Cargo.toml / package.json / pyproject.toml / go.mod), STOP and report the failure in tdd-red-report.md — do not guess.

STEP 2 — TRACER BULLET (first cycle):
Pick the single most critical acceptance criterion. Write ONE test for it.
Run it. The test must FAIL for the right reason — a missing implementation, not a compile error or wrong assertion. Fix setup issues until the failure reason is correct before writing more tests.

STEP 3 — REMAINING TESTS (one behavior at a time):
Each acceptance criterion may describe multiple behaviors. Decompose each criterion into individual behaviors. For each behavior:
- Test name describes WHAT: 'user_can_log_in_with_email' not 'calls_verify_password'
- Calls public APIs only — no private methods, no direct DB queries to verify
- Write the test, confirm it compiles and fails, then move to the next

STEP 4 — QUALITY CHECK before filling the report:
Every test must: describe observable behavior (not HOW), use public interface only, survive a complete internal refactor, have one logical assertion or coherent group.
If a test mocks an internal collaborator, rewrite it.

STEP 5 — Fill in tdd-red-report.md:
Record interface decisions, framework, cycle table (one row per behavior), total tests written, total failing, and any unexpectedly passing tests (those signal already-implemented behavior — list them by name, they will be excluded from the implement phase).

FORBIDDEN: Writing any real implementation logic. Tests must fail because the production code does not yet exist.