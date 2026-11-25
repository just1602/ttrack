mod cli;
mod handlers;
mod records;
mod storage;

use std::fs;
use std::path::PathBuf;

use clap::Parser;
use cli::Cli;
use cli::Command;
use rootcause::bail;
use rootcause::compat::MainReport;
use rootcause::prelude::ResultExt;

fn main() -> Result<(), MainReport> {
    let args = Cli::parse();

    // TODO: move all this to a function
    let current_task_directory = match std::env::var("XDG_STATE_HOME") {
        Ok(dir) => PathBuf::from(dir).join("ttrack"),
        Err(_) => {
            let home_path = std::env::var("HOME")
                .context("Failed to fetch HOME environment variable")
                .attach("XDG_STATE_HOME is not set")?;
            PathBuf::from(home_path).join(".local/state/ttrack")
        }
    };

    if !current_task_directory.exists()
        && let Err(e) = fs::create_dir_all(&current_task_directory)
    {
        bail!(
            "Failed to create ttrack data folder: {} (path: {})",
            e,
            current_task_directory.display()
        );
    }

    let Some(data_file_name) = args.file.file_name() else {
        bail!(
            "Failed to extract data filename from path: {}",
            args.file.display()
        );
    };

    let current_task_file = current_task_directory.join(data_file_name);

    match args.command {
        Command::Track(cmd) => handlers::handle_track(cmd, args.file)?,
        Command::Report(cmd) => handlers::handle_report(cmd, args.file)?,
        Command::Start(cmd) => handlers::handle_start(cmd, current_task_file)?,
        Command::Pause => handlers::handle_pause(current_task_file)?,
        Command::Resume => handlers::handle_resume(current_task_file)?,
        Command::Stop => handlers::handle_stop(current_task_file, args.file)?,
    }

    Ok(())
}
