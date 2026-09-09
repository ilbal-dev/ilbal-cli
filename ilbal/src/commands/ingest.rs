use crate::registries::registry;
use crate::typedefs::BINARIES;
use crate::typedefs::SelectTheme;
use clap::ArgMatches;
use dialoguer::Select;
use owo_colors::OwoColorize;
use std::fs;
use std::io::Write;
use std::process::Command;
use crate::typedefs::Config;

pub fn run(cmd: &mut clap::Command, matches: &ArgMatches) -> anyhow::Result<()> {
    let cwd = std::env::current_dir()
        .expect("Could not determine current directory")
        .to_string_lossy()
        .to_string();
    let ilbal_dir = std::path::Path::new(&cwd).join("ilbal");
    if !ilbal_dir.is_dir()
        || !ilbal_dir.join("compose.yml").is_file()
        || !ilbal_dir.join("config.toml").is_file()
        || !std::path::Path::new(&cwd).join("data").is_dir() {
            anyhow::bail!("Not an ilbal project directory. Expected ilbal/compose.yml, ilbal/config.toml, and data/ under{}", cwd);
    };
    let volume = format!("{cwd}/data:/data");

    let config_toml = fs::read_to_string("ilbal/config.toml")?;
    let config: Config = toml::from_str(&config_toml)?;

    let args: Vec<&str> = matches
        .get_many::<String>("args")
        .unwrap_or_default()
        .map(|s| s.as_str())
        .collect();

    match args.split_first() {
        Some((binary, tool_args)) => {
            let save_file: Option<String> = matches.get_one::<String>("save").cloned();

            match save_file {
                Some(path) => {
                    let output = Command::new("docker")
                        .arg("run")
                        .arg("--rm")
                        .arg("--network")
                        .arg(&config.project.name)
                        .arg("-v")
                        .arg(&volume)
                        .arg("-w")
                        .arg("/data")
                        .arg(registry::images("ingest"))
                        .arg(binary)
                        .args(tool_args)
                        .output()?;
                    std::fs::write(&path, &output.stdout)?;
                    std::io::stderr().write_all(&output.stderr)?;
                }
                None => {
                    Command::new("docker")
                        .arg("run")
                        .arg("--rm")
                        .arg("--network")
                        .arg(&config.project.name)
                        .arg("-v")
                        .arg(&volume)
                        .arg("-w")
                        .arg("/data")
                        .arg(registry::images("ingest"))
                        .arg(binary)
                        .args(tool_args)
                        .status()?;
                }
            }
        }
        None => {
            cmd.print_help()?;
            println!();
            println!("{}", "Available binaries:".yellow());
            Select::with_theme(&SelectTheme)
                .with_prompt("Press Enter to close")
                .items(BINARIES)
                .max_length(10)
                .interact()?;
        }
    }
    Ok(())
}
