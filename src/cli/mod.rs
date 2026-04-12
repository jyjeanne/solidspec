pub mod analyze;
pub mod check;
pub mod checklist;
pub mod clarify;
pub mod completions;
pub mod extension;
pub mod implement;
pub mod init;
pub mod pipeline;
pub mod plan;
pub mod preset;
pub mod review;
pub mod specify;
pub mod tasks;
pub mod tests_cmd;
pub mod upgrade;
pub mod ux;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "rustyspec",
    version,
    about = "Specification-Driven Development CLI"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Verbose debug output
    #[arg(long, global = true)]
    pub debug: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new RustySpec project
    Init {
        /// Project name (initializes in current directory if omitted)
        name: Option<String>,

        /// Initialize in current directory
        #[arg(long)]
        here: bool,

        /// Skip Git repository initialization
        #[arg(long)]
        no_git: bool,

        /// Skip confirmation prompts
        #[arg(long)]
        force: bool,

        /// Target AI agent (e.g., copilot, claude, cursor). Auto-detected if omitted.
        #[arg(long)]
        agent: Option<String>,
    },

    /// Create a new feature specification
    Specify {
        /// Feature description
        #[arg(name = "feature-name")]
        feature_name: String,
    },

    /// Resolve ambiguities in a specification
    Clarify {
        /// Feature ID (e.g., 001) — auto-detected if omitted
        feature_id: Option<String>,
    },

    /// Generate an architecture plan from a specification
    Plan {
        /// Feature ID (e.g., 001) — auto-detected if omitted
        feature_id: Option<String>,
    },

    /// Generate a story-driven task breakdown from the plan
    Tasks {
        /// Feature ID (e.g., 001) — auto-detected if omitted
        feature_id: Option<String>,
    },

    /// Execute tasks from the task breakdown
    Implement {
        /// Feature ID (e.g., 001) — auto-detected if omitted
        feature_id: Option<String>,

        /// Multi-pass implementation (for iterative refinement)
        #[arg(long)]
        pass: Option<u32>,
    },

    /// Generate test scaffolds from acceptance scenarios
    Tests {
        /// Feature ID (e.g., 001) — auto-detected if omitted
        feature_id: Option<String>,

        /// Override auto-detected test framework (jest, pytest, cargo, go, generic)
        #[arg(long)]
        framework: Option<String>,

        /// Override test output directory
        #[arg(long)]
        output: Option<String>,

        /// Preview without writing files
        #[arg(long)]
        dry_run: bool,
    },

    /// Validate cross-artifact consistency (read-only)
    Analyze {
        /// Feature ID (e.g., 001) — auto-detected if omitted
        feature_id: Option<String>,
    },

    /// Review spec quality with preflight heuristics
    Review {
        /// Feature ID (e.g., 001) — auto-detected if omitted
        feature_id: Option<String>,
    },

    /// Generate a quality validation checklist
    Checklist {
        /// Feature ID (e.g., 001) — auto-detected if omitted
        feature_id: Option<String>,

        /// Append to existing checklist instead of creating new
        #[arg(long)]
        append: bool,
    },

    /// Run the multi-agent SDD pipeline
    Pipeline {
        /// Feature ID (auto-detected if omitted; mutually exclusive with --new)
        feature_id: Option<String>,

        /// Create a new feature and run full pipeline
        #[arg(long, value_name = "DESCRIPTION")]
        new: Option<String>,

        /// Start from this phase
        #[arg(long)]
        from: Option<String>,

        /// Stop after this phase
        #[arg(long)]
        to: Option<String>,

        /// Run a single phase only
        #[arg(long)]
        only: Option<String>,

        /// Re-run phases even if artifacts exist
        #[arg(long)]
        force: bool,

        /// Preview without executing
        #[arg(long)]
        dry_run: bool,

        /// Skip user confirmation at handoff phases
        #[arg(long)]
        auto: bool,

        /// Scaffold only — skip AI agent invocation (generate templates without filling)
        #[arg(long)]
        no_agent: bool,
    },

    /// Manage workflow presets
    Preset {
        #[command(subcommand)]
        command: preset::PresetCommands,
    },

    /// Manage extensions
    Extension {
        #[command(subcommand)]
        command: extension::ExtensionCommands,
    },

    /// Refresh templates and scripts after a RustySpec update
    Upgrade {
        /// Skip confirmation prompts
        #[arg(long)]
        force: bool,
    },

    /// Generate shell completions
    Completions {
        /// Shell type: bash, zsh, fish, powershell
        shell: String,
    },

    /// Verify system prerequisites
    Check,
}

pub fn run(cli: Cli) -> Result<()> {
    // Logger already initialized in main() based on --debug flag

    match cli.command {
        Commands::Init {
            name,
            here,
            no_git,
            force,
            agent,
        } => init::run(name, here, no_git, force, agent),
        Commands::Specify { feature_name } => specify::run(&feature_name),
        Commands::Clarify { feature_id } => clarify::run(feature_id.as_deref()),
        Commands::Plan { feature_id } => plan::run(feature_id.as_deref()),
        Commands::Tasks { feature_id } => tasks::run(feature_id.as_deref()),
        Commands::Implement { feature_id, pass } => implement::run(feature_id.as_deref(), pass),
        Commands::Tests {
            feature_id,
            framework,
            output,
            dry_run,
        } => tests_cmd::run(
            feature_id.as_deref(),
            framework.as_deref(),
            output.as_deref(),
            dry_run,
        ),
        Commands::Analyze { feature_id } => analyze::run(feature_id.as_deref()),
        Commands::Review { feature_id } => review::run(feature_id.as_deref()),
        Commands::Checklist { feature_id, append } => checklist::run(feature_id.as_deref(), append),
        Commands::Pipeline {
            feature_id,
            new,
            from,
            to,
            only,
            force,
            dry_run,
            auto,
            no_agent,
        } => pipeline::run(
            feature_id.as_deref(),
            new.as_deref(),
            from.as_deref(),
            to.as_deref(),
            only.as_deref(),
            force,
            dry_run,
            auto,
            no_agent,
        ),
        Commands::Preset { command } => preset::run(command),
        Commands::Extension { command } => extension::run(command),
        Commands::Upgrade { force } => upgrade::run(force),
        Commands::Completions { shell } => completions::run(&shell),
        Commands::Check => check::run(),
    }
}
