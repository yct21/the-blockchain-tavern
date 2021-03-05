use crate::balances::BalancesCmd;
use crate::tx::TxCmd;
use clap::Clap;

/// The Blockchain Tarvern CLI
#[derive(Clap)]
#[clap(
    version = "0.3",
    author = "Chutian Yang <yct21@12tcy.com>",
    name = "tbt"
)]
pub(crate) struct Opts {
    #[clap(subcommand)]
    pub subcmd: BaseSubCmd,

    /// Path of database directory
    #[clap(long = "database", short = 'd', default_value = "data")]
    pub database: String,
}

#[derive(Clap)]
pub(crate) enum BaseSubCmd {
    #[clap(name = "balances")]
    BalancesCmd(BalancesCmd),

    #[clap(name = "tx")]
    TxCmd(TxCmd),
}
