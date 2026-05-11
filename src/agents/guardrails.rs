//! Anti-rationalization guardrails injected into every agent prompt.
//! Prevents common shortcuts agents take during SDD workflows.

/// The anti-rationalization table: common excuses agents make, with rebuttals.
/// Appended to every pipeline prompt and command file body.
pub const ANTI_RATIONALIZATION: &str = r#"
## ⚠ Compliance — Before You Skip Any Step

If you catch yourself thinking any of these, **stop and reconsider**:

| If you think... | The reality is... |
|-----------------|-------------------|
| "I already understand what to do" | Undocumented understanding only exists in your context window. The next person or agent won't have it. |
| "I'll add tests after implementation" | Tests written after cover 30% fewer edge cases. Write them first or simultaneously. |
| "This spec section is boilerplate" | Every section serves a purpose. An empty section signals incomplete thinking, not efficiency. |
| "The constitution check is unnecessary here" | Constitution gates are NON-NEGOTIABLE. Plans violating them MUST be revised, not ignored. |
| "I can just infer the missing requirements" | Inferred requirements diverge over time. Make them explicit now with [NEEDS CLARIFICATION] markers. |
| "It works — ship it" | "It works" is not a review. Every change needs spec compliance, security review, and test coverage. |
| "A TODO comment is good enough for now" | TODOs without linked issues are gravestones, not plans. Reference the issue number or remove it. |
| "This is too simple to need a [P] marker" | Parallel safety is not about complexity — it's about data dependencies. Mark it explicitly. |
| "I'll update the docs later" | Docs and code rot at different rates. Update inline docs alongside the code change. |
"#;

/// Mandatory compliance checklist appended to every agent prompt.
pub const COMPLIANCE_CHECKLIST: &str = r#"
## ✅ Mandatory Verification Checklist

Before considering your work complete, verify ALL of the following:

- [ ] All [NEEDS CLARIFICATION] markers resolved or escalated with specific questions
- [ ] Every functional requirement (FR-###) addressed in the output artifact
- [ ] No placeholder text ([TODO], [TBD], [Brief Title]) remains in any file
- [ ] All task checkboxes updated from `- [ ]` to `- [x]` for completed work
- [ ] Constitution gates checked — no violations introduced
- [ ] New files created only within the feature directory (specs/<id>-<name>/)
- [ ] Agent command files not modified unless explicitly asked
- [ ] Acceptance scenarios mapped to testable Given/When/Then format
"#;

/// Full compliance footer: anti-rationalization table + verification checklist.
/// Append this to every pipeline prompt and command file body.
pub fn compliance_footer() -> String {
    format!("{ANTI_RATIONALIZATION}\n{COMPLIANCE_CHECKLIST}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anti_rationalization_table_is_nonempty() {
        assert!(ANTI_RATIONALIZATION.len() > 200);
        assert!(ANTI_RATIONALIZATION.contains("I already understand what to do"));
        assert!(ANTI_RATIONALIZATION.contains("The reality is"));
    }

    #[test]
    fn compliance_checklist_is_nonempty() {
        assert!(COMPLIANCE_CHECKLIST.len() > 100);
        assert!(COMPLIANCE_CHECKLIST.contains("Mandatory Verification Checklist"));
        assert!(COMPLIANCE_CHECKLIST.contains("[NEEDS CLARIFICATION]"));
    }

    #[test]
    fn compliance_footer_contains_both_sections() {
        let footer = compliance_footer();
        assert!(footer.contains("Before You Skip Any Step"));
        assert!(footer.contains("Mandatory Verification Checklist"));
    }
}
