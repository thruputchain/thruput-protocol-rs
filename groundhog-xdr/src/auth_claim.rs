// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 1e1e39628b29553aba80b0ca0698248ea45c2bdf

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::unnecessary_cast)]

use crate::*;
use core::option::Option;
use core::ops::{Deref, DerefMut};

#[cfg(feature = "std")]
use std::io::{Read, Write};
#[cfg(feature = "xdrs")]
use xdrs::*;
#[cfg(feature = "spec")]
use xdrs::xdr_repr::*;


#[cfg(feature = "serde")]
use ::serde::{{Serialize, Deserialize}};
#[cfg(feature = "serde")]
use ::serde_arrays;



//==============================================================================
// Authentication Claims
//
// This module defines the authorization model for Groundhog transactions.
// Each transaction includes a set of AuthClaims that specify which signatures
// are required to authorize the transaction's execution.
//
// Workflow:
// 1. A user constructs a Transaction, which includes some number of AuthClaims.
// 2. Signatures are added to create a SignedTransaction
// 3. Groundhog verifies that signatures match the auth claims (before execution)
// 4. During execution, contracts can read the auth claims but NOT the signatures
//
// This design ensures transaction behavior depends only on the auth claims
// (which are included in what gets signed), not on the signature bytes themselves.
// This makes transactions signature-scheme agnostic and prevents signature
// reordering or re-randomization from changing transaction behavior.
//
// The authentication model supports:
// - Single-signature authorization (one specific key must sign)
// - Multi-signature authorization (weighted threshold of keys must sign)
// - Cross-contract authorization (signatures from other contract addresses)
//
// Auth claims reference AddressAuthData stored at each contract address,
// which defines the available keys and their weights.
//==============================================================================

//------------------------------------------------------------------------------
// Basic Types
//------------------------------------------------------------------------------

/// Represents the weight of a signature key in multi-signature schemes.
///
/// Weights are accumulated from signing keys and compared against a required
/// threshold to determine if an auth claim is satisfied.
// uint32_t
pub type Weight = u32;

/// A bitmap representing a set of key IDs.
///
/// Each bit position corresponds to a key ID (bit 0 = key 0, bit 1 = key 1, etc.).
/// Used to specify which keys are acceptable for satisfying an auth claim.
pub type BitMap = u32;

/// Identifier for a public key within an address's key set.
///
/// Valid range is 0-31 (must fit in one byte for CXS encoding).
/// This is why `kMaxSigners` is constrained to be <= 31.
pub type KeyID = u32;

//------------------------------------------------------------------------------
// Constants
//------------------------------------------------------------------------------

/// Maximum number of signers that can be configured on an address.
///
/// This matches Stellar's `MAX_SIGNERS` (20) plus 1 for the primary key.
/// Must be <= 31 to fit in the `BitMap` representation.
// 
pub const kMaxSigners : u32 = 21;

/// Maximum number of gas permission configurations per address.
///
/// Multiple configs allow separation of concerns (e.g., sponsored gas vs
/// contract owner management operations).
// 
pub const kMaxGasConfigs : u32 = 5;

/// Maximum number of auth claims allowed in a single transaction.
///
/// This limits the verification complexity of a transaction.
pub const kMaxAuthClaims : u32 = 10;

//------------------------------------------------------------------------------
// Auth Claim Types
//------------------------------------------------------------------------------

/// Discriminator for the different types of authorization claims.
///
/// Claims are categorized by:
/// 1. Whether they reference the invoked contract or another contract
/// 2. Whether they require a single signature or weighted multi-sig
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct AuthClaimType(pub i32);

impl AuthClaimType {
    /// Requires a signature from a specific key on the invoked contract address.
    ///
    /// This is the simplest and most common authorization pattern.
    pub const kInvokedAddressSignature: Self = Self(0);

    /// Requires a weighted threshold of signatures from the invoked contract.
    ///
    /// The bitmap specifies which keys are acceptable, and signatures are
    /// accumulated until the `required_weight` threshold is met.
    pub const kInvokedAddressSignedWeight: Self = Self(1);

    /// Requires a signature from a specific key on a different contract address.
    ///
    /// Used for cross-contract authorization (e.g., a contract authorizing
    /// operations on behalf of another contract).
    pub const kOtherAddressSignature: Self = Self(2);

    /// Requires a weighted threshold of signatures from a different contract.
    ///
    /// Combines cross-contract auth with multi-signature requirements.
    pub const kOtherAddressSignedWeight: Self = Self(3);

