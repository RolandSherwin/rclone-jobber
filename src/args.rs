use crate::job::RcloneActions;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
pub(crate) struct Arguments {
    // Action to perform, ie check, copy, sync
    #[structopt(short = "a", long = "action", default_value = "check")]
    pub action: RcloneActions,

    // Path to a job.json file
    #[structopt(short = "y", long = "yamlPath", parse(from_os_str))]
    pub yaml_path: PathBuf,

    #[structopt(short = "r", long = "rclonePath", parse(from_os_str))]
    pub rclone_path: PathBuf,
}

fn validate_path(path: &PathBuf) {
    if !path.is_absolute() {
        panic!("Path: {:?} in not an absoulte path!", path);
    }
    if !path.exists() {
        panic!("Path: {:?} does not exists", path);
    }
}
fn validate_yaml(path: &PathBuf) {
    validate_path(path);
    if path.extension().unwrap() != "yaml" {
        panic!("Path: {:?} is not a YAML file!", path);
    }
}

pub(crate) fn get_args() -> Arguments {
    let args = Arguments::from_args();
    validate_yaml(&args.yaml_path);
    validate_path(&args.rclone_path);
    return args;
}
