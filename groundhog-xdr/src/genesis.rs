// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 61d57e8bc7352aa3310343599c50db9b92a3256c

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



// clang-format on

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct PresetDeploymentConfig {
    pub initial_nonce: SHA256Hash,
    pub contract_hash: SHA256Hash,
    pub entrypoint: ContractEntrypointID,
    pub calldata: Calldata,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct InitialConfigurationEntry {
    pub config: PresetDeploymentConfig,
    pub amount: int128_xdr,
}

pub type InitialConfigurationList = VecM::<InitialConfigurationEntry>;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct GenesisConfiguration {
    pub initial_config: InitialConfigurationList,
    pub storage_cost_model: StorageCostModelXdr,
}

// These translated into the contract addresses that are initialized
// at the start of the chain.
//
// The addresses (32 bytes) are all zeros, with the last 4 bytes equal
// to the enum value.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct GenesisContractAddresses(pub i32);

impl GenesisContractAddresses {
    // Null address (reserved)
    pub const kAddressNull: Self = Self(0);

    // Precompiled native contract for the HogToken contract
    pub const kAddressPrecompiledHogToken: Self = Self(1);

    // InitialTokenClaim contract, for managing genesis token/contract setup
    pub const kAddressInitialTokenClaim: Self = Self(2);
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
            x if x == Self::kAddressPrecompiledHogToken.0 => {
                write!(f, "GenesisContractAddresses::kAddressPrecompiledHogToken")
            },
            x if x == Self::kAddressInitialTokenClaim.0 => {
                write!(f, "GenesisContractAddresses::kAddressInitialTokenClaim")
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


