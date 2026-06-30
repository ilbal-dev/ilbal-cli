use clap::{Arg, Command, command};
use indoc::indoc;
mod commands;

fn main() -> anyhow::Result<()> {
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
        .subcommand(
            Command::new("pgbranch")
                .about("Git-like actions on local ilbal database")
                .arg(Arg::new("args").num_args(0..).trailing_var_arg(true)),
        )
        .subcommand(Command::new("pgroll").about("Push database changes"))
        .subcommand(Command::new("pull").about("Pull database from ilbal cloud"))
        .subcommand(ingest_cmd.clone())
        .get_matches();

    // ***** Process command line arguments *****
    let mut ingest_cmd = ingest_cmd;
    match match_result.subcommand_name() {
        Some("ingest") => commands::ingest::run(
            &mut ingest_cmd,
            match_result.subcommand_matches("ingest").unwrap(),
        ),
        _ => Ok(()),
    }
}
