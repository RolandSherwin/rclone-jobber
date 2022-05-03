use crate::types::{Graceful, Job, PathType, StringError};
use crate::utils;
use serde_yaml::Value;
use std::{env, path::PathBuf};
use std::fs::File;
use log::warn;

pub(crate) fn get_jobs(path: PathBuf) -> Vec<Job> {
    return _get_jobs(utils::read_yaml(path));
}

fn _get_jobs(file_contents: Value) -> Vec<Job> {
    let mut jobs: Vec<Job> = Vec::new();
    for job_data in file_contents.as_mapping().unwrap().iter() {
        // its a tuple of (Value, Value) eg ("job1", contents..) so get index 1 to
        let job_name = job_data.0.as_str().unwrap();
        let contents = serde_yaml::to_value(job_data.1).unwrap();

        let source = contents
            .get("source")
            .graceful(format!("'source' field not found for job: {}", job_name).as_str());
        let source = match extract_path_field(source, false)
            .graceful(format!("reading 'source' of job: {}", job_name).as_str())
        {
            PathType::Local(path) => path,
            PathType::Remote(path) => path,
        };

        let destination = contents
            .get("dest")
            .graceful(format!("'dest' field not found for job: {}", job_name).as_str());
        let destination = match extract_path_field(destination, false)
            .graceful(format!("reading 'dest' of job: {}", job_name).as_str())
        {
            PathType::Local(path) => path,
            PathType::Remote(path) => path,
        };
        let options: Vec<String> = contents
            .get("options")
            .graceful(format!("'options' field not found for job: {}", job_name).as_str())
            .as_str()
            .graceful("as_str while getting 'options'")
            .split(" ")
            .map(|ele| String::from(ele))
            .filter(|ele| *ele != String::from(""))
            .collect();

        let log_path = match contents.get("log_path") {
            None => {
                println!("'log_path' not found for job: {}, skipping!", job_name);
                None
            }
            Some(val) => {
                match extract_path_field(val, true)
                    .graceful(format!("reading 'log_path' of job: {}", job_name).as_str())
                {
                    PathType::Local(path) => Some(path),
                    // don't consider remote for log location
                    PathType::Remote(_) => None,
                }
            }
        };
        let filters = match contents.get("filters") {
            None => {
                // println!("'filters' not found for job: {}, skipping!", job_name);
                None
            }
            Some(vals) => {
                let mut opt = Vec::new();
                for val in vals
                    .as_sequence()
                    .graceful(format!("'filters' field is empty in job: {:?}", job_name).as_str())
                {
                    opt.push(
                        val.as_str()
                            .graceful(
                                format!("cannot get 'filters' field in job: {:?}", job_name)
                                    .as_str(),
                            )
                            .to_string(),
                    );
                }
                Some(opt)
            }
        };

        let job: Job = Job {
            source,
            destination,
            options,
            log_path,
            filters,
        };
        jobs.push(job)
    }
    return jobs;
}

fn extract_path_field(val: &Value, create_file: bool) -> Result<PathType, StringError> {
    // extracts the PathFields present eg: source: -linux: /path/
    // can contain windows/linux together or remote alone.
    if let Some(remote) = val.get("remote") {
        if let Some(_) = val.get("linux") {
            return Err(String::from("'remote' can only exist alone."));
        };
        if let Some(_) = val.get("windows") {
            return Err(String::from("'remote' can only exist alone."));
        };
        let remote = PathBuf::from(
            remote
                .as_str()
                .graceful("cannot convert 'remote' Value to str"),
        );
        return Ok(PathType::Remote(remote));
    };

    match val.get(env::consts::OS) {
        Some(path) => {
            let path = path.as_str().graceful("cannot convert 'path' Value to str");
            let path = PathBuf::from(path);
            if path.exists() {
                if env::consts::OS == "linux" {
                    return Ok(PathType::Local(path));
                } else if env::consts::OS == "windows" {
                    return Ok(PathType::Local(path));
                } else {
                    return Err(String::from("only works in linux or windows."));
                }
            } else {
                if create_file{
                    warn!("{}", format!("creating file at {:?}", path));
                    File::create(path.clone()).graceful(format!("while creating file at {:?}", path).as_str());
                    return Ok(PathType::Local(path));
                }
                else{
                    return Err(format!("path {:?} does not exists.", path));
                }
            }
        }
        None => return Err(String::from("OS field not found or is empty.")),
    };
}
