use std::{path::PathBuf, str::FromStr};

use clap::Parser;

mod epoch_manager;
mod message_samples;

/// Block Oracle automation scripts
#[derive(Parser)]
#[clap()]
enum Tasks {
    /// Compile and display encoded message samples
    EncodeMessageSamples,
    /// Queries the Epoch Manager for the current epoch
    CurrentEpoch {
        #[clap(long, short)]
        environment: Environment,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    match Tasks::parse() {
        Tasks::EncodeMessageSamples => message_samples::encode()?,
        Tasks::CurrentEpoch { environment } => epoch_manager::query(environment).await?,
    };
    Ok(())
}

pub enum Environment {
    Development,
    Staging,
    Production,
}

impl FromStr for Environment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            x if x.starts_with("dev") => Ok(Self::Development),
            x if x.starts_with("stag") => Ok(Self::Staging),
            "prod" | "production" => Ok(Self::Production),
            _ => anyhow::bail!("failed to parse configuration name"),
        }
    }
}

impl Environment {
    fn resolve_configuration_path(&self) -> anyhow::Result<PathBuf> {
        let mut base = PathBuf::from("crates//oracle/config/");
        let mid = match self {
            Environment::Development => ("dev"),
            Environment::Staging => ("staging"),
            Environment::Production => ("prod"),
        };
        base.push(mid);
        base.push("config.toml");
        let path = base.canonicalize()?;
        anyhow::ensure!(
            path.exists(),
            "Could not find configuration file at: {path:?}"
        );
        Ok(path)
    }
}
