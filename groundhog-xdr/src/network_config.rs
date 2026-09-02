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

/// The parameters a chain created before `max_new_contracts_per_tx` existed
/// states. Field meanings are the ones documented on [`NetworkConfigV1`].
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct NetworkConfigV0 {
    pub max_sha256_buffers: u32,
    pub max_cross_invoke_depth: u32,
}

/// The parameters a chain states as of `max_new_contracts_per_tx`.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct NetworkConfigV1 {
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

/// The consensus-critical execution parameters a block executes under.
///
/// Every parameter is a named field, so a configuration is structurally
/// complete: it cannot omit one, repeat one, or order them differently. The
/// protocol blesses no value for any of them either -- a chain states all of
/// them at genesis, so there is nothing a node can silently inherit and no way
/// for two node versions to disagree about what is in force.
///
/// Versioned as a union rather than extended in place. XDR is positional, so a
/// field appended to a struct changes the layout of everything embedding it,
/// and a chain created under the older layout could no longer be read. Each arm
/// instead states the parameters in force in full, and a chain keeps the arm it
/// was created with until an upgrade moves it.
///
/// The discriminant occupies the four bytes the leading `ExtensionPoint` used
/// to, so a configuration written before this union existed decodes as `v0`
/// byte for byte -- which is what that extension point was reserved for (see
/// `ExtensionPoint` in types.x).
///
/// Adding a parameter means adding an arm here that carries the previous arm's
/// fields plus the new one, along with a [`ConfigParamID`] and a
/// [`ConfigUpgrade`] arm. A node that meets an arm it cannot name must fail to
/// decode rather than guess which parameters are in force.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[repr(i32)]
#[non_exhaustive]
pub enum NetworkConfig {
    // case 0:
    //   NetworkConfigV0 v0;
    _0 { v0: NetworkConfigV0 } = 0 as i32,
    // case 1:
    //   NetworkConfigV1 v1;
    _1 { v1: NetworkConfigV1 } = 1 as i32,
    
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

impl NetworkConfig {
    pub fn v(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl ConfigUpgrade {
    pub fn param(&self) -> ConfigParamID {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


