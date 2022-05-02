pub(crate) mod error;

use clap::Parser;
use error::CliError;
use regex::Regex;

#[macro_use]
extern crate log;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate lazy_static;

const API_URI: &str = "https://api-ratp.pierre-grimaud.fr/v4/schedules";

lazy_static! {
    static ref REPLACEABLE : Regex = Regex::new(r"\s+|\-+").unwrap();
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ResponseResult {
    schedules: Vec<Schedule>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Response {
    result: ResponseResult,
}

#[derive(serde::Serialize, serde::Deserialize, Display, Debug)]
#[display(fmt = "{} direction {}", message, destination)]
struct Schedule {
    message: String,
    destination: String,
}

#[derive(Parser, Debug)]
struct Args {
    #[clap(arg_enum)]
    transport_type: TransportType,
    code: String,
    station: String,
    #[clap(arg_enum)]
    way: WayType,
}

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

#[derive(clap::ArgEnum, Debug, Clone, Display)]
#[display(fmt = "{}")]
enum WayType {
    #[display(fmt = "A")]
    A,
    #[display(fmt = "R")]
    R,
    #[display(fmt = "A+R")]
    AR,
}

fn slugify(raw_str : &str) -> String {
    return REPLACEABLE.replace_all(&raw_str.to_lowercase(), "+").to_string();
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

async fn run_main() -> Result<(), CliError> {
    debug!("Process started");
    let args: Args = Args::parse();
    debug!("Args : {:?}", args);
    let station_slug : String = slugify(&args.station);
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
