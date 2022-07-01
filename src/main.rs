//! # rat-rs
//!
//! <img src="https://raw.githubusercontent.com/nag763/rat-rs/main/.github/logo.png" alt="drawing" width="300" style="margin-left:auto;margin-right:auto;display:block;"/>
//!
//! A simple cli tool to fetch transports schedules on the Ile-de-France région
//!
//! ## Main assets
//!
//! * Allows to fetch the result of the following transports : Buses, Metros, Tramways and RERs
//! * Up 24h/24h , 7day/7day
//!* Fast response time
//!* Real time results
//!* Lightweight
//!* Easy to use
//!* Open-source
//! * Crossplatform : you only need to have cargo installed !
//!
//! ## How to download
//!
//! ### From crates.io
//!
//! ```
//! cargo install rat-rs
//! ```
//!
//! ### From github
//!
//! ```
//! cargo install --git https://github.com/nag763/rat-rs
//! ```
//!
//! ### From release page
//!
//! Might come soon
//!
//! ## Usage
//!
//! ### Print help
//!
//! ```
//! usr@penguin:$ rat-rs --help
//! rat-rs 0.1.2
//! LABEYE Loïc <loic.labeye@pm.me>
//! This tool has for purpose to show the schedules of the parisians transports for the given arguments.
//!
//! USAGE:
//!     rat-rs <TRANSPORT_TYPE> <CODE> <STATION> <WAY>
//!
//! ARGS:
//!     <TRANSPORT_TYPE>    Desired transport type [possible values: metro, rer, tramway, bus,
//!                         noctilien]
//!     <CODE>              Code of the transport
//!     <STATION>           Station where you would like to have the next schedules
//!     <WAY>               What direction you want to go [possible values: a, r, ar]
//!
//! OPTIONS:
//!     -h, --help       Print help information
//!     -V, --version    Print version information
//!
//! All of the data reported by this tool belongs to the RATP.
//! ```
//!
//! ### Get the next metro (line 9) from Pont de Sèvres
//!
//! ```
//! usr@penguin:$ rat-rs metro 9 "Pont de Sèvres" a
//! 1 mn direction Mairie de Montreuil
//! 6 mn direction Mairie de Montreuil
//! 11 mn direction Mairie de Montreuil
//! 15 mn direction Mairie de Montreuil
//! ```
//!
//! ### Get the next RER A from La Défense, any direction
//!
//! ```
//! usr@penguin:$ rat-rs rer a "Auber" ar
//! 21:07 direction Poissy
//! 21:11 direction Saint-Germain-en-Laye
//! 21:13 direction Cergy-Le-Haut
//! 21:18 direction Saint-Germain-en-Laye
//! 21:23 direction Poissy
//! 21:30 direction Saint-Germain-en-Laye
//! 21:06 direction Boissy-Saint-Leger
//! 21:09 direction Marne-la-Vallee Chessy
//! 21:16 direction Boissy-Saint-Leger
//! 21:23 direction Marne-la-Vallee Chessy
//! 21:31 direction Boissy-Saint-Leger
//! 21:38 direction Marne-la-Vallee Chessy
//! ```
//!
//! ## Special thanks
//!
//! * This tool uses [Pierre Grimaud's RATP API](https://github.com/pgrimaud/horaires-ratp-api), support him if you like this project.


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

use args::CliArgs;
use clap::Parser;
use structs::{Response, Schedule};
use utils::slugify;
use std::process::{ExitCode, Termination};

#[macro_use]
extern crate log;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate lazy_static;

/// Our API endpoint
const API_URI: &str = "https://api-ratp.pierre-grimaud.fr/v4/schedules";

#[tokio::main]
async fn main() -> ExitCode {
    env_logger::init();
    match run_main().await {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("An error happened : {}", &err);
            eprintln!("The application finished with return code {}", &err.error_code());
            err.report()
        }
    }
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
