use std::io::Error;

use clap::CommandFactory;
use clap_complete::generate_to;
use clap_complete::shells::Shell;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    let mut cmd = Cli::command();

    for &shell in &vec![Shell::Bash, Shell::Zsh, Shell::Fish] {
        generate_to(shell, &mut cmd, "ttrack", "dist")?;
    }

    Ok(())
}
