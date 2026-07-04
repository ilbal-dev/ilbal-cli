// src/commands/init.rs
use anyhow::Result;
use dialoguer::{Confirm, Input, Select};
use owo_colors::OwoColorize;
use walkdir::WalkDir;

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
        .default(String::from("quack"))
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.contains([' ']) {
                Err("Password cannot contain spaces")
            } else {
                Ok(())
            }
        })
        .interact_text()?;

    // let pgadmin_email: String = Input::new()
    //     .with_prompt("PgAdmin4 email")
    //     .default(String::from("duck@duck.com"))
    //     .interact_text()?;
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
            ("${IMAGE_POSTGRES}",    registry::images("postgresql")),
            ("${IMAGE_PGADMIN}",     registry::images("pgadmin4")),
            ("${IMAGE_PGDOG}",       registry::images("pgdog")),
            ("${IMAGE_SEQUIN}",      registry::images("sequin")),
            ("${IMAGE_REDIS}",       registry::images("redis")),
            ("${PROJECT_NAME}",      &project),
            ("${POSTGRES_USER}",     &pg_username),
            ("${POSTGRES_PASSWORD}", &pg_password),
            ("${POSTGRES_DB}",       &pg_db),
            ("${POSTGRES_PORT}",     &postgres_port.to_string()),
            ("${PGADMIN_EMAIL}",     &pgadmin_email.to_string()),
            ("${PGADMIN_PORT}",      &pgadmin_port.to_string()),
            ("${PGDOG_PORT}",        &pgdog_port.to_string()),
            ("${SEQUIN_PORT}",       &sequin_port.to_string()),
            ("${REDIS_PORT}",        &redis_port.to_string()),
        ];

        let template_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("template");

        for entry in WalkDir::new(&template_dir) {
            let entry = entry?;
            let relative = entry.path().strip_prefix(&template_dir)?;
            let dest = config_dir.join(relative);

            if entry.file_type().is_dir() {
                std::fs::create_dir_all(&dest)?;
            } else {
                let mut content = std::fs::read_to_string(entry.path())?;
                for (key, val) in &subs {
                    content = content.replace(key, val);
                }
                std::fs::write(&dest, &content)?;
            }
        }


        // let compose = std::fs::read_to_string(template_dir.join("compose.yml"))?
        //     .replace("${IMAGE_POSTGRES}", registry::images("postgresql"))
        //     .replace("${IMAGE_PGADMIN}", registry::images("pgadmin4"))
        //     .replace("${IMAGE_PGDOG}", registry::images("pgdog"))
        //     .replace("${IMAGE_SEQUIN}", registry::images("sequin"))
        //     .replace("${PROJECT_NAME}", &project)
        //     .replace("${POSTGRES_USER}", &pg_username)
        //     .replace("${POSTGRES_PASSWORD}", &pg_password)
        //     .replace("${POSTGRES_DB}", &pg_db)
        //     .replace("${POSTGRES_PORT}", &postgres_port.to_string())
        //     .replace("${PGADMIN_EMAIL}", &pgadmin_email.to_string())
        //     .replace("${PGADMIN_PORT}", &pgadmin_port.to_string())
        //     .replace("${PGDOG_PORT}", &pgdog_port.to_string())
        //     .replace("${SEQUIN_PORT}", &sequin_port.to_string())
        //     .replace("${REDIS_PORT}", &redis_port.to_string());
        // std::fs::write(config_dir.join("compose.yml"), &compose)?;

        // // TODO: pomdan.yml needs substitution!!
        // let podman = std::fs::read_to_string(template_dir.join("podman.yml"))?;
        // std::fs::write(config_dir.join("podman.yml"), &podman)?;

        // // --- postgresql.conf
        // let pgconfig = std::fs::read_to_string(template_dir.join("postgresql.conf"))?;
        // std::fs::write(pgconfig_dir.join("postgresql.conf"), &pgconfig)?;

        // // --- Initializing sql script
        // let init = std::fs::read_to_string(template_dir.join("init.sql"))?
        //     .replace("${POSTGRES_DB}", &pg_db);
        // std::fs::write(config_dir.join("init.sql"), &init)?;

        // let pgpass = std::fs::read_to_string(template_dir.join("pgpass"))?
        //     .replace("${PROJECT_NAME}", &project)
        //     .replace("${POSTGRES_USER}", &pg_username)
        //     .replace("${POSTGRES_PASSWORD}", &pg_password)
        //     .replace("${POSTGRES_DB}", &pg_db)
        //     .replace("${POSTGRES_PORT}", &postgres_port.to_string());
        // std::fs::write(config_dir.join("pgpass"), &pgpass)?;

        // let servers = std::fs::read_to_string(template_dir.join("servers.json"))?
        //     .replace("${PROJECT_NAME}", &project)
        //     .replace("${POSTGRES_USER}", &pg_username)
        //     .replace("${POSTGRES_PASSWORD}", &pg_password)
        //     .replace("${POSTGRES_DB}", &pg_db)
        //     .replace("${POSTGRES_PORT}", &postgres_port.to_string())
        //     .replace("${PGADMIN_EMAIL}", &pgadmin_email.to_string())
        //     .replace("${PGADMIN_PORT}", &pgadmin_port.to_string());
        // std::fs::write(config_dir.join("servers.json"), &servers)?;

        // let sequin = std::fs::read_to_string(template_dir.join("sequin.yml"))?
        //     .replace("${PROJECT_NAME}", &project)
        //     .replace("${POSTGRES_USER}", &pg_username)
        //     .replace("${POSTGRES_PASSWORD}", &pg_password)
        //     .replace("${POSTGRES_DB}", &pg_db)
        //     .replace("${POSTGRES_PORT}", &postgres_port.to_string())
        //     .replace("${PGADMIN_EMAIL}", &pgadmin_email.to_string())
        //     .replace("${PGADMIN_PORT}", &pgadmin_port.to_string())
        //     .replace("${PGDOG_PORT}", &pgdog_port.to_string())
        //     .replace("${SEQUIN_PORT}", &sequin_port.to_string());
        // std::fs::write(config_dir.join("sequin.yml"), &sequin)?;

        // --- Pull docker images
        match config.runtime.runtime {
            RuntimeType::Docker => {
                // The image below will be codeberg/ilbal/ilbal-postgresql:latest
                // let postgres_status = std::process::Command::new("docker")
                //     .args(["pull", registry::images("postgresql")])
                //     .status()?;
                // if !postgres_status.success() {
                //     anyhow::bail!("Failed to pull PostgreSQL image");
                // }

                // The image below will be codeberg/ilbal/ilbal-ingest:latest
                // let ingest_status = std::process::Command::new("docker")
                //     .args(["pull", registry::images("ingest")])
                //     .status()?;
                // if !ingest_status.success() {
                //     anyhow::bail!("Failed to pull ilbal-ingest image");
                // }

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

        println!("{}", format!("Initialized at {project}").green());
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
