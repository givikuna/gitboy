use anyhow::{Context, Result};
use git2::Repository;
use log::info;
use std::path::{Path, PathBuf};

pub fn extract_repo_name(url: &str) -> Result<String> {
    let path: &Path = Path::new(url);

    Ok(path
        .file_stem()
        .or_else(|| path.file_name())
        .context("invalid url: cannot extract repo")?
        .to_string_lossy()
        .to_string())
}

pub fn clone_repo(repo_url: &str, target_dir: &str) -> Result<()> {
    let repo_name: String = extract_repo_name(repo_url)?;

    let clone_path: PathBuf =
        Path::new(&shellexpand::tilde(target_dir).to_string()).join(&repo_name);

    if clone_path.exists() {
        return Ok(());
    }

    if let Some(parent) = clone_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("failed to create directory {:?}", parent))?;
        }
    }

    info!("cloning {} into {:?}", repo_url, clone_path);
    Repository::clone(repo_url, &clone_path)
        .with_context(|| format!("failed to clone from {}", repo_url))?;

    info!(" cloned {}", repo_name);
    Ok(())
}
