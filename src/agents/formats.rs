use super::config::AgentFormat;

/// Translate argument placeholders between agent formats.
pub fn translate_placeholder(content: &str, target_placeholder: &str) -> String {
    let mut result = content.to_string();

    if target_placeholder == "{{args}}" {
        // Markdown → TOML: replace $ARGUMENTS with {{args}}
        // Only replace if not already translated
        if result.contains("$ARGUMENTS") {
            result = result.replace("$ARGUMENTS", "{{args}}");
        }
    } else if target_placeholder == "$ARGUMENTS" {
        // TOML → Markdown: replace {{args}} with $ARGUMENTS
        if result.contains("{{args}}") {
            result = result.replace("{{args}}", "$ARGUMENTS");
        }
    }

    result
}

/// Render a command file in Markdown format with YAML frontmatter.
pub fn render_markdown(description: &str, body: &str) -> String {
    format!("---\ndescription: \"{description}\"\n---\n\n{body}\n")
}

/// Render a command file in TOML format.
pub fn render_toml(description: &str, body: &str) -> String {
    format!("description = \"{description}\"\n\nprompt = \"\"\"\n{body}\n\"\"\"\n")
}

/// Render a command for the given agent format.
pub fn render_command(format: AgentFormat, description: &str, body: &str) -> String {
    match format {
        AgentFormat::Markdown => render_markdown(description, body),
        AgentFormat::Toml => render_toml(description, body),
    }
}

/// Adjust script paths from relative template paths to installed paths.
pub fn adjust_script_paths(content: &str) -> String {
    content.replace("../../scripts/", ".rustyspec/scripts/")
}

/// Render a Copilot `.agent.md` file with proper VS Code custom agent frontmatter.
/// Includes `tools:` aliases and `argument-hint` for agent picker UX.
pub fn render_copilot_agent(description: &str, body: &str) -> String {
    format!(
        "---\n\
         description: \"{description}\"\n\
         tools: [read, edit, search, execute]\n\
         argument-hint: \"Feature ID (e.g. 001-feature-name)\"\n\
         ---\n\n\
         {body}\n"
    )
}

/// Render a Copilot `.prompt.md` file with `agent: \"agent\"` mode and tools.
pub fn render_copilot_prompt(description: &str, body: &str) -> String {
    format!(
        "---\n\
         description: \"{description}\"\n\
         agent: \"agent\"\n\
         tools: [read, edit, search, execute]\n\
         ---\n\n\
         {body}\n"
    )
}

/// Render a Vibe skill SKILL.md with the required frontmatter fields.
pub fn render_vibe_skill(cmd_name: &str, description: &str, body: &str) -> String {
    let name = standard_command_name(cmd_name);
    format!(
        "---\n\
         name: {name}\n\
         description: \"{description}\"\n\
         user-invocable: true\n\
         allowed-tools:\n\
         \x20 - read_file\n\
         \x20 - write_file\n\
         \x20 - edit_file\n\
         \x20 - bash\n\
         \x20 - grep\n\
         \x20 - glob\n\
         ---\n\n\
         {body}\n"
    )
}

/// Generate a Kimi dot-separator command name (rustyspec.specify).
pub fn kimi_command_name(cmd: &str) -> String {
    format!("rustyspec.{cmd}")
}

/// Generate a standard hyphen-separator command name (rustyspec-specify).
pub fn standard_command_name(cmd: &str) -> String {
    format!("rustyspec-{cmd}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translate_to_toml_replaces_arguments() {
        let input = "Read specs/$ARGUMENTS/spec.md";
        let result = translate_placeholder(input, "{{args}}");
        assert_eq!(result, "Read specs/{{args}}/spec.md");
        assert!(!result.contains("$ARGUMENTS"));
    }

    #[test]
    fn translate_to_markdown_replaces_args() {
        let input = "Read specs/{{args}}/spec.md";
        let result = translate_placeholder(input, "$ARGUMENTS");
        assert_eq!(result, "Read specs/$ARGUMENTS/spec.md");
        assert!(!result.contains("{{args}}"));
    }

    #[test]
    fn no_double_replacement() {
        let already_translated = "Read specs/{{args}}/spec.md";
        let result = translate_placeholder(already_translated, "{{args}}");
        assert_eq!(result, already_translated, "Should not double-replace");
    }

    #[test]
    fn markdown_has_frontmatter_delimiters() {
        let output = render_markdown("Test command", "Do something with $ARGUMENTS");
        assert!(output.starts_with("---\n"));
        assert!(output.contains("description: \"Test command\""));
        assert!(output.contains("---\n\n"));
        assert!(output.contains("Do something with $ARGUMENTS"));
    }

    #[test]
    fn toml_has_description_and_prompt() {
        let output = render_toml("Test command", "Do something with {{args}}");
        assert!(output.contains("description = \"Test command\""));
        assert!(output.contains("prompt = \"\"\""));
        assert!(output.contains("Do something with {{args}}"));
    }

    #[test]
    fn adjust_script_paths_replaces() {
        let input =
            "scripts:\n  sh: ../../scripts/bash/setup.sh\n  ps: ../../scripts/powershell/setup.ps1";
        let result = adjust_script_paths(input);
        assert!(result.contains(".rustyspec/scripts/bash/setup.sh"));
        assert!(result.contains(".rustyspec/scripts/powershell/setup.ps1"));
        assert!(!result.contains("../../scripts/"));
    }

    #[test]
    fn already_adjusted_paths_not_double_adjusted() {
        let input = ".rustyspec/scripts/bash/setup.sh";
        let result = adjust_script_paths(input);
        assert_eq!(result, input);
    }

    #[test]
    fn vibe_skill_has_required_frontmatter() {
        let output = render_vibe_skill("specify", "Create a spec", "Do something with $ARGUMENTS");
        assert!(output.starts_with("---\n"));
        assert!(output.contains("name: rustyspec-specify"));
        assert!(output.contains("user-invocable: true"));
        assert!(output.contains("allowed-tools:"));
        assert!(output.contains("- read_file"));
        assert!(output.contains("- write_file"));
        assert!(output.contains("Do something with $ARGUMENTS"));
    }

    #[test]
    fn kimi_dot_separator_naming() {
        assert_eq!(kimi_command_name("specify"), "rustyspec.specify");
        assert_eq!(kimi_command_name("plan"), "rustyspec.plan");
    }

    #[test]
    fn standard_hyphen_separator_naming() {
        assert_eq!(standard_command_name("specify"), "rustyspec-specify");
        assert_eq!(standard_command_name("plan"), "rustyspec-plan");
    }
}
