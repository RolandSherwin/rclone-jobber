mod args;
mod job;
mod types;
mod utils;
use types::{Job, RcloneActions};

use crate::types::Graceful;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;
use subprocess::Exec;

fn main() {
    let arguments = args::get_args();
    let jobs = job::get_jobs(arguments.yaml_path);
    for job in jobs {
        if let Some(_) = job.log_path {
            write_log_header(&job, &arguments.action);
        }
        get_command(&job, &arguments.rclone_path, &arguments.action);
    }
}

fn get_command(job: &Job, rclone_exe: &PathBuf, action: &RcloneActions) {
    let log_path: Vec<String> = match job.log_path_str() {
        Some(log) => vec![String::from("--log-file"), log],
        None => vec![String::new()], // filtered out in the line below
    };
    let log_path = log_path
        .into_iter()
        .filter(|ele| *ele != String::from(""))
        .collect::<Vec<String>>();
    let filters = match job.filters_str() {
        Some(fil) => vec![String::from("--filter-from"), fil],
        None => vec![String::new()],
    };
    let filters = filters
        .into_iter()
        .filter(|ele| *ele != String::from(""))
        .collect::<Vec<String>>();

    let command = Exec::cmd(
        rclone_exe
            .to_str()
            .graceful("can't convert rclone_exe to str"),
    )
    .arg(action.to_string())
    .arg(job.source_str())
    .arg(job.destination_str())
    .args(&job.options)
    .args(&log_path)
    .args(&filters)
    .join();
    println!("command {:?}", &command);
}

fn write_log_header(job: &Job, action: &RcloneActions) {
    if let Some(path) = &job.log_path {
        let mut file = OpenOptions::new().append(true).open(path).graceful(
            format!("unable to open log file at {}", job.log_path_str().unwrap()).as_str(),
        );
        let mut header = format!("\n\n_____________________________________________{}_____________________________________________", action.to_string());
        header += &format!(
            "\nFROM: {}             TO: {}",
            job.source_str(),
            job.destination_str()
        )[..];
        write!(file, "{}", header).graceful("cannot write to log file");
        println!("Successfully wrote log header")
    }
}
