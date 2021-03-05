//! Transaction structure

use serde::{ Deserialize, Serialize };

/// customer
pub type Account = String;

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

impl std::fmt::Display for Tx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "from {}, to {}, value {}, data {}", self.from, self.to, self.value, self.data)
    }
}
