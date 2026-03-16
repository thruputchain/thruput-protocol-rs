// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 443aa4e97f3a0684c47b4f27fa9b4cd32b74f7fb

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

// input

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct SystemType(pub i32);

impl SystemType {
    pub const SYSTEM_VALIDATOR: Self = Self(1);
    pub const SYSTEM_LOADGEN: Self = Self(2);
}
impl From<i32> for SystemType {
    fn from(v: i32) -> Self {
        Self(v)
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct UnitType(pub i32);

impl UnitType {
    // validator
    pub const UNIT_COORDINATOR: Self = Self(1);

    pub const UNIT_MEMPOOL: Self = Self(2);
    pub const UNIT_EXEC: Self = Self(3);
    pub const UNIT_DB: Self = Self(4);
    pub const UNIT_EVENT: Self = Self(5);
    pub const UNIT_INDEXER: Self = Self(6);
    pub const UNIT_AUTH: Self = Self(7);

    // loadgen
    pub const UNIT_LOADGEN: Self = Self(100);
}
impl From<i32> for UnitType {
    fn from(v: i32) -> Self {
        Self(v)
    }
}


pub type ChainID = StringM;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct Context {
    pub assembly_id: AssemblyID,
    pub chain_id: ChainID,
    pub uniquifier: StringM,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct UnitRole {
    pub unit_type: UnitType,
    pub unit_shard: StringM,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct UnitID {
    pub ctx: Context,
    pub host_id: HostID,
    pub unit_role: UnitRole,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct UnitBuild {
    pub pkg: StringM,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct UnitAgreement {
    pub meta: StringMap,
    pub endpoint: SocketAddressMap,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct UnitCommand {
    pub binary_file: Path,
    // for persistent application data; APP_ROOT/system_id/unit_type/unit_shard
    pub app_dir: Path,
    // for configs, logs and core dumps; VAR_ROOT/system_id/session_id/unit_type/unit_shard
    pub var_dir: Path,
    // for scratch; TMP_ROOT/system_id/session_id/unit_type/unit_shard
    pub tmp_dir: Path,
    // var_dir/unit_type-unit_shard.stdout
    pub stdout_file: Path,
    // var_dir/unit_type-unit_shard.stderr
    pub stderr_file: Path,
    // var_dir/unit_type-unit_shard.combined
    pub combined_file: Path,
    pub flag: VecM<Flag>,
    pub env: StringMap,
    // supporting files are placed in var_dir
    pub files: Files,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct ProcessStatus(pub i32);

impl ProcessStatus {
    pub const PROCESS_RUNNING: Self = Self(1);
    pub const PROCESS_EXITED: Self = Self(2);
    pub const PROCESS_INTERRUPTED: Self = Self(3);
}
impl From<i32> for ProcessStatus {
    fn from(v: i32) -> Self {
        Self(v)
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[repr(i32)]
pub enum ProcessState {
    // case PROCESS_RUNNING:
    //   u32 pid;
    PROCESS_RUNNING { pid: u32 } = ProcessStatus::PROCESS_RUNNING.0 as i32,
    // case PROCESS_EXITED:
    //   i32 exit;
    PROCESS_EXITED { exit: i32 } = ProcessStatus::PROCESS_EXITED.0 as i32,
    // case PROCESS_INTERRUPTED:
    //   void;
    PROCESS_INTERRUPTED = ProcessStatus::PROCESS_INTERRUPTED.0 as i32,
    
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ProcessTcpInfo {
    pub listening: VecM<SocketAddress>,
    pub connections: VecM<SocketConnection>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct UnitState {
    pub time: Timestamp,
    pub def: UnitDef,
    pub user: StringM,
    pub started: Timestamp,
    pub died: Option<Timestamp>,
    pub process_state: ProcessState,
    pub tcp_info: Option<ProcessTcpInfo>,
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


