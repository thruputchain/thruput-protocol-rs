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

/// The consensus-critical execution parameters a block executes under.
///
/// Every parameter is a named field, so a configuration is structurally
/// complete: it cannot omit one, repeat one, or order them differently. The
/// protocol blesses no value for any of them either -- a chain states all of
/// them at genesis, so there is nothing a node can silently inherit and no way
/// for two node versions to disagree about what is in force.
///
/// Adding a parameter means adding a field here and an arm to
/// [`ConfigUpgrade`].
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct NetworkConfig {
    pub ext: ExtensionPoint,
    /// Concurrently open incremental SHA-256 buffers permitted per invocation
    /// frame (`SHA256Update`, `SHA256Copy`, `SHA256Finalize`).
    pub max_sha256_buffers: u32,
    /// Invocation frames one transaction may have open at once, counting every
    /// invoke rather than only the savepoint-pushing ones that
    /// `kMaxSavepointDepth` bounds. Each open frame holds a live contract
    /// runtime, so this is what stops a recursive call chain from growing until
    /// the executor runs out of room for it.
    pub max_cross_invoke_depth: u32,
    /// Contracts one transaction may attach in `contracts_to_deploy`.
    ///
    /// Counted over what is attached rather than what a contract goes on to
    /// deploy: the executor holds every attached contract for the whole
    /// transaction, and the ones deployed are a subset chosen during execution.
    /// Attaching the same contract twice is rejected outright, so the count is
    /// over distinct contracts either way.
    ///
    /// Together with the per-contract blob size cap this is what bounds the
    /// transaction's final commit batch, which the executor sends to storage as
    /// a single message. A chain must set it low enough that the largest batch
    /// it admits still fits that message; hogd rejects a genesis that does not.
    pub max_new_contracts_per_tx: u32,
}

/// Names one consensus-critical configuration parameter.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct ConfigParamID(pub i32);

impl ConfigParamID {
    pub const kConfigParamMaxSHA256Buffers: Self = Self(0);
    pub const kConfigParamMaxCrossInvokeDepth: Self = Self(1);
    pub const kConfigParamMaxNewContractsPerTx: Self = Self(2);
}
impl From<i32> for ConfigParamID {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for ConfigParamID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::kConfigParamMaxSHA256Buffers.0 => {
                write!(f, "ConfigParamID::kConfigParamMaxSHA256Buffers")
            },
            x if x == Self::kConfigParamMaxCrossInvokeDepth.0 => {
                write!(f, "ConfigParamID::kConfigParamMaxCrossInvokeDepth")
            },
            x if x == Self::kConfigParamMaxNewContractsPerTx.0 => {
                write!(f, "ConfigParamID::kConfigParamMaxNewContractsPerTx")
            },
            _ => {
                write!(f, "ConfigParamID({})", self.0)
            }
        }
    }
}


/// A change to one network configuration parameter.
///
/// Consensus carries these in `ConsensusValue::upgrades`, so a block that
/// changes several parameters carries one per parameter and everything it does
/// not name keeps the value it already has. The encoding is 8 bytes, well
/// inside the `Upgrade` that holds it.
///
/// There is no `default` arm on purpose: a node that cannot name a parameter
/// cannot apply a change to it, so an unrecognized id must fail to decode
/// rather than decode to something the node then ignores.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[repr(i32)]
#[non_exhaustive]
pub enum ConfigUpgrade {
    // case kConfigParamMaxSHA256Buffers:
    //   u32 max_sha256_buffers;
    kConfigParamMaxSHA256Buffers { max_sha256_buffers: u32 } = ConfigParamID::kConfigParamMaxSHA256Buffers.0 as i32,
    // case kConfigParamMaxCrossInvokeDepth:
    //   u32 max_cross_invoke_depth;
    kConfigParamMaxCrossInvokeDepth { max_cross_invoke_depth: u32 } = ConfigParamID::kConfigParamMaxCrossInvokeDepth.0 as i32,
    // case kConfigParamMaxNewContractsPerTx:
    //   u32 max_new_contracts_per_tx;
    kConfigParamMaxNewContractsPerTx { max_new_contracts_per_tx: u32 } = ConfigParamID::kConfigParamMaxNewContractsPerTx.0 as i32,
    
}

/// The network configuration committed to by ProposalHeader::config_hash.
/// New consensus-critical configuration belongs in this preimage.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ConfigHashPreimage {
    pub storage_cost_model: StorageCostModelXdr,
    pub network_config: NetworkConfig,
}


//
// XDR internals below this point
//

impl ConfigUpgrade {
    pub fn param(&self) -> ConfigParamID {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


