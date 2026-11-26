use std::{collections::BTreeMap, fs::File, path::PathBuf, time::Duration};

use rootcause::{bail, option_ext::OptionExt, prelude::ResultExt};

use crate::{
    cli::ReportCommand,
    storage::{
        append_records_to_file, read_records_from_file, replace_file_with_records,
        write_record_to_file,
    },
};
use crate::{
    cli::TrackCommand,
    records::{CurrentTaskRecord, TimeRecord},
};
use chrono::{Days, Local};

pub fn handle_report(cmd: ReportCommand, filename: PathBuf) -> rootcause::Result<()> {
    let mut data: Vec<TimeRecord> = read_records_from_file(&filename)
        .context("Failed to load records for report")
        .attach_with(|| format!("path: {}", filename.display()))?;

    if cmd.today {
        let today = Local::now().date_naive();

        data.retain(|tr| tr.created_at == today);
    }

    if cmd.yesterday {
        let Some(yesterday) = Local::now().date_naive().checked_sub_days(Days::new(1)) else {
            bail!("Failed to compute yesterday's date");
        };

        data.retain(|tr| tr.created_at == yesterday)
    }

    if cmd.this_week {
        let week = Local::now().date_naive().week(chrono::Weekday::Mon);

        data.retain(|tr| tr.created_at >= week.first_day() && tr.created_at <= week.last_day())
    }

    if cmd.last_week {
        let Some(last_week_day) = Local::now().date_naive().checked_sub_days(Days::new(7)) else {
            bail!("Failed to compute last week's date");
        };
        let week = last_week_day.week(chrono::Weekday::Mon);

        data.retain(|tr| tr.created_at >= week.first_day() && tr.created_at <= week.last_day())
    }

    if let Some(date) = cmd.date {
        data.retain(|tr| tr.created_at == date);
    }

    if let Some(since) = cmd.since {
        data.retain(|tr| tr.created_at >= since)
    }

    if let Some(until) = cmd.until {
        data.retain(|tr| tr.created_at <= until)
    };

    if cmd.by_project {
        report_by_project(&data);
    }

    report_total_hours(&data);

    Ok(())
}

fn report_total_hours(data: &[TimeRecord]) {
    let total_duration = data.iter().map(|tr| tr.duration).sum();

    println!("total hours: {}", format_duration(total_duration));
}

fn report_by_project(data: &[TimeRecord]) {
    // Use `BTreeMap` since keys are sorted, and `HashMap` keys can't be sorted.
    let mut duration_by_project: BTreeMap<&str, Duration> = BTreeMap::new();

    for tr in data {
        let project = tr.project.as_deref().unwrap_or("other");
        *duration_by_project.entry(project).or_default() += tr.duration;
    }

    duration_by_project
        .into_iter()
        .for_each(|(k, v)| println!("{}: {}", k, format_duration(v)));
    println!("---")
}

pub fn handle_track(cmd: TrackCommand, filename: PathBuf) -> rootcause::Result<()> {
    let tr = TimeRecord {
        created_at: cmd.date,
        duration: cmd.time,
        description: cmd.description,
        project: cmd.project,
    };

    write_record_to_file(&filename, tr)
        .context("Failed to track time record")
        .attach_with(|| format!("path: {}", filename.display()))?;

    Ok(())
}

pub fn handle_start(
    cmd: crate::cli::StartCommand,
    current_task_file: PathBuf,
) -> rootcause::Result<()> {
    let current_task = CurrentTaskRecord {
        created_at: Local::now().naive_local(),
        end_at: None,
        description: cmd.description,
        project: cmd.project,
    };

    write_record_to_file(&current_task_file, current_task)
        .context("Failed to write current task data")
        .attach_with(|| format!("path: {}", current_task_file.display()))?;

    Ok(())
}

pub fn handle_pause(current_task_file_path: PathBuf) -> rootcause::Result<()> {
    let mut records: Vec<CurrentTaskRecord> = read_records_from_file(&current_task_file_path)
        .context("Failed to read current task records")
        .attach_with(|| format!("path: {}", current_task_file_path.display()))?;

    let Some(rec) = records.last_mut() else {
        bail!("There's no current task to pause");
    };

    if rec.end_at.is_some() {
        bail!("Current task is already paused");
    }

    rec.end_at = Some(Local::now().naive_local());

    replace_file_with_records(&current_task_file_path, &records)
        .context("Failed to persist current task state")
        .attach_with(|| format!("path: {}", current_task_file_path.display()))?;

    Ok(())
}

