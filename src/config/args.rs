use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args {
    /// Outputs the json schema file.
    #[arg(short, long)]
    pub schema: bool,

    /// Outputs a template config file, this implies -s as well.
    #[arg(short, long)]
    pub template: bool,

    /// The path to the config file.
    #[arg(short, long, default_value_t = String::from("config.yaml"))]
    pub config: String,
}

pub fn parse_args() -> Args {
    Args::parse()
}
