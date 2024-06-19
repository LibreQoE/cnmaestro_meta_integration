//! Configuration

use anyhow::{Context, Result};

pub struct CnMaestroConfig {
    pub username: String,
    pub secret: String,
    pub url: String,
    pub shaped_devices_path: String,
}

impl CnMaestroConfig {
    pub fn from_env() -> Result<Self> {
        // It's not an error to not have a .env file specifying the environment
        let _ = dotenvy::dotenv();

        let username = std::env::var("CNMAESTRO_USERNAME")
            .context("You must provide a CNMAESTRO_USERNAME environment variable")?;
        let secret =
            std::env::var("CNMAESTRO_SECRET").context("You must provide a CNMAESTRO_SECRET")?;
        let url = std::env::var("CNMAESTRO_URL").context("You must provide a CNMAESTRO_URL")?;
        let shaped_devices_path =
            std::env::var("SHAPED_DEVICES_PATH").context("You must set SHAPED_DEVICES_PATH")?;

        Ok(Self {
            username,
            secret,
            url,
            shaped_devices_path,
        })
    }
}
