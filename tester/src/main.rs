use clap::{Arg, ArgMatches, command};
use colored::Colorize;

fn main() {
    let match_result: ArgMatches = command!()
        .about("This is a tool to do stuff, like print your name. \nCan this be on multiple lines?")
        .arg(
            Arg::new("firstname")
                .long("firstname")
                .short('f')
                .help("The first name of the person")
                .required(true),
        )
        .arg(
            Arg::new("lastname")
                .long("lastname")
                .short('l')
                .help("The last name of the person")
                .required(true),
        )
        .get_matches();
    let firstname = match_result.get_one::<String>("firstname").unwrap();
    let lastname = match_result.get_one::<String>("lastname").unwrap();
    println!("Hello, {} {}!", firstname.on_red().blue(), lastname.red());
}
