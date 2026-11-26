use std::{path::PathBuf, time::Duration};

use chrono::{Days, Local, NaiveDate};
use clap::{ArgGroup, Parser, Subcommand};

fn ttrack_clap_date_parser(date: &str) -> Result<NaiveDate, String> {
    match date {
        "today" => Ok(Local::now().date_naive()),
        "yesterday" => Local::now()
            .date_naive()
            .checked_sub_days(Days::new(1))
            .ok_or_else(|| "Failed to compute yesterday's date".to_string()),
        _ => date
            .parse::<NaiveDate>()
            .map_err(|error| format!("Invalid date '{date}': {error}")),
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum TimeUnit {
    Hours,
    Minutes,
    Seconds,
}

impl TimeUnit {
    fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            b'h' => Some(Self::Hours),
            b'm' => Some(Self::Minutes),
            b's' => Some(Self::Seconds),
            _ => None,
        }
    }

    fn multiplier(self) -> u64 {
        match self {
            Self::Hours => 3600,
            Self::Minutes => 60,
            Self::Seconds => 1,
        }
    }

    fn next_smaller(self) -> Option<Self> {
        match self {
            Self::Hours => Some(Self::Minutes),
            Self::Minutes => Some(Self::Seconds),
            Self::Seconds => None,
        }
    }
}

fn add_duration_unit(total: u64, value: u64, time_unit: TimeUnit) -> Result<u64, String> {
    let part = value
        .checked_mul(time_unit.multiplier())
        .ok_or_else(|| "Duration is too large".to_string())?;
    total
        .checked_add(part)
        .ok_or_else(|| "Duration is too large".to_string())
}

