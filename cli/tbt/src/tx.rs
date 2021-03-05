use anyhow::Result;
use clap::Clap;
use tavern_database::{State, Tx};

/// Interact with transactions
#[derive(Clap)]
#[clap(name = "tx")]
pub(crate) struct TxCmd {
    #[clap(subcommand)]
    subcmd: TxSubCmd,
}

#[derive(Clap)]
pub(crate) enum TxSubCmd {
    /// Add new tx to database
    #[clap(name = "add")]
    AddTxCmd(AddTxCmd),
}

#[derive(Clap)]
pub(crate) struct AddTxCmd {
    /// From what account to send token
    from: String,

    /// To what account to send token
    to: String,

    /// How many tokens to send
    value: u64,

    /// Data of transaction
    data: Option<String>,
}

impl TxCmd {
    pub(crate) fn process(self, state: &mut State, file: &mut std::fs::File) -> Result<()> {
        match self.subcmd {
            TxSubCmd::AddTxCmd(add_tx_cmd) => {
                let tx = Tx {
                    from: add_tx_cmd.from,
                    to: add_tx_cmd.to,
                    value: add_tx_cmd.value,
                    data: add_tx_cmd.data.unwrap_or(String::new()),
                };

                add_tx(state, &tx)?;
                state.persist(file)?;

                if tx.is_reward() {
                    println!("{} tbt rewarded to {}", tx.value, tx.to);
                } else {
                    println!("{} tbt transfered from {} to {}", tx.value, tx.from, tx.to);
                }
            }
        }

        Ok(())
    }
}

fn add_tx(state: &mut State, tx: &Tx) -> Result<()> {
    state.add(tx)?;

    Ok(())
}
