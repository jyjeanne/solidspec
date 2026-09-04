Read the project context from .solidspec/AGENT.md if this is a SolidSpec
project.

Topic: $ARGUMENTS

Exploratory mode — no commitment, no files written. Think through the
problem before deciding whether to run /spcx:<schema>:new (e.g.
/spcx:sdd:new — see `solidspec schemas` for the short name of every
built-in schema):

1. If a feature is already in progress, run `solidspec status` to see where
   it stands and what's ready next.
2. If a knowledge-graph bundle exists (docs/graph/knowledge/ or
   .solidspec/knowledge/), use it — or the `okf-rs` CLI if installed — to
   understand the relevant part of the codebase before proposing anything.
3. Discuss trade-offs, open questions, and possible approaches for the topic
   above.
4. Do not create or edit any spec/plan/tasks files in this mode.

End with a concrete recommendation: which /spcx:<schema>:new to run and
what it should cover — see `solidspec schemas` for the full list and their
use cases if the project's default schema doesn't fit.
