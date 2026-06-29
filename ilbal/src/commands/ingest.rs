use clap::ArgMatches;
use owo_colors::OwoColorize;
use std::io::Write;
use std::process::Command;

const DOCKER_IMAGE: &str = "ilbal-postgresql-loaders-0.1.0:latest";

pub fn run(matches: &ArgMatches) -> anyhow::Result<()> {
    let cwd = std::env::current_dir()
        .expect("Could not determine current directory")
        .to_string_lossy()
        .to_string();
    let volume = format!("{cwd}:/data");

    let args: Vec<&str> = matches
        .get_many::<String>("args")
        .unwrap_or_default()
        .map(|s| s.as_str())
        .collect();

    match args.split_first() {
        Some((binary, tool_args)) => {
            Command::new("docker")
                .arg("run")
                .arg("--rm")
                .arg("-v")
                .arg(&volume)
                .arg("-w")
                .arg("/data")
                .arg(DOCKER_IMAGE)
                .arg(binary)
                .args(tool_args)
                .status()?;
        }
        None => {
            let output = Command::new("docker")
                .arg("run")
                .arg("--rm")
                .arg(DOCKER_IMAGE)
                .arg("ls")
                .arg("-1")
                .arg("/bin")
                .output()?;
            println!("{}", "Available binaries:".yellow());
            std::io::stdout().write_all(&output.stdout)?;
            eprintln!("Usage: ilbal ingest <binary> [args...]");
        }
    }
    Ok(())
}
