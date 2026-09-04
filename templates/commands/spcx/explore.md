Read the project context from .solidspec/AGENT.md if this is a SolidSpec
project.

Topic: $ARGUMENTS

Exploratory mode — no commitment, no files written. Think through the
problem before deciding whether to run /spcx:new:

1. If a feature is already in progress, run `solidspec status` to see where
   it stands and what's ready next.
2. If a knowledge-graph bundle exists (docs/graph/knowledge/ or
   .solidspec/knowledge/), use it — or the `okf-rs` CLI if installed — to
   understand the relevant part of the codebase before proposing anything.
3. Discuss trade-offs, open questions, and possible approaches for the topic
   above.
4. Do not create or edit any spec/plan/tasks files in this mode.

End with a concrete recommendation: what /spcx:new should cover, or — if
the default schema doesn't fit (see `solidspec schemas` for the full list
and their use cases) — which one to use instead via `solidspec pipeline
--schema <name>`.
