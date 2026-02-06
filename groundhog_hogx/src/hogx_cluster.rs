// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 4cdeb628f95f47a7054bbf893ab281e195473e83

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

#[cfg(feature = "std")]
use std::io::{Read, Write};


use xdrs::*;
#[cfg(feature = "spec")]
use xdrs::xdr_repr::*;


#[cfg(feature = "serde")]
use ::serde::{Serialize, Deserialize};
#[cfg(feature = "serde")]
use ::serde_arrays;



// clang-format on

// -- host def

pub type HostID = StringM;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]
#[cfg_attr(feature = "spec", derive(XDR))]
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

// SystemPorts specifies service ports.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SystemPorts {
    // coordinator
    pub coordinator_public_port: u32,
    pub coordinator_public_wallet_port: u32,
    pub coordinator_private_port: u32,
    pub coordinator_private_metrics_port: u32,
    pub coordinator_scpd_local_port: u32,
    pub scpd_coordinator_local_port: u32,
    // mempool
    pub mempool_coordinator_private_port: u32,
    pub mempool_exec_private_port: u32,
    pub mempool_db_private_port: u32,
    pub mempool_event_private_port: u32,
    pub mempool_auth_private_port: u32,
    // exec
    pub exec_mempool_private_port: u32,
    // db
    pub db_exec_private_port: u32,
    pub db_mempool_private_port: u32,
    pub db_auth_private_port: u32,
    // event
    pub event_mempool_private_port: u32,
    pub event_exec_private_port: u32,
    // auth
    pub auth_db_private_port: u32,
    pub auth_mempool_private_port: u32,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ValidatorSystemConfig {
    pub ports: SystemPorts,
    pub exec_num_threads: u32,
    pub db_ram_bytes: u64,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct LoadgenSystemConfig {
    pub validator_config: ValidatorSystemConfig,
    //
    pub coordinator_private_debug_port: u32,
    pub loadgen_private_port: u32,
    //
    pub num_accounts: u64,
    pub txs_per_block: u64,
    pub num_blocks: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]
#[cfg_attr(feature = "spec", derive(XDR))]
#[repr(i32)]
pub enum SystemConfig {
    // case SYSTEM_VALIDATOR:
    //   ValidatorSystemConfig validator_config;
    SYSTEM_VALIDATOR { validator_config: ValidatorSystemConfig } = SystemType::SYSTEM_VALIDATOR as i32,
    // case SYSTEM_LOADGEN:
    //   LoadgenSystemConfig loadgen_config;
    SYSTEM_LOADGEN { loadgen_config: LoadgenSystemConfig } = SystemType::SYSTEM_LOADGEN as i32,
    
}

// LayoutUnit maps (unit_type, unit_shard) to a host.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct LayoutUnit {
    pub unit_role: UnitRole,
    pub host_id: HostID,
}

pub type SystemLayout = VecM::<LayoutUnit>;

pub type AssemblyID = StringM;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SystemDef {
    pub id: AssemblyID,
    pub config: SystemConfig,
    pub layout: SystemLayout,
}

// -- cluster config

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ClusterConfig {
    pub host_defs: VecM<HostDef>,
    pub system_defs: VecM<SystemDef>,
}


//
// XDR internals below this point
//

impl SystemConfig {
    pub fn system_type(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}


