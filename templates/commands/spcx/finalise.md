Read the project context from .solidspec/AGENT.md.

Feature: $ARGUMENTS (auto-detected if left empty).

1. Run: `solidspec validate $ARGUMENTS`
   Cross-artifact consistency: requirement traceability (FR-### → plan →
   tasks), entity coverage, constitution compliance. Fix any gaps it reports
   before continuing.
2. Run: `solidspec review $ARGUMENTS`
   Scaffolds review-report.md. Fill it in: check for placeholder text and
   incomplete sections, requirement quality and testability, cross-artifact
   consistency, and security/performance/maintainability concerns — score
   each dimension 0-10.
3. Run: `solidspec ship $ARGUMENTS`
   Parallel fan-out review (code, security, tests, performance) → SHIP or
   HOLD. On HOLD, fix the blocking findings it lists, then re-run this
   command.

Report the final SHIP/HOLD decision plainly at the end.
