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



//==============================================================================
// Address Authorization Data
//
// This module defines the authorization configuration stored at each address.
// This configuration data tells Groundhog whether or not a given transaction
// can begin execution in a given block.
//
// In Groundhog, every address can originate transactions and can be configured
// with multiple signing keys and weighted multi-signature requirements.
//
// TODO(geoff): give an example of what that might look like here.
//
// Unlike the common blockchain model (as in, for example, Ethereum), in which
// there is an explicit demarcation between so-called "externally owned addresses" (EOAs)
// that can initiate transactions but have no associated code, and "contract addresses",
// which hold invocable code but cannot initiate transactions,
// Groundhog treats every address uniformly: all addresses can both hold a contract
// and initiate a transaction.
//==============================================================================

//------------------------------------------------------------------------------
// Public Key Configuration
//------------------------------------------------------------------------------

/// Configuration for a single public key registered to an address.
///
/// Each address can have up to `kMaxSigners` public keys, each with an
/// associated weight for multi-signature authorization.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct PublicKeyConfig {
    /// The public key (supports multiple cryptographic schemes via union).
    pub key: PublicKey,
    /// The weight of this key in multi-signature calculations.
    ///
    /// When verifying weighted auth claims, weights from signing keys
    /// are accumulated and compared against the required threshold.
    pub max_weight: Weight,
}

/// Optional `PublicKeyConfig`, used in the `public_keys` array.
///
/// A null value indicates that key slot is unoccupied.
// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct PublicKeyConfigOpt {
    inner: Option<PublicKeyConfig>,
}
impl From<Option<PublicKeyConfig>> for PublicKeyConfigOpt {
    fn from(_inner: Option<PublicKeyConfig>) -> Self {
        Self { inner: _inner }
    }
}
impl PublicKeyConfigOpt {
    pub fn into_inner(self) -> Option<PublicKeyConfig> {
        self.inner
    }
    pub const fn new(_inner: Option<PublicKeyConfig>) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for PublicKeyConfigOpt {
    type Target = Option<PublicKeyConfig>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for PublicKeyConfigOpt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

//------------------------------------------------------------------------------
// Default Initialization Values
//------------------------------------------------------------------------------

/// Default required weight for a newly initialized address.
///
/// When an address is first created (e.g., from a public key hash),
/// the auth data is initialized with these defaults.
pub const kDefaultNewAddressRequiredWeight : u32 = 1;

/// Default accepted keys bitmap for a newly initialized address.
///
/// `0xFFFFFFFF` means all key slots are initially acceptable.
pub const kDefaultNewAddressAcceptedKeys : u32 = 0xFFFFFFFF;

/// Default key ID for the initial key of a newly initialized address.
///
/// The public key used to derive the address is installed as key 0.
pub const kDefaultNewAddressKeyID : u32 = 0;

//------------------------------------------------------------------------------
// Address Authorization Data
//------------------------------------------------------------------------------

/// The complete authorization configuration for an address.
///
/// This struct stores all the information needed to verify transactions
/// originating from an address:
/// - The set of public keys and their weights
/// - The minimum weight threshold required to authorize a transaction
///
/// When a transaction is submitted, its `AuthClaims` are verified against
/// this data to determine if the transaction is properly authorized.
///
/// This is the type stored at storage key `AddressAuthData`.
///
/// # TODO
///
/// (geoff) and (dobog): The initialization workflow (when upgrading
/// an address from just a pk to a full contract) should explicitly set the
/// auth data with sensible defaults, so that the default is NOT to accidentally
/// lock the address.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct AddressAuthData {
    /// Array of up to `kMaxSigners` public keys.
    ///
    /// Null entries indicate unoccupied key slots.
    pub public_keys: [PublicKeyConfigOpt; kMaxSigners as usize],
    /// The minimum total weight required from signing keys to authorize
    /// a transaction from this address.
    pub required_weight: Weight,
    /// The block height at which this auth data was last modified.
    ///
    /// # TODO
    ///
    /// (geoff): this can be removed once hog_db has a way to track this directly
    /// on each db item.
    pub last_written_height: BlockHeight,
}


//
// XDR internals below this point
//


