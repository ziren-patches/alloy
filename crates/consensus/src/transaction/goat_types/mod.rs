// !Goat Types

pub mod constant;
pub mod tx;
pub mod tx_bridge;
pub mod tx_locking;

use alloy_primitives::{Address, U256};

pub use tx::*;
pub use tx_bridge::*;
pub use tx_locking::*;

pub type Module = u8;
pub type Action = u8;
pub type MethodId = [u8; 4];

// modules
pub const BIRDGE_MODULE: Module = 1;
pub const LOCKING_MODULE: Module = 2;

// bridge module actions
pub const BRIDGE_DEPOIT_ACTION: Action = 1;
pub const BRIDGE_CANCEL2_ACTION: Action = 2;
pub const BRIDGE_PAID_ACTION: Action = 3;
pub const BITCOIN_NEW_BLOCK_ACTION: Action = 4;

// locking module actions
pub const LOCKING_COMPLETE_UNLOCK_ACTION: Action = 1;
pub const LOCKING_DISTRIBUTE_REWARD_ACTION: Action = 2;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Mint {
    pub address: Address,
    pub amount: U256,
    pub tax: U256,
}