pub fn handle_resume(current_task_file_path: PathBuf) -> rootcause::Result<()> {
    let mut records: Vec<CurrentTaskRecord> = read_records_from_file(&current_task_file_path)
        .context("Failed to read current task records")
        .attach_with(|| format!("path: {}", current_task_file_path.display()))?;

    let Some(last_record) = records.pop() else {
        bail!("There is no current task to resume");
    };

    if last_record.end_at.is_none() {
        bail!("Current task is already running");
    }

    let resume_record = CurrentTaskRecord::resume_from(last_record);

    write_record_to_file(&current_task_file_path, resume_record)
        .context("Failed to write resumed current task data")
        .attach_with(|| format!("path: {}", current_task_file_path.display()))?;

    Ok(())
}

fn time_records_for_segments(
    records: Vec<CurrentTaskRecord>,
) -> rootcause::Result<Vec<TimeRecord>> {
    let mut time_records = Vec::new();

    for record in records {
        let CurrentTaskRecord {
            created_at,
            end_at,
            description,
            project,
        } = record;

        let Some(end_at) = end_at else {
            bail!("Current task record is missing an end time");
        };

        let mut chunks = Vec::new();
        let mut chunk_start = created_at;
        while chunk_start < end_at {
            let next_midnight = chunk_start
                .date()
                .succ_opt()
                .context("Failed to compute next calendar day")?
                .and_hms_opt(0, 0, 0)
                .context("Failed to compute midnight")?;
            let chunk_end = end_at.min(next_midnight);

            let duration = chunk_end
                .signed_duration_since(chunk_start)
                .to_std()
                .context("Failed to compute task duration")?;

            chunks.push((chunk_start.date(), duration));
            chunk_start = chunk_end;
        }

        let mut segment_records: Vec<TimeRecord> = chunks
            .into_iter()
            .map(|(date, duration)| TimeRecord {
                created_at: date,
                duration,
                description: String::new(),
                project: None,
            })
            .collect();

        if let Some((last, earlier)) = segment_records.split_last_mut() {
            for record in earlier {
                record.description = description.clone();
                record.project = project.clone();
            }

            last.description = description;
            last.project = project;
        }

        time_records.extend(segment_records);
    }

    Ok(time_records)
}

pub fn handle_stop(
    current_task_file_path: PathBuf,
    time_record_file_path: PathBuf,
) -> rootcause::Result<()> {
    let mut records: Vec<CurrentTaskRecord> = read_records_from_file(&current_task_file_path)
        .context("Failed to read current task records")
        .attach_with(|| format!("path: {}", current_task_file_path.display()))?;

    let Some(rec) = records.last_mut() else {
        bail!("There is no current task to stop");
    };

    if rec.end_at.is_some() {
        bail!("Current task is already paused");
    }

    rec.end_at = Some(Local::now().naive_local());

    let time_records = time_records_for_segments(records)?;

    append_records_to_file(&time_record_file_path, &time_records)
        .context("Failed to write time records from the current task to file")
        .attach_with(|| format!("path: {}", time_record_file_path.display()))?;

    File::create(&current_task_file_path)
        .context("Failed to truncate current task state file")
        .attach_with(|| format!("path: {}", current_task_file_path.display()))?;

    Ok(())
}

