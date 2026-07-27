Read the project context from .solidspec/AGENT.md.

Feature ID: $ARGUMENTS
Find the matching directory under specs/.

`solidspec security-review $ARGUMENTS` has already written a baseline security-review.md
using deterministic OWASP Top 10 heuristics (pattern-matching plan.md for
authentication, authorization, injection, sensitive-data, rate-limiting, and
logging concerns). Read it first, then go deeper than the heuristics can:

1. Read spec.md and plan.md in full.
2. For each OWASP Top 10 category, assess whether the heuristic findings are
   accurate and complete — heuristics can miss context-dependent risks and
   raise false positives.
3. Add any Critical/High/Medium/Low findings the heuristics missed, with a
   concrete remediation for each.
4. Remove or downgrade any heuristic finding that doesn't actually apply,
   with a one-line justification.
5. Update security-review.md with the final findings by severity.

Every Critical or High finding MUST be resolved or explicitly accepted with
justification before `solidspec tasks $ARGUMENTS` is run — the security-first
schema requires every finding to map to a mitigation task.
