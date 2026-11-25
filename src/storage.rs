use std::{
    fs::{File, OpenOptions},
    path::{Path, PathBuf},
};

use csv::{ReaderBuilder, WriterBuilder};
use rootcause::prelude::ResultExt;
use serde::{Serialize, de::DeserializeOwned};
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
    R: DeserializeOwned,
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

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use crate::records::TimeRecord;

    use super::{StorageError, read_records_from_file};

    fn temp_dir(label: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("ttrack-a2-storage-{}-{label}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("Failed to create test temp dir");
        dir
    }

    #[test]
    fn missing_file_maps_to_open_for_read() {
        let dir = temp_dir("missing");
        let data_file = dir.join("data.csv");

        let error = read_records_from_file::<TimeRecord>(&data_file).unwrap_err();

        assert!(
            matches!(error.current_context(), StorageError::OpenForRead { path } if path == &data_file),
            "unexpected error: {error:?}"
        );
        let rendered = error.to_string();
        assert!(rendered.contains("Failed to open data file for reading"));
        assert!(rendered.contains("No such file or directory"));
    }

    #[test]
    fn malformed_csv_maps_to_deserialize_csv() {
        let dir = temp_dir("malformed");
        let data_file = dir.join("data.csv");
        fs::write(&data_file, "2026-11-25|not-a-number\n").expect("Failed to write test data");

        let error = read_records_from_file::<TimeRecord>(&data_file).unwrap_err();

        assert!(
            matches!(error.current_context(), StorageError::DeserializeCsv { path } if path == &data_file),
            "unexpected error: {error:?}"
        );
        let rendered = error.to_string();
        assert!(rendered.contains("Failed to deserialize records from CSV"));
        assert!(rendered.contains("invalid digit found in string"));
    }
}
