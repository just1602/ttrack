use chrono::{Days, Local, NaiveDate};
use clap::{Arg, ArgAction, ArgMatches, Command};

fn tt_clap_date_parser(date: &str) -> Result<NaiveDate, std::io::Error> {
    match date {
        "today" => Ok(Local::now().date_naive()),
        "yesterday" => Ok(Local::now()
            .date_naive()
            .checked_sub_days(Days::new(1))
            .unwrap()),
        _ => Ok(date.parse::<NaiveDate>().unwrap()),
    }
}

pub fn get_cli_args() -> ArgMatches {
    Command::new("ttrack")
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
                        .help("The date since when we want the report to start.")
                        .value_parser(clap::builder::ValueParser::new(tt_clap_date_parser)),
                )
                .arg(
                    Arg::new("until")
                        .long("until")
                        .help("The date until when we want the report to end.")
                        .value_parser(clap::builder::ValueParser::new(tt_clap_date_parser)),
                ),
        )
        .get_matches()
}
