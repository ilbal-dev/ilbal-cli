use clap::{Arg, Command, command};
use indoc::indoc;
mod commands;

fn main() -> anyhow::Result<()> {
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
                // .disable_help_flag(true)
                .arg(Arg::new("args").num_args(0..).trailing_var_arg(true)),
        )
        // .disable_help_flag(true)
        .subcommand(Command::new("pgroll").about("Push database changes"))
        .subcommand(Command::new("pull").about("Pull database from ilbal cloud"))
        .subcommand(
            Command::new("ingest")
                .about("Convert, transform, and load data into ilbal database")
                .arg(
                    Arg::new("save")
                        .long("save")
                        .short('s')
                        .num_args(0..=1)
                        .value_name("data/output.csv")
                        .help("Save the output to a file"),
                )
                .arg(
                    Arg::new("args")
                        .num_args(0..)
                        .trailing_var_arg(true)
                        .help("Command line arguments unique to each binary"),
                ),
        )
        .get_matches();

    // ***** Process command line arguments *****
    match match_result.subcommand_name() {
        // Some("init") => commands::init::run(&match_result),
        // Some("start") => commands::start::run(),
        // Some("stop") => commands::stop::run(),
        // Some("status") => commands::status::run(),
        // Some("pgbranch") => {
        //     commands::pgbranch::run(match_result.subcommand_matches("pgbranch").unwrap())
        // }
        // Some("pgroll") => commands::pgroll::run(match_result.subcommand_matches("pgroll").unwrap()),
        // Some("pull") => commands::pull::run(),
        Some("ingest") => commands::ingest::run(match_result.subcommand_matches("ingest").unwrap()),
        _ => Ok(()),
    }
}
