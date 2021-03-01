//! Seed of database

use std::collections::HashMap;

use serde::Deserialize;

use crate::tx::Account;

#[derive(Deserialize, Debug)]
pub(crate) struct Genesis {
    pub genesis_time: String,
    pub chain_id: String,
    pub balances: HashMap<Account, u64>,
}
