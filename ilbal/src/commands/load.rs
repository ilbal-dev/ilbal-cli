use clap::ArgMatches;
use std::process::Command;

pub fn run(matches: &ArgMatches) -> anyhow::Result<()> {
    match matches.subcommand_name() {
        Some("dbcrossbar") => run_tool("dbcrossbar"),
        Some("gdal") => {
            let g = matches.subcommand_matches("gdal").unwrap();
            match g.subcommand_name() {
                Some("ogr2ogr") => run_tool("ogr2ogr"),
                Some("gdal_translate") => run_tool("gdal_translate"),
                _ => Ok(()),
            }
        }
        Some("pgferry") => run_tool("pgferry"),
        Some("csv") => {
            let c = matches.subcommand_matches("csv").unwrap();
            match c.subcommand_name() {
                Some("catcsv") => run_tool("catcsv"),
                Some("fixed2csv") => run_tool("fixed2csv"),
                Some("geochunk") => run_tool("geochunk"),
                Some("geocode-csv") => run_tool("geocode-csv"),
                Some("hashcsv") => run_tool("hashcsv"),
                Some("scrubcsv") => run_tool("scrubcsv"),
                _ => Ok(()),
            }
        }
        _ => Ok(()),
    }
}

fn run_tool(name: &str) -> anyhow::Result<()> {
    let status = Command::new(name)
        .args(std::env::args().skip_while(|a| a != name).skip(1))
        .status()?;
    anyhow::ensure!(status.success(), "{name} failed");
    Ok(())
}
