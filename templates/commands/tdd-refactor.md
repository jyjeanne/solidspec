Read the project context from .solidspec/AGENT.md, then execute the TDD REFACTOR phase.

The feature ID is: $ARGUMENTS
Find the matching directory under specs/ (e.g. specs/001-feature-name/).

PRE-CONDITION: Run the full test suite. Every test must be GREEN before starting.
If any test is failing, STOP and return to the implement phase.

REFACTOR candidates — work through in this priority order:
1. Duplication → extract to a shared function or class
2. Long methods → extract private helpers (keep tests targeting the public interface, not the extracted helpers)
3. Shallow modules → deepen: reduce public methods, hide more complexity inside
4. Feature envy → move logic to where the data lives
5. Primitive obsession → introduce value objects for domain concepts
6. Existing code that the new code reveals as problematic

INTERFACE RULE: the public surface area must stay the same size or shrink. Do not add new public methods during refactor.

After EVERY individual change: run the full test suite. Every test must remain GREEN. If any test goes RED, revert the change.

Fill in tdd-refactor-report.md: for each change, record the file, the refactor type (from the list above), a before/after description, and the test run result (must be GREEN).

FORBIDDEN: Changing test code. FORBIDDEN: Adding new behavior. FORBIDDEN: Expanding the public interface.