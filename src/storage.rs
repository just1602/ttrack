use std::{
    fs::{File, OpenOptions},
    path::Path,
};

use csv::{ReaderBuilder, WriterBuilder};
use serde::{Deserialize, Serialize};

pub fn read_records_from_file<R>(file_path: &Path) -> Result<Vec<R>, std::io::Error>
where
    R: for<'a> Deserialize<'a>,
{
    let file = File::open(file_path)?;

    let mut csv_reader = ReaderBuilder::new()
        .delimiter(b'|')
        .has_headers(false)
        .from_reader(file);

    Ok(csv_reader.deserialize().map(|f| f.unwrap()).collect())
}

/**
* TODO: Extract csv_writer creation in a function
**/

pub fn write_record_to_file<R>(file_path: &Path, record: R) -> Result<(), std::io::Error>
where
    R: Serialize,
{
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    let mut csv_writer = WriterBuilder::new()
        .delimiter(b'|')
        .has_headers(false)
        .from_writer(file);

    csv_writer.serialize(record)?;

    csv_writer.flush()?;

    Ok(())
}

pub fn write_records_to_file<R>(file_path: &Path, records: &Vec<R>) -> Result<(), std::io::Error>
where
    R: Serialize,
{
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    let mut csv_writer = WriterBuilder::new()
        .delimiter(b'|')
        .has_headers(false)
        .from_writer(file);

    for record in records {
        csv_writer.serialize(record)?;
    }

    csv_writer.flush()?;

    Ok(())
}
