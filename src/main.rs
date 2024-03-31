mod cli;
mod handlers;
mod records;
mod storage;

use core::panic;
use std::fs;
use std::path::Path;

use clap::Parser;
use cli::Cli;
use cli::Command;

fn main() {
    let args = Cli::parse();

    // TODO: move all this to a function
    let current_task_directory = match std::env::var("XDG_STATE_HOME") {
        Ok(dir) => format!("{}/ttrack", dir),
        Err(_) => {
            let home_path = std::env::var("HOME").expect("Failed to fetch HOME env var");
            format!("{home_path}/.local/state/ttrack")
        }
    };

    let current_task_directory = Path::new(&current_task_directory);

    if !current_task_directory.exists() {
        if let Err(e) = fs::create_dir_all(current_task_directory) {
            panic!("Failed to create ttrack data folder: {}", e)
        }
    }

    let Some(data_file_name) = args.file.file_name() else {
        panic!("Failed to extract data filename from args")
    };

    let current_task_file = Path::new(current_task_directory).join(data_file_name);

    match args.command {
        Command::Track(cmd) => handlers::handle_track(cmd, args.file),
        Command::Report(cmd) => handlers::handle_report(cmd, args.file),
        Command::Start(cmd) => handlers::handle_start(cmd, current_task_file),
        Command::Pause => handlers::handle_pause(current_task_file),
        Command::Resume => handlers::handle_resume(current_task_file),
        Command::Stop => handlers::handle_stop(current_task_file, args.file),
    }
}