fn format_duration(duration: Duration) -> String {
    let mut ret = String::from("");
    let hours = duration.as_secs() / 3600;
    if hours > 0 {
        ret.push_str(&format!("{}h", hours));
    }
    let mins = (duration.as_secs() - hours * 3600) / 60;
    if mins > 0 {
        ret.push_str(&format!("{}m", mins));
    }
    let secs = duration.as_secs() - hours * 3600 - mins * 60;
    if secs > 0 {
        ret.push_str(&format!("{}s", secs));
    }

    ret
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use chrono::{NaiveDate, NaiveDateTime};

    use crate::records::CurrentTaskRecord;

    use super::{TimeRecord, format_duration, time_records_for_segments};

    fn segment(start: &str, end: Option<&str>, description: &str) -> CurrentTaskRecord {
        CurrentTaskRecord {
            created_at: NaiveDateTime::parse_from_str(start, "%Y-%m-%dT%H:%M:%S").unwrap(),
            end_at: end
                .map(|value| NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S").unwrap()),
            description: description.to_string(),
            project: Some("demo".to_string()),
        }
    }

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    fn assert_record(record: &TimeRecord, expected_date: NaiveDate, expected_seconds: u64) {
        assert_eq!(record.created_at, expected_date);
        assert_eq!(record.duration, Duration::from_secs(expected_seconds));
        assert_eq!(record.description, "overnight");
        assert_eq!(record.project.as_deref(), Some("demo"));
    }

    #[test]
    fn format_duration_can_print_hours() {
        let result = format_duration(Duration::from_secs(3600));

        assert_eq!(result, String::from("1h"));
    }

    #[test]
    fn format_duration_can_print_minutes() {
        let result = format_duration(Duration::from_secs(600));

        assert_eq!(result, String::from("10m"));
    }

    #[test]
    fn format_duration_can_print_seconds() {
        let result = format_duration(Duration::from_secs(55));

        assert_eq!(result, String::from("55s"));
    }

    #[test]
    fn format_duration_can_print_combination_of_time_units() {
        let result = format_duration(Duration::from_secs(5405));

        assert_eq!(result, String::from("1h30m5s"));
    }

    #[test]
    fn time_records_for_single_day_segment() {
        let records = time_records_for_segments(vec![segment(
            "2026-11-25T10:00:00",
            Some("2026-11-25T11:30:00"),
            "overnight",
        )])
        .expect("Failed to split segment");

        assert_eq!(records.len(), 1);
        assert_record(&records[0], date(2026, 11, 25), 90 * 60);
    }

    #[test]
    fn time_records_split_at_midnight() {
        let records = time_records_for_segments(vec![segment(
            "2026-11-25T23:00:00",
            Some("2026-11-26T01:00:00"),
            "overnight",
        )])
        .expect("Failed to split segment");

        assert_eq!(records.len(), 2);
        assert_record(&records[0], date(2026, 11, 25), 60 * 60);
        assert_record(&records[1], date(2026, 11, 26), 60 * 60);
    }

    #[test]
    fn time_records_end_exactly_at_midnight() {
        let records = time_records_for_segments(vec![segment(
            "2026-11-25T23:00:00",
            Some("2026-11-26T00:00:00"),
            "overnight",
        )])
        .expect("Failed to split segment");

        assert_eq!(records.len(), 1);
        assert_record(&records[0], date(2026, 11, 25), 60 * 60);
    }

    #[test]
    fn time_records_split_across_two_midnights() {
        let records = time_records_for_segments(vec![segment(
            "2026-11-25T23:00:00",
            Some("2026-11-27T01:00:00"),
            "overnight",
        )])
        .expect("Failed to split segment");

        assert_eq!(records.len(), 3);
        assert_record(&records[0], date(2026, 11, 25), 60 * 60);
        assert_record(&records[1], date(2026, 11, 26), 24 * 60 * 60);
        assert_record(&records[2], date(2026, 11, 27), 60 * 60);
    }

    #[test]
    fn time_records_keep_pause_resume_chunks_unmerged() {
        let records = time_records_for_segments(vec![
            segment(
                "2026-11-25T23:00:00",
                Some("2026-11-26T01:00:00"),
                "overnight",
            ),
            segment(
                "2026-11-26T01:30:00",
                Some("2026-11-26T02:00:00"),
                "overnight",
            ),
        ])
        .expect("Failed to split segments");

        assert_eq!(records.len(), 3);
        assert_record(&records[0], date(2026, 11, 25), 60 * 60);
        assert_record(&records[1], date(2026, 11, 26), 60 * 60);
        assert_record(&records[2], date(2026, 11, 26), 30 * 60);
    }

    #[test]
    fn time_records_reject_segment_without_end_time() {
        let error =
            time_records_for_segments(vec![segment("2026-11-25T23:00:00", None, "overnight")])
                .unwrap_err();

        assert!(error.to_string().contains("missing an end time"));
    }
}
