//! State struct

use crate::genesis::Genesis;
use crate::tx::{Account, Tx};

use std::collections::HashMap;

use thiserror::Error;

pub struct State {
    balances: HashMap<Account, u64>,
    tx_mem_pool: Vec<Tx>,
}

#[derive(Error, Debug)]
pub enum InitStateError {
    #[error("Failed to deserialize genesis")]
    DeserializeGenesisError {
        #[source]
        source: serde_json::Error,
    },

    #[error("Failed to read transactions")]
    ReadTransactionsError {
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to deserialize transactions")]
    DeserializeTxError {
        #[source]
        source: serde_json::Error,
    },

    #[error("Error happened during applying transactions from past")]
    ApplyTxError(Tx, ApplyTxError),
}

#[derive(Error, Debug)]
pub enum ApplyTxError {
    #[error("Sender not found")]
    SenderAccountNotFound,

    #[error("Receiver not found")]
    ReceiverAccountNotFound,

    #[error("Insufficient balance")]
    InsufficientBalance,
}

#[derive(Error, Debug)]
pub enum PersistError {
    #[error("Failed to serialize tx")]
    SerializeError {
        #[source]
        source: serde_json::Error,
    },

    #[error("Failed to persist transactions")]
    WriteError {
        #[source]
        source: std::io::Error,
    },
}

impl State {
    pub fn new<R, T>(genesis_reader: &mut R, tx_reader: &mut T) -> Result<Self, InitStateError>
    where
        R: std::io::Read,
        T: std::io::Read,
    {
        let genesis: Genesis = serde_json::from_reader(genesis_reader)
            .map_err(|err| InitStateError::DeserializeGenesisError { source: err })?;

        let mut txs_string = String::new();
        tx_reader
            .read_to_string(&mut txs_string)
            .map_err(|err| InitStateError::ReadTransactionsError { source: err })?;

        let txs = txs_string
            .lines()
            .map(|line| {
                serde_json::from_str::<Tx>(line)
                    .map_err(|err| InitStateError::DeserializeTxError { source: err })
            })
            .collect::<Result<Vec<Tx>, InitStateError>>()?;

        let mut state = Self {
            balances: genesis.balances,
            tx_mem_pool: Vec::new(),
        };

        for tx in txs.iter() {
            state
                .apply(tx)
                .map_err(|err| InitStateError::ApplyTxError(tx.clone(), err))?;
        }

        Ok(state)
    }

    pub fn balances(&self) -> &HashMap<Account, u64> {
        &self.balances
    }

    pub fn add(&mut self, tx: Tx) -> Result<(), ApplyTxError> {
        self.apply(&tx)?;
        self.tx_mem_pool.push(tx);

        Ok(())
    }

    pub fn persist<W: std::io::Write>(&mut self, ts_writer: &mut W) -> Result<(), PersistError> {
        let content = self
            .tx_mem_pool
            .iter()
            .map(|tx| serde_json::to_string(tx))
            .collect::<Result<Vec<String>, serde_json::Error>>()
            .map_err(|err| PersistError::SerializeError { source: err })?
            .join("\n");

        ts_writer
            .write(content.as_bytes())
            .map_err(|err| PersistError::WriteError { source: err })?;

        Ok(())
    }

