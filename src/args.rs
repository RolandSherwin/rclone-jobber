use crate::types::RcloneActions;
use crate::utils;
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

pub(crate) fn get_args() -> Arguments {
    let args = Arguments::from_args();
    utils::validate_yaml(&args.yaml_path);
    utils::validate_path(&args.rclone_path);
    args
}
