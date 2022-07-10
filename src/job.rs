use crate::types::{Graceful, Job, PathType, StringError};
use crate::utils;
use log::warn;
use serde_yaml::Value;
use std::{
    env, fs,
    io::Write,
    path::{Path, PathBuf},
};

pub(crate) fn get_jobs(path: PathBuf, tmp_dir: &Path) -> Vec<Job> {
    _get_jobs(utils::read_yaml(path), tmp_dir)
}

fn _get_jobs(file_contents: Value, tmp_dir: &Path) -> Vec<Job> {
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
            .split(' ')
            .map(String::from)
            .filter(|ele| ele.as_str() != "")
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
        let filters_path = tmp_dir.join(job_name);
        let mut filters_file =
            fs::File::create(&filters_path).graceful("Unable to create temp 'filter' file");
        if let Some(vals) = contents.get("filters") {
            for val in vals
                .as_sequence()
                .graceful(format!("'filters' field is empty in job: {:?}", job_name).as_str())
            {
                writeln!(
                    filters_file,
                    "{}",
                    val.as_str().graceful(
                        format!("cannot get 'filters' field in job: {:?}", job_name).as_str(),
                    )
                )
                .graceful("Error writing to 'filters' file");
            }
        };

        let job: Job = Job {
            source,
            destination,
            options,
            log_path,
            tmp_filter_file: filters_path, // empty file if no filter is found
        };
        jobs.push(job)
    }
    jobs
}

fn extract_path_field(val: &Value, create_file: bool) -> Result<PathType, StringError> {
    // extracts the PathFields present eg: source: -linux: /path/
    // can contain windows/linux together or remote alone.
    if let Some(remote) = val.get("remote") {
        if val.get("windows").is_some() || val.get("linux").is_some() {
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
                let os = env::consts::OS;
                if os == "linux" || os == "windows" {
                    Ok(PathType::Local(path))
                } else {
                    Err(String::from("only works in linux or windows."))
                }
            } else if create_file {
                warn!("{}", format!("creating file at {:?}", path));
                fs::File::create(path.clone())
                    .graceful(format!("while creating file at {:?}", path).as_str());
                Ok(PathType::Local(path))
            } else {
                Err(format!("path {:?} does not exists.", path))
            }
        }
        None => Err(String::from("OS field not found or is empty.")),
    }
}
