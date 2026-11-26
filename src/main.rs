mod cli;
mod completion;
mod handlers;
mod records;
mod storage;

use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;
use cli::{Cli, Command};
use rootcause::compat::MainReport;
use rootcause::{bail, option_ext::OptionExt, prelude::ResultExt};

fn current_task_file_for(file: &Path) -> rootcause::Result<PathBuf> {
    let state_directory = match std::env::var_os("XDG_STATE_HOME") {
        Some(dir) => PathBuf::from(dir).join("ttrack"),
        None => {
            let home_path = std::env::var_os("HOME")
                .context("Failed to fetch HOME environment variable")
                .attach("XDG_STATE_HOME is not set")?;
            PathBuf::from(home_path)
                .join(".local")
                .join("state")
                .join("ttrack")
        }
    };

    if !state_directory.exists()
        && let Err(e) = fs::create_dir_all(&state_directory)
    {
        bail!(
            "Failed to create ttrack data folder: {} (path: {})",
            e,
            state_directory.display()
        );
    }

    let Some(data_file_name) = file.file_name() else {
        bail!(
            "Failed to extract data filename from path: {}",
            file.display()
        );
    };

    Ok(state_directory.join(data_file_name))
}

fn main() -> Result<(), MainReport> {
    let args: Vec<_> = std::env::args_os().collect();
    if completion::try_run_from(&args) {
        return Ok(());
    }

    let Cli { file, command } = Cli::parse_from(args);

    match command {
        Command::Track(cmd) => handlers::handle_track(cmd, file)?,
        Command::Report(cmd) => handlers::handle_report(cmd, file)?,
        Command::Start(cmd) => {
            let current_task_file = current_task_file_for(&file)?;
            handlers::handle_start(cmd, current_task_file)?;
        }
        Command::Pause => {
            let current_task_file = current_task_file_for(&file)?;
            handlers::handle_pause(current_task_file)?;
        }
        Command::Resume => {
            let current_task_file = current_task_file_for(&file)?;
            handlers::handle_resume(current_task_file)?;
        }
        Command::Stop => {
            let current_task_file = current_task_file_for(&file)?;
            handlers::handle_stop(current_task_file, file)?;
        }
    }

    Ok(())
}
