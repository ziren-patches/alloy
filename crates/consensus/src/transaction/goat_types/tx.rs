use alloy_primitives::Address;
use alloy_rlp::Decodable;

use crate::transaction::goat_types::*;

pub trait GoatTx: Decodable {
    fn is_goat_tx(&self) -> bool;

    fn deposit(&self) -> Option<Mint>; // deposit of the bridge
    fn withdraw(&self) -> Option<Mint>; // withdraw from consensus layer

    fn sender(&self) -> Address;
    fn contract(&self) -> Address;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TxGoatInner {
    NewBtcBlockTx(NewBtcBlockTx),
    CompleteUnlockTx(CompleteUnlockTx),
    DistributeRewardTx(DistributeRewardTx),
    DepositTx(DepositTx),
    Cancel2Tx(Cancel2Tx),
    PaidTx(PaidTx),
}

impl Default for TxGoatInner {
    fn default() -> Self {
        Self::NewBtcBlockTx(NewBtcBlockTx::default())
    }
}

impl TxGoatInner {
    pub fn sender(&self) -> Address {
        match &self {
            Self::NewBtcBlockTx(tx) => tx.sender(),
            Self::CompleteUnlockTx(tx) => tx.sender(),
            Self::DistributeRewardTx(tx) => tx.sender(),
            Self::DepositTx(tx) => tx.sender(),
            Self::Cancel2Tx(tx) => tx.sender(),
            Self::PaidTx(tx) => tx.sender(),
        }
    }

    pub fn to(&self) -> Address {
        match &self {
            Self::NewBtcBlockTx(tx) => tx.contract(),
            Self::CompleteUnlockTx(tx) => tx.contract(),
            Self::DistributeRewardTx(tx) => tx.contract(),
            Self::DepositTx(tx) => tx.contract(),
            Self::Cancel2Tx(tx) => tx.contract(),
            Self::PaidTx(tx) => tx.contract(),
        }
    }

    pub fn deposit(&self) -> Option<Mint> {
        match &self {
            Self::NewBtcBlockTx(tx) => tx.deposit(),
            Self::CompleteUnlockTx(tx) => tx.deposit(),
            Self::DistributeRewardTx(tx) => tx.deposit(),
            Self::DepositTx(tx) => tx.deposit(),
            Self::Cancel2Tx(tx) => tx.deposit(),
            Self::PaidTx(tx) => tx.deposit(),
        }
    }

    pub fn withdraw(&self) -> Option<Mint> {
        match &self {
            Self::NewBtcBlockTx(tx) => tx.withdraw(),
            Self::CompleteUnlockTx(tx) => tx.withdraw(),
            Self::DistributeRewardTx(tx) => tx.withdraw(),
            Self::DepositTx(tx) => tx.withdraw(),
            Self::Cancel2Tx(tx) => tx.withdraw(),
            Self::PaidTx(tx) => tx.withdraw(),
        }
    }
}

pub fn decode_goat_tx(
    module: Module,
    action: Action,
    buf: &mut &[u8],
) -> alloy_rlp::Result<TxGoatInner> {
    let tx = match module {
        BIRDGE_MODULE => match action {
            BITCOIN_NEW_BLOCK_ACTION => TxGoatInner::NewBtcBlockTx(NewBtcBlockTx::decode(buf)?),
            BRIDGE_CANCEL2_ACTION => TxGoatInner::Cancel2Tx(Cancel2Tx::decode(buf)?),
            BRIDGE_DEPOIT_ACTION => TxGoatInner::DepositTx(DepositTx::decode(buf)?),
            BRIDGE_PAID_ACTION => TxGoatInner::PaidTx(PaidTx::decode(buf)?),
            _ => return Err(alloy_rlp::Error::Custom("Unknown action for Bridge module")),
        },
        LOCKING_MODULE => match action {
            LOCKING_COMPLETE_UNLOCK_ACTION => {
                TxGoatInner::CompleteUnlockTx(CompleteUnlockTx::decode(buf)?)
            }
            LOCKING_DISTRIBUTE_REWARD_ACTION => {
                TxGoatInner::DistributeRewardTx(DistributeRewardTx::decode(buf)?)
            }
            _ => return Err(alloy_rlp::Error::Custom("Unknown action for Locking module")),
        },
        _ => return Err(alloy_rlp::Error::Custom("Unknown module for TxGoat")),
    };
    Ok(tx)
}
