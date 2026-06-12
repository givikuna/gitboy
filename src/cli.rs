use clap::Parser;

#[derive(Parser)]
#[command(name = "gitboy")]
#[command(about = "clone repositories defined in a config file")]
#[command(version = "0.0.1")]
pub struct Args {
    #[arg(short, long)]
    config: Option<String>,
}

impl Args {
    pub fn config_path(&self) -> String {
        self.config.clone().unwrap_or_else(|| {
            std::env::var("XDG_CONFIG_HOME")
                .map(|path| format!("{}/gitboy/config.toml", path))
                .unwrap_or_else(|_| "~/.config/gitboy/config.toml".to_string())
        })
    }
}

pub fn parse_args() -> Args {
    Args::parse()
}
