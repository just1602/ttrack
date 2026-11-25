use std::{
    fs::{File, OpenOptions},
    path::{Path, PathBuf},
};

use csv::{ReaderBuilder, WriterBuilder};
use rootcause::prelude::ResultExt;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Failed to open data file for reading: {}", path.display())]
    OpenForRead { path: PathBuf },

    #[error("Failed to deserialize records from CSV: {}", path.display())]
    DeserializeCsv { path: PathBuf },

    #[error("Failed to open data file for writing: {}", path.display())]
    OpenForWrite { path: PathBuf },

    #[error("Failed to serialize record to CSV: {}", path.display())]
    SerializeCsv { path: PathBuf },

    #[error("Failed to flush CSV data: {}", path.display())]
    FlushCsv { path: PathBuf },
}

pub type StorageResult<T> = rootcause::Result<T, StorageError>;

pub fn read_records_from_file<R>(file_path: &Path) -> StorageResult<Vec<R>>
where
    R: for<'a> Deserialize<'a>,
{
    let file = File::open(file_path).context(StorageError::OpenForRead {
        path: file_path.to_path_buf(),
    })?;

    let mut csv_reader = ReaderBuilder::new()
        .delimiter(b'|')
        .has_headers(false)
        .from_reader(file);

    let records = csv_reader
        .deserialize()
        .collect::<Result<Vec<R>, csv::Error>>()
        .context(StorageError::DeserializeCsv {
            path: file_path.to_path_buf(),
        })?;

    Ok(records)
}

pub fn write_record_to_file<R>(file_path: &Path, record: R) -> StorageResult<()>
where
    R: Serialize,
{
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .context(StorageError::OpenForWrite {
            path: file_path.to_path_buf(),
        })?;

    // TODO: Extract csv_writer creation in a function
    let mut csv_writer = WriterBuilder::new()
        .delimiter(b'|')
        .has_headers(false)
        .from_writer(file);

    csv_writer
        .serialize(record)
        .context(StorageError::SerializeCsv {
            path: file_path.to_path_buf(),
        })?;

    csv_writer.flush().context(StorageError::FlushCsv {
        path: file_path.to_path_buf(),
    })?;

    Ok(())
}

pub fn write_records_to_file<R>(file_path: &Path, records: &Vec<R>) -> StorageResult<()>
where
    R: Serialize,
{
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .context(StorageError::OpenForWrite {
            path: file_path.to_path_buf(),
        })?;

    // TODO: Extract csv_writer creation in a function
    let mut csv_writer = WriterBuilder::new()
        .delimiter(b'|')
        .has_headers(false)
        .from_writer(file);

    for record in records {
        csv_writer
            .serialize(record)
            .context(StorageError::SerializeCsv {
                path: file_path.to_path_buf(),
            })?;
    }

    csv_writer.flush().context(StorageError::FlushCsv {
        path: file_path.to_path_buf(),
    })?;

    Ok(())
}
