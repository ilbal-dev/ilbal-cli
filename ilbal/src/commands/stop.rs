use crate::typedefs::structs::{Config, RuntimeType};
use std::fs;

pub fn run() -> anyhow::Result<()> {
    let config_toml = fs::read_to_string("ilbal/config.toml")?;
    let config: Config = toml::from_str(&config_toml)?;

    match config.runtime.runtime {
        RuntimeType::Docker => {
            let status = std::process::Command::new("docker")
                .args(["compose", "-f", "ilbal/compose.yml", "stop"])
                .status()?;
            if !status.success() {
                anyhow::bail!("Failed to stop Docker containers");
            }
        }
        RuntimeType::Podman => {
            let status = std::process::Command::new("podman")
                .args(["stop", "--all"])
                .status()?;
            if !status.success() {
                anyhow::bail!("Failed to stop Podman containers");
            }
        }
    }

    Ok(())
}