fn ttrack_clap_duration_parser(duration: &str) -> Result<Duration, String> {
    if duration.is_empty() {
        return Err("Duration cannot be empty".to_string());
    }

    let bytes = duration.as_bytes();
    let mut index = 0;
    let mut total = 0u64;
    let mut last_time_unit: Option<TimeUnit> = None;

    while index < bytes.len() {
        let digits_start = index;
        while index < bytes.len() && bytes[index].is_ascii_digit() {
            index += 1;
        }

        if index == digits_start {
            return Err(format!(
                "Invalid duration '{duration}': expected digits at position {index}"
            ));
        }

        let digits = &duration[digits_start..index];
        let value = digits
            .parse::<u64>()
            .map_err(|_| format!("Invalid duration '{duration}': {digits:?} is too large"))?;

        if index == bytes.len() {
            let time_unit = match last_time_unit {
                None => TimeUnit::Seconds,
                Some(unit) => unit.next_smaller().ok_or_else(|| {
                    format!("Invalid duration '{duration}': no unit smaller than seconds")
                })?,
            };
            total = add_duration_unit(total, value, time_unit)?;
            break;
        }

        let Some(time_unit) = TimeUnit::from_byte(bytes[index]) else {
            let character = bytes[index] as char;
            return Err(format!(
                "Invalid duration '{duration}': unexpected character '{character}'"
            ));
        };
        index += 1;

        // Units must be strictly descending: hours, then minutes, then seconds.
        if let Some(last_time_unit) = last_time_unit
            && time_unit <= last_time_unit
        {
            return Err(format!(
                "Invalid duration '{duration}': units must be in descending order"
            ));
        }

        total = add_duration_unit(total, value, time_unit)?;
        last_time_unit = Some(time_unit);
    }

    Ok(Duration::from_secs(total))
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
        help = "The time duration of the record (e.g. 3600, 2h, 1h30m, 1h30)",
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
    about = "Generate report from the records",
    group(
        ArgGroup::new("period")
            .args(["today", "yesterday", "this_week", "last_week", "date"])
            .conflicts_with("range")
    ),
    group(
        ArgGroup::new("range")
            .args(["since", "until"])
            .multiple(true)
    )
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

    #[arg(
        long,
        help = "The specific date we want to report on",
        value_parser = clap::builder::ValueParser::new(ttrack_clap_date_parser)
    )]
    pub date: Option<NaiveDate>,

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
        help = "Only report data from last week (monday to sunday)",
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
#[command(version, about = "A simple, but yet powerful time tracker", long_about = None)]
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
    use clap::{CommandFactory, Parser, error::ErrorKind};
    use std::time::Duration;

    use crate::cli::{Cli, ttrack_clap_date_parser, ttrack_clap_duration_parser};

    fn parse_report(args: &[&str]) -> Result<Cli, clap::Error> {
        let mut full_args = vec!["ttrack", "-f", "data.csv", "report"];
        full_args.extend_from_slice(args);
        Cli::try_parse_from(full_args)
    }

    fn parse_track(args: &[&str]) -> Result<Cli, clap::Error> {
        let mut full_args = vec!["ttrack", "-f", "data.csv", "track", "-d", "task"];
        full_args.extend_from_slice(args);
        Cli::try_parse_from(full_args)
    }

    fn assert_conflict(args: &[&str]) {
        let error = match parse_report(args) {
            Ok(_) => panic!("conflicting report flags were accepted: {args:?}"),
            Err(error) => error,
        };
        assert_eq!(error.kind(), ErrorKind::ArgumentConflict, "args: {args:?}");
    }

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
    fn ttrack_clap_duration_parser_parse_mixed_hours_and_minutes_correctly() {
        let result = ttrack_clap_duration_parser("1h30m");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Duration::from_secs(5400));
    }

    #[test]
    fn duration_parser_accepts_shorthand_units() {
        for (input, expected) in [
            ("1h30", Duration::from_secs(5400)),
            ("3m20", Duration::from_secs(200)),
            ("1h30m20", Duration::from_secs(5420)),
            ("1h30s", Duration::from_secs(3630)),
            ("90s", Duration::from_secs(90)),
        ] {
            assert_eq!(
                ttrack_clap_duration_parser(input).unwrap(),
                expected,
                "input: {input:?}"
            );
        }
    }

    #[test]
    fn duration_parser_rejects_malformed_input() {
        for input in [
            "", "h30m", "1hm", "m30", "1h2h", "30m1h", "1x30m", "1h30m5x", "1h30s5", "-1h",
            "1h 30m",
        ] {
            assert!(
                ttrack_clap_duration_parser(input).is_err(),
                "accepted {input:?}"
            );
        }
    }

    #[test]
    fn duration_parser_rejects_overflow() {
        let max = u64::MAX.to_string();
        let too_large_seconds = (u64::MAX as u128 + 1).to_string();
        let max_hours = format!("{max}h");
        let max_minutes = format!("{max}m");

        for input in [
            too_large_seconds.as_str(),
            max_hours.as_str(),
            max_minutes.as_str(),
        ] {
            assert!(
                ttrack_clap_duration_parser(input).is_err(),
                "accepted overflowing duration {input:?}"
            );
        }
    }

    #[test]
    fn date_parser_rejects_invalid_input() {
        for input in ["", "not-a-date", "2024-02-30", "2024-13-01"] {
            assert!(
                ttrack_clap_date_parser(input).is_err(),
                "accepted {input:?}"
            );
        }
    }

    #[test]
    fn track_accepts_shorthand_duration() {
        assert!(parse_track(&["-t", "1h30"]).is_ok());
    }

    #[test]
    fn track_rejects_invalid_duration_value() {
        let error = match parse_track(&["-t", "h30m"]) {
            Ok(_) => panic!("invalid duration was accepted"),
            Err(error) => error,
        };

        assert_eq!(error.kind(), ErrorKind::ValueValidation);
    }

    #[test]
    fn report_rejects_invalid_date_value() {
        let error = match Cli::try_parse_from([
            "ttrack",
            "-f",
            "data.csv",
            "report",
            "--date",
            "not-a-date",
        ]) {
            Ok(_) => panic!("invalid date was accepted"),
            Err(error) => error,
        };

        assert_eq!(error.kind(), ErrorKind::ValueValidation);
    }

    #[test]
    fn report_rejects_multiple_period_flags() {
        for args in [
            &["--today", "--yesterday"][..],
            &["--today", "--this-week"],
            &["--today", "--last-week"],
            &["--yesterday", "--this-week"],
            &["--yesterday", "--last-week"],
            &["--this-week", "--last-week"],
            &["--today", "--date", "2026-11-25"],
            &["--yesterday", "--date", "2026-11-25"],
            &["--this-week", "--date", "2026-11-25"],
            &["--last-week", "--date", "2026-11-25"],
        ] {
            assert_conflict(args);
        }
    }

    #[test]
    fn report_rejects_period_flag_combined_with_date_range() {
        assert_conflict(&["--today", "--since", "2026-01-01"]);
        assert_conflict(&["--last-week", "--until", "2026-12-31"]);
        assert_conflict(&["--date", "2026-11-25", "--since", "2026-01-01"]);
    }

    #[test]
    fn report_accepts_one_period_flag() {
        for args in [
            &["--today"][..],
            &["--yesterday"],
            &["--this-week"],
            &["--last-week"],
            &["--date", "2026-11-25"],
            &["--today", "--by-project"],
        ] {
            assert!(parse_report(args).is_ok(), "args: {args:?}");
        }
    }

    #[test]
    fn report_accepts_since_and_until_together() {
        assert!(parse_report(&["--since", "2026-01-01", "--until", "2026-12-31"]).is_ok());
    }

    #[test]
    fn last_week_help_text_describes_last_week() {
        let help = Cli::command()
            .find_subcommand("report")
            .expect("missing report subcommand")
            .get_arguments()
            .find(|arg| arg.get_id() == "last_week")
            .expect("missing last_week arg")
            .get_help()
            .expect("missing last_week help")
            .to_string();

        assert!(help.contains("last week"), "unexpected help: {help}");
        assert!(!help.contains("current week"), "unexpected help: {help}");
    }
}
