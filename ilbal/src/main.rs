use clap::{Command, command};
use dialoguer::{Confirm, Input, InputValidator, MultiSelect, Password, PasswordValidator, Select};
use indoc::indoc;
use owo_colors::OwoColorize;

fn main() {
    // ***** Collect command line arguments *****
    let match_result = command!()
        .about(indoc!(
            "The ilbal CLI provides collaborative database-centric development workflows
            for fast iteration. Tools include database branching, data ingress, and more."
        ))
        .subcommand_required(true)
        .subcommand(Command::new("init").about("Initialize an ilbal project (default: .)"))
        .subcommand(Command::new("start").about("Start the ilbal database"))
        .subcommand(Command::new("stop").about("Stop the ilbal database"))
        .subcommand(Command::new("status").about("Get the status of the ilbal project"))
        .subcommand(Command::new("pgbranch").about("Git-like actions on local ilbal database"))
        .subcommand(Command::new("pgroll").about("Push database changes"))
        .subcommand(Command::new("pull").about("Pull database from ilbal cloud"))
        .subcommand(
            Command::new("load")
                .about("Access data loaders")
                .subcommand_required(true)
                .subcommand(
                    Command::new("dbcrossbar")
                        .about("Translate between database and storage formats (including CSV). (https://www.dbcrossbar.org)"),
                )
                .subcommand(
                    Command::new("gdal")
                        .about("Translate between geospatial raster and vector data formats. (https://www.gdal.org)")
                        .subcommand_required(true)
                        .subcommand(Command::new("ogr2ogr")
                            .about("Geospatial vector translator. (https://www.gdal.org/ogr2ogr.html)"))
                        .subcommand(Command::new("gdal_translate")
                            .about("Geospatial raster translator. (https://www.gdal.org/gdal_translate.html)")),
                )
                .subcommand(
                    Command::new("pgferry")
                        .about("Migrate common database formats to PostgreSQL. (https://www.pgferry.com)"),
                )
                .subcommand(
                    Command::new("csv")
                        .about("Tools for working with CSV files. (https://github.com/faradayio/csv-tools)")
                        .subcommand_required(true)
                        .subcommand(Command::new("catcsv")
                            .about("Concatenate directories of CSV files. (https://github.com/faradayio/csv-tools/tree/main/catcsv)"))
                        .subcommand(Command::new("fixed2csv")
                            .about("Convert fixed-width files to CSV. (https://github.com/faradayio/csv-tools/tree/main/fixed2csv)"))
                        .subcommand(Command::new("geochunk")
                            .about("Partition data sets by estimated population. (https://github.com/faradayio/csv-tools/tree/main/geochunk)"))
                        .subcommand(Command::new("geocode-csv")
                            .about("Geocode CSV files. (https://github.com/faradayio/geocode-csv)"))
                        .subcommand(Command::new("hashcsv")
                            .about("Append unique id columns to CSV files. (https://github.com/faradayio/csv-tools/tree/main/hashcsv)"))
                        .subcommand(Command::new("scrubcsv")
                            .about("Clean and normalize CSV files. (https://github.com/faradayio/csv-tools/tree/main/scrubcsv)")),
                ),
        )
        // .arg(
        //     Arg::new("start")
        //         .long("start")
        //         .help("Start the ilbal database"),
        // )
        .get_matches();

    // ***** Process command line arguments *****
    // match match_result.subcommand_name() {
    // Some("init") => commands::init::run(…),
    // Some("start") => commands::start::run(),
    // Some("load") => commands::load::run(…),
    // _ => { let _ = command!().print_help(); }
    // }

    match match_result.subcommand_name() {
        Some("init") => {
            let m = match_result.subcommand_matches("init").unwrap();
            let _dir = m.get_one::<String>("directory");
            // TODO: implement init logic
            println!("{}", "Running command: init".red());
            println!("{}", "Copy config.toml to local project directory".yellow());
            println!("{}", "Add other configuration.".yellow());
            println!("{}", "Add directories for mapping to containers.".yellow());
        }
        Some("start") => {
            // TODO: implement start logic
            println!("{}", "Running command: start".red());
            println!(
                "{}",
                "Run containers with selected OCI daemon (podman/docker)".yellow()
            );
        }
        Some("stop") => {
            // TODO: implement stop logic
            println!("{}", "Running command: stop".red());
        }
        Some("status") => {
            // TODO: implement status logic
            println!("{}", "Running command: status".red());
        }
        Some("pgbranch") => {
            let m = match_result.subcommand_matches("pgbranch").unwrap();
            let _name = m.get_one::<String>("name").unwrap();
            // TODO: implement pgbranch logic
            println!("{}", "Running command: pgbranch".red());
        }
        Some("pgroll") => {
            // TODO: implement pgroll logic
            println!("{}", "Running command: pgroll".red());
        }
        Some("pull") => {
            // TODO: implement pull logic
            println!("{}", "Running command: pull".red());
        }
        Some("load") => {
            let m = match_result.subcommand_matches("load").unwrap();
            match m.subcommand_name() {
                Some("dbcrossbar") => {
                    // TODO: implement load dbcrossbar logic
                    println!("{}", "Running command: load dbcrossbar".red());
                }
                Some("gdal") => {
                    let g = m.subcommand_matches("gdal").unwrap();
                    match g.subcommand_name() {
                        Some("ogr2ogr") => {
                            // TODO: implement load gdal ogr2ogr logic
                            println!("{}", "Running command: load gdal ogr2ogr".red());
                        }
                        Some("gdal_translate") => {
                            // TODO: implement load gdal gdal_translate logic
                            println!("{}", "Running command: load gdal gdal_translate".red());
                        }
                        _ => {}
                    }
                }
                Some("pgferry") => {
                    // TODO: implement load pgferry logic
                    println!("{}", "Running command: load pgferry".red());
                }
                Some("csv") => {
                    let c = m.subcommand_matches("csv").unwrap();
                    match c.subcommand_name() {
                        Some("catcsv") => {
                            // TODO: implement load csv catcsv logic
                            println!("{}", "Running command: load csv catcsv".red());
                        }
                        Some("fixed2csv") => {
                            // TODO: implement load csv fixed2csv logic
                            println!("{}", "Running command: load csv fixed2csv".red());
                        }
                        Some("geochunk") => {
                            // TODO: implement load csv geochunk logic
                            println!("{}", "Running command: load csv geochunk".red());
                        }
                        Some("geocode-csv") => {
                            // TODO: implement load csv geocode-csv logic
                            println!("{}", "Running command: load csv geocode-csv".red());
                        }
                        Some("hashcsv") => {
                            // TODO: implement load csv hashcsv logic
                            println!("{}", "Running command: load csv hashcsv".red());
                        }
                        Some("scrubcsv") => {
                            // TODO: implement load csv scrubcsv logic
                            println!("{}", "Running command: load csv scrubcsv".red());
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        _ => {
            // No subcommand or unknown — show help
            let _ = command!().print_help();
        }
    }
}
