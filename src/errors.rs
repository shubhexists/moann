use std::{error::Error, fmt::Display, io};

#[derive(Debug)]
pub enum PulseErrors {
    CouldNotCreatePulseDirectory { err: io::Error },
    UnableToPlayFile { err: rodio::PlayError },
    UnableToDownloadFile { err: Box<dyn Error> },
    UnwantedSelectionIndex { index: usize },
    UnzipError {err: io::Error},
    RemoveFileError {err: io::Error},
    JSONReadError { err: io::Error},
    JSONParseError { err: serde_json::Error}
}

impl Display for PulseErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PulseErrors::CouldNotCreatePulseDirectory { err } => {
                writeln!(f, "Error creating pulse directory: {}", err)
            }
            PulseErrors::UnableToPlayFile { err } => {
                writeln!(f, "Unable to play file: {}", err)
            }
            PulseErrors::UnableToDownloadFile { err } => {
                writeln!(f, "Error: Downloading Sound File: {}", err)
            }
            PulseErrors::UnwantedSelectionIndex { index } => {
                writeln!(f, "Error: Unwanted Selection: {}", index)
            }
            PulseErrors::UnzipError { err } => {
                writeln!(f, "Error: Couldn't extract zip: {}", err)
            }
            PulseErrors::RemoveFileError { err } => {
                writeln!(f, "Error: Couldn't remove zip file: {}", err)
            }
            PulseErrors::JSONReadError { err } => {
                writeln!(f, "Error: Couldn't read json file: {}", err)
            }
            PulseErrors::JSONParseError { err } => {
                writeln!(f, "Error: Couldn't parse json file: {}", err)
            }
        }
    }
}

impl Error for PulseErrors {}
