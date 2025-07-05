use core::mem;

use alloy_eips::{
    eip2718::GOAT_TX_TYPE_ID, eip2930::AccessList, eip7702::SignedAuthorization, Typed2718,
};
use alloy_primitives::{Address, Bytes, ChainId, Signature, TxKind, B256, U256};
use alloy_rlp::{BufMut, Decodable, Encodable};

pub use super::goat_types::*;
use super::{RlpEcdsaDecodableTx, RlpEcdsaEncodableTx};
use crate::{SignableTransaction, Transaction, TxType};

const GOAT_CHAIN_ID: u64 = 2345;
 #[cfg(feature = "goat-testnet")]
const GOAT_TESTNET_CHAIN_ID: u64 = 48816;

/// A transaction with a priority fee (Goat).
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[doc(alias = "GoatTransaction", alias = "TransactionGoat", alias = "GoatTx")]
pub struct TxGoat {
    pub module: Module,
    pub action: Action,
    /// A scalar value equal to the number of transactions sent by the sender; formally Tn.
    #[cfg_attr(feature = "serde", serde(with = "alloy_serde::quantity"))]
    pub nonce: u64,
    /// An unlimited size byte array specifying the
    /// input data of the message call, formally Td.
    pub input: Bytes,

    #[cfg_attr(feature = "serde", serde(skip))]
    pub inner: TxGoatInner,

    #[cfg_attr(feature = "serde", serde(skip))]
    pub chain_id: u64,
}

impl TxGoat {
    /// Get the transaction type.
    #[doc(alias = "transaction_type")]
    pub const fn tx_type() -> TxType {
        TxType::Goat
    }

    pub fn sender(&self) -> Address {
        self.inner.sender()
    }

    pub fn to(&self) -> Address {
        self.inner.to()
    }

    pub fn deposit(&self) -> Option<Mint> {
        self.inner.deposit()
    }

    pub fn withdraw(&self) -> Option<Mint> {
        self.inner.withdraw()
    }

    /// Calculates a heuristic for the in-memory size of the [TxGoat] transaction.
    #[inline]
    pub fn size(&self) -> usize {
        mem::size_of::<Module>() + // module
        mem::size_of::<Action>() + // action
        mem::size_of::<u64>() + // nonce
        self.input.len() // input
    }

    pub fn decode_tx(&mut self) -> alloy_rlp::Result<TxGoatInner> {
        let buf = &mut self.input.as_ref();
        decode_goat_tx(self.module, self.action, buf)
    }
}

impl RlpEcdsaEncodableTx for TxGoat {
    /// Outputs the length of the transaction's fields, without a RLP header.
    #[doc(hidden)]
    fn rlp_encoded_fields_length(&self) -> usize {
        self.module.length() + self.action.length() + self.nonce.length() + self.input.0.length()
    }

    fn rlp_encode_fields(&self, out: &mut dyn alloy_rlp::BufMut) {
        self.module.encode(out);
        self.action.encode(out);
        self.nonce.encode(out);
        self.input.0.encode(out);
    }
}

impl RlpEcdsaDecodableTx for TxGoat {
    const DEFAULT_TX_TYPE: u8 = { Self::tx_type() as u8 };

    fn rlp_decode_fields(buf: &mut &[u8]) -> alloy_rlp::Result<Self> {
        let module = Decodable::decode(buf)?;
        let action = Decodable::decode(buf)?;
        let nonce = Decodable::decode(buf)?;
        let input = Bytes(Decodable::decode(buf)?);

        let buf = &mut input.0.as_ref();
        let inner = decode_goat_tx(module, action, buf)?;

        Ok(Self { module, action, nonce, input, inner, chain_id: GOAT_CHAIN_ID })
    }
}

impl Transaction for TxGoat {
    #[inline]
    fn chain_id(&self) -> Option<ChainId> {
        Some(
            #[cfg(feature = "goat-testnet")]
            GOAT_TESTNET_CHAIN_ID,
            #[cfg(not(feature = "goat-testnet"))]
            GOAT_CHAIN_ID,
        )
    }

    #[inline]
    fn nonce(&self) -> u64 {
        self.nonce
    }

    #[inline]
    fn gas_limit(&self) -> u64 {
        0
    }