    /// Requires a signature from a specific key
    ///
    /// XXX
    pub const kPubkeySignature: Self = Self(4);
}
impl From<i32> for AuthClaimType {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for AuthClaimType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::kInvokedAddressSignature.0 => {
                write!(f, "AuthClaimType::kInvokedAddressSignature")
            },
            x if x == Self::kInvokedAddressSignedWeight.0 => {
                write!(f, "AuthClaimType::kInvokedAddressSignedWeight")
            },
            x if x == Self::kOtherAddressSignature.0 => {
                write!(f, "AuthClaimType::kOtherAddressSignature")
            },
            x if x == Self::kOtherAddressSignedWeight.0 => {
                write!(f, "AuthClaimType::kOtherAddressSignedWeight")
            },
            x if x == Self::kPubkeySignature.0 => {
                write!(f, "AuthClaimType::kPubkeySignature")
            },
            _ => {
                write!(f, "AuthClaimType({})", self.0)
            }
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct PubkeyAccountPreimage {
    pub pubkey: PublicKey,
    pub nonce: u64,
}

/// An authorization claim specifying what signatures are needed to authorize
/// a transaction.
///
/// Each transaction includes one or more auth claims. During verification,
/// the signatures in the `SignedTransaction` are matched against these claims
/// to determine if the transaction is properly authorized.
///
/// The four variants support different authorization patterns:
/// - Single key from invoked contract (most common)
/// - Multi-sig from invoked contract (for high-security operations)
/// - Single key from other contract (for delegation)
/// - Multi-sig from other contract (for complex delegated operations)
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct AuthClaim_invoked_address_signed_weight {
    /// Bitmap of acceptable keys (bit `i` = 1 means key `i` can contribute).
    pub accepted_keys: BitMap,
    /// Total weight required from the accepted keys' signatures.
    pub required_weight: Weight,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct AuthClaim_other_address_signature_id {
    /// The contract address whose key must sign.
    pub address: ContractAddress,
    /// The ID of the key from that address's key set.
    pub key_id: KeyID,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct AuthClaim_other_address_signed_weight {
    /// The contract address whose keys must sign.
    pub address: ContractAddress,
    /// Bitmap of acceptable keys from that address's key set.
    pub accepted_keys: BitMap,
    /// Total weight required from the accepted keys' signatures.
    pub required_weight: Weight,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[repr(i32)]
#[non_exhaustive]
pub enum AuthClaim {
    /// The ID of the key that must sign, from the invoked address's key set.
    // case kInvokedAddressSignature:
    //   KeyID invoked_address_signature_id;
    kInvokedAddressSignature { invoked_address_signature_id: KeyID } = AuthClaimType::kInvokedAddressSignature.0 as i32,
    // case kInvokedAddressSignedWeight:
    //   invoked_address_signed_weight invoked_address_signed_weight;
    kInvokedAddressSignedWeight { invoked_address_signed_weight: AuthClaim_invoked_address_signed_weight } = AuthClaimType::kInvokedAddressSignedWeight.0 as i32,
    // case kOtherAddressSignature:
    //   other_address_signature_id other_address_signature_id;
    kOtherAddressSignature { other_address_signature_id: AuthClaim_other_address_signature_id } = AuthClaimType::kOtherAddressSignature.0 as i32,
    // case kOtherAddressSignedWeight:
    //   other_address_signed_weight other_address_signed_weight;
    kOtherAddressSignedWeight { other_address_signed_weight: AuthClaim_other_address_signed_weight } = AuthClaimType::kOtherAddressSignedWeight.0 as i32,
    // case kPubkeySignature:
    //   PubkeyAccountPreimage pubkey_preimage;
    kPubkeySignature { pubkey_preimage: PubkeyAccountPreimage } = AuthClaimType::kPubkeySignature.0 as i32,
    
}

/// A list of auth claims for a transaction.
///
/// Each transaction must include at least one auth claim to be valid.
/// The list is bounded by `kMaxAuthClaims` to limit verification complexity.
pub type AuthClaimData = VecM::<AuthClaim, kMaxAuthClaims>;

//------------------------------------------------------------------------------
// Auth Verification Error Codes
//------------------------------------------------------------------------------

/// Error codes returned when verifying authorization on a SignedTransaction.
///
/// There are three levels of verification:
/// 1. Well-formedness: basic structural checks (e.g., key IDs are in range)
/// 2. Claims validity: signatures match claimed keys and weights are satisfied
/// 3. Signature verification: cryptographic validation of all signatures
///
/// These error codes distinguish between different failure modes to enable
/// appropriate handling at different stages of transaction processing.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct AuthVerifyErrorCode(pub i32);

impl AuthVerifyErrorCode {
    /// The transaction's auth claims are malformed.
    ///
    /// Transactions with this error should never be flooded over the network
    /// as they are structurally invalid. Examples include:
    /// - Public key IDs that are out of bounds
    /// - Invalid bitmaps (bits set beyond kMaxSigners)
    /// - Mismatched signature counts
    pub const kAuthMalformed: Self = Self(1);

    /// Cryptographic signature verification failed.
    ///
    /// This indicates that one or more signatures did not pass cryptographic
    /// validation against the claimed public keys. The transaction structure
    /// is valid but the signatures themselves are incorrect.
    pub const kSignatureVerificationFailed: Self = Self(2);

    /// Mismatch between auth claims and actual auth data on an account.
    ///
    /// This indicates that the transaction references keys or permissions
    /// that do not exist or do not satisfy the requirements. Examples include:
    /// - Referencing a public key ID that doesn't exist in the auth data
    /// - Failing to meet required weight thresholds
    /// - Insufficient gas permissions for the claimed sponsorship
    pub const kAuthDataMismatch: Self = Self(3);
}
impl From<i32> for AuthVerifyErrorCode {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for AuthVerifyErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::kAuthMalformed.0 => {
                write!(f, "AuthVerifyErrorCode::kAuthMalformed")
            },
            x if x == Self::kSignatureVerificationFailed.0 => {
                write!(f, "AuthVerifyErrorCode::kSignatureVerificationFailed")
            },
            x if x == Self::kAuthDataMismatch.0 => {
                write!(f, "AuthVerifyErrorCode::kAuthDataMismatch")
            },
            _ => {
                write!(f, "AuthVerifyErrorCode({})", self.0)
            }
        }
    }
}



//
// XDR internals below this point
//

impl AuthClaim {
    pub fn claim_type(&self) -> AuthClaimType {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


