use std::{collections::BTreeMap, fs::File, path::PathBuf, time::Duration};

use rootcause::{bail, prelude::ResultExt};

use crate::{
    cli::ReportCommand,
    storage::{read_records_from_file, replace_file_with_records, write_record_to_file},
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
    let mut duration_by_project: BTreeMap<String, Duration> = BTreeMap::new();

    data.iter().for_each(|tr| {
        if let Some(project) = &tr.project {
            if let Some(cur) = duration_by_project.get(project) {
                duration_by_project.insert(project.to_string(), *cur + tr.duration);
            } else {
                duration_by_project.insert(project.to_string(), tr.duration);
            }
        } else if let Some(cur) = duration_by_project.get("other") {
            duration_by_project.insert("other".to_string(), *cur + tr.duration);
        } else {
            duration_by_project.insert("other".to_string(), tr.duration);
        }
    });

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
    let records: Vec<CurrentTaskRecord> = read_records_from_file(&current_task_file_path)
        .context("Failed to read current task records")
        .attach_with(|| format!("path: {}", current_task_file_path.display()))?;

    let Some(last_record) = records.last() else {
        bail!("There is no current task to resume");
    };

    if last_record.end_at.is_none() {
        bail!("Current task is already running");
    }

    let resume_record = CurrentTaskRecord::resume_from(last_record.clone());

    write_record_to_file(&current_task_file_path, resume_record)
        .context("Failed to write resumed current task data")
        .attach_with(|| format!("path: {}", current_task_file_path.display()))?;

    Ok(())
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

    let mut total = Duration::default();
    for record in &records {
        let Some(end_at) = record.end_at else {
            bail!("Current task record is missing an end time");
        };

        total += end_at
            .signed_duration_since(record.created_at)
            .to_std()
            .context("Failed to compute task duration")?;
    }

    // we need a record to get the description and the project
    let Some(rec) = records.last() else {
        bail!("Failed to retrieve current task information");
    };

    let tr = TimeRecord {
        created_at: Local::now().date_naive(),
        duration: total,
        description: rec.description.clone(),
        project: rec.project.clone(),
    };

    write_record_to_file(&time_record_file_path, tr)
        .context("Failed to write new time record from the current task to file")
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

    use super::format_duration;

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
}
