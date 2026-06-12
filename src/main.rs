mod cli;
mod config;
mod git;

use anyhow::Result;
use log::info;

fn main() -> Result<()> {
    env_logger::init();

    let args = cli::parse_args();
    let cfg = config::load_config(&args.config)?;

    info!("Loaded configuration for {} folders.", cfg.folders.len());

    for (path, folder_config) in cfg.folders {
        for repo_url in folder_config.repos {
            if let Err(e) = git::clone_repo(&repo_url, &path) {
                eprintln!("Failed to clone {}: {}", repo_url, e);
            }
        }
    }

    Ok(())
}
