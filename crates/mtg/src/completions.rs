use crate::prelude::*;
use clap::CommandFactory;
use clap_complete::{generate, Generator, Shell};
use std::io;

#[derive(Debug, clap::Parser)]
pub struct App {
    #[command(subcommand)]
    pub command: CompletionCommands,
}

#[derive(Debug, clap::Parser)]
pub enum CompletionCommands {
    /// Generate shell completions
    Generate {
        /// The shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

pub async fn run(app: App, _global: crate::Global) -> Result<()> {
    match app.command {
        CompletionCommands::Generate { shell } => {
            generate_completions_for_shell(shell);
            Ok(())
        }
    }
}

fn generate_completions<G: Generator>(gen: G) {
    let mut cmd = crate::App::command();
    generate(gen, &mut cmd, "mtg", &mut io::stdout());
}

fn generate_completions_for_shell(shell: Shell) {
    match shell {
        Shell::Bash => generate_completions(clap_complete::shells::Bash),
        Shell::Zsh => generate_completions(clap_complete::shells::Zsh),
        Shell::Fish => generate_completions(clap_complete::shells::Fish),
        Shell::PowerShell => generate_completions(clap_complete::shells::PowerShell),
        Shell::Elvish => generate_completions(clap_complete::shells::Elvish),
        _ => {
            eprintln!("Unsupported shell: {shell}");
            std::process::exit(1);
        }
    }
}