    #[inline]
    fn gas_price(&self) -> Option<u128> {
        Some(0)
    }

    #[inline]
    fn max_fee_per_gas(&self) -> u128 {
        0
    }

    #[inline]
    fn max_priority_fee_per_gas(&self) -> Option<u128> {
        Some(0)
    }

    #[inline]
    fn max_fee_per_blob_gas(&self) -> Option<u128> {
        Some(0)
    }

    #[inline]
    fn priority_fee_or_price(&self) -> u128 {
        0
    }

    fn effective_gas_price(&self, _base_fee: Option<u64>) -> u128 {
        0
    }

    #[inline]
    fn is_dynamic_fee(&self) -> bool {
        false
    }

    #[inline]
    fn kind(&self) -> TxKind {
        self.inner.to().into()
    }

    #[inline]
    fn is_create(&self) -> bool {
        false
    }

    #[inline]
    fn value(&self) -> U256 {
        U256::ZERO
    }

    #[inline]
    fn input(&self) -> &Bytes {
        &self.input
    }

    #[inline]
    fn access_list(&self) -> Option<&AccessList> {
        None
    }

    #[inline]
    fn blob_versioned_hashes(&self) -> Option<&[B256]> {
        None
    }

    #[inline]
    fn authorization_list(&self) -> Option<&[SignedAuthorization]> {
        None
    }

    #[inline]
    fn caller(&self) -> Option<Address> {
        Some(self.sender())
    }

    #[inline]
    fn module(&self) -> Option<Module> {
        Some(self.module)
    }

    #[inline]
    fn action(&self) -> Option<Action> {
        Some(self.action)
    }

    #[inline]
    fn deposit(&self) -> Option<Mint> {
        self.deposit()
    }

    #[inline]
    fn withdraw(&self) -> Option<Mint> {
        self.withdraw()
    }

    #[inline]
    fn is_goat_tx(&self) -> bool {
        true
    }
}

impl SignableTransaction<Signature> for TxGoat {
    fn set_chain_id(&mut self, chain_id: ChainId) {
        self.chain_id = chain_id;
    }

    fn encode_for_signing(&self, out: &mut dyn alloy_rlp::BufMut) {
        out.put_u8(GOAT_TX_TYPE_ID);
        self.encode(out)
    }

    fn payload_len_for_signature(&self) -> usize {
        self.length() + 1
    }
}

impl Typed2718 for TxGoat {
    fn ty(&self) -> u8 {
        TxType::Goat as u8
    }
}

impl Encodable for TxGoat {
    fn encode(&self, out: &mut dyn BufMut) {
        self.rlp_encode(out);
    }

    fn length(&self) -> usize {
        self.rlp_encoded_length()
    }
}

impl Decodable for TxGoat {
    fn decode(data: &mut &[u8]) -> alloy_rlp::Result<Self> {
        Self::rlp_decode(data)
    }
}

/// Bincode-compatible [`TxGoat`] serde implementation.
#[cfg(all(feature = "serde", feature = "serde-bincode-compat"))]
pub(super) mod serde_bincode_compat {
    use alloc::borrow::Cow;
    use alloy_primitives::Bytes;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use serde_with::{DeserializeAs, SerializeAs};

    use super::TxGoatInner;
    use crate::transaction::goat_types::*;

