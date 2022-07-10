use log::error;
use std::fmt::{Debug, Display, Formatter};
use std::{fmt, path::PathBuf, process, str::FromStr};

pub(crate) type StringError = String;

pub trait Graceful {
    type V;
    fn graceful(self, message: &str) -> Self::V;
}

impl<T> Graceful for Option<T> {
    type V = T;
    fn graceful(self, message: &str) -> Self::V {
        match self {
            Some(val) => val,
            None => {
                let msg = format!("Error: {}", message);
                error!("{}", msg);
                println!("{}", msg);
                process::exit(1);
            }
        }
    }
}

impl<T, E: Display + Debug> Graceful for Result<T, E> {
    type V = T;
    fn graceful(self, message: &str) -> Self::V {
        match self {
            Ok(val) => val,
            Err(err) => {
                let msg = format!("Error: {}\nReason: {}", message, err);
                error!("{}", msg);
                println!("{}", msg);
                process::exit(1);
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct Job {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub options: Vec<String>,
    pub log_path: Option<PathBuf>,
    pub tmp_filter_file: PathBuf,
}

pub(crate) enum PathType {
    Local(PathBuf),
    Remote(PathBuf),
}

#[derive(Debug)]
pub(crate) enum RcloneActions {
    Check,
    Copy,
    Sync,
}

impl Job {
    pub(crate) fn source_str(&self) -> String {
        self.source
            .to_str()
            .graceful("can't convert 'Job.source' to str")
            .to_string()
    }
    pub(crate) fn destination_str(&self) -> String {
        self.destination
            .to_str()
            .graceful("can't convert 'Job.destination' to str")
            .to_string()
    }
    pub(crate) fn log_path_str(&self) -> Option<String> {
        self.log_path.as_ref().map(|path| {
            path.to_str()
                .graceful("can't convert 'Job.log_path' to str")
                .to_string()
        })
    }
    // pub(crate) fn filters_str(&self) -> Option<String> {
    //     if let Some(filters) = &self.filters {
    //         Some(filters.join(" "))
    //     } else {
    //         None
    //     }
    // }
}

// To be used in StructOpt in args, we have to implement the trait FromStr for
// the enum - https://stackoverflow.com/questions/54687403/how-can-i-use-enums-in-structopt
// any error type implementing Display is acceptable.
impl FromStr for RcloneActions {
    type Err = StringError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "check" | "Check" => Ok(RcloneActions::Check),
            "copy" | "Copy" => Ok(RcloneActions::Copy),
            "sync" | "Sync" => Ok(RcloneActions::Sync),
            _ => Err(String::from("Please provide check or copy or sync.")),
        }
    }
}
// to convert enum to string; enables to_string()
impl Display for RcloneActions {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match &self {
            RcloneActions::Check => write!(fmt, "check"),
            RcloneActions::Copy => write!(fmt, "copy"),
            RcloneActions::Sync => write!(fmt, "sync"),
        }
    }
}

// impl Display for PathField {
//     fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
//         match &self {
//             PathField::Remote(_) => write!(fmt, "remote"),
//             PathField::Windows(_) => write!(fmt, "windows"),
//             PathField::Linux(_) => write!(fmt, "linux"),
//         }
//     }
// }
