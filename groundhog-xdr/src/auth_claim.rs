// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 8b242a5dc6ea5d82760cff0d3e5bc41a8c6aec6c

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unexpected_cfgs)]
#![allow(unused_variables)]
#![allow(clippy::unnecessary_cast)]

use crate::*;
use core::option::Option;
use core::ops::{Deref, DerefMut};

#[cfg(feature="std")]
use std::io::{Read, Write};

#[cfg(feature="xdrs")]
use xdrs::*;
#[cfg(feature="spec")]
use xdrs::xdr_repr::*;


#[cfg(not(feature="xdrs"))]
use xdr_common::*;

#[cfg(feature = "serde")]
use ::serde::{Serialize, Deserialize};
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
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(i32)]
pub enum AuthClaimType {
    /// Requires a signature from a specific key on the invoked contract address.
    ///
    /// This is the simplest and most common authorization pattern.
    kInvokedAddressSignature = 0 as i32,

    /// Requires a weighted threshold of signatures from the invoked contract.
    ///
    /// The bitmap specifies which keys are acceptable, and signatures are
    /// accumulated until the `required_weight` threshold is met.
    kInvokedAddressSignedWeight = 1 as i32,

    /// Requires a signature from a specific key on a different contract address.
    ///
    /// Used for cross-contract authorization (e.g., a contract authorizing
    /// operations on behalf of another contract).
    kOtherAddressSignature = 2 as i32,

    /// Requires a weighted threshold of signatures from a different contract.
    ///
    /// Combines cross-contract auth with multi-signature requirements.
    kOtherAddressSignedWeight = 3 as i32,
}
impl TryFrom<i32> for AuthClaimType {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::kInvokedAddressSignature),
            1 => Ok(Self::kInvokedAddressSignedWeight),
            2 => Ok(Self::kOtherAddressSignature),
            3 => Ok(Self::kOtherAddressSignedWeight),
            _ => Err(()),
        }
    }
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
pub enum AuthClaim {
    // case kInvokedAddressSignature:
    //   /// The ID of the key that must sign, from the invoked address's key set.
    //   KeyID invoked_address_signature_id;
    kInvokedAddressSignature { invoked_address_signature_id: KeyID } = AuthClaimType::kInvokedAddressSignature as i32,
    // case kInvokedAddressSignedWeight:
    //   invoked_address_signed_weight invoked_address_signed_weight;
    kInvokedAddressSignedWeight { invoked_address_signed_weight: AuthClaim_invoked_address_signed_weight } = AuthClaimType::kInvokedAddressSignedWeight as i32,
    // case kOtherAddressSignature:
    //   other_address_signature_id other_address_signature_id;
    kOtherAddressSignature { other_address_signature_id: AuthClaim_other_address_signature_id } = AuthClaimType::kOtherAddressSignature as i32,
    // case kOtherAddressSignedWeight:
    //   other_address_signed_weight other_address_signed_weight;
    kOtherAddressSignedWeight { other_address_signed_weight: AuthClaim_other_address_signed_weight } = AuthClaimType::kOtherAddressSignedWeight as i32,
    
}

/// A list of auth claims for a transaction.
///
/// Each transaction must include at least one auth claim to be valid.
/// The list is bounded by `kMaxAuthClaims` to limit verification complexity.
pub type AuthClaimData = VecM::<AuthClaim, kMaxAuthClaims>;


//
// XDR internals below this point
//

impl AuthClaim {
    pub fn claim_type(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}


