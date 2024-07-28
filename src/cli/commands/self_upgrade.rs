use crate::cli::{config::read_config, version::get_or_download_version};
use clap::Args;

#[derive(Debug, Args)]
pub struct SelfUpgradeCommand {
    #[cfg(windows)]
    #[arg(short, long)]
    skip_add_to_path: bool,
}

impl SelfUpgradeCommand {
    pub fn run(self, reqwest: reqwest::blocking::Client) -> anyhow::Result<()> {
        let config = read_config()?;

        get_or_download_version(&reqwest, &config.last_checked_updates.unwrap().1)?;

        Ok(())
    }
}
