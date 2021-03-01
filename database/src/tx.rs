//! Transaction structure

use serde::{ Deserialize, Serialize };

/// customer
pub(crate) type Account = String;

/// Transaction
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Tx {
    pub from: Account,
    pub to: Account,
    pub value: u64,
    pub data: String,
}

impl Tx {
    pub fn is_reward(&self) -> bool {
        self.data == "reward"
    }
}

