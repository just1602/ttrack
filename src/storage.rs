use std::{
    fs::{File, OpenOptions},
    path::Path,
};

use csv::{ReaderBuilder, WriterBuilder};
use rootcause::prelude::ResultExt;
use serde::{Deserialize, Serialize};

pub fn read_records_from_file<R>(file_path: &Path) -> rootcause::Result<Vec<R>>
where
    R: for<'a> Deserialize<'a>,
{
    let file = File::open(file_path)
        .context("Failed to open data file")
        .attach_with(|| format!("path: {}", file_path.display()))?;

    let mut csv_reader = ReaderBuilder::new()
        .delimiter(b'|')
        .has_headers(false)
        .from_reader(file);

    let records = csv_reader
        .deserialize()
        .collect::<Result<Vec<R>, csv::Error>>()
        .context("Failed to deserialize records from CSV")
        .attach_with(|| format!("path: {}", file_path.display()))?;

    Ok(records)
}

pub fn write_record_to_file<R>(file_path: &Path, record: R) -> rootcause::Result<()>
where
    R: Serialize,
{
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .context("Failed to open data file for writing")
        .attach_with(|| format!("path: {}", file_path.display()))?;

    // TODO: Extract csv_writer creation in a function
    let mut csv_writer = WriterBuilder::new()
        .delimiter(b'|')
        .has_headers(false)
        .from_writer(file);

    csv_writer
        .serialize(record)
        .context("Failed to serialize record to CSV")
        .attach_with(|| format!("path: {}", file_path.display()))?;

    csv_writer
        .flush()
        .context("Failed to flush CSV data")
        .attach_with(|| format!("path: {}", file_path.display()))?;

    Ok(())
}

pub fn write_records_to_file<R>(file_path: &Path, records: &Vec<R>) -> rootcause::Result<()>
where
    R: Serialize,
{
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .context("Failed to open data file for writing")
        .attach_with(|| format!("path: {}", file_path.display()))?;

    // TODO: Extract csv_writer creation in a function
    let mut csv_writer = WriterBuilder::new()
        .delimiter(b'|')
        .has_headers(false)
        .from_writer(file);

    for record in records {
        csv_writer
            .serialize(record)
            .context("Failed to serialize record to CSV")
            .attach_with(|| format!("path: {}", file_path.display()))?;
    }

    csv_writer
        .flush()
        .context("Failed to flush CSV data")
        .attach_with(|| format!("path: {}", file_path.display()))?;

    Ok(())
}
