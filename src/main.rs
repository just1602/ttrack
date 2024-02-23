mod cli;
mod cmd;
mod record;

use clap::Parser;
use cli::Cli;
use cli::Command;

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Track(cmd) => cmd::handle_track(cmd, args.file),
        Command::Report(cmd) => cmd::handle_report(cmd, args.file),
    }
}
