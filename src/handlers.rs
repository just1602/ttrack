use core::panic;
use std::{collections::HashMap, fs::File, path::PathBuf, time::Duration};

use crate::{
    cli::ReportCommand,
    storage::{read_records_from_file, write_record_to_file, write_records_to_file},
};
use crate::{
    cli::TrackCommand,
    records::{CurrentTaskRecord, TimeRecord},
};
use chrono::{Days, Local};
use csv::ReaderBuilder;

pub fn handle_report(cmd: ReportCommand, filename: PathBuf) {
    let file = File::open(filename).expect("Failed to open data file.");
    let mut csv_reader = ReaderBuilder::new()
        .delimiter(b'|')
        .has_headers(false)
        .from_reader(file);

    let mut data: Vec<TimeRecord> = csv_reader.deserialize().map(|f| f.unwrap()).collect();

    if cmd.today {
        let today = Local::now().date_naive();

        data.retain(|tr| tr.created_at == today);
    }

    if cmd.yesterday {
        let yesterday = Local::now()
            .checked_sub_days(Days::new(1))
            .unwrap()
            .date_naive();

        data.retain(|tr| tr.created_at == yesterday)
    }

    if cmd.this_week {
        let week = Local::now().date_naive().week(chrono::Weekday::Mon);

        data.retain(|tr| tr.created_at >= week.first_day() && tr.created_at <= week.last_day())
    }

    if cmd.last_week {
        let week = Local::now()
            .date_naive()
            .checked_sub_days(Days::new(7))
            .expect("Failed to compute last week.")
            .week(chrono::Weekday::Mon);

        data.retain(|tr| tr.created_at >= week.first_day() && tr.created_at <= week.last_day())
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
}

fn report_total_hours(data: &[TimeRecord]) {
    let total_duration = data.iter().map(|tr| tr.duration).sum();

    println!("total hours: {}", format_duration(total_duration));
}

    let mut duration_by_project: HashMap<String, Duration> = HashMap::new();
fn report_by_project(data: &Vec<TimeRecord>) {

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

pub fn handle_track(cmd: TrackCommand, filename: PathBuf) {
    let tr = TimeRecord {
        created_at: cmd.date,
        duration: cmd.time,
        description: cmd.description,
        project: cmd.project,
    };

    if let Err(e) = write_record_to_file(&filename, tr) {
        eprintln!("Failed to write new time record to file: {}", e);
        std::process::exit(1);
    }
}

pub fn handle_start(cmd: crate::cli::StartCommand, current_task_file: PathBuf) {
    let current_task = CurrentTaskRecord {
        created_at: Local::now().naive_local(),
        end_at: None,
        description: cmd.description,
        project: cmd.project,
    };

    if let Err(e) = write_record_to_file(&current_task_file, current_task) {
        eprintln!("Failed to write current task data to file: {}", e);
        std::process::exit(1);
    }
}

pub fn handle_pause(current_task_file_path: PathBuf) {
    let mut records: Vec<CurrentTaskRecord> = match read_records_from_file(&current_task_file_path)
    {
        Ok(records) => records,
        Err(e) => {
            eprintln!("Failed to read current task records from file: {}", e);
            std::process::exit(1);
        }
    };

    if let Some(rec) = records.last_mut() {
        rec.end_at = Some(Local::now().naive_local());
    } else {
        eprintln!("There's no current task to pause")
    }

    // TODO: cleanup this path hack code and try to write directly to file
    let tmp_file = format!(
        "{}.tmp",
        &current_task_file_path
            .file_name()
            .expect("The task file should be a file")
            .to_str()
            .expect("This is supposed to be a valid file name")
    );

    let tmp_file_path = current_task_file_path.with_file_name(tmp_file);

    if let Err(e) = write_records_to_file(&tmp_file_path, &records) {
        eprintln!("Failed to write current task data to file: {}", e);
        std::process::exit(1);
    }

    if let Err(e) = std::fs::rename(tmp_file_path, current_task_file_path) {
        eprintln!("Failed to persist ttrack current task data: {}", e);
        std::process::exit(1);
    }
}

pub fn handle_resume(current_task_file_path: PathBuf) {
    let records: Vec<CurrentTaskRecord> = match read_records_from_file(&current_task_file_path) {
        Ok(records) => records,
        Err(e) => {
            eprintln!("Failed to read current task records from file: {}", e);
            std::process::exit(1);
        }
    };

    let Some(last_record) = records.last() else {
        eprintln!("There is now current task to resume");
        std::process::exit(1);
    };

    let resume_record = CurrentTaskRecord::resume_from(last_record.clone());

    if let Err(e) = write_record_to_file(&current_task_file_path, resume_record) {
        eprintln!("Failed to write current task data to file: {}", e);
        std::process::exit(1);
    }
}

pub fn handle_stop(current_task_file_path: PathBuf, time_record_file_path: PathBuf) {
    let mut records: Vec<CurrentTaskRecord> = match read_records_from_file(&current_task_file_path)
    {
        Ok(records) => records,
        Err(e) => {
            eprintln!("Failed to read current task records from file: {}", e);
            std::process::exit(1);
        }
    };

    if let Some(rec) = records.last_mut() {
        if rec.end_at.is_none() {
            rec.end_at = Some(Local::now().naive_local());
        }
    } else {
        panic!("There is no current task to stop")
    }

    let mut total = Duration::default();
    for record in &records {
        total += record
            .end_at
            .expect("At this point, there should always be a value for `end_at`")
            .signed_duration_since(record.created_at)
            .to_std()
            .expect("Failed to extract task duration");
    }

    // we need a record to get the description and the project
    let Some(rec) = &records.last() else {
        panic!("Failed to retrieve current task information")
    };

    let tr = TimeRecord {
        created_at: Local::now().date_naive(),
        duration: total,
        description: rec.description.clone(),
        project: rec.project.clone(),
    };

    if let Err(e) = write_record_to_file(&time_record_file_path, tr) {
        eprintln!(
            "Failed to write new time record from the current task to file: {}",
            e
        );
        std::process::exit(1);
    }

    File::create(current_task_file_path).expect("Failed to truncate current task state file");
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
