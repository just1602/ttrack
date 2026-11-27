mod common;

use std::path::Path;
use std::{fs, process::Output};

use chrono::{Days, Duration as ChronoDuration, Local, NaiveDate};
use tempfile::tempdir;

use crate::common::{assert_clean_error, data_file, run_in, run_ok, state_dir, state_file};

const GOLDEN_RECORDS_CSV: &str = include_str!("fixtures/records.csv");

fn stdout(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).expect("stdout was not UTF-8")
}

fn seed_golden_records(temp_dir: &Path) {
    fs::write(data_file(temp_dir), GOLDEN_RECORDS_CSV).expect("Failed to seed golden records");
}

fn days_in_week_containing(date: NaiveDate) -> Vec<NaiveDate> {
    let week = date.week(chrono::Weekday::Mon);
    (0..7)
        .map(|offset| week.first_day() + ChronoDuration::days(offset))
        .collect()
}

#[test]
fn report_filters_by_exact_date() {
    let temp_dir = tempdir().expect("Failed to create test temp dir");
    seed_golden_records(temp_dir.path());

    let output = run_ok(temp_dir.path(), &["report", "--date", "2026-11-25"]);

    assert_eq!(stdout(&output).trim(), "total hours: 1h30m");
}

#[test]
fn report_filters_by_date_range() {
    let temp_dir = tempdir().expect("Failed to create test temp dir");
    seed_golden_records(temp_dir.path());

    let output = run_ok(
        temp_dir.path(),
        &["report", "--since", "2026-11-24", "--until", "2026-11-25"],
    );

    assert_eq!(stdout(&output).trim(), "total hours: 2h");
}

#[test]
fn report_groups_by_project() {
    let temp_dir = tempdir().expect("Failed to create test temp dir");
    seed_golden_records(temp_dir.path());

    let output = run_ok(temp_dir.path(), &["report", "--by-project"]);

    assert_eq!(
        stdout(&output),
        "demo: 1h30m1s\nother: 30m1s\n---\ntotal hours: 2h3s\n"
    );
}

#[test]
fn report_relative_flags_select_expected_days() {
    let temp_dir = tempdir().expect("Failed to create test temp dir");
    let today = Local::now().date_naive();
    let last_week_day = today
        .checked_sub_days(Days::new(7))
        .expect("test date is not the minimum");
    let dates = [
        days_in_week_containing(last_week_day),
        days_in_week_containing(today),
    ]
    .concat();
    let rows: String = dates
        .iter()
        .map(|date| format!("{date}|60|0|relative|\n"))
        .collect();
    fs::write(data_file(temp_dir.path()), rows).expect("Failed to seed relative records");

    for (flag, expected) in [
        ("--today", "total hours: 1m"),
        ("--yesterday", "total hours: 1m"),
        ("--this-week", "total hours: 7m"),
        ("--last-week", "total hours: 7m"),
    ] {
        let output = run_ok(temp_dir.path(), &["report", flag]);
        assert_eq!(
            stdout(&output).trim(),
            expected,
            "unexpected output for {flag}"
        );
    }
}

#[test]
fn stop_appends_segments_and_clears_state() {
    let temp_dir = tempdir().expect("Failed to create test temp dir");
    run_ok(
        temp_dir.path(),
        &[
            "track",
            "--date",
            "2026-11-24",
            "-t",
            "1h",
            "-d",
            "existing",
            "-p",
            "demo",
        ],
    );

    let running_start = Local::now().naive_local() - ChronoDuration::hours(1);
    let state = format!(
        "2026-11-25T10:00:00|2026-11-25T10:15:00|first segment|demo\n{}||running segment|demo\n",
        running_start.format("%Y-%m-%dT%H:%M:%S")
    );
    fs::create_dir_all(state_dir(temp_dir.path())).expect("Failed to create state dir");
    fs::write(state_file(temp_dir.path()), state).expect("Failed to seed state file");

    run_ok(temp_dir.path(), &["stop"]);

    let records = fs::read_to_string(data_file(temp_dir.path())).expect("Failed to read records");
    let lines: Vec<_> = records.lines().collect();
    assert_eq!(lines[0], "2026-11-24|3600|0|existing|demo");
    assert_eq!(lines[1], "2026-11-25|900|0|first segment|demo");

    let running_records: Vec<_> = lines
        .iter()
        .skip(2)
        .filter(|line| line.contains("running segment"))
        .collect();
    assert!(
        !running_records.is_empty(),
        "running segment was not appended:\n{records}"
    );
    for line in &running_records {
        let fields: Vec<_> = line.split('|').collect();
        assert_eq!(fields.len(), 5, "unexpected record: {line}");
        let date = fields[0]
            .parse::<NaiveDate>()
            .unwrap_or_else(|_| panic!("invalid record date: {line}"));
        assert!(date >= running_start.date(), "segment moved backwards");
        assert!(
            date <= Local::now().date_naive(),
            "segment date is in the future"
        );
        assert_eq!(fields[3], "running segment");
        assert_eq!(fields[4], "demo");
    }

    assert_eq!(
        fs::read_to_string(state_file(temp_dir.path())).expect("Failed to read state file"),
        "",
        "stop did not clear current task state"
    );
    assert!(
        !state_dir(temp_dir.path()).join("data.csv.tmp").exists(),
        "stop left a temporary state file behind"
    );
}

#[test]
fn stop_rejects_malformed_state_without_modifying_files() {
    let temp_dir = tempdir().expect("Failed to create test temp dir");
    let original_records = "2026-11-25|60|0|existing|\n";
    fs::write(data_file(temp_dir.path()), original_records).expect("Failed to seed records");
    fs::create_dir_all(state_dir(temp_dir.path())).expect("Failed to create state dir");
    let invalid_state = "2026-11-25T10:00:00|not-a-timestamp|bad|demo\n";
    fs::write(state_file(temp_dir.path()), invalid_state).expect("Failed to seed state file");

    let output = run_in(temp_dir.path(), &["stop"]);

    assert_clean_error(&output, "Failed to read current task records");
    assert_eq!(
        fs::read_to_string(data_file(temp_dir.path())).expect("Failed to read records"),
        original_records,
        "rejected stop modified the records"
    );
    assert_eq!(
        fs::read_to_string(state_file(temp_dir.path())).expect("Failed to read state file"),
        invalid_state,
        "rejected stop modified the state file"
    );
}
