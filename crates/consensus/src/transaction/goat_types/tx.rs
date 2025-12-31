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

pub fn goat_tx_sender(module: Module, action: Action) -> Address {
    match module {
        BIRDGE_MODULE => match action {
            BITCOIN_NEW_BLOCK_ACTION => NewBtcBlockTx::SENDER,
            BRIDGE_CANCEL2_ACTION => Cancel2Tx::SENDER,
            BRIDGE_DEPOIT_ACTION => DepositTx::SENDER,
            BRIDGE_PAID_ACTION => PaidTx::SENDER,
            _ => unreachable!(),
        },
        LOCKING_MODULE => match action {
            LOCKING_COMPLETE_UNLOCK_ACTION => CompleteUnlockTx::SENDER,
            LOCKING_DISTRIBUTE_REWARD_ACTION => DistributeRewardTx::SENDER,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

pub fn goat_tx_to(module: Module, action: Action) -> Address {
    match module {
        BIRDGE_MODULE => match action {
            BITCOIN_NEW_BLOCK_ACTION => NewBtcBlockTx::CONTRACT,
            BRIDGE_CANCEL2_ACTION => Cancel2Tx::CONTRACT,
            BRIDGE_DEPOIT_ACTION => DepositTx::CONTRACT,
            BRIDGE_PAID_ACTION => PaidTx::CONTRACT,
            _ => unreachable!(),
        },
        LOCKING_MODULE => match action {
            LOCKING_COMPLETE_UNLOCK_ACTION => CompleteUnlockTx::CONTRACT,
            LOCKING_DISTRIBUTE_REWARD_ACTION => DistributeRewardTx::CONTRACT,
            _ => unreachable!(),
        },
        _ => unreachable!(),
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
