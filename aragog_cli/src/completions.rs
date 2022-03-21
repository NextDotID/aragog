use crate::app::AragogCliApp;
use clap::{ArgEnum, Command, IntoApp, Parser};
use clap_complete::Shell::{Bash, Elvish, Fish, PowerShell, Zsh};
use clap_complete::{generate, Generator};

#[derive(Parser, Debug)]
pub struct CompletionOptions {
    /// target shell type
    #[clap(arg_enum, default_value = "bash")]
    pub shell_type: ShellType,
}
#[derive(Parser, ArgEnum, Debug, Copy, Clone)]
pub enum ShellType {
    Bash,
    Elvish,
    Fish,
    #[clap(name = "powershell")]
    PowerShell,
    Zsh,
}

fn print_completions<G: Generator>(generator: G, command: &mut Command) {
    let name = command.get_name().to_string();
    generate(generator, command, name, &mut std::io::stdout());
}

impl CompletionOptions {
    pub fn generate(&self) {
        let mut command = AragogCliApp::command();
        match self.shell_type {
            ShellType::Bash => print_completions(Bash, &mut command),
            ShellType::Elvish => print_completions(Elvish, &mut command),
            ShellType::Fish => print_completions(Fish, &mut command),
            ShellType::PowerShell => print_completions(PowerShell, &mut command),
            ShellType::Zsh => print_completions(Zsh, &mut command),
        };
    }
}
