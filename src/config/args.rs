use clap::Parser;

#[allow(clippy::struct_excessive_bools, reason = "These are flags for config")]
#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args {
    /// Outputs the JSON schema file.
    #[arg(short, long)]
    pub schema: bool,

    /// Outputs a template config file.
    #[arg(short, long)]
    pub template: bool,

    /// Outputs the docker compose file to run this.
    #[arg(short = 'd', long)]
    pub compose: bool,

    /// The path to the config file.
    #[arg(short, long, default_value_t = String::from("config.yaml"))]
    pub config: String,

    /// Enables the controller.
    #[arg(short, long)]
    pub run_controller: bool,
}

pub fn parse_args() -> Args {
    Args::parse()
}
