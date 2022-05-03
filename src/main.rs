/// Application errors
///
/// Used to ease the handling of the different errors that can be thrown at runtime
mod error;

/// Structures used by the CLI
///
/// Lists all the structures used by the application
mod structs;

/// Application arguments
///
/// Gather all the arguments that can be received as input of the application
mod args;


/// Application tools
///
/// Contains all the functions and tools that can be used by the CLI at the runtime
mod utils;


use structs::{Schedule, Response};
use args::CliArgs;
use utils::slugify;
use clap::Parser;

#[macro_use]
extern crate log;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate lazy_static;

/// Our API endpoint
const API_URI: &str = "https://api-ratp.pierre-grimaud.fr/v4/schedules";

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
    let args: CliArgs = CliArgs::parse();
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
