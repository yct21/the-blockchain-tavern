use clap::Clap;
use anyhow::Result;
use tavern_database::State;

/// Interact with balances
#[derive(Clap)]
#[clap(name = "balances")]
pub(crate) struct BalancesCmd {
    #[clap(subcommand)]
    subcmd: BalancesSubCmd,
}

#[derive(Clap)]
pub(crate) enum BalancesSubCmd {
    /// List balances
    #[clap(name = "list")]
    ListBalancesCmd(ListBalancesCmd),
}

#[derive(Clap)]
pub(crate) struct ListBalancesCmd;

impl BalancesCmd {
    pub fn process(&self, state: State) -> Result<()>  {
        match self.subcmd {
            BalancesSubCmd::ListBalancesCmd(_) => {
                list(state);
                Ok(())
            }
        }
    }
}

fn list(state: State) {
    println!("Accounts balances:");
    println!("__________________");
    println!("");
    for (name, value) in state.balances() {
        println!("{}: {}", name, value);
    }
}
