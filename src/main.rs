use chrono::NaiveDate;
use clap::ArgAction;
use clap::{Arg, ArgMatches, Command};

mod cmd;
mod record;
use cmd::handle_report;
use cmd::handle_track;

fn get_cli_args() -> ArgMatches {
    Command::new("tt")
        .version("0.0.1-dev")
        .about("A simple, but yet powerfull time tracker.")
        .arg(
            Arg::new("file")
                .short('f')
                .help("File where to store tracking data.")
                .required(true),
        )
        .subcommand(
            Command::new("track")
                .about("Track a new time record.")
                .arg(
                    Arg::new("time")
                        .short('t')
                        .help("The time duration of the record in seconds.")
                        .value_parser(clap::value_parser!(u64))
                        .required(true),
                )
                .arg(
                    Arg::new("description")
                        .short('d')
                        .help("The description of the time record (what has been done).")
                        .required(true),
                )
                .arg(
                    Arg::new("project")
                        .short('p')
                        .help("The project with which this record is associated with."),
                ),
        )
        .subcommand(
            Command::new("report")
                .about("Generate report from the records.")
                .arg(
                    Arg::new("by-project")
                        .long("by-project")
                        .help("Report time by project.")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("since")
                        .long("since")
                        .help("The date since when we wnat the report to start.")
                        .value_parser(clap::value_parser!(NaiveDate)),
                ),
        )
        .get_matches()
}

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