    /// Bincode-compatible [`super::TxGoat`] serde implementation.
    ///
    /// Intended to use with the [`serde_with::serde_as`] macro in the following way:
    /// ```rust
    /// use alloy_consensus::{serde_bincode_compat, TxGoat};
    /// use serde::{Deserialize, Serialize};
    /// use serde_with::serde_as;
    ///
    /// #[serde_as]
    /// #[derive(Serialize, Deserialize)]
    /// struct Data {
    ///     #[serde_as(as = "serde_bincode_compat::transaction::TxGoat")]
    ///     transaction: TxGoat,
    /// }
    /// ```
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TxGoat<'a> {
        module: Module,
        action: Action,
        nonce: u64,
        input: Cow<'a, Bytes>,
        #[serde(skip)]
        pub inner: TxGoatInner,
        #[serde[skip]]
        pub chain_id: u64,
    }

    impl<'a> From<&'a super::TxGoat> for TxGoat<'a> {
        fn from(value: &'a super::TxGoat) -> Self {
            Self {
                module: value.module,
                action: value.action,
                nonce: value.nonce,
                input: Cow::Borrowed(&value.input),
                inner: value.inner.clone(),
                chain_id: value.chain_id,
            }
        }
    }

    impl<'a> From<TxGoat<'a>> for super::TxGoat {
        fn from(value: TxGoat<'a>) -> Self {
            Self {
                module: value.module,
                action: value.action,
                nonce: value.nonce,
                input: value.input.into_owned(),
                inner: value.inner,
                chain_id: value.chain_id,
            }
        }
    }

    impl SerializeAs<super::TxGoat> for TxGoat<'_> {
        fn serialize_as<S>(source: &super::TxGoat, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            TxGoat::from(source).serialize(serializer)
        }
    }

    impl<'de> DeserializeAs<'de, super::TxGoat> for TxGoat<'de> {
        fn deserialize_as<D>(deserializer: D) -> Result<super::TxGoat, D::Error>
        where
            D: Deserializer<'de>,
        {
            let mut tx: super::TxGoat = TxGoat::deserialize(deserializer).map(Into::into)?;
            tx.inner = tx.decode_tx().map_err(serde::de::Error::custom)?;
            Ok(tx)
        }
    }

    #[cfg(test)]
    mod tests {
        use arbitrary::Arbitrary;
        use bincode::config;
        use rand::Rng;
        use serde::{Deserialize, Serialize};
        use serde_with::serde_as;

        use super::super::{serde_bincode_compat, TxGoat};

        #[test]
        fn test_tx_goat_bincode_roundtrip() {
            #[serde_as]
            #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
            struct Data {
                #[serde_as(as = "serde_bincode_compat::TxGoat")]
                transaction: TxGoat,
            }

            let mut bytes = [0u8; 1024];
            rand::thread_rng().fill(bytes.as_mut_slice());
            let data = Data {
                transaction: TxGoat::arbitrary(&mut arbitrary::Unstructured::new(&bytes)).unwrap(),
            };

            let encoded = bincode::serde::encode_to_vec(&data, config::legacy()).unwrap();
            let (decoded, _) =
                bincode::serde::decode_from_slice::<Data, _>(&encoded, config::legacy()).unwrap();
            assert_eq!(decoded, data);
        }
    }
}

#[cfg(all(test, feature = "k256"))]
mod tests {
    use super::*;
    use alloy_primitives::{hex, U256};

    #[test]
    fn test_decode_btc_tx() {
        let mut btc_tx = TxGoat {
            module: BIRDGE_MODULE,
            action: BITCOIN_NEW_BLOCK_ACTION,
            input: hex!("94f490bdbb7ba5e4830730dfa97c1eaaf199a8ef8ea2a865ca44c600fa032772a7af9edc")
                .into(),
            ..Default::default()
        };

        let decoded = btc_tx.decode_tx().unwrap();
        if let TxGoatInner::NewBtcBlockTx(tx) = decoded {
            assert_eq!(
                tx.hash.to_string(),
                "0xbb7ba5e4830730dfa97c1eaaf199a8ef8ea2a865ca44c600fa032772a7af9edc".to_string()
            );
        } else {
            panic!();
        }
    }

    #[test]
    fn test_decode_cancel2_tx() {
        let mut cancel2_tx = TxGoat {
            module: BIRDGE_MODULE,
            action: BRIDGE_CANCEL2_ACTION,
            input: hex!("c19dd32000000000000000000000000000000000000000000000000000000000c64ab11e")
                .into(),
            ..Default::default()
        };

        let decoded = cancel2_tx.decode_tx().unwrap();
        if let TxGoatInner::Cancel2Tx(tx) = decoded {
            assert_eq!(tx.id, U256::from_str_radix("c64ab11e", 16).unwrap());
        } else {
            panic!();
        }
    }

