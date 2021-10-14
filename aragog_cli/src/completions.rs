use crate::app::AragogCliApp;
use clap::{App, ArgEnum, Clap, IntoApp};
use clap_generate::generators::*;
use clap_generate::{generate, Generator};

#[derive(Clap, Debug)]
pub struct CompletionOptions {
    /// target shell type
    #[clap(arg_enum, default_value = "bash")]
    pub shell_type: ShellType,
}
#[derive(Clap, ArgEnum, Debug)]
pub enum ShellType {
    Bash,
    Elvish,
    Fish,
    #[clap(name = "powershell")]
    PowerShell,
    Zsh,
}

fn print_completions<G: Generator>(app: &mut App) {
    let name = app.get_name().to_string();
    generate::<G, _>(app, name, &mut std::io::stdout());
}

impl CompletionOptions {
    pub fn generate(&self) {
        let mut app = AragogCliApp::into_app();
        match self.shell_type {
            ShellType::Bash => print_completions::<Bash>(&mut app),
            ShellType::Elvish => print_completions::<Elvish>(&mut app),
            ShellType::Fish => print_completions::<Fish>(&mut app),
            ShellType::PowerShell => print_completions::<PowerShell>(&mut app),
            ShellType::Zsh => print_completions::<Zsh>(&mut app),
        };
    }
}
