use crate::app::AragogCliApp;
use clap::{App, ArgEnum, IntoApp, Parser};
use clap_generate::generators::{Bash, Elvish, Fish, PowerShell, Zsh};
use clap_generate::{generate, Generator};

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

fn print_completions<G: Generator>(generator: G, app: &mut App) {
    let name = app.get_name().to_string();
    generate(generator, app, name, &mut std::io::stdout());
}

impl CompletionOptions {
    pub fn generate(&self) {
        let mut app = AragogCliApp::into_app();
        match self.shell_type {
            ShellType::Bash => print_completions(Bash, &mut app),
            ShellType::Elvish => print_completions(Elvish, &mut app),
            ShellType::Fish => print_completions(Fish, &mut app),
            ShellType::PowerShell => print_completions(PowerShell, &mut app),
            ShellType::Zsh => print_completions(Zsh, &mut app),
        };
    }
}
