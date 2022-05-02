/// Common errors that can be thrown on runtime
///
/// It mostly handles external libraries errors
#[derive(Debug, Display)]
pub enum CliError {
    /// An error sent on request to an external library
    #[display(fmt = "{}", _0)]
    RequestError(String),
}

impl std::error::Error for CliError {}

impl From<reqwest::Error> for CliError {
    fn from(err: reqwest::Error) -> Self {
        Self::RequestError(err.to_string())
    }
}
