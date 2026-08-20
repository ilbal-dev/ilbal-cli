use clap::{Arg, Command, command};
use indoc::indoc;
mod commands;
mod registries;
mod typedefs;
use owo_colors::OwoColorize;

fn gradient(t: f32) -> (u8, u8, u8) {
    let lerp = |a: u8, b: u8| (a as f32 + (b as f32 - a as f32) * t).round() as u8;
    (lerp(255, 0), lerp(165, 255), lerp(0, 255))
}

fn print_banner() -> anyhow::Result<()> {
    let font = figlet_rs::FIGlet::standard().map_err(anyhow::Error::msg)?;
    let figure = font.convert("ilbal.dev").expect("ascii renders fine");
    let text = figure.as_str();
    let width = text.lines().next().map_or(1, |l| l.chars().count().max(1));
    let gradient_text = text
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .map(|(i, c)| {
                    let t = if width > 1 {
                        i as f32 / (width - 1) as f32
                    } else {
                        0.0
                    };
                    let (r, g, b) = gradient(t);
                    format!("{}", c.truecolor(r, g, b))
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n");
    println!("{gradient_text}");
    Ok(())
}

fn main() {
    if let Err(e) = run_program() {
        if e.downcast_ref::<std::io::Error>()
            .is_some_and(|io| io.kind() == std::io::ErrorKind::Interrupted)
        {
            // Ctrl+C - exit gracefully
        } else {
            eprintln!("Error: {e:#}");
        }
    }
}

fn run_program() -> anyhow::Result<()> {
    const INGEST_LONG_HELP: &str = indoc!(
        "Command line arguments unique to each binary

        All binaries come from the following sources.
        Details about each binary can be found at the respective link.

        https://www.dbcrossbar.org
        https://www.gdal.org
        https://www.pgferry.com
        https://github.com/faradayio/csv-tools."
    );

    let ingest_cmd = Command::new("ingest")
        .about("Convert, transform, and load data into ilbal database")
        .arg(
            Arg::new("save")
                .long("save")
                .short('s')
                .num_args(0..=1)
                .value_name("data/output.csv")
                .help("Save the output to a file, if applicable"),
        )
        .arg(
            Arg::new("args")
                .num_args(0..)
                .trailing_var_arg(true)
                .help("Command line arguments unique to each binary")
                .long_help(INGEST_LONG_HELP),
        );

    // ***** Collect command line arguments *****
    let mut cmd = command!()
        .about(
            indoc!(
                "The ilbal CLI provides collaborative database-centric development workflows
            for fast iteration. Tools include database branching, data ingress, and more."
            )
            .cyan()
            .to_string(),
        )
        .subcommand(Command::new("init").about("Initialize an ilbal project"))
        .subcommand(Command::new("start").about("Start the ilbal database"))
        .subcommand(Command::new("stop").about("Stop the ilbal database"))
        .subcommand(Command::new("status").about("Get the status of the ilbal project"))
        .subcommand(ingest_cmd.clone())
        .subcommand(Command::new("reset").about("Reset the ilbal project"))
        .subcommand(
            Command::new("pgbranch")
                .about("Git-like actions on local ilbal database")
                .arg(Arg::new("args").num_args(0..).trailing_var_arg(true)),
        )
        .subcommand(Command::new("pgroll").about("Push database changes"))
        .subcommand(Command::new("push").about("Push database to ilbal cloud"))
        .subcommand(Command::new("pull").about("Pull database from ilbal cloud"));

    let match_result = match cmd.clone().try_get_matches() {
        Ok(matches) => matches,
        Err(e) => {
            if e.kind() == clap::error::ErrorKind::DisplayHelp {
                print_banner()?;
                e.print()?;
                return Ok(());
            }
            e.exit();
        }
    };

    // ***** Process command line arguments *****
    let mut ingest_cmd = ingest_cmd;
    match match_result.subcommand_name() {
        Some("init") => commands::init::run(),
        Some("start") => commands::start::run(),
        Some("stop") => commands::stop::run(),
        Some("status") => commands::status::run(),
        Some("reset") => commands::reset::run(),
        Some("pgbranch") => commands::pgbranch::run(),
        Some("pgroll") => commands::pgroll::run(),
        Some("pull") => commands::pull::run(),
        Some("ingest") => commands::ingest::run(
            &mut ingest_cmd,
            match_result.subcommand_matches("ingest").unwrap(),
        ),
        None => {
            print_banner()?;
            cmd.print_help()?;
            Ok(())
        }
        Some(_) => Ok(()),
    }
}
