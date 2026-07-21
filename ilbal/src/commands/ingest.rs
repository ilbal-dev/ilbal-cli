use crate::typedefs::binaries::BINARIES;
use clap::ArgMatches;
use dialoguer::Select;
use owo_colors::OwoColorize;
use std::io::Write;
use std::process::Command;

const DOCKER_IMAGE: &str = "ilbal-ingest:latest";

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
            Select::new()
                .with_prompt("Press Enter to close")
                .items(BINARIES)
                .max_length(10)
                .interact()?;
        }
    }
    Ok(())
}

// const BINARIES: &[&str] = &[
//     "catcsv",
//     "cs2cs",
//     "dbcrossbar",
//     "fixed2csv",
//     "gdal",
//     "gdal-config",
//     "gdal2tiles",
//     "gdal2xyz",
//     "gdal_calc",
//     "gdal_contour",
//     "gdal_create",
//     "gdal_edit",
//     "gdal_fillnodata",
//     "gdal_footprint",
//     "gdal_grid",
//     "gdal_merge",
//     "gdal_pansharpen",
//     "gdal_polygonize",
//     "gdal_proximity",
//     "gdal_rasterize",
//     "gdal_retile",
//     "gdal_sieve",
//     "gdal_translate",
//     "gdal_viewshed",
//     "gdaladdo",
//     "gdalattachpct",
//     "gdalbuildvrt",
//     "gdalcompare",
//     "gdaldem",
//     "gdalenhance",
//     "gdalinfo",
//     "gdallocationinfo",
//     "gdalmanage",
//     "gdalmdiminfo",
//     "gdalmdimtranslate",
//     "gdalmove",
//     "gdalsrsinfo",
//     "gdaltindex",
//     "gdaltransform",
//     "gdalwarp",
//     "geochunk",
//     "geocode-csv",
//     "geod",
//     "gie",
//     "gnmanalyse",
//     "gnmmanage",
//     "hashcsv",
//     "invgeod",
//     "invproj",
//     "nearblack",
//     "ogr2ogr",
//     "ogr_layer_algebra",
//     "ogrinfo",
//     "ogrlineref",
//     "ogrmerge",
//     "ogrtindex",
//     "pct2rgb",
//     "proj",
//     "projinfo",
//     "projsync",
//     "rgb2pct",
//     "scrubcsv",
// ];
