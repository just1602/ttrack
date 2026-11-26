use std::{
    fs::{File, OpenOptions},
    io,
    path::{Path, PathBuf},
};

use csv::{Reader, ReaderBuilder, Writer, WriterBuilder};
use rootcause::prelude::ResultExt;
use serde::{Serialize, de::DeserializeOwned};
use thiserror::Error;

const CSV_DELIMITER: u8 = b'|';

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

    #[error("File path has no file name: {}", path.display())]
    MissingFileName { path: PathBuf },

    #[error("Failed to rename temporary file {} to {}", from.display(), to.display())]
    Rename { from: PathBuf, to: PathBuf },
}

pub type StorageResult<T> = rootcause::Result<T, StorageError>;

fn csv_reader<R: io::Read>(reader: R) -> Reader<R> {
    ReaderBuilder::new()
        .delimiter(CSV_DELIMITER)
        .has_headers(false)
        .from_reader(reader)
}

fn csv_writer<W: io::Write>(writer: W) -> Writer<W> {
    WriterBuilder::new()
        .delimiter(CSV_DELIMITER)
        .has_headers(false)
        .from_writer(writer)
}

pub fn read_records_from_file<R>(file_path: &Path) -> StorageResult<Vec<R>>
where
    R: DeserializeOwned,
{
    let file = File::open(file_path).context(StorageError::OpenForRead {
        path: file_path.to_path_buf(),
    })?;

    let mut csv_reader = csv_reader(file);

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

    write_all_records_to(file, &[record], file_path)
}

fn write_all_records_to<R, W>(writer: W, records: &[R], file_path: &Path) -> StorageResult<()>
where
    R: Serialize,
    W: io::Write,
{
    let mut csv_writer = csv_writer(writer);

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

pub fn write_records_to_file<R>(file_path: &Path, records: &[R]) -> StorageResult<()>
where
    R: Serialize,
{
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)
        .context(StorageError::OpenForWrite {
            path: file_path.to_path_buf(),
        })?;

    write_all_records_to(file, records, file_path)
}

pub fn append_records_to_file<R>(file_path: &Path, records: &[R]) -> StorageResult<()>
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

    write_all_records_to(file, records, file_path)
}

fn temp_path_for(file_path: &Path) -> StorageResult<PathBuf> {
    let Some(file_name) = file_path.file_name() else {
        return Err(rootcause::report!(StorageError::MissingFileName {
            path: file_path.to_path_buf(),
        }));
    };

    let mut temp_file_name = file_name.to_os_string();
    temp_file_name.push(".tmp");
    Ok(file_path.with_file_name(temp_file_name))
}

pub fn replace_file_with_records<R>(file_path: &Path, records: &[R]) -> StorageResult<()>
where
    R: Serialize,
{
    let temp_file_path = temp_path_for(file_path)?;

    write_records_to_file(&temp_file_path, records)?;

    std::fs::rename(&temp_file_path, file_path).context(StorageError::Rename {
        from: temp_file_path,
        to: file_path.to_path_buf(),
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path, time::Duration};

    use chrono::NaiveDate;
    use tempfile::tempdir;

    use crate::records::TimeRecord;

    use super::{
        StorageError, append_records_to_file, read_records_from_file, replace_file_with_records,
        write_record_to_file, write_records_to_file,
    };

    fn record(description: &str) -> TimeRecord {
        TimeRecord {
            created_at: NaiveDate::from_ymd_opt(2026, 11, 25).unwrap(),
            duration: Duration::from_secs(60),
            description: description.to_string(),
            project: None,
        }
    }

    #[test]
    fn missing_file_maps_to_open_for_read() {
        let temp_dir = tempdir().expect("Failed to create test temp dir");
        let data_file = temp_dir.path().join("data.csv");

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
        let temp_dir = tempdir().expect("Failed to create test temp dir");
        let data_file = temp_dir.path().join("data.csv");
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

    #[test]
    fn write_records_to_file_truncates_stale_content() {
        let temp_dir = tempdir().expect("Failed to create test temp dir");
        let data_file = temp_dir.path().join("data.csv");

        write_record_to_file(&data_file, record("stale")).expect("Failed to write stale record");
        write_records_to_file(&data_file, &[record("fresh")]).expect("Failed to write records");

        let records =
            read_records_from_file::<TimeRecord>(&data_file).expect("Failed to read records");
        assert_eq!(records.len(), 1, "stale rows were not truncated");
        assert_eq!(records[0].description, "fresh");
    }

    #[test]
    fn replace_file_with_records_discards_stale_temp_file() {
        let temp_dir = tempdir().expect("Failed to create test temp dir");
        let data_file = temp_dir.path().join("data.csv");
        let temp_file = temp_dir.path().join("data.csv.tmp");

        write_record_to_file(&temp_file, record("stale"))
            .expect("Failed to write stale temp record");
        assert!(temp_file.exists());

        replace_file_with_records(&data_file, &[record("fresh")])
            .expect("Failed to replace file with records");

        let records =
            read_records_from_file::<TimeRecord>(&data_file).expect("Failed to read records");
        assert_eq!(records.len(), 1, "stale temp rows were appended");
        assert_eq!(records[0].description, "fresh");
        assert!(
            !temp_file.exists(),
            "temp file should have been renamed away"
        );
    }

    #[test]
    fn append_records_to_file_preserves_existing_records() {
        let temp_dir = tempdir().expect("Failed to create test temp dir");
        let data_file = temp_dir.path().join("data.csv");

        write_record_to_file(&data_file, record("existing")).expect("Failed to write record");
        append_records_to_file(&data_file, &[record("first"), record("second")])
            .expect("Failed to append records");

        let records =
            read_records_from_file::<TimeRecord>(&data_file).expect("Failed to read records");
        assert_eq!(records.len(), 3);
        assert_eq!(records[0].description, "existing");
        assert_eq!(records[1].description, "first");
        assert_eq!(records[2].description, "second");
    }

    #[test]
    fn replace_file_with_records_rejects_path_without_file_name() {
        let error = replace_file_with_records(Path::new(""), &[record("fresh")]).unwrap_err();

        assert!(
            matches!(
                error.current_context(),
                StorageError::MissingFileName { .. }
            ),
            "unexpected error: {error:?}"
        );
    }

    #[cfg(unix)]
    #[test]
    fn replace_file_with_records_supports_non_utf8_file_name() {
        use std::os::unix::ffi::OsStringExt;

        let temp_dir = tempdir().expect("Failed to create test temp dir");
        let data_file = temp_dir
            .path()
            .join(std::ffi::OsString::from_vec(b"data-\xff.csv".to_vec()));

        replace_file_with_records(&data_file, &[record("fresh")])
            .expect("Failed to replace file with records");

        let records =
            read_records_from_file::<TimeRecord>(&data_file).expect("Failed to read records");
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].description, "fresh");
    }
}
