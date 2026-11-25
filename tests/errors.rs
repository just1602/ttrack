use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn temp_dir(label: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("ttrack-a1-{}-{label}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("Failed to create test temp dir");
    dir
}

fn run_ttrack(args: &[&str], state_home: &PathBuf) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_ttrack"))
        .args(args)
        .env("XDG_STATE_HOME", state_home)
        .output()
        .expect("Failed to run ttrack")
}

fn assert_clean_error(output: &std::process::Output, expected: &str) {
    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains(expected), "unexpected stderr:\n{stderr}");
    assert!(!stderr.contains("panicked"), "ttrack panicked:\n{stderr}");
}

#[test]
fn missing_data_file_reports_error_without_panic() {
    let dir = temp_dir("missing");
    let data_file = dir.join("data.csv");

    let output = run_ttrack(
        &["-f", data_file.to_str().unwrap(), "report"],
        &dir.join("state"),
    );

    assert_clean_error(&output, "Failed to load records for report");
}

#[test]
fn malformed_csv_reports_error_without_panic() {
    let dir = temp_dir("malformed");
    let data_file = dir.join("data.csv");
    fs::write(&data_file, "2026-11-25|not-a-number\n").expect("Failed to write test data");

    let output = run_ttrack(
        &["-f", data_file.to_str().unwrap(), "report"],
        &dir.join("state"),
    );

    assert_clean_error(&output, "Failed to deserialize records from CSV");
}

#[test]
fn pause_without_current_task_reports_error_without_panic() {
    let dir = temp_dir("pause");
    let state_dir = dir.join("state/ttrack");
    fs::create_dir_all(&state_dir).expect("Failed to create state dir");
    fs::write(state_dir.join("data.csv"), "").expect("Failed to write empty state file");

    let output = run_ttrack(
        &["-f", dir.join("data.csv").to_str().unwrap(), "pause"],
        &dir.join("state"),
    );

    assert_clean_error(&output, "There's no current task to pause");
}
