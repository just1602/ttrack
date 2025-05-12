use std::path::PathBuf;

use chrono::{Days, Local, NaiveDate};
use clap::{Parser, Subcommand};
use std::time::Duration;

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

fn ttrack_clap_duration_parser(duration: &str) -> Result<Duration, std::io::Error> {
    if let Ok(numeric_duration) = duration.parse::<u64>() {
        return Ok(Duration::from_secs(numeric_duration));
    }

    let mut secs = 0;
    let mut start_idx = 0;
    for (idx, c) in duration.char_indices() {
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

    Ok(Duration::from_secs(secs))
}

#[derive(Parser)]
#[command(name = "track", visible_alias = "t", about = "Track a new time record")]
pub struct TrackCommand {
    #[arg(
        long,
        help = "The date the record is created",
        value_parser = clap::builder::ValueParser::new(ttrack_clap_date_parser),
        default_value_t = Local::now().date_naive(),
    )]
    pub date: NaiveDate,

    #[arg(
        short,
        long,
        help = "The time duration of the record",
        value_parser = clap::builder::ValueParser::new(ttrack_clap_duration_parser),
        required = true
    )]
    pub time: Duration,

    #[arg(
        short,
        long,
        help = "The description of the time record (what has been done)",
        required = true
    )]
    pub description: String,

    #[arg(
        short,
        long,
        help = "The project with which this record is associated with"
    )]
    pub project: Option<String>,
}

#[derive(Parser)]
#[command(
    name = "report",
    visible_alias = "r",
    about = "Generate report from the records"
)]
pub struct ReportCommand {
    #[arg(
        long,
        help = "The date since when we want the report to start",
        value_parser = clap::builder::ValueParser::new(ttrack_clap_date_parser)
    )]
    pub since: Option<NaiveDate>,

    #[arg(
        long,
        help = "The date until when we want the report to end",
        value_parser = clap::builder::ValueParser::new(ttrack_clap_date_parser)
    )]
    pub until: Option<NaiveDate>,

    #[arg(long, help = "Report time by project", default_value_t = false)]
    pub by_project: bool,

    #[arg(long, help = "Only report data from today", default_value_t = false)]
    pub today: bool,

    #[arg(
        long,
        help = "Only report data from yesterday",
        default_value_t = false
    )]
    pub yesterday: bool,

    #[arg(
        long,
        help = "Only report data from the current week (monday to sunday)",
        default_value_t = false
    )]
    pub this_week: bool,

    #[arg(
        long,
        help = "Only report data from the current week (monday to sunday)",
        default_value_t = false
    )]
    pub last_week: bool,
}

#[derive(Parser)]
#[command(name = "start", visible_alias = "s", about = "Start to track a task")]
pub struct StartCommand {
    #[arg(short, long, help = "The description of the task we start tracking")]
    pub description: String,

    #[arg(
        short,
        long,
        help = "The project with which this task will be associated"
    )]
    pub project: Option<String>,
}

#[derive(Subcommand)]
pub enum Command {
    Track(TrackCommand),
    Report(ReportCommand),
    Start(StartCommand),
    Pause,
    Resume,
    Stop,
}

#[derive(Parser)]
#[command(version, about = "A simple, but yet powerfull time tracker", long_about = None)]
pub struct Cli {
    #[arg(
        short,
        long,
        help = "File where to store tracking data",
        required = true
    )]
    pub file: PathBuf,

    #[command(subcommand)]
    pub command: Command,
}

#[cfg(test)]
mod tests {
    use chrono::{Days, Local, NaiveDate};
    use std::time::Duration;

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
        assert_eq!(result.unwrap(), Duration::from_secs(3600));
    }

    #[test]
    fn ttrack_clap_duration_parser_parse_hours_correctly() {
        let result = ttrack_clap_duration_parser("2h");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Duration::from_secs(7200));
    }

    #[test]
    fn ttrack_clap_duration_parser_parse_minutes_correctly() {
        let result = ttrack_clap_duration_parser("30m");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Duration::from_secs(1800));
    }

    #[test]
    fn ttrack_clap_duration_parser_parse_mised_hours_and_minutes_correctly() {
        let result = ttrack_clap_duration_parser("1h30m");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Duration::from_secs(5400));
    }
}
