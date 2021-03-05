// TODO:
// check balance
// add tx

mod balances;
mod cli;
mod tx;

use anyhow::{anyhow, Result};
use clap::Clap;
use cli::BaseSubCmd;
use std::fs::{File, OpenOptions};
use tavern_database::State;

fn run() -> Result<()> {
    let opts = cli::Opts::parse();

    let genesis_file_path = format!("{}/genesis.json", opts.database);
    let mut genesis_file = File::open(&genesis_file_path).map_err(|err| {
        anyhow!(
            "Could not open genesis file in {}:\n  {}",
            genesis_file_path,
            err
        )
    })?;

    let tx_file_path = format!("{}/tx.db", opts.database);
    let mut tx_file = OpenOptions::new().read(true).write(true).open(&tx_file_path)
        .map_err(|err| anyhow!("Could not open tx file in {}:\n  {}", tx_file_path, err))?;

    let mut state = State::new(&mut genesis_file, &mut tx_file)?;

    match opts.subcmd {
        BaseSubCmd::BalancesCmd(balances_cmd) => {
            balances_cmd.process(state)?;
        }
        BaseSubCmd::TxCmd(tx_cmd) => {
            tx_cmd.process(&mut state, &mut tx_file)?;
        }
    }

    Ok(())
}

fn main() {
    let result = run();

    if let Err(err) = result {
        println!("{}", err);
    }
}
