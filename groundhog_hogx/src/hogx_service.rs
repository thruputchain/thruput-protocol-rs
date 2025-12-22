// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 3557ba2b43fa86e8979c6b959082b8c54f8f5966

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
pub struct ServiceID {
    // sets HOGX_SYSTEM_ID; identifies a set of service instances comprising a system deployment
    pub system_id: BytesM,
    // sets HOGX_SERVICE_TYPE; e.g. `coordinator`, `mempool`, `exec`, `db`, `event`
    pub service_type: BytesM,
    // sets HOGX_SERVICE_SHARD
    pub service_shard: BytesM,
}

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ServiceBuild {
    // sets HOGX_PKG
    pub pkg: BytesM,
}

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ServiceAgreement {
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
pub struct ServiceDef {
    pub id: ServiceID,
    pub build: ServiceBuild,
    pub agreement: ServiceAgreement,
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
pub struct ServiceState {
    pub service_def: ServiceDef,
    pub started: Timestamp,
    pub stopped: Option<Timestamp>,
    // service and its child processes
    pub process_state: VecM<ProcessState>,
    pub stdout: Path,
    pub stderr: Path,
}


//
// XDR internals below this point
//


