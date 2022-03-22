// use serde_json;
use serde_yaml;
use std::env;
use std::fmt;
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug)]
pub(crate) enum RcloneActions {
    CHECK,
    COPY,
    SYNC,
}

// To be used in StructOpt in args, we have to implement the trait FromStr for
// the enum - https://stackoverflow.com/questions/54687403/how-can-i-use-enums-in-structopt
// any error type implementing Display is acceptable.
type ParseError = &'static str;
impl FromStr for RcloneActions {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "check" | "Check" => Ok(RcloneActions::CHECK),
            "copy" | "Copy" => Ok(RcloneActions::COPY),
            "sync" | "Sync" => Ok(RcloneActions::SYNC),
            _ => Err("Please provide check or copy or sync."),
        }
    }
}
// For printing an enum as String
impl fmt::Display for RcloneActions {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            RcloneActions::CHECK => write!(fmt, "check"),
            RcloneActions::COPY => write!(fmt, "copy"),
            RcloneActions::SYNC => write!(fmt, "sync"),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Job {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub options: String,
    pub log_path: Option<PathBuf>,
    pub filters: Option<Vec<String>>,
}

pub(crate) fn get_jobs(path: PathBuf) -> Vec<Job> {
    return _get_jobs(read_yaml(path));
}

fn read_yaml(path: PathBuf) -> serde_yaml::Value {
    let file = File::open(path).expect("Unable to open file");
    let contents: serde_yaml::Value = serde_yaml::from_reader(file).expect("Unable to read YAML");
    return contents;
}

fn _get_jobs(contents: serde_yaml::Value) -> Vec<Job> {
    let mut jobs: Vec<Job> = Vec::new();
    for (i, entities) in contents.as_mapping().unwrap().iter().enumerate() {
        // its a tuple of (Value, Value) eg ("job1", contents..) so get index 1
        let source: PathBuf;
        let destination: PathBuf;
        let options: String;
        let mut log_path: Option<PathBuf> = None;
        let mut filters: Option<Vec<String>> = None;
        let entities: serde_yaml::Value = serde_yaml::to_value(entities.1).unwrap();
        match entities.get("source") {
            None => panic!("'source' field not found in job number: {}", i + 1),
            Some(val) => match extract_path_field(val) {
                Ok(path) => source = path,
                Err(e) => panic!(
                    "Error: While getting 'source' of job{}, the following error occured: {}",
                    i + 1,
                    e
                ),
            },
        };
        match entities.get("dest") {
            None => panic!("'dest' field not found in job number: {}", i + 1),
            Some(val) => match extract_path_field(val) {
                Ok(path) => destination = path,
                Err(e) => panic!(
                    "Error: While getting 'destination' of job{}, the following error occured: {}",
                    i + 1,
                    e
                ),
            },
        };
        match entities.get("options") {
            None => panic!("'options' field not found in job{}", i + 1),
            Some(val) => options = val.as_str().unwrap().to_string(),
        };
        match entities.get("log_path") {
            None => println!("'log_path' not found for job{}, skipping!", i + 1),
            Some(val) => match extract_path_field(val) {
                Ok(path) => log_path = Some(path),
                // if log_path present, but trouble reading values, panic
                Err(e) => panic!(
                    "Error: While reading 'log_path' of job{}, the following error occured: {}",
                    i + 1,
                    e
                ),
            },
        };
        match entities.get("filters") {
            None => println!("'filters' not found for job{}, skipping!", i + 1),
            Some(values) => {
                let mut opt: Vec<String> = Vec::new();
                let error_msg: String = format!("'filters' field is empty in job{}", i + 1);
                let error_msg: &str = &error_msg[..];
                for val in values.as_sequence().expect(error_msg) {
                    let error_msg: &str = &format!("'filters' field is empty in job{}", i + 1)[..];
                    opt.push(val.as_str().expect(error_msg).to_string());
                }
                filters = Some(opt);
            }
        }
        let job: Job = Job {
            source,
            destination,
            options,
            log_path,
            filters,
        };
        // println!("{:?}\n", job);
        jobs.push(job)
    }
    return jobs;
}

fn extract_path_field(val: &serde_yaml::Value) -> Result<PathBuf, &'static str> {
    // if given directly, ie as source: "/home/roland/", capture it directly
    // else if given as options like - windows -linux, get the value for the current OS

    // If value given directly, return it.
    if let Some(i) = val.as_str() {
        return Ok(PathBuf::from(i));
    }
    // Else val.as_str() returns None if no value is passed eg (- source: )
    // or if it has mapping with multiple values ie (- source: \n -window: "/path/" \n-linux:"/path/")
    let path: &str;
    match val.get(env::consts::OS) {
        Some(i) => path = i.as_str().unwrap(),
        None => return Err("OS field not found or is empty."),
    };

    // check if path is valid
    let path: PathBuf = PathBuf::from(path);
    if path.exists() {
        return Ok(path);
    } else {
        return Err("Path does not exists.");
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::job;
//     use std::path::PathBuf;
//     #[test]
//     fn it_works() {
//         let contents = job::read_yaml(PathBuf::from(
//             "E:\\New-Folder-2\\Projects\\rclone-batcher\\test.yaml",
//         ));
//         let jobs = job::get_jobs(contents);
//     }
// }
