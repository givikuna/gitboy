use clap::Parser;

#[derive(Parser)]
#[command(name = "gitboy")]
#[command(about = "Clone repositories defined in a config file")]
#[command(version = "0.0.1")]

pub struct Args {
    #[arg(short, long, default_value = "~/.config/gitboy/config.toml")]
    pub config: String,
}

pub fn parse_args() -> Args {
    Args::parse()
}
