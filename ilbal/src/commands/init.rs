// src/commands/init.rs
use anyhow::Result;
use clap::ArgMatches;
use dialoguer::{Confirm, Input, Select};
use owo_colors::OwoColorize;

pub fn run(matches: &ArgMatches) -> Result<()> {
    let dir = matches
        .get_one::<String>("directory")
        .cloned()
        .unwrap_or_else(|| {
            Input::new()
                .with_prompt("Project directory")
                .default(".".into())
                .interact_text()
                .unwrap()
        });

    let ptype = Select::new()
        .with_prompt("Project type")
        .items(&["Default", "Web API", "Data Pipeline"])
        .default(0)
        .interact()?;

    if Confirm::new().with_prompt("Proceed?").interact()? {
        println!("{}", format!("Initialized at {dir}").green());
    }
    Ok(())
}
