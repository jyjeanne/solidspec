# SolidSpec Feature Context

**Feature:** {{feature_id}}
**Spec:** specs/{{feature_id}}/spec.md
**Plan:** specs/{{feature_id}}/plan.md
**Tasks:** specs/{{feature_id}}/tasks.md

---

## Functional Requirements (from spec.md)

{{spec_requirements_section}}

## User Scenarios (from spec.md)

{{spec_user_stories}}

## Architecture Plan (from plan.md — first 60 lines)

{{plan_summary_section}}

## Pending Tasks (from tasks.md)

{{pending_tasks}}

_({{pending_tasks_count}} pending / {{completed_tasks_count}} done)_

---

_This context was injected by `solidspec apex`. The APEX analyze phase
should treat this as pre-loaded discovery — do NOT re-read these files
unless you need full detail beyond what is shown here. Focus analysis on
the implementation side (existing code, patterns, dependencies) rather
than re-analyzing the spec._
