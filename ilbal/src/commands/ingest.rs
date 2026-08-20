use crate::typedefs::binaries::BINARIES;
use clap::ArgMatches;
use console::style;
use dialoguer::Select;
use dialoguer::theme::Theme;
use owo_colors::OwoColorize;
use std::fmt;
use std::io::Write;
use std::process::Command;

const DOCKER_IMAGE: &str = "ilbal-ingest:latest";

struct SelectTheme;

impl Theme for SelectTheme {
    fn format_select_prompt_item(
        &self,
        f: &mut dyn fmt::Write,
        text: &str,
        active: bool,
    ) -> fmt::Result {
        if active {
            write!(
                f,
                "{} {}",
                style(">").for_stderr().cyan().bold(),
                style(text).for_stderr().cyan().bold()
            )
        } else {
            write!(f, "  {text}")
        }
    }
}

pub fn run(cmd: &mut clap::Command, matches: &ArgMatches) -> anyhow::Result<()> {
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
            let save_file: Option<String> = matches.get_one::<String>("save").cloned();

            match save_file {
                Some(path) => {
                    let output = Command::new("docker")
                        .arg("run")
                        .arg("--rm")
                        .arg("-v")
                        .arg(&volume)
                        .arg("-w")
                        .arg("/data")
                        .arg(DOCKER_IMAGE)
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
                        .arg("-v")
                        .arg(&volume)
                        .arg("-w")
                        .arg("/data")
                        .arg(DOCKER_IMAGE)
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
