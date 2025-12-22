use alloy_primitives::{Address, U256};
use alloy_rlp::{Decodable, Error};

use crate::transaction::goat_types::{constant::NATIVE_TOKEN, tx::GoatTx, Mint};

use super::constant::{LOCKING_CONTRACT, LOCKING_EXECUTOR};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct CompleteUnlockTx {
    pub id: u64,
    pub recipient: Address,
    pub token: Address,
    pub amount: U256,
}

impl CompleteUnlockTx {
    // completeUnlock(uint64 id,address recipient,address token,uint256 amount)
    pub const METHOD_ID: [u8; 4] = [0x00, 0xab, 0xa5, 0x1a];

    pub const SIZE: usize = 132;
}

impl GoatTx for CompleteUnlockTx {
    fn is_goat_tx(&self) -> bool {
        true
    }

    fn sender(&self) -> Address {
        LOCKING_EXECUTOR
    }

    fn contract(&self) -> Address {
        LOCKING_CONTRACT
    }

    fn deposit(&self) -> Option<Mint> {
        None
    }

    fn withdraw(&self) -> Option<Mint> {
        if self.token == NATIVE_TOKEN {
            return Some(Mint { address: self.recipient, amount: self.amount, tax: U256::ZERO });
        }
        None
    }
}

impl Decodable for CompleteUnlockTx {
    fn decode(buf: &mut &[u8]) -> Result<Self, Error> {
        // Verify input length
        if buf.len() != Self::SIZE {
            return Err(Error::ListLengthMismatch { expected: Self::SIZE, got: buf.len() });
        }

        // Verify method ID (first 4 bytes)
        let mut i = 0;
        let input_method = <[u8; 4]>::try_from(&buf[i..4])
            .map_err(|_| Error::Custom("Invalid method ID format"))?;

        if input_method != Self::METHOD_ID {
            return Err(Error::Custom("not a completeUnlock tx"));
        }

        // Parse id (8 bytes)
        i += 4;
        let id = u64::from_be_bytes(
            buf[i + 24..i + 32].try_into().map_err(|_| Error::Custom("Failed to decode id"))?,
        );

        // Parse recipient (20 bytes)
        i += 32;
        let recipient = Address::from_slice(&buf[i + 12..i + 32]); // Skip first 12 padding bytes

        // Parse token (20 bytes)
        i += 32;
        let token = Address::from_slice(&buf[i + 12..i + 32]); // Skip first 12 padding bytes

        // Parse amount (32 bytes)
        i += 32;
        let amount = U256::from_be_bytes(
            <[u8; 32]>::try_from(&buf[i..i + 32])
                .map_err(|_| Error::Custom("Failed to decode amount"))?,
        );

        Ok(Self { id, recipient, token, amount })
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct DistributeRewardTx {
    pub id: u64,
    pub recipient: Address,
    pub goat: U256,
    pub gas_reward: U256,
}

impl DistributeRewardTx {
    // distributeReward(uint64 id,address recipient,uint256 goat,uint256 amount)
    pub const METHOD_ID: [u8; 4] = [0xbd, 0x9f, 0xad, 0xb5];

    pub const SIZE: usize = 132;
}

impl GoatTx for DistributeRewardTx {
    fn is_goat_tx(&self) -> bool {
        true
    }

    fn sender(&self) -> Address {
        LOCKING_EXECUTOR
    }

    fn contract(&self) -> Address {
        LOCKING_CONTRACT
    }

    fn deposit(&self) -> Option<Mint> {
        None
    }

    fn withdraw(&self) -> Option<Mint> {
        Some(Mint { address: self.recipient, amount: self.gas_reward, tax: U256::ZERO })
    }
}

impl Decodable for DistributeRewardTx {
    fn decode(buf: &mut &[u8]) -> Result<Self, Error> {
        // Verify input length
        if buf.len() != Self::SIZE {
            return Err(Error::ListLengthMismatch { expected: Self::SIZE, got: buf.len() });
        }

        // Verify method ID (first 4 bytes)
        let mut i = 0;
        let input_method = <[u8; 4]>::try_from(&buf[i..4])
            .map_err(|_| Error::Custom("Invalid method ID format"))?;

        if input_method != Self::METHOD_ID {
            return Err(Error::Custom("not a completeUnlock tx"));
        }

        // Parse id (8 bytes)
        i += 4;
        let id = u64::from_be_bytes(
            buf[i + 24..i + 32].try_into().map_err(|_| Error::Custom("Failed to decode id"))?,
        );

        // Parse recipient (20 bytes)
        i += 32;
        let recipient = Address::from_slice(&buf[i + 12..i + 32]); // Skip first 12 padding bytes

        // Parse goat (32 bytes)
        i += 32;
        let goat = U256::from_be_bytes(
            <[u8; 32]>::try_from(&buf[i..i + 32])
                .map_err(|_| Error::Custom("Failed to decode goat"))?,
        );

        // Parse gas_reward (32 bytes)
        i += 32;
        let gas_reward = U256::from_be_bytes(
            <[u8; 32]>::try_from(&buf[i..i + 32])
                .map_err(|_| Error::Custom("Failed to decode gas_reward"))?,
        );

        Ok(Self { id, recipient, goat, gas_reward })
    }
}
