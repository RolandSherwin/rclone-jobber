mod args;
mod job;
mod types;
mod utils;

use log::{debug, LevelFilter};
use log4rs::append::rolling_file::{
    policy::compound::{
        roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
    },
    RollingFileAppender,
};
use log4rs::{
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config, Handle,
};
use std::{fs::OpenOptions, io::prelude::*, path::Path};
use subprocess::Exec;
use types::{Graceful, Job, RcloneActions};

fn main() {
    let arguments = args::get_args();

    let file_name = arguments
        .yaml_path
        .file_stem()
        .graceful("cannot get job.yaml file_name")
        .to_str()
        .graceful("cannot get job.yaml to str");
    let _handle = get_logger(file_name);

    let jobs = job::get_jobs(arguments.yaml_path);
    for job in jobs {
        if job.log_path.is_some() {
            write_log_header(&job, &arguments.action);
        }
        get_command(&job, &arguments.rclone_path, &arguments.action);
    }
}

fn get_command(job: &Job, rclone_exe: &Path, action: &RcloneActions) {
    let log_path: Vec<String> = match job.log_path_str() {
        Some(log) => vec![String::from("--log-file"), log],
        None => vec![String::new()], // filtered out in the line below
    };
    let log_path = log_path
        .into_iter()
        .filter(|ele| ele.as_str() != "")
        .collect::<Vec<String>>();
    let filters = match &job.filters {
        Some(f) => f
            .iter()
            .map(|fil| {
                let mut op = String::from("--filter=");
                op += fil.as_str();
                op
            })
            .collect::<Vec<String>>(),
        None => vec![String::new()],
    };
    let filters = filters
        .into_iter()
        .filter(|ele| ele.as_str() != "")
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
    debug!("command {:?}", &command);
}

fn write_log_header(job: &Job, action: &RcloneActions) {
    if let Some(path) = &job.log_path {
        let mut file = OpenOptions::new().append(true).open(path).graceful(
            format!("unable to open log file at {}", job.log_path_str().unwrap()).as_str(),
        );
        let mut header = format!("\n\n_____________________________________________{}_____________________________________________", action);
        header += &format!(
            "\nFROM: {}             TO: {}\n",
            job.source_str(),
            job.destination_str()
        )[..];
        write!(file, "{}", header).graceful("cannot write to log file");
    }
}

// Different logs for each job.yaml file.
// https://medium.com/nikmas-group-rust/advanced-logging-in-rust-with-log4rs-2d712bb322de
fn get_logger(filename: &str) -> Handle {
    let log_line_pattern = "{d(%Y-%m-%d %H:%M:%S)} | {({l}):5.5} | {f}:{L} — {m}{n}";
    let trigger = Box::new(SizeTrigger::new(10485760)); //10mb

    let roller_pattern = String::from("logs/") + filename + "_{}.gz";
    let roller_pattern = roller_pattern.as_str();
    let roller_count = 3;
    let roller_base = 1;
    let roller = Box::new(
        FixedWindowRoller::builder()
            .base(roller_base)
            .build(roller_pattern, roller_count)
            .graceful("log4rs roller error"),
    );
    let compound_policy = Box::new(CompoundPolicy::new(trigger, roller));

    let log_location = String::from("logs/app/") + filename + ".log";
    let log_location = log_location.as_str();
    let root_ap = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(log_line_pattern)))
        .build(log_location, compound_policy)
        .graceful("log4rs root_ap error");

    let config = Config::builder()
        .appender(Appender::builder().build("my_root", Box::new(root_ap)))
        .build(
            Root::builder()
                .appender("my_root")
                .build(LevelFilter::Debug),
        )
        .graceful("log4rs config error");

    log4rs::init_config(config).graceful("log4rs return error")
}