    #[test]
    fn test_decode_deposit_tx() {
        let mut deposit_tx =  TxGoat {
            module: BIRDGE_MODULE,
            action: BRIDGE_DEPOIT_ACTION,
            input: hex!("904183cb15bb90fa63b9a92e31d31f8d8d30bf8da9d9a21314c65dd517f27740ae676d6e000000000000000000000000000000000000000000000000000000002a71a7780000000000000000000000005e4e4d79f08120352f04d638adec7d3892b2804500000000000000000000000000000000000000000000000000000000157f7f970000000000000000000000000000000000000000000000000000000000000064").into(),
            ..Default::default()
        };

        let decoded = deposit_tx.decode_tx().unwrap();
        if let TxGoatInner::DepositTx(tx) = decoded {
            assert_eq!(
                tx.tx_id.to_string(),
                "0x15bb90fa63b9a92e31d31f8d8d30bf8da9d9a21314c65dd517f27740ae676d6e".to_string()
            );
            assert_eq!(tx.tx_out, 0x2a71a778);
            assert_eq!(
                tx.target.to_string().to_lowercase(),
                "0x5e4e4d79f08120352f04d638adec7d3892b28045".to_string()
            );
            assert_eq!(tx.amount, U256::from_str_radix("157f7f97", 16).unwrap());
            assert_eq!(tx.tax, U256::from_str_radix("100", 10).unwrap());
        } else {
            panic!();
        }
    }

    #[test]
    fn test_decode_paid_tx() {
        let mut paid_tx =  TxGoat {
            module: BIRDGE_MODULE,
            action: BRIDGE_PAID_ACTION,
            input: hex!("b670ab5e00000000000000000000000000000000000000000000000000000000fe171e2553b11234d8e3e2c9066afe89364da7315eefd30b28430715a56a08d5905365110000000000000000000000000000000000000000000000000000000032cc827f00000000000000000000000000000000000000000000000000000000ba606dcd").into(),
            ..Default::default()
        };

        let decoded = paid_tx.decode_tx().unwrap();
        if let TxGoatInner::PaidTx(tx) = decoded {
            assert_eq!(tx.id, U256::from_str_radix("fe171e25", 16).unwrap());
            assert_eq!(
                tx.tx_id.to_string(),
                "0x53b11234d8e3e2c9066afe89364da7315eefd30b28430715a56a08d590536511".to_string()
            );
            assert_eq!(tx.tx_out, 0x32cc827f);
            assert_eq!(tx.amount, U256::from_str_radix("ba606dcd", 16).unwrap());
        } else {
            panic!();
        }
    }

    #[test]
    fn test_decode_unlock_tx() {
        let mut unlock_tx =  TxGoat {
            module: LOCKING_MODULE,
            action: LOCKING_COMPLETE_UNLOCK_ACTION,
            input: hex!("00aba51a00000000000000000000000000000000000000000000000000000000000000640000000000000000000000005b38da6a701c568545dcfcb03fcb875f56beddc400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001").into(),
            ..Default::default()
        };

        let decoded = unlock_tx.decode_tx().unwrap();
        if let TxGoatInner::CompleteUnlockTx(tx) = decoded {
            assert_eq!(tx.id, 100);
            assert_eq!(
                tx.recipient.to_string(),
                "0x5B38Da6a701c568545dCfcB03FcB875f56beddC4".to_string()
            );
            assert_eq!(
                tx.token.to_string(),
                "0x0000000000000000000000000000000000000000".to_string()
            );
            assert_eq!(tx.amount, U256::from_str_radix("1", 10).unwrap());
        } else {
            panic!();
        }
    }

    #[test]
    fn test_decode_reward_tx() {
        let mut reward_tx =  TxGoat {
            module: LOCKING_MODULE,
            action: LOCKING_DISTRIBUTE_REWARD_ACTION,
            input: hex!("bd9fadb500000000000000000000000000000000000000000000000000000000000000020000000000000000000000009ae387acdafe4b9d315d0bb56b06ab91f31b967000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000064").into(),
            ..Default::default()
        };

        let decoded = reward_tx.decode_tx().unwrap();
        if let TxGoatInner::DistributeRewardTx(tx) = decoded {
            assert_eq!(tx.id, 2);
            assert_eq!(
                tx.recipient.to_string().to_lowercase(),
                "0x9ae387acdafe4b9d315d0bb56b06ab91f31b9670".to_string()
            );
            assert_eq!(tx.goat, U256::from(1));
            assert_eq!(tx.gas_reward, U256::from(100));
        } else {
            panic!();
        }
    }
}
