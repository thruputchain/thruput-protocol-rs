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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct HostDef {
    pub host_id: StringM,
    pub host_tags: VecM<Tag>,
    //
    pub ssh_user: StringM,
    pub ssh_address: StringM,
    pub ssh_port: Option<u32>,
    pub ssh_private_key: StringM,
    //
    pub host_config: HostConfig,
}

// -- system layouts

// LayoutUnit maps a unit_type:unit_shard to a cluster host.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct LayoutUnit {
    pub unit_role: UnitRole,
    pub host_id: StringM,
}

// LayoutPorts specifies service ports.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct LayoutPorts {
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

// LayoutConfig provides services operational parameters.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct LayoutConfig {
    pub exec_num_threads: u32,
    pub db_ram_bytes: u64,
}

// LayoutDef maps all system units to cluster hosts.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct LayoutDef {
    pub layout_id: StringM,
    pub system_type: SystemType,
    pub layout_units: VecM<LayoutUnit>,
    pub layout_ports: LayoutPorts,
    pub layout_config: LayoutConfig,
}

// -- cluster config

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ClusterConfig {
    pub host_defs: VecM<HostDef>,
    pub layout_defs: VecM<LayoutDef>,
}


//
// XDR internals below this point
//


