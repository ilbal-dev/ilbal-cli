use crate::typedefs::structs::Config;
use std::fs;

pub fn run() -> anyhow::Result<()> {
    let config_toml = fs::read_to_string("ilbal/config.toml")?;
    let config: Config = toml::from_str(&config_toml)?;

    println!("{}", toml::to_string_pretty(&config)?);

    Ok(())
}
