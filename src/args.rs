use crate::types::RcloneActions;
use crate::utils;
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(name = "Rclone Batcher")]
pub(crate) struct Arguments {
    /// Action to perform, ie check, copy, sync
    #[clap(short = 'a', long = "action", default_value = "check")]
    pub action: RcloneActions,

    /// Path to a job.json file
    #[clap(short = 'y', long = "yamlPath", parse(from_os_str))]
    pub yaml_path: PathBuf,

    /// path to rclone binary
    #[clap(short = 'r', long = "rclonePath", parse(from_os_str))]
    pub rclone_path: PathBuf,
}

pub(crate) fn get_args() -> Arguments {
    let args = Arguments::from_args();
    utils::validate_yaml(&args.yaml_path);
    utils::validate_path(&args.rclone_path);
    args
}
