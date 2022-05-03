#[derive(clap::Parser, Debug)]
#[clap(
    name = "rat-rs",
    author = "LABEYE Loïc <loic.labeye@pm.me>",
    version = "0.1",
    about = "This tool has for purpose to show the schedules of the parisians transports for the given arguments.",
    after_help = "All of the data reported by this tool belongs to the RATP."
)]
pub struct CliArgs {
    /// Desired transport type
    #[clap(arg_enum)]
    pub(crate) transport_type: TransportType,
    /// Code of the transport
    pub(crate) code: String,
    /// Station where you would like to have the next schedules
    pub(crate) station: String,
    /// What direction you want to go
    #[clap(arg_enum)]
    pub(crate) way: WayType,
}

/// A transport type is a way to go from a point A to a point B
#[derive(clap::ArgEnum, Debug, Clone, Display)]
#[display(fmt = "{}")]
pub(crate) enum TransportType {
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
pub(crate) enum WayType {
    #[display(fmt = "A")]
    A,
    #[display(fmt = "R")]
    R,
    /// A+R means any direction
    #[display(fmt = "A+R")]
    AR,
}
