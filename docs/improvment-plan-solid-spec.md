Yes. Based on the IDSD article, SolidSpec appears to be a particularly good candidate because it is already centered around specifications. The main shift would be changing the source of truth from **"what the system should contain"** (specification) to **"why the system exists and what outcome it must create"** (intent). The IDSD argument is that specifications become stale because they describe a solution too early, while intent remains stable and can generate multiple evolving implementations. ([Medium][1])

I cannot fully verify your repository architecture because the repository contents were not accessible through search indexing, but I can provide a practical IDSD adaptation plan for SolidSpec.

The repository:

[SolidSpec Repository](https://github.com/jyjeanne/solidspec?utm_source=chatgpt.com)

---

# Suggested improvment plan

```markdown
# Intent-Driven Development (IDSD) Adoption for SolidSpec

## Purpose

SolidSpec exists to help developers create, validate, evolve, and synchronize software specifications while reducing ambiguity between human intent and implementation.

Rather than treating specifications as the primary artifact, SolidSpec treats Intent as the primary artifact and specifications as generated operational views of that intent.

---

# Core Intent

## Intent Statement

Enable developers and AI agents to build software that remains aligned with business goals, user outcomes, and architectural constraints throughout the software lifecycle.

---

# Desired Outcomes

### O1 — Clarity

Every feature should have a clearly defined purpose.

Success Criteria:

- Human reviewer understands feature purpose within 2 minutes.
- AI agent can explain feature objective before implementation.

---

### O2 — Traceability

Every implementation decision must be traceable to an intent.

Success Criteria:

- Code → Spec → Intent mapping exists.
- Pull Requests reference intent identifiers.

---

### O3 — Evolvability

Requirements can change without rewriting large specification trees.

Success Criteria:

- Intent remains stable across implementation revisions.
- New implementations can be regenerated from existing intent.

---

### O4 — Agent Compatibility

AI agents can operate autonomously while preserving user intent.

Success Criteria:

- Intent artifacts contain goals, constraints, and success criteria.
- Agents can derive implementation plans automatically.

---

# Intent Model

Intent consists of three layers.

## I — Goal

Why the capability exists.

Example:

"Allow users to validate specifications automatically."

---

## C — Constraints

Boundaries that must remain true.

Examples:

- Must support Markdown
- Must be deterministic
- Must be CI-compatible

---

## E — Evidence

How success is measured.

Examples:

- Validation passes
- Generated report produced
- Test coverage above threshold

---

# Intent Artifact Structure

/intents

    001-spec-validation.md
    002-agent-planning.md
    003-traceability.md

Each intent contains:

- Goal
- Constraints
- Evidence
- Risks
- Open Questions

---

# Workflow

## Step 1

Capture Intent

Output:

intent.md

---

## Step 2

Generate Operational Specification

Output:

spec.md

---

## Step 3

Generate Tasks

Output:

tasks.md

---

## Step 4

Implementation

Output:

code

---

## Step 5

Evidence Collection

Output:

tests
metrics
reports

---

## Step 6

Intent Review

Questions:

- Did implementation satisfy intent?
- Did constraints remain valid?
- Is intent still correct?

---

# Traceability Matrix

Intent → Specification → Tasks → Code → Tests

Every artifact references parent intent ID.

Example:

INT-001
 └─ SPEC-001
     └─ TASK-001
         └─ PR-001
             └─ TEST-001

---

# AI Agent Rules

Agents must:

- Read intent before specification
- Explain intent before coding
- Produce evidence after implementation
- Refuse implementation when intent is ambiguous

---

# Success Metrics

- Intent coverage %
- Spec coverage %
- Test coverage %
- Intent drift score
- Requirement volatility score

```

---

# How IDSD Fits SolidSpec

From the article, the strongest idea is the **ICE model**:

* Intent
* Constraints
* Evidence

instead of traditional requirements-first specification writing. ([Medium][2])

For SolidSpec, I would add three major improvements.

## Improvement 1 — Intent Files Become First-Class

Current specification tools often store:

```text
/specs
```

Add:

```text
/intents
```

Example:

```text
/intents
    INT-001-user-auth.md
    INT-002-reporting.md
```

Each intent produces one or more specifications.

Relationship:

```text
Intent
   ↓
Specification
   ↓
Tasks
   ↓
Code
```

---

## Improvement 2 — Intent Drift Detection

One weakness of SDD noted by both supporters and critics is that specs become outdated. ([Medium][1])

SolidSpec could calculate:

```text
Intent Drift Score
```

Example:

```yaml
intent:
  user_can_export_pdf

implementation:
  supports_pdf: false

drift: 100%
```

This would be a strong differentiator from Spec Kit style systems.

---

## Improvement 3 — Evidence-Driven Validation

Instead of:

```text
Spec passes
```

Use:

```text
Intent satisfied
```

Evidence sources:

* Tests
* Benchmarks
* Acceptance checks
* User feedback

Example:

```yaml
intent: reduce_report_generation_time

evidence:
  avg_generation_time: 1.2s

status: satisfied
```

---

# My Assessment

Can SolidSpec adopt IDSD?

**Yes, very naturally.**

In fact, SolidSpec could evolve into:

```text
Intent
  → Spec
  → Task
  → Implementation
  → Evidence
```

which is closer to the operational model described in the IDSD articles than traditional Spec-Driven Development. ([Medium][1])

The most valuable enhancement would be adding:

1. Intent artifacts (`/intents`)
2. ICE schema (Intent, Constraints, Evidence)
3. Intent drift analysis
4. Evidence-based validation
5. Traceability graph (Intent → Code → Tests)

Those additions would move SolidSpec from a specification management tool toward a full Intent-Driven Development platform.

[1]: https://medium.com/activated-thinker/the-method-that-replaces-spec-driven-development-idsd-66e921f6cdf7?utm_source=chatgpt.com "The Method That Replaces Spec-Driven Development — IDSD | by Kapil Viren Ahuja | Activated Thinker | May, 2026 | Medium"
[2]: https://medium.com/activated-thinker/the-anatomy-of-intent-ice-in-idsd-built-from-where-spec-driven-breaks-1597e5a16659?utm_source=chatgpt.com "The Anatomy of Intent (ICE in IDSD). Built from Where Spec-Driven Breaks. | by Kapil Viren Ahuja | Activated Thinker | May, 2026 | Medium"
