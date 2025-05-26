use alloy_primitives::{Address, TxHash, U256};
use alloy_rlp::{Decodable, Error};

use super::{
    constant::{BITCOINT_CONTRACT, BRIDGE_CONTRACT, RELAYER_EXECUTOR},
    tx::GoatTx,
};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DepositTx {
    pub tx_id: TxHash,
    pub tx_out: u32,
    pub target: Address,
    pub amount: U256,
    pub tax: U256,
}

impl DepositTx {
    // deposit(bytes32 txid, uint32 txout, address target, uint256 amount,uint256 tax)
    pub const METHOD_ID: [u8; 4] = [0x90, 0x41, 0x83, 0xcb];

    pub const SIZE: usize = 164;
}

impl GoatTx for DepositTx {
    fn is_goat_tx(&self) -> bool {
        true
    }

    fn sender(&self) -> Address {
        RELAYER_EXECUTOR
    }

    fn contract(&self) -> Address {
        BRIDGE_CONTRACT
    }

    fn deposit(&self) -> Option<super::Mint> {
        Some(super::Mint { address: self.target, amount: self.amount, tax: self.tax })
    }

    fn withdraw(&self) -> Option<super::Mint> {
        None
    }
}

impl Decodable for DepositTx {
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
            return Err(Error::Custom("not a deposit tx"));
        }

        // Parse txid (32 bytes)
        i += 4;
        let tx_id = TxHash::from_slice(&buf[i..i + 32]);

        // Parse tx_out (last 4 bytes of 32-byte chunk)
        i += 32;
        let tx_out = u32::from_be_bytes(
            buf[i + 28..i + 32].try_into().map_err(|_| Error::Custom("Failed to decode tx_out"))?,
        );

        // Parse target address (20 bytes right-aligned in 32 bytes)
        i += 32;
        let target = Address::from_slice(&buf[i + 12..i + 32]); // Skip first 12 padding bytes

        // Parse amount (32 bytes)
        i += 32;
        let amount = U256::from_be_bytes(
            <[u8; 32]>::try_from(&buf[i..i + 32])
                .map_err(|_| Error::Custom("Failed to decode amount"))?,
        );

        // Parse tax (32 bytes)
        i += 32;
        let tax = U256::from_be_bytes(
            <[u8; 32]>::try_from(&buf[i..i + 32])
                .map_err(|_| Error::Custom("Failed to decode tax"))?,
        );

        Ok(Self { tx_id, tx_out, target, amount, tax })
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cancel2Tx {
    pub id: U256,
}

impl Cancel2Tx {
    // cancel2(uint256)
    pub const METHOD_ID: [u8; 4] = [0xc1, 0x9d, 0xd3, 0x20];

    pub const SIZE: usize = 36;
}

impl GoatTx for Cancel2Tx {
    fn is_goat_tx(&self) -> bool {
        true
    }

    fn sender(&self) -> Address {
        RELAYER_EXECUTOR
    }

    fn contract(&self) -> Address {
        BRIDGE_CONTRACT
    }

    fn deposit(&self) -> Option<super::Mint> {
        None
    }

    fn withdraw(&self) -> Option<super::Mint> {
        None
    }
}

impl Decodable for Cancel2Tx {
    fn decode(buf: &mut &[u8]) -> Result<Self, Error> {
        // Verify input length
        if buf.len() != Self::SIZE {
            return Err(Error::ListLengthMismatch { expected: Self::SIZE, got: buf.len() });
        }

        // Verify method ID (first 4 bytes)
        let input_method = <[u8; 4]>::try_from(&buf[..4])
            .map_err(|_| Error::Custom("Invalid method ID format"))?;

        if input_method != Self::METHOD_ID {
            return Err(Error::Custom("not a cancel2 tx"));
        }

        // Convert remaining bytes to hash (32 bytes)
        let id = U256::from_be_bytes(
            <[u8; 32]>::try_from(&buf[4..4 + 32])
                .map_err(|_| Error::Custom("Failed to decode id"))?,
        );

        Ok(Self { id })
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct PaidTx {
    pub id: U256,
    pub tx_id: TxHash,
    pub tx_out: u32,
    pub amount: U256,
}

impl PaidTx {
    // paid(uint256 id,bytes32 txid,uint32 txout,uint256 paid)
    pub const METHOD_ID: [u8; 4] = [0xb6, 0x70, 0xab, 0x5e];

    pub const SIZE: usize = 132;
}

impl GoatTx for PaidTx {
    fn is_goat_tx(&self) -> bool {
        true
    }

    fn sender(&self) -> Address {
        RELAYER_EXECUTOR
    }

    fn contract(&self) -> Address {
        BRIDGE_CONTRACT
    }

    fn deposit(&self) -> Option<super::Mint> {
        None
    }

    fn withdraw(&self) -> Option<super::Mint> {
        None
    }
}

impl Decodable for PaidTx {
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
            return Err(Error::Custom("not a paid tx"));
        }

        // Parse id (32 bytes)
        i += 4;
        let id = U256::from_be_bytes(
            <[u8; 32]>::try_from(&buf[i..i + 32])
                .map_err(|_| Error::Custom("Failed to decode id"))?,
        );

        // Parse tx_id (32 bytes)
        i += 32;
        let tx_id = TxHash::from_slice(&buf[i..i + 32]);

        // Parse tx_out (last 4 bytes of 32-byte chunk)
        i += 32;
        let tx_out = u32::from_be_bytes(
            buf[i + 28..i + 32].try_into().map_err(|_| Error::Custom("Failed to decode tx_out"))?,
        );

        // Parse amount (32 bytes)
        i += 32;
        let amount = U256::from_be_bytes(
            <[u8; 32]>::try_from(&buf[i..i + 32])
                .map_err(|_| Error::Custom("Failed to decode amount"))?,
        );

        Ok(Self { id, tx_id, tx_out, amount })
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct NewBtcBlockTx {
    pub hash: TxHash,
}

impl NewBtcBlockTx {
    /// newBlockHash(bytes32 hash)
    pub const METHOD_ID: [u8; 4] = [0x94, 0xf4, 0x90, 0xbd];

    /// Tx size
    pub const SIZE: usize = 36;
}

impl GoatTx for NewBtcBlockTx {
    fn is_goat_tx(&self) -> bool {
        true
    }

    fn sender(&self) -> Address {
        RELAYER_EXECUTOR
    }

    fn contract(&self) -> Address {
        BITCOINT_CONTRACT
    }

    fn deposit(&self) -> Option<super::Mint> {
        None
    }

    fn withdraw(&self) -> Option<super::Mint> {
        None
    }
}

impl Decodable for NewBtcBlockTx {
    fn decode(buf: &mut &[u8]) -> Result<Self, Error> {
        // Verify input length
        if buf.len() != Self::SIZE {
            return Err(Error::ListLengthMismatch { expected: Self::SIZE, got: buf.len() });
        }

        // Verify method ID (first 4 bytes)
        let input_method = <[u8; 4]>::try_from(&buf[..4])
            .map_err(|_| Error::Custom("Invalid method ID format"))?;

        if input_method != Self::METHOD_ID {
            return Err(Error::Custom("not a newBlockHash tx"));
        }

        // Convert remaining bytes to hash (32 bytes)
        Ok(Self { hash: TxHash::from_slice(&buf[4..36]) })
    }
}
