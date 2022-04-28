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
                println!("Error: {}", message);
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
                println!("Error: {}\nReason: {}", message, err);
                process::exit(1);
            }
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

// pub(crate) enum PathField {
//     Linux(PathBuf),
//     Windows(PathBuf),
//     Remote(PathBuf),
// }

#[derive(Debug)]
pub(crate) enum RcloneActions {
    CHECK,
    COPY,
    SYNC,
}

// To be used in StructOpt in args, we have to implement the trait FromStr for
// the enum - https://stackoverflow.com/questions/54687403/how-can-i-use-enums-in-structopt
// any error type implementing Display is acceptable.
impl FromStr for RcloneActions {
    type Err = StringError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "check" | "Check" => Ok(RcloneActions::CHECK),
            "copy" | "Copy" => Ok(RcloneActions::COPY),
            "sync" | "Sync" => Ok(RcloneActions::SYNC),
            _ => Err(String::from("Please provide check or copy or sync.")),
        }
    }
}
// to convert enum to string; enables to_string()
impl Display for RcloneActions {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match &self {
            RcloneActions::CHECK => write!(fmt, "check"),
            RcloneActions::COPY => write!(fmt, "copy"),
            RcloneActions::SYNC => write!(fmt, "sync"),
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
