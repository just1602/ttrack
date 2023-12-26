mod cli;
mod cmd;
mod record;

use cli::get_cli_args;
use cmd::handle_report;
use cmd::handle_track;

fn main() {
    let args = get_cli_args();
    let filename = args
        .get_one::<String>("file")
        .expect("Filename must exists.")
        .to_string();

    match args.subcommand() {
        Some(("track", cmd)) => handle_track(cmd.clone(), filename),
        Some(("report", cmd)) => handle_report(cmd.clone(), filename),
        _ => (),
    }
}