    fn apply(&mut self, tx: &Tx) -> Result<(), ApplyTxError> {
        // In order to keep borrow checker happy,
        // control flow is changed weiredly here.
        if !self.balances.keys().any(|key| *key == tx.to) {
            return Err(ApplyTxError::ReceiverAccountNotFound);
        }

        let sender_balance = self
            .balances
            .get_mut(&tx.from)
            .ok_or(ApplyTxError::SenderAccountNotFound)?;

        if tx.is_reward() {
            *sender_balance += tx.value;
            Ok(())
        } else {
            if *sender_balance >= tx.value {
                *sender_balance -= tx.value;

                let receiver_balance = self.balances.get_mut(&tx.to).unwrap(); // since we checked it before

                *receiver_balance += tx.value;

                Ok(())
            } else {
                Err(ApplyTxError::InsufficientBalance)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_state() {
        let genesis = r#"
        {
            "genesis_time": "2021-02-25:00:00.000000000Z",
            "chain_id": "the-blockchain-tavern-ledger",
            "balances": {
                "andrej": 1000000,
                "babayaga": 0
            }
        }
        "#;

        let txs = r#" {"from":"andrej","to":"andrej","value":3,"data":""}
            {"from":"andrej","to":"andrej","value":700,"data":"reward"}
            {"from":"andrej","to":"babayaga","value":2000,"data":""}
            {"from":"andrej","to":"andrej","value":100,"data":"reward"}
            {"from":"babayaga","to":"andrej","value":1,"data":""}
            {"from":"andrej","to":"andrej","value":3,"data":""} "#;

        let state = State::new(&mut genesis.as_bytes(), &mut txs.as_bytes()).unwrap();

        assert_eq!(*state.balances.get("andrej").unwrap(), 998801);
        assert_eq!(*state.balances.get("babayaga").unwrap(), 1999);
    }

    #[test]
    fn add_reward_transaction() {
        let mut state = State {
            balances: vec![("andrej".to_string(), 100), ("babayaga".to_string(), 20)]
                .into_iter()
                .collect(),
            tx_mem_pool: Vec::new(),
        };

        state
            .add(Tx {
                from: "andrej".to_string(),
                to: "andrej".to_string(),
                value: 100,
                data: "reward".to_string(),
            })
            .unwrap();

        assert_eq!(*state.balances.get("andrej").unwrap(), 200);
        assert_eq!(state.tx_mem_pool[0].from, "andrej");
        assert_eq!(state.tx_mem_pool[0].to, "andrej");
        assert_eq!(state.tx_mem_pool[0].value, 100);
        assert_eq!(state.tx_mem_pool[0].data, "reward");
    }

    #[test]
    fn add_normal_transaction() {
        let mut state = State {
            balances: vec![("andrej".to_string(), 100), ("babayaga".to_string(), 20)]
                .into_iter()
                .collect(),
            tx_mem_pool: Vec::new(),
        };

        state
            .add(Tx {
                from: "andrej".to_string(),
                to: "babayaga".to_string(),
                value: 1,
                data: String::new(),
            })
            .unwrap();

        assert_eq!(*state.balances.get("andrej").unwrap(), 99);
        assert_eq!(*state.balances.get("babayaga").unwrap(), 21);
        assert_eq!(state.tx_mem_pool[0].from, "andrej");
        assert_eq!(state.tx_mem_pool[0].to, "babayaga");
        assert_eq!(state.tx_mem_pool[0].value, 1);
        assert_eq!(state.tx_mem_pool[0].data, "");
    }

    #[test]
    fn transaction_with_insufficient_funds() {
        let mut state = State {
            balances: vec![("andrej".to_string(), 100), ("babayaga".to_string(), 20)]
                .into_iter()
                .collect(),
            tx_mem_pool: Vec::new(),
        };

        let result = state.add(Tx {
            from: "andrej".to_string(),
            to: "babayaga".to_string(),
            value: 101,
            data: String::new(),
        });

        assert!(matches!(result, Err(ApplyTxError::InsufficientBalance)));
    }

    #[test]
    fn persist() {
        let mut state = State {
            balances: vec![("andrej".to_string(), 100), ("babayaga".to_string(), 20)]
                .into_iter()
                .collect(),
            tx_mem_pool: vec![
                Tx {
                    from: "andrej".to_string(),
                    to: "andrej".to_string(),
                    value: 100,
                    data: "reward".to_string(),
                },
                Tx {
                    from: "andrej".to_string(),
                    to: "babayaga".to_string(),
                    value: 1,
                    data: String::new(),
                },
            ],
        };

        let mut buf: Vec<u8> = Vec::new();

        state.persist(&mut buf).unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            r#"{"from":"andrej","to":"andrej","value":100,"data":"reward"}
{"from":"andrej","to":"babayaga","value":1,"data":""}"#
        );
    }
}
