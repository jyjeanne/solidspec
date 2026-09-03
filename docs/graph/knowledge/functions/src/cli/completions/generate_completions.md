---
type: Rust Function
title: generate_completions
resource: src/cli/completions.rs#L9-L14
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/completions/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/completions/bash_completions_contain_subcommands
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/completions/powershell_completions_generated
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/completions/fish_completions_generated
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/completions/zsh_completions_generated
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn generate_completions(shell: Shell) -> Result<String>`

# Called by

- [run](../../../../functions/src/cli/completions/run.md)
- [bash_completions_contain_subcommands](../../../../functions/src/cli/completions/bash_completions_contain_subcommands.md)
- [powershell_completions_generated](../../../../functions/src/cli/completions/powershell_completions_generated.md)
- [fish_completions_generated](../../../../functions/src/cli/completions/fish_completions_generated.md)
- [zsh_completions_generated](../../../../functions/src/cli/completions/zsh_completions_generated.md)