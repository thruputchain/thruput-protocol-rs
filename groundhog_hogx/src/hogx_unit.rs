// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 5bee6dc970fe45e481c12f19fa8c1ea7f649cb75

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

#[cfg(feature = "serde")]
use ::serde::{Serialize, Deserialize};
#[cfg(feature = "serde")]
use ::serde_arrays;



// clang-format on

// input

pub type SystemID = StringM;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(i32)]
pub enum UnitType {
    UNIT_COORDINATOR = 1 as i32,
    UNIT_MEMPOOL = 2 as i32,
    UNIT_EXEC = 3 as i32,
    UNIT_DB = 4 as i32,
    UNIT_EVENT = 5 as i32,
}
impl TryFrom<i32> for UnitType {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::UNIT_COORDINATOR),
            2 => Ok(Self::UNIT_MEMPOOL),
            3 => Ok(Self::UNIT_EXEC),
            4 => Ok(Self::UNIT_DB),
            5 => Ok(Self::UNIT_EVENT),
            _ => Err(()),
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct UnitRole {
    // sets HOGX_UNIT_TYPE
    pub unit_type: UnitType,
    // sets HOGX_UNIT_SHARD
    pub unit_shard: StringM,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct UnitID {
    // sets HOGX_UNIT_SYSTEM_ID; identifies a set of unit instances comprising a system deployment
    pub system_id: SystemID,
    pub unit_role: UnitRole,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct UnitBuild {
    // sets HOGX_UNIT_PKG
    pub pkg: StringM,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct UnitAgreement {
    // can use link variables
    pub meta: StringMap,
    // can use link variables
    pub endpoint: StringMap,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct UnitCommand {
    // can use link variables
    pub binary: Path,
    // can use link variables
    pub flag: VecM<Flag>,
    // can use link variables
    pub env: StringMap,
    // config files land in var_dir; can use link variables
    pub files: Files,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct UnitDef {
    pub id: UnitID,
    pub build: UnitBuild,
    // can use link variables
    pub agreement: UnitAgreement,
    // can use link variables
    pub command: UnitCommand,
}

// output: linked definitions

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct UnitAgreementLinked {
    pub meta: StringMap,
    pub endpoint: SocketAddressMap,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct UnitCommandLinked {
    pub binary_file: Path,
    // holds persistent application data; does not depend on system_id
    pub app_dir: Path,
    // holds configs, logs and core dumps; mounted as /var; depends on system_id
    pub var_dir: Path,
    // mounted as /tmp; depends on system_id
    pub tmp_dir: Path,
    // resides in var_dir
    pub stdout_file: Path,
    // resides in var_dir
    pub stderr_file: Path,
    pub flag: VecM<Flag>,
    pub env: StringMap,
    // config files placed in var_dir
    pub files: Files,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct UnitDefLinked {
    pub id: UnitID,
    pub build: UnitBuild,
    pub agreement: UnitAgreementLinked,
    pub command: UnitCommandLinked,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(i32)]
pub enum ProcessStatus {
    PROCESS_RUNNING = 1 as i32,
    PROCESS_EXITED = 2 as i32,
}
impl TryFrom<i32> for ProcessStatus {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::PROCESS_RUNNING),
            2 => Ok(Self::PROCESS_EXITED),
            _ => Err(()),
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[repr(i32)]
pub enum ProcessState {
    // case PROCESS_RUNNING:
    //   u32 pid;
    PROCESS_RUNNING { pid: u32 } = ProcessStatus::PROCESS_RUNNING as i32,
    // case PROCESS_EXITED:
    //   i32 exit;
    PROCESS_EXITED { exit: i32 } = ProcessStatus::PROCESS_EXITED as i32,
    
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct UnitState {
    pub time: Timestamp,
    pub def: UnitDef,
    pub def_linked: UnitDefLinked,
    pub started: Timestamp,
    pub process_state: ProcessState,
}


//
// XDR internals below this point
//

impl ProcessState {
    pub fn status(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}


