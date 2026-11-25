use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use tempfile::tempdir;

fn run_ttrack(args: &[&str], state_home: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_ttrack"))
        .args(args)
        .env("XDG_STATE_HOME", state_home)
        .output()
        .expect("Failed to run ttrack")
}

fn state_home(temp_dir: &Path) -> PathBuf {
    temp_dir.join("state")
}

fn state_file(temp_dir: &Path) -> PathBuf {
    state_home(temp_dir).join("ttrack/data.csv")
}

fn run_in(temp_dir: &Path, args: &[&str]) -> std::process::Output {
    let data_file = temp_dir.join("data.csv");
    let mut full_args = vec!["-f", data_file.to_str().unwrap()];
    full_args.extend_from_slice(args);
    run_ttrack(&full_args, &state_home(temp_dir))
}

fn run_ok(temp_dir: &Path, args: &[&str]) {
    let output = run_in(temp_dir, args);
    assert!(
        output.status.success(),
        "ttrack {args:?} failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn assert_clean_error(output: &std::process::Output, expected: &str) {
    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains(expected), "unexpected stderr:\n{stderr}");
    assert!(!stderr.contains("panicked"), "ttrack panicked:\n{stderr}");
}

#[test]
fn missing_data_file_reports_error_without_panic() {
    let temp_dir = tempdir().expect("Failed to create test temp dir");
    let data_file = temp_dir.path().join("data.csv");

    let output = run_ttrack(
        &["-f", data_file.to_str().unwrap(), "report"],
        &state_home(temp_dir.path()),
    );

    assert_clean_error(&output, "Failed to load records for report");
}

#[test]
fn malformed_csv_reports_error_without_panic() {
    let temp_dir = tempdir().expect("Failed to create test temp dir");
    let data_file = temp_dir.path().join("data.csv");
    fs::write(&data_file, "2026-11-25|not-a-number\n").expect("Failed to write test data");

    let output = run_ttrack(
        &["-f", data_file.to_str().unwrap(), "report"],
        &state_home(temp_dir.path()),
    );

    assert_clean_error(&output, "Failed to deserialize records from CSV");
}

#[test]
fn pause_without_current_task_reports_error_without_panic() {
    let temp_dir = tempdir().expect("Failed to create test temp dir");
    let data_file = temp_dir.path().join("data.csv");
    let state_dir = temp_dir.path().join("state/ttrack");
    fs::create_dir_all(&state_dir).expect("Failed to create state dir");
    fs::write(state_dir.join("data.csv"), "").expect("Failed to write empty state file");

    let output = run_ttrack(
        &["-f", data_file.to_str().unwrap(), "pause"],
        &state_home(temp_dir.path()),
    );

    assert_clean_error(&output, "There's no current task to pause");
}

#[test]
fn resume_and_stop_without_current_task_report_errors_without_panic() {
    for (command, expected) in [
        ("resume", "There is no current task to resume"),
        ("stop", "There is no current task to stop"),
    ] {
        let temp_dir = tempdir().expect("Failed to create test temp dir");
        fs::create_dir_all(state_file(temp_dir.path()).parent().unwrap())
            .expect("Failed to create state dir");
        fs::write(state_file(temp_dir.path()), "").expect("Failed to write empty state file");

        let output = run_in(temp_dir.path(), &[command]);

        assert_clean_error(&output, expected);
    }
}

#[test]
fn second_pause_is_rejected_without_touching_state() {
    let temp_dir = tempdir().expect("Failed to create test temp dir");
    run_ok(temp_dir.path(), &["start", "-d", "A4 task"]);
    run_ok(temp_dir.path(), &["pause"]);
    let before = fs::read(state_file(temp_dir.path())).expect("Failed to read state file");

    let output = run_in(temp_dir.path(), &["pause"]);

    assert_clean_error(&output, "Current task is already paused");
    assert_eq!(
        fs::read(state_file(temp_dir.path())).expect("Failed to read state file"),
        before,
        "rejected pause changed the state file"
    );
}

#[test]
fn resume_of_active_task_is_rejected_without_touching_state() {
    let temp_dir = tempdir().expect("Failed to create test temp dir");
    run_ok(temp_dir.path(), &["start", "-d", "A4 task"]);
    let before = fs::read(state_file(temp_dir.path())).expect("Failed to read state file");

    let output = run_in(temp_dir.path(), &["resume"]);

    assert_clean_error(&output, "Current task is already running");
    assert_eq!(
        fs::read(state_file(temp_dir.path())).expect("Failed to read state file"),
        before,
        "rejected resume changed the state file"
    );
}

#[test]
fn stop_of_paused_task_is_rejected_without_touching_state() {
    let temp_dir = tempdir().expect("Failed to create test temp dir");
    run_ok(temp_dir.path(), &["start", "-d", "A4 task"]);
    run_ok(temp_dir.path(), &["pause"]);
    let before = fs::read(state_file(temp_dir.path())).expect("Failed to read state file");

    let output = run_in(temp_dir.path(), &["stop"]);

    assert_clean_error(&output, "Current task is already paused");
    assert_eq!(
        fs::read(state_file(temp_dir.path())).expect("Failed to read state file"),
        before,
        "rejected stop changed the state file"
    );
    assert!(
        !temp_dir.path().join("data.csv").exists(),
        "rejected stop wrote a time record"
    );
}

#[test]
fn valid_start_pause_resume_stop_sequence_succeeds() {
    let temp_dir = tempdir().expect("Failed to create test temp dir");
    run_ok(temp_dir.path(), &["start", "-d", "A4 task"]);
    run_ok(temp_dir.path(), &["pause"]);
    run_ok(temp_dir.path(), &["resume"]);
    run_ok(temp_dir.path(), &["stop"]);

    let records = fs::read_to_string(temp_dir.path().join("data.csv"))
        .expect("Failed to read time record file");
    assert_eq!(records.lines().count(), 1);
    assert!(records.contains("A4 task"));
}
