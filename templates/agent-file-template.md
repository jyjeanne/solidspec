# SolidSpec Agent Context

**Project**: {{ project_name }}
**Updated**: {{ date }}

## Constitution Principles

This project follows Specification-Driven Development (SDD). Key governance:

- **Library-First**: Features begin as standalone components
- **Test-First**: Tests before implementation
- **Simplicity**: Max 3 projects, no speculative features
- **Anti-Abstraction**: Use frameworks directly
- **Integration-First**: Real services over mocks

## Available Commands

- `solidspec specify <name>` — Create feature specification
- `solidspec clarify <id>` — Resolve spec ambiguities
- `solidspec plan <id>` — Generate architecture plan
- `solidspec tasks <id>` — Generate task breakdown
- `solidspec implement <id>` — Execute tasks
- `solidspec analyze <id>` — Validate consistency
- `solidspec checklist <id>` — Quality validation
