use std::fmt;

#[derive(Debug)]
pub enum CliError {
    RequestError(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match &*self {
                CliError::RequestError(reason) => reason,
            }
        )
    }
}

impl std::error::Error for CliError {}

impl From<reqwest::Error> for CliError {
    fn from(err: reqwest::Error) -> Self {
        Self::RequestError(err.to_string())
    }
}
