// TODO:
// check balance
// add tx

mod balances;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("The Blockchain Tarvern")
        .author("Chutian Yang <yct21@12tcy.com>")
        .about("The Blockchain Tarvern CLI")
        .subcommand(
            SubCommand::with_name("balances")
                .about("Interact with balances")
                .subcommand(SubCommand::with_name("list").about("List all balances")),
        )
        .subcommand(
            SubCommand::with_name("tx")
                .about("Interact with transaction")
                .subcommand(
                    SubCommand::with_name("add")
                        .about("Add new tx to database")
                        .arg(
                            Arg::with_name("from")
                                .required(true)
                                .takes_value(true)
                                .help("From what account to send tokens"),
                        )
                        .arg(
                            Arg::with_name("to")
                                .required(true)
                                .takes_value(true)
                                .help("To what account to send tokens"),
                        )
                        .arg(
                            Arg::with_name("value")
                                .required(true)
                                .takes_value(true)
                                .help("How many tokens to send"),
                        )
                        .arg(
                            Arg::with_name("data")
                                .takes_value(true)
                                .possible_values(&["reward"])
                                .help("Data of transaction"),
                        ),
                ),
        )
        .get_matches();
}
