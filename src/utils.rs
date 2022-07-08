use crate::types::Graceful;
use serde_yaml::Value;
use std::{fs::File, path::PathBuf, process};

pub(crate) fn validate_path(path: &PathBuf) {
    if !path.is_absolute() {
        println!("{:?} in not an absolute path!", path);
        process::exit(1);
    }
    if !path.exists() {
        println!("{:?} does not exists", path);
        process::exit(1);
    }
}

pub(crate) fn validate_yaml(path: &PathBuf) {
    validate_path(path);
    if path.extension().graceful("Cant get extension") != "yaml" {
        println!("{:?} is not a YAML file!", path);
        process::exit(1);
    }
}

pub(crate) fn read_yaml(path: PathBuf) -> Value {
    let file = File::open(path).graceful("Unable to open file");
    serde_yaml::from_reader(file).graceful("Unable to read YAML")
}
