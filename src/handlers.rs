use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
    time::Duration,
};

use crate::cli::ReportCommand;
use crate::{
    cli::TrackCommand,
    record::TimeRecord,
};
use chrono::{Days, Local};
use csv::{ReaderBuilder, WriterBuilder};

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
        report_by_project(data.clone());
    }
    report_total_hours(data);
}

fn report_total_hours(data: Vec<TimeRecord>) {
    let total_duration = data.into_iter().map(|tr| tr.duration).sum();

    println!("total hours: {}", format_duration(total_duration));
}

fn report_by_project(data: Vec<TimeRecord>) {
    let mut duration_by_project: HashMap<String, Duration> = HashMap::new();
    data.into_iter().for_each(|tr| {
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
        created_at: Local::now().date_naive(),
        duration: cmd.time,
        description: cmd.description,
        project: cmd.project,
    };

    let mut csv_writer = WriterBuilder::new()
        .delimiter(b'|')
        .has_headers(false)
        .from_writer(vec![]);

    csv_writer
        .serialize::<TimeRecord>(tr)
        .expect("Failed to serialize the new TimeRecord instance.");
    let mut file = OpenOptions::new().append(true).open(filename).unwrap();

    file.write_all(
        &csv_writer
            .into_inner()
            .expect("Failed to retrieve the CSVWriter's writer."),
    )
    .expect("Failed to write data in file.");
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
