extern crate core;

mod args;
mod job;
mod types;
mod utils;
use types::{Job, RcloneActions};

use crate::types::Graceful;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;
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
    let log_path = match job.log_path_str() {
        Some(log) => format!("--log-file {}", log),
        None => String::new(),
    };
    let filters = match job.filters_str() {
        Some(fil) => format!("--filter-from {}", fil),
        None => String::new(),
    };

    let command = Exec::cmd(
        rclone_exe
            .to_str()
            .graceful("can't convert rclone_exe to str"),
    )
    .arg(action.to_string())
    .arg(job.source_str())
    .arg(job.destination_str())
    .arg(&job.options)
    .arg(log_path)
    .arg(filters).join();
    // command.arg(filter);
    // if let Some(_) = &job.filters {
    //     &command.arg(job.filters_str().unwrap());
    // }
    println!("command {:?}", &command);

}

// fn get_command(job: &Job, rclone_exe: &PathBuf, action: &RcloneActions) {
//     let mut command = Command::new(rclone_exe.to_str().unwrap().clone());
//     command
//         .arg(action.to_string().clone())
//         .arg(job.source.to_str().unwrap().clone())
//         .arg(job.destination.to_str().unwrap().clone())
//         .arg(job.options.clone());
//     // .output()
//     // .expect("failed to execute process");
//     // let mut temp_command: std::process::Command;
//     // println!("{:#?}", command);
//     match &job.log_path {
//         Some(path) => {
//             command.arg("--log-file").arg(path.clone());
//         }
//         None => (),
//     };
//     match &job.filters {
//         Some(filters) => {
//             for filter in filters {
//                 let f: String = format!("--filter\"{}\"", filter);
//                 command.arg(f);
//             }
//         }
//         None => (),
//     };
//     println!("{:#?}", command);
//     let output = command.output().expect("failed to execute");
//
//     // let mut command: String = rclone_exe.to_str().unwrap().to_string();
//     // command += " ";
//     // command += &action.to_string();
//     // command += " ";
//     // command += &job.source.to_str().unwrap().to_string();
//     // command += " ";
//     // command += &job.destination.to_str().unwrap().to_string();
//     // command += " ";
//     // command += &job.options;
//     // match &job.log_path {
//     //     Some(path) => {
//     //         command += " --log-file ";
//     //         let p = path.into_os_string().into_string().unwrap();
//     //         println!("{}", p);
//     //         // command += p;
//     //     }
//     //     None => (),
//     // };
//     // match &job.filters {
//     //     Some(filters) => {
//     //         for filter in filters {
//     //             command += " --filter \"";
//     //             command += filter;
//     //             command += "\"";
//     //         }
//     //     }
//     //     None => (),
//     // };
//     // println!("{:?}", command);
//     println!("status: {}", output.status);
//     println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
//     println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
// }

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
