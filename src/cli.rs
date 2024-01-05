use chrono::{Days, Local, NaiveDate};
use clap::{Arg, ArgAction, ArgMatches, Command};

fn ttrack_clap_date_parser(date: &str) -> Result<NaiveDate, std::io::Error> {
    match date {
        "today" => Ok(Local::now().date_naive()),
        "yesterday" => Ok(Local::now()
            .date_naive()
            .checked_sub_days(Days::new(1))
            .unwrap()),
        _ => Ok(date.parse::<NaiveDate>().unwrap()),
    }
}

fn ttrack_clap_duration_parser(duration: &str) -> Result<u64, std::io::Error> {
    if let Ok(numeric_duration) = duration.parse::<u64>() {
        return Ok(numeric_duration);
    }

    let mut secs = 0;
    let mut start_idx = 0;
    for (idx, c) in duration.chars().enumerate() {
        match c {
            'h' => {
                secs += duration[start_idx..idx].parse::<u64>().unwrap() * 3600;
                start_idx = idx + 1;
            }
            'm' => {
                secs += duration[start_idx..idx].parse::<u64>().unwrap() * 60;
                start_idx = idx + 1;
            }
            _ => continue,
        }
    }

    Ok(secs)
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
                        .value_parser(clap::builder::ValueParser::new(ttrack_clap_duration_parser))
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
                        .value_parser(clap::builder::ValueParser::new(ttrack_clap_date_parser)),
                )
                .arg(
                    Arg::new("until")
                        .long("until")
                        .help("The date until when we want the report to end.")
                        .value_parser(clap::builder::ValueParser::new(ttrack_clap_date_parser)),
                )
                .arg(
                    Arg::new("today")
                        .long("today")
                        .help("Only report data from today")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("yesterday")
                        .long("yesterday")
                        .help("Only report data from yesterday")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("this-week")
                        .long("this-week")
                        .help("Only report data from the current week (monday to sunday).")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("last-week")
                        .long("last-week")
                        .help("Only report data from the previous week (monday to sunday).")
                        .action(ArgAction::SetTrue),
                ),
        )
        .get_matches()
}

#[cfg(test)]
mod tests {
    use chrono::{Days, Local, NaiveDate};

    use crate::cli::{ttrack_clap_date_parser, ttrack_clap_duration_parser};

    #[test]
    fn ttrack_clap_date_parser_returns_the_date_directly_if_its_a_date() {
        let result = ttrack_clap_date_parser("2024-01-01");

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()
        );
    }

    #[test]
    fn ttrack_clap_date_parser_parse_yesterday_string_correctly() {
        let result = ttrack_clap_date_parser("yesterday");

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Local::now()
                .checked_sub_days(Days::new(1))
                .unwrap()
                .date_naive()
        );
    }

    #[test]
    fn ttrack_clap_date_parser_parse_today_string_correctly() {
        let result = ttrack_clap_date_parser("today");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Local::now().date_naive());
    }

    #[test]
    fn ttrack_clap_duration_parser_parse_duration_directly_if_its_seconds() {
        let result = ttrack_clap_duration_parser("3600");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3600);
    }

    #[test]
    fn ttrack_clap_duration_parser_parse_hours_correctly() {
        let result = ttrack_clap_duration_parser("2h");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 7200);
    }

    #[test]
    fn ttrack_clap_duration_parser_parse_minutes_correctly() {
        let result = ttrack_clap_duration_parser("30m");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1800);
    }

    #[test]
    fn ttrack_clap_duration_parser_parse_mised_hours_and_minutes_correctly() {
        let result = ttrack_clap_duration_parser("1h30m");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5400);
    }
}
