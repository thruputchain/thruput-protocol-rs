// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 14959ad52a564640d44d7079849fb9f146d330c0

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



// clang-format on

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct GenesisConfiguration {
    pub network_hash: NetworkHash,
    pub storage_cost_model: StorageCostModelXdr,
    /// The execution parameters block 0 runs under, in canonical form. Carried
    /// into the DbTip at genesis load and forward by every later commit.
    pub network_config: NetworkConfig,
    pub root_contract: Contract,
    /// The contract resolved for root invocations of undeployed addresses.
    /// Committed to by `ProposalHeader::config_hash` through
    /// `ConfigHashPreimage`, so a node cannot run under a different one than
    /// the chain agreed.
    pub default_wallet: DeployedContractHash,
}

// These translated into the contract addresses that are initialized
// at the start of the chain.
//
// The addresses (32 bytes) are all zeros, with the last 4 bytes equal
// to the enum value.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct GenesisContractAddresses(pub i32);

impl GenesisContractAddresses {
    // Null address (reserved)
    pub const kAddressNull: Self = Self(0);

    // Precompiled native-token contract
    pub const kAddressNativeToken: Self = Self(1);

    // Genesis root contract
    pub const kAddressGenesisRoot: Self = Self(2);
}
impl From<i32> for GenesisContractAddresses {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for GenesisContractAddresses {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::kAddressNull.0 => {
                write!(f, "GenesisContractAddresses::kAddressNull")
            },
            x if x == Self::kAddressNativeToken.0 => {
                write!(f, "GenesisContractAddresses::kAddressNativeToken")
            },
            x if x == Self::kAddressGenesisRoot.0 => {
                write!(f, "GenesisContractAddresses::kAddressGenesisRoot")
            },
            _ => {
                write!(f, "GenesisContractAddresses({})", self.0)
            }
        }
    }
}



//
// XDR internals below this point
//


