// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 6d7793493815cb08a683232d5b391d635721c369

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
use xdrs::*;



#[cfg(feature = "serde")]
use ::serde::{{Serialize, Deserialize}};
#[cfg(feature = "serde")]
use ::serde_arrays;



// clang-format on

// -- host def

pub type HostID = StringM;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct HostDef {
    pub host_id: HostID,
    pub host_tags: VecM<Tag>,
    //
    pub ssh_user: StringM,
    pub ssh_address: StringM,
    pub ssh_port: Option<u32>,
    pub ssh_private_key: StringM,
    //
    pub host_config: HostConfig,
}

// -- system definition and layouts

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ValidatorSystemConfig {
    pub base_private_port: u32,
    pub coordinator_public_port: u32,
    //
    pub exec_num_threads: u32,
    pub db_ram_bytes: u64,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct LoadgenSystemConfig {
    pub validator_config: ValidatorSystemConfig,
    //
    pub num_accounts: u64,
    pub txs_per_block: u64,
    pub num_blocks: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum SystemConfig {
    // case SYSTEM_VALIDATOR:
    //   ValidatorSystemConfig validator_config;
    SYSTEM_VALIDATOR { validator_config: ValidatorSystemConfig } = SystemType::SYSTEM_VALIDATOR.0 as i32,
    // case SYSTEM_LOADGEN:
    //   LoadgenSystemConfig loadgen_config;
    SYSTEM_LOADGEN { loadgen_config: LoadgenSystemConfig } = SystemType::SYSTEM_LOADGEN.0 as i32,
    
}

// LayoutUnit maps (unit_type, unit_shard) to a host.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct LayoutUnit {
    pub unit_role: UnitRole,
    pub host_id: HostID,
}

pub type SystemLayout = VecM::<LayoutUnit>;

pub type AssemblyID = StringM;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct AssemblyDef {
    pub id: AssemblyID,
    pub config: SystemConfig,
    pub layout: SystemLayout,
}

// -- cluster config

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ClusterConfig {
    pub host_defs: VecM<HostDef>,
    pub assembly_defs: VecM<AssemblyDef>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct VariantPlan {
    pub unit_defs: VecM<UnitDef>,
    pub public_address: SocketAddress,
}


//
// XDR internals below this point
//

impl SystemConfig {
    pub fn system_type(&self) -> SystemType {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


