use crate::typedefs::structs::{Config, RuntimeType};
use std::fs;

pub fn run() -> anyhow::Result<()> {
    let config_toml = fs::read_to_string("ilbal/config.toml")?;
    let config: Config = toml::from_str(&config_toml)?;
    let project = &config.project.name;

    match config.runtime.runtime {
        RuntimeType::Docker => {
            // Stop and remove containers + default network (Compose manages both)
            let down = std::process::Command::new("docker")
                .args(["compose", "-f", "ilbal/compose.yml", "down"])
                .status()?;
            if !down.success() {
                anyhow::bail!("Failed to bring down Docker Compose services");
            }

            // Remove volumes (Compose doesn't do this by default)
            let volume_names = [
                format!("{project}_pgdata"),
                format!("{project}_pgadmin_data"),
                format!("{project}_sequin_redis_data"),
            ];
            let vol_rm = std::process::Command::new("docker")
                .args(["volume", "rm", "-f"])
                .args(&volume_names)
                .status()?;
            if !vol_rm.success() {
                // volumes may not exist — that's fine, just warn
                eprintln!("Warning: some volumes could not be removed (may not exist)");
            }
        }
        RuntimeType::Podman => {
            // Podman play kube down removes containers and pods
            let down = std::process::Command::new("podman")
                .args(["play", "kube", "--down", "ilbal/podman.yml"])
                .status()?;
            if !down.success() {
                anyhow::bail!("Failed to bring down Podman pods");
            }

            // Podman volumes still need manual removal
            let vol_rm = std::process::Command::new("podman")
                .args(["volume", "rm", "-f",
                    &format!("{project}_pgdata"),
                    &format!("{project}_pgadmin_data"),
                ])
                .status()?;
            if !vol_rm.success() {
                eprintln!("Warning: some volumes could not be removed");
            }
        }
    }

    println!("Reset complete for {project}");
    Ok(())
}
