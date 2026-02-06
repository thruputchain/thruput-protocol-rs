// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 8afc1e2714d6876f31ee25c4344341aefc1b9c5f

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
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(i32)]
pub enum GenesisContractAddresses {
    // Null address (reserved)
    kAddressNull = 0 as i32,

    // Precompiled native contract for the HogToken contract
    kAddressPrecompiledHogToken = 1 as i32,

    // InitialTokenClaim contract, for managing genesis token/contract setup
    kAddressInitialTokenClaim = 2 as i32,
}
impl TryFrom<i32> for GenesisContractAddresses {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::kAddressNull),
            1 => Ok(Self::kAddressPrecompiledHogToken),
            2 => Ok(Self::kAddressInitialTokenClaim),
            _ => Err(()),
        }
    }
}



//
// XDR internals below this point
//


