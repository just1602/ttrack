mod cli;
mod handlers;
mod record;

use clap::Parser;
use cli::Cli;
use cli::Command;

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Track(cmd) => handlers::handle_track(cmd, args.file),
        Command::Report(cmd) => handlers::handle_report(cmd, args.file),
    }
}
