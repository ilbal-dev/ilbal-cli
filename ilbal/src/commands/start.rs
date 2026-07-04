use crate::typedefs::structs::{Config, RuntimeType};
use std::fs;

pub fn run() -> anyhow::Result<()> {
    let config_toml = fs::read_to_string("ilbal/config.toml")?;
    let config: Config = toml::from_str(&config_toml)?;

    match config.runtime.runtime {
        RuntimeType::Docker => {
            let status = std::process::Command::new("docker")
                .args(["compose", "-f", "ilbal/compose.yml", "up", "-d"])
                .status()?;
            if !status.success() {
                anyhow::bail!("Failed to start Docker containers");
            }
        }
        RuntimeType::Podman => {
            let status = std::process::Command::new("podman")
                .args(["play", "kube", "ilbal/podman.yml"])
                .status()?;
            if !status.success() {
                anyhow::bail!("Failed to start Podman containers");
            }
        }
    }

    Ok(())
}
