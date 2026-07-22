// src/commands/init.rs
use anyhow::Result;
use dialoguer::{Confirm, Input, Select};
use include_dir::{Dir, include_dir};
use owo_colors::OwoColorize;

use crate::registries::registry;
use crate::typedefs::{
    Config, PgadminConfig, PostgresqlConfig, ProjectConfig, RuntimeConfig, structs::RuntimeType,
};

pub fn run() -> Result<()> {
    // --- GATHER USER INPUT ---
    // --- Project information
    let project: String = Input::new()
        .with_prompt("Project name")
        .validate_with(|input: &String| -> Result<(), &str> {
            let config_dir = std::path::Path::new(input).join("ilbal");
            if config_dir.exists() {
                Err("Project directory already exists")
            } else if input.contains([' ']) {
                Err("Project name cannot contain spaces")
            } else if input.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                Err("Project name cannot start with a number")
            } else {
                Ok(())
            }
        })
        .interact_text()?;

    // --- Project description
    let desription: String = Input::new()
        .with_prompt("Project Description")
        .interact_text()?;

    // --- Authors
    let mut authors: Vec<String> = Vec::new();
    loop {
        let author = Input::<String>::new()
            .with_prompt("Author")
            .allow_empty(true)
            .interact_text()?;
        if author.is_empty() {
            break;
        }
        authors.push(author);
    }

    // --- Runtime, Docker or Podman
    let runtime_types = [RuntimeType::Podman, RuntimeType::Docker];

    let runtime_idx = Select::new()
        .with_prompt("Container runtime")
        .items(&runtime_types)
        .default(0)
        .interact()?;

    let runtime = runtime_types[runtime_idx];

    // Database name
    let pg_db: String = Input::new()
        .with_prompt("Database name")
        .default(String::from("app"))
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.contains([' ', '-']) {
                Err("Database name cannot contain spaces or hyphens")
            } else if input.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                Err("Database name cannot start with a number")
            } else {
                Ok(())
            }
        })
        .interact_text()?;

    // Local dev username
    let pg_username: String = Input::new()
        .with_prompt("Username")
        .default(String::from("duck"))
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.contains([' ']) {
                Err("Username cannot contain spaces")
            } else {
                Ok(())
            }
        })
        .interact_text()?;

    // Local dev password
    let pg_password: String = Input::new()
        .with_prompt("Password")
        .default(String::from("quack123"))
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.contains([' ']) {
                Err("Password cannot contain spaces")
            } else if input.len() < 8 {
                Err("Password must be at least 8 characters")
            } else {
                Ok(())
            }
        })
        .interact_text()?;

    let pgadmin_email: String = String::from("duck@duck.com");

    let confirmed = Confirm::new()
        .with_prompt("Proceed?")
        .default(true)
        .interact()?;

    // --- PROCESS INPUTS ---
    if confirmed {
        // Assign ports
        let ports = find_ports(5)?;
        let postgres_port = ports[0];
        let pgadmin_port = ports[1];
        let pgdog_port = ports[2];
        let sequin_port = ports[3];
        let redis_port = ports[4];

        // Create the following diretory structure
        let project_dir = std::path::Path::new(&project);
        let data_dir = project_dir.join("data");
        let config_dir = project_dir.join("ilbal");
        let backup_dir = config_dir.join("backups");
        // let pgconfig_dir = config_dir.join("pgconfig");
        std::fs::create_dir_all(&data_dir)?;
        std::fs::create_dir_all(&config_dir)?;
        std::fs::create_dir_all(&backup_dir)?;
        // std::fs::create_dir_all(&pgconfig_dir)?;

        // --- Create config...
        let config = Config {
            project: ProjectConfig {
                name: project.clone(),
                description: desription.clone(),
                authors: authors.clone(),
            },
            runtime: RuntimeConfig { runtime: runtime },
            postgresql: PostgresqlConfig {
                pg_username: pg_username.clone(),
                pg_password: pg_password.clone(),
                pg_uri: format!(
                    "postgresql://{pg_username}:{pg_password}@localhost:{postgres_port}/{pg_db}"
                ),
            },
            pgadmin: PgadminConfig {
                pgadmin_email: pgadmin_email.clone(),
                pgadmin_password: pg_password.clone(),
                pgadmin_url: format!("http://localhost:{pgadmin_port}/"),
            },
        };
        // ...and save it to config.toml
        let config_path = config_dir.join("config.toml");
        let toml_string = toml::to_string(&config)?;
        std::fs::write(&config_path, toml_string)?;

        // --- Copy additional configuration files with substitutions
        let subs = [
            ("${IMAGE_POSTGRES}", registry::images("postgresql")),
            ("${IMAGE_PGADMIN}", registry::images("pgadmin4")),
            ("${IMAGE_PGDOG}", registry::images("pgdog")),
            ("${IMAGE_SEQUIN}", registry::images("sequin")),
            ("${IMAGE_REDIS}", registry::images("redis")),
            ("${PROJECT_NAME}", &project),
            ("${POSTGRES_USER}", &pg_username),
            ("${POSTGRES_PASSWORD}", &pg_password),
            ("${POSTGRES_DB}", &pg_db),
            ("${POSTGRES_PORT}", &postgres_port.to_string()),
            ("${PGADMIN_EMAIL}", &pgadmin_email.to_string()),
            ("${PGADMIN_PORT}", &pgadmin_port.to_string()),
            ("${PGDOG_PORT}", &pgdog_port.to_string()),
            ("${SEQUIN_PORT}", &sequin_port.to_string()),
            ("${REDIS_PORT}", &redis_port.to_string()),
        ];

        static TEMPLATE_DIR: Dir = include_dir!("template");

        for entry in TEMPLATE_DIR.find("**/*").unwrap() {
            if let Some(file) = entry.as_file() {
                let dest = config_dir.join(file.path());
                if let Some(parent) = dest.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                let mut content = file.contents_utf8().unwrap_or_default().to_string();
                for (key, val) in &subs {
                    content = content.replace(key, val);
                }
                std::fs::write(&dest, &content)?;
            }
        }

        // --- Pull docker images
        match config.runtime.runtime {
            RuntimeType::Docker => {
                let postgres_status = std::process::Command::new("docker")
                    .args(["pull", registry::images("postgresql")])
                    .status()?;
                if !postgres_status.success() {
                    anyhow::bail!("Failed to pull PostgreSQL image");
                }

                let ingest_status = std::process::Command::new("docker")
                    .args(["pull", registry::images("ingest")])
                    .status()?;
                if !ingest_status.success() {
                    anyhow::bail!("Failed to pull ilbal-ingest image");
                }

                let pgadmin_status = std::process::Command::new("docker")
                    .args(["pull", registry::images("pgadmin4")])
                    .status()?;
                if !pgadmin_status.success() {
                    anyhow::bail!("Failed to pull pgAdmin4 image");
                }

                let pgdog_status = std::process::Command::new("docker")
                    .args(["pull", registry::images("pgdog")])
                    .status()?;
                if !pgdog_status.success() {
                    anyhow::bail!("Failed to pull pgDog image");
                }

                let sequin_status = std::process::Command::new("docker")
                    .args(["pull", registry::images("sequin")])
                    .status()?;
                if !sequin_status.success() {
                    anyhow::bail!("Failed to pull pgDog image");
                }

                let redis_status = std::process::Command::new("docker")
                    .args(["pull", registry::images("redis")])
                    .status()?;
                if !redis_status.success() {
                    anyhow::bail!("Failed to pull redis image");
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

        println!("{}", format!("Project Initialized!").green());
        println!(
            "{}",
            format!("Run `cd {project}`, then `ilbal start`").green()
        );
        Ok(())
    } else {
        println!("{}", "Cancelled.".yellow());
        Ok(())
    }
}

pub fn find_ports(num_in_sequence: u16) -> Result<Vec<u16>> {
    let mut ports = Vec::new();
    let mut base = 33330;
    while ports.len() < num_in_sequence as usize {
        ports.clear();
        for offset in 0..num_in_sequence {
            let port = base + offset;
            match std::net::TcpListener::bind(format!("127.0.0.1:{port}")) {
                Ok(listener) => {
                    drop(listener);
                    ports.push(port);
                }
                Err(_) => {
                    // Port in use, try next one
                    base = port + 1;
                    break;
                }
            }
        }
    }
    Ok(ports)
}
