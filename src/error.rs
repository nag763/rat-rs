use std::process::{ExitCode, Termination};

/// Common errors that can be thrown on runtime
///
/// It mostly handles external libraries errors
#[derive(Debug, Display, Clone)]
pub enum CliError {
    /// An error sent on request to an external library
    #[display(fmt = "{}", _0)]
    RequestError(String),
}

impl CliError {
    pub fn error_code(&self) -> u8 {
        match self {
            CliError::RequestError(_) => 10
        }
    }
}

impl std::error::Error for CliError {}

impl Termination for CliError {
    fn report(self) -> ExitCode {
        let err_code : u8 = self.error_code();
        ExitCode::from(err_code)
    }
}

impl From<reqwest::Error> for CliError {
    fn from(err: reqwest::Error) -> Self {
        Self::RequestError(err.to_string())
    }
}

