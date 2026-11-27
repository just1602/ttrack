use std::{
    path::{Path, PathBuf},
    process::{Command, Output},
};

pub fn run_ttrack(args: &[&str], state_home: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_ttrack"))
        .args(args)
        .env("XDG_STATE_HOME", state_home)
        .output()
        .expect("Failed to run ttrack")
}

pub fn state_home(temp_dir: &Path) -> PathBuf {
    temp_dir.join("state")
}

pub fn state_dir(temp_dir: &Path) -> PathBuf {
    state_home(temp_dir).join("ttrack")
}

pub fn state_file(temp_dir: &Path) -> PathBuf {
    state_dir(temp_dir).join("data.csv")
}

pub fn data_file(temp_dir: &Path) -> PathBuf {
    temp_dir.join("data.csv")
}

pub fn run_in(temp_dir: &Path, args: &[&str]) -> Output {
    let data_path = data_file(temp_dir);
    let mut full_args = vec!["-f", data_path.to_str().expect("data path is UTF-8")];
    full_args.extend_from_slice(args);
    run_ttrack(&full_args, &state_home(temp_dir))
}

pub fn run_ok(temp_dir: &Path, args: &[&str]) -> Output {
    let output = run_in(temp_dir, args);
    assert!(
        output.status.success(),
        "ttrack {args:?} failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

pub fn assert_clean_error(output: &Output, expected: &str) {
    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains(expected), "unexpected stderr:\n{stderr}");
    assert!(!stderr.contains("panicked"), "ttrack panicked:\n{stderr}");
}
