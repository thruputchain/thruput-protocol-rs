// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: a138999f82aee3194ca1e6460e1084adff7b8161

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


#[cfg(not(feature="xdrs"))]
use xdr_common::*;



// clang-format on

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct UnitID {
    // sets HOGX_SYSTEM_ID; identifies a set of unit instances comprising a system deployment
    pub system_id: BytesM,
    // sets HOGX_UNIT_TYPE; e.g. `coordinator`, `mempool`, `exec`, `db`, `event`
    pub unit_type: BytesM,
    // sets HOGX_UNIT_SHARD
    pub unit_shard: BytesM,
}

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct UnitBuild {
    // sets HOGX_PKG
    pub pkg: BytesM,
}

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct UnitAgreement {
    // can use env variables
    pub meta: VecM<StringMap>,
    // can use env variables; e.g. {"public-metrics": 178.10.0.3:2222}
    pub endpoint: VecM<AddressMap>,
}

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct Command {
    // can use env variables
    pub binary: Path,
    // can use env variables
    pub flag: VecM<Flag>,
    // can use env variables
    pub env: VecM<StringMap>,
}

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct UnitDef {
    pub id: UnitID,
    pub build: UnitBuild,
    pub agreement: UnitAgreement,
    pub cmd: Command,
}

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ProcessState {
    pub pid: i64,
    pub age: i64,
}

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct UnitState {
    pub unit_def: UnitDef,
    pub started: Timestamp,
    pub stopped: Option<Timestamp>,
    // unit and its child processes
    pub process_state: VecM<ProcessState>,
    pub stdout: Path,
    pub stderr: Path,
}


//
// XDR internals below this point
//


