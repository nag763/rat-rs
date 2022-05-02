/// Application errors
///
/// Used to ease the handling of the different errors that can be thrown at runtime
mod error;

use clap::Parser;
use regex::Regex;

#[macro_use]
extern crate log;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate lazy_static;

/// Our API endpoint
const API_URI: &str = "https://api-ratp.pierre-grimaud.fr/v4/schedules";

lazy_static! {
    /// The regex used to slugify our inputs
    static ref REPLACEABLE: Regex = Regex::new(r"\s+|\-+").unwrap();
}

/// The result inside the response from the distant endpoint
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ResponseResult {
    /// The list of schedules that we will use to display our results
    schedules: Vec<Schedule>,
}

/// The overall response, contains our response result and other fields
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Response {
    /// The result of our request
    result: ResponseResult,
}

/// The fetched schedules
///
/// A schedule is a combination of a message and a direction, that
/// both indicates to the user when the next transports will be
#[derive(serde::Serialize, serde::Deserialize, Display, Debug)]
#[display(fmt = "{} direction {}", message, destination)]
struct Schedule {
    /// A message contains either a time (ie 11mn) or a temporal indication (ie coming soon)
    message: String,
    /// The destination 
    destination: String,
}

#[derive(Parser, Debug)]
#[clap(
    name = "rat-rs",
    author = "LABEYE Loïc <loic.labeye@pm.me>",
    version = "0.1",
    about = "This tool has for purpose to show the schedules of the parisians transports for the given arguments.",
    after_help = "All of the data reported by this tool belongs to the RATP."
)]
struct Args {
    /// Desired transport type
    #[clap(arg_enum)]
    transport_type: TransportType,
    /// Code of the transport
    code: String,
    /// Station where you would like to have the next schedules
    station: String,
    /// What direction you want to go
    #[clap(arg_enum)]
    way: WayType,
}

/// A transport type is a way to go from a point A to a point B
#[derive(clap::ArgEnum, Debug, Clone, Display)]
#[display(fmt = "{}")]
enum TransportType {
    #[display(fmt = "metros")]
    Metro,
    #[display(fmt = "rers")]
    Rer,
    #[display(fmt = "tramways")]
    Tramway,
    #[display(fmt = "buses")]
    Bus,
    #[display(fmt = "noctiliens")]
    Noctilien,
}

/// A way type is similar to a direction
#[derive(clap::ArgEnum, Debug, Clone, Display)]
#[display(fmt = "{}")]
enum WayType {
    #[display(fmt = "A")]
    A,
    #[display(fmt = "R")]
    R,
    /// A+R means any direction
    #[display(fmt = "A+R")]
    AR,
}

/// This method transforms a string into a slug to request the api
///
/// # Args
///
/// * raw_str : The string to transform as slug
///
/// # Examples
///
/// ```
/// assert_eq!(slugify(" La Défense ", "la-défense"));
/// ```
fn slugify(raw_str: &str) -> String {
    return REPLACEABLE
        .replace_all(raw_str.to_lowercase().trim(), "+")
        .to_string();
}

#[tokio::main]
async fn main() {
    env_logger::init();
    std::process::exit(match run_main().await {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("An error happened : {}", err);
            eprintln!("The application finished with return code 1");
            1
        }
    });
}

async fn run_main() -> Result<(), error::CliError> {
    debug!("Process started");
    let args: Args = Args::parse();
    debug!("Args : {:?}", args);
    let station_slug: String = slugify(&args.station);
    debug!("Station slug : {:?}", station_slug);
    let endpoint = format!(
        "{}/{}/{}/{}/{}",
        API_URI, &args.transport_type, &args.code, &station_slug, &args.way
    );
    debug!("Endpoint : {}", endpoint);
    let response = reqwest::get(endpoint).await?.json::<Response>().await?;
    debug!("Response : {:?}", response);
    let schedules: Vec<Schedule> = response.result.schedules;
    for schedule in schedules {
        println!("{}", schedule);
    }
    debug!("Process completed with success");
    Ok(())
}
