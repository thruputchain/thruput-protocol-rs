// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 98c0bc7253fa32253c7d4b8c722da75c3a5e0350

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


#[cfg(feature = "serde")]
use ::serde::{Serialize, Deserialize};
#[cfg(feature = "serde")]
use ::serde_arrays;



// clang-format on

// -- host def

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct HostDef {
    pub host_id: StringM,
    pub host_tags: VecM<Tag>,
    //
    pub ssh_user: StringM,
    pub ssh_address: StringM,
    pub ssh_private_key: StringM,
    //
    pub host_config: HostConfig,
}

// -- system layouts

// LayoutUnit maps a unit_type:unit_shard to a cluster host.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct LayoutUnit {
    pub unit_role: UnitRole,
    pub host_id: StringM,
}

// LayoutDef maps all system units to cluster hosts.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct LayoutDef {
    pub layout_id: StringM,
    pub layout_units: VecM<LayoutUnit>,
}

// -- cluster config

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
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


