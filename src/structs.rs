
/// The result inside the response from the distant endpoint
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ResponseResult {
    /// The list of schedules that we will use to display our results
    pub schedules: Vec<Schedule>,
}

/// The overall response, contains our response result and other fields
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Response {
    /// The result of our request
    pub result: ResponseResult,
}

/// The fetched schedules
///
/// A schedule is a combination of a message and a direction, that
/// both indicates to the user when the next transports will be
#[derive(serde::Serialize, serde::Deserialize, Display, Debug)]
#[display(fmt = "{} direction {}", message, destination)]
pub struct Schedule {
    /// A message contains either a time (ie 11mn) or a temporal indication (ie coming soon)
    message: String,
    /// The destination 
    destination: String,
}

