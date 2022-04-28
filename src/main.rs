mod args;
mod job;
mod types;
mod utils;

use types::{Job, RcloneActions};

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let arguments = args::get_args();
    let jobs = job::get_jobs(arguments.yaml_path);
    for job in jobs {
        // println!("{:#?}", job);
        match &job.log_path {
            Some(_) => write_log_header(&job, &arguments.action),
            None => (),
        }
        get_command(&job, &arguments.rclone_path, &arguments.action);

        // println!("{:#?}", command);
    }
}

fn get_command(job: &Job, rclone_exe: &PathBuf, action: &RcloneActions) {
    // let gg: std::string::String = String::new();
    let mut command: std::process::Command = Command::new(rclone_exe.to_str().unwrap().clone());
    command
        .arg(action.to_string().clone())
        .arg(job.source.to_str().unwrap().clone())
        .arg(job.destination.to_str().unwrap().clone())
        .arg(job.options.clone());
    // .output()
    // .expect("failed to execute process");
    // let mut temp_command: std::process::Command;
    // println!("{:#?}", command);
    match &job.log_path {
        Some(path) => {
            command.arg("--log-file").arg(path.clone());
        }
        None => (),
    };
    match &job.filters {
        Some(filters) => {
            for filter in filters {
                let f: String = format!("--filter\"{}\"", filter);
                command.arg(f);
            }
        }
        None => (),
    };
    println!("{:#?}", command);
    let output = command.output().expect("failed to execute");

    // let mut command: String = rclone_exe.to_str().unwrap().to_string();
    // command += " ";
    // command += &action.to_string();
    // command += " ";
    // command += &job.source.to_str().unwrap().to_string();
    // command += " ";
    // command += &job.destination.to_str().unwrap().to_string();
    // command += " ";
    // command += &job.options;
    // match &job.log_path {
    //     Some(path) => {
    //         command += " --log-file ";
    //         let p = path.into_os_string().into_string().unwrap();
    //         println!("{}", p);
    //         // command += p;
    //     }
    //     None => (),
    // };
    // match &job.filters {
    //     Some(filters) => {
    //         for filter in filters {
    //             command += " --filter \"";
    //             command += filter;
    //             command += "\"";
    //         }
    //     }
    //     None => (),
    // };
    // println!("{:?}", command);
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}

fn write_log_header(job: &Job, action: &RcloneActions) {
    let path = job.log_path.as_ref().unwrap();
    let mut file = OpenOptions::new().append(true).open(path).unwrap();
    // .expect(&format!("Error opening log file at {}", path.to_str().unwrap())[..]);
    let mut header: String = format!("\n\n_____________________________________________{}_____________________________________________", &action.to_string());
    header += &format!(
        "\nFROM: {}             TO: {}",
        &job.source.to_str().unwrap().to_string(),
        &job.destination.to_str().unwrap().to_string()
    )[..];
    // let header: &str = &header[..];
    write!(file, "{}", header).unwrap();
    println!("Succesfully wrote log header")
}
