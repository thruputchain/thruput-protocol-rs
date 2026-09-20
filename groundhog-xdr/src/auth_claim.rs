// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: e671a3694d7312d78364456bb86ee8a18de1502d

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


#[cfg(feature = "serde")]
use ::serde::{{Serialize, Deserialize}};
#[cfg(feature = "serde")]
use ::serde_arrays;



//==============================================================================
// Authentication Claims
//
// This module defines the authorization model for Groundhog transactions.
// Each transaction includes an AuthClaim that specifies which signatures
// are required to authorize the transaction's execution.
//
// Workflow:
// 1. A user constructs a Transaction, which includes an AuthClaim.
// 2. Signatures are added to create a SignedTransaction
// 3. Groundhog verifies that signatures match the AuthClaim (before execution)
// 4. During execution, contracts can read the AuthClaim and relevant compressed metadata,
// (such as which keys were used to authorize the transaction), but not the actual signatures.
//
// This design ensures transaction behavior depends only on the AuthClaim
// (which are included in what gets signed), not on the signature bytes themselves.
// This makes transactions signature-scheme agnostic and prevents signature
// reordering or re-randomization from changing transaction behavior.
//
// The authentication model supports two claim types:
// - Source address auth data: the transaction is signed by the keys in the source address's auth data.
//   and the account stores per-key weights and a minimum threshold
// - Pubkey signature: a standalone public key signs the transaction
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

//------------------------------------------------------------------------------
// Auth Claim Types
//------------------------------------------------------------------------------

/// Discriminator for the different types of authorization claims.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct AuthClaimType(pub i32);

impl AuthClaimType {
    /// Requires a set of signatures from the source address's auth data.
    /// The runtime checks whether there are sufficient signatures associated with the address
    /// to satisfy the address auth data's required weight.
    // TODO(geoff): consider surfacing the data about which keys signed in a syscall.
    pub const kSourceAddressAuthData: Self = Self(0);

    /// Requires a signature from a specific public key.
    ///
    /// Used for accounts identified by a pubkey preimage (pubkey + nonce).
    pub const kPubkeySignature: Self = Self(1);
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
            x if x == Self::kSourceAddressAuthData.0 => {
                write!(f, "AuthClaimType::kSourceAddressAuthData")
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
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct PubkeyAccountPreimage {
    pub pubkey: PublicKey,
    pub nonce: u64,
}

/// An authorization claim specifying what signatures are needed to authorize
/// a transaction.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct AuthClaim_source_address_claim {
    pub allowed_keys: BitMap,
    pub required_weight: Weight,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[repr(i32)]
#[non_exhaustive]
pub enum AuthClaim {
    /// The transaction is signed by the keys in the source address's auth data.
    /// The bitmap specifies which are allowed to be used to sign the transaction.
    // case kSourceAddressAuthData:
    //   source_address_claim source_address_claim;
    kSourceAddressAuthData { source_address_claim: AuthClaim_source_address_claim } = AuthClaimType::kSourceAddressAuthData.0 as i32,
    // case kPubkeySignature:
    //   PubkeyAccountPreimage pubkey_preimage;
    kPubkeySignature { pubkey_preimage: PubkeyAccountPreimage } = AuthClaimType::kPubkeySignature.0 as i32,
    
}

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
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
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


