//! Crate to manage database of blockchain

mod state;
mod tx;
mod genesis;

// new
// persistance
// add transaction
// check balances

pub use state::State;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
