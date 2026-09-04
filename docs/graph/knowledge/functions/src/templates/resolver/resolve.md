---
type: Rust Function
title: resolve
resource: src/templates/resolver.rs#L54-L114
generated:
  by: okf-rs/0.7.0
---

# Signature

`pub fn resolve( template_name: &str, project_root: &Path, preset_priorities: &[(String, u32)], // (preset_id, priority) sorted by priority ) -> ResolvedTemplate`