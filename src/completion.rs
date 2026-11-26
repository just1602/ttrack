use std::{
    ffi::{OsStr, OsString},
    io,
};

use clap::{CommandFactory, Parser};
use clap_complete::{Shell, generate};

use crate::cli::Cli;

#[derive(Parser)]
#[command(
    name = "completion",
    about = "Generate shell completion scripts",
    hide = true
)]
struct CompletionCli {
    #[arg(value_enum, help = "The shell to generate completions for")]
    shell: Shell,
}

pub fn try_run_from(args: &[OsString]) -> bool {
    if !args
        .get(1)
        .is_some_and(|arg| arg.as_os_str() == OsStr::new("completion"))
    {
        return false;
    }

    let program = args.first().cloned().unwrap_or_default();
    let completion_args = std::iter::once(program)
        .chain(args.iter().skip(2).cloned())
        .collect::<Vec<_>>();
    let CompletionCli { shell } = CompletionCli::parse_from(completion_args);

    let mut command = Cli::command();
    generate(shell, &mut command, "ttrack", &mut io::stdout().lock());

    true
}
