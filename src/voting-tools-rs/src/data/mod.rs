use bigdecimal::BigDecimal;
use microtype::microtype;
use serde::{Deserialize, Serialize};

use self::crypto::{PublicKeyHex, SignatureHex};

pub mod crypto;

/// The source of voting power for a given registration
///
/// The voting power can either come from:
///  - a single wallet, OR
///  - a set of delegations
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
#[derive(Debug, Clone, PartialEq)]
pub enum VotingPowerSource {
    /// Direct voting. String should contain catalyst identifier
    Legacy(VotingKeyHex),

    /// Delegated one. Collection of catalyst identifiers joined it weights
    Delegated(Vec<(VotingKeyHex, u32)>),
}

/// A catalyst registration on Cardano in either CIP-15 or CIP-36 format
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Registration {
    #[serde(rename = "1")]
    pub voting_power_source: VotingPowerSource,
    #[serde(rename = "2")]
    pub stake_key: StakeKeyHex,
    #[serde(rename = "3")]
    pub rewards_address: RewardsAddress,
    // note, this must be monotonically increasing. Typically, the current slot
    // number is used
    #[serde(rename = "4")]
    pub nonce: Nonce,
    #[serde(rename = "5")]
    #[serde(default)]
    pub voting_purpose: VotingPurpose,
}

/// A signature for a registration as defined in CIP-15
///
/// This is compatible with CIP-36 registrations
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Signature {
    /// The actual signature
    ///
    /// CIP-15 specifies this must be a field, so an extra layer of nesting is required
    #[serde(rename = "1")]
    pub inner: SignatureHex,
}

/// A Catalyst registration in either CIP-15 or CIP-36 format, along with its signature
///
/// The signature is generated by:
///  - CBOR encoding the registration as a single entry map with a key of `61284` and a value of
///  the registration
///  - Hashing the bytes of the CBOR encoding with blake2b_256
///  - Signing the hash with the public key
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SignedRegistration {
    /// The registration
    pub registration: Registration,
    /// The signature
    pub signature: Signature,

    /// The id of the transaction that created this registration
    pub tx_id: TxId,
}

/// Single output element of voting tools calculations
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotEntry {
    /// Registration content
    pub voting_power_source: VotingPowerSource,

    /// Mainnet rewards address
    pub rewards_address: RewardsAddress,

    /// Stake public key
    pub stake_key: StakeKeyHex,

    /// Voting power expressed in ada
    ///
    /// This is computed from `voting_power_source`
    ///
    /// If
    pub voting_power: BigDecimal,

    /// Voting purpose
    ///
    /// Catalyst expects the voting purpose is set to `0`
    pub voting_purpose: VotingPurpose,

    /// Registration transaction id
    pub tx_id: TxId,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Reg { 
    pub tx_id: TxId,
    pub metadata: Registration,
    pub signature: Signature,
}

// Create newtype wrappers for better type safety
microtype! {

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub PublicKeyHex {
        StakeKeyHex,
        VotingKeyHex,
    }


    #[derive(Debug, PartialEq, Clone)]
    #[string]
    pub String {
        /// Database name
        DbName,
        /// Database user
        DbUser,
        /// Database host
        DbHost,
        StakeAddr,
        StakePubKey,
    }

    #[secret]
    #[string]
    pub String {
        /// Database password
        DbPass,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
    #[int]
    pub u64 {
        #[cfg_attr(test, derive(test_strategy::Arbitrary))]
        Nonce,
        /// A slot number
        #[cfg_attr(test, derive(test_strategy::Arbitrary))]
        SlotNo,
        VotingPurpose,
        TxId,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub Vec<u8> {
        /// A rewards address in a catalyst registation
        ///
        /// This type deliberately does not enforce a particular format for addresses, since the
        /// spec only requires this field to be a byte array, with no other constraints
        RewardsAddress,
    }
}

impl SlotNo {
    /// Attempt to convert this to an `i64`
    ///
    /// Returns none if the underlying `u64` doesn't fit into an `i64`
    pub fn into_i64(self) -> Option<i64> {
        self.0.try_into().ok()
    }
}
