use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::Write,
    time::Duration,
};

use crate::record::{format_duration, TimeRecord};
use chrono::{Local, NaiveDate};
use clap::ArgMatches;
use csv::{ReaderBuilder, WriterBuilder};

pub fn handle_report(cmd: ArgMatches, filename: String) {
    let file = File::open(filename).expect("Failed to open data file.");
    let mut csv_reader = ReaderBuilder::new()
        .delimiter(b'|')
        .has_headers(false)
        .from_reader(file);

    let data: Vec<TimeRecord> = match cmd.get_one::<NaiveDate>("since") {
        Some(since) => csv_reader
            .deserialize()
            .map(|f| f.unwrap())
            .filter(|tr: &TimeRecord| tr.created_at >= *since)
            .collect(),
        None => csv_reader.deserialize().map(|f| f.unwrap()).collect(),
    };

    if cmd.get_flag("by-project") {
        report_by_project(data);
    } else {
        report_total_hours(data);
    }
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
}

pub fn handle_track(cmd: ArgMatches, filename: String) {
    let time = cmd
        .get_one::<u64>("time")
        .expect("Time duration must be present.");
    let description = cmd
        .get_one::<String>("description")
        .expect("Description must be present.");
    let project = cmd.get_one::<String>("project");

    let tr = TimeRecord {
        created_at: Local::now().date_naive(),
        duration: Duration::from_secs(*time),
        description: description.to_string(),
        project: project.cloned(),
    };

    let mut csv_writer = WriterBuilder::new()
        .delimiter(b'|')
        .has_headers(false)
        .from_writer(vec![]);

    csv_writer
        .serialize::<TimeRecord>(tr)
        .expect("Failed to serialize the new TimeRecord instance.");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(filename)
        .unwrap();

    file.write_all(
        &csv_writer
            .into_inner()
            .expect("Failed to retrieve the CSVWriter's writer."),
    )
    .expect("Failed to write data in file.");
}
