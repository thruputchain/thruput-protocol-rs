// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 708ebf9bf535633eea730b810b2fb07494dca14e

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
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(i32)]
pub enum SystemType {
    SYSTEM_VALIDATOR = 1 as i32,
    SYSTEM_LOADGEN = 2 as i32,
}
impl TryFrom<i32> for SystemType {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::SYSTEM_VALIDATOR),
            2 => Ok(Self::SYSTEM_LOADGEN),
            _ => Err(()),
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(i32)]
pub enum UnitType {
    // validator
    UNIT_COORDINATOR = 1 as i32,

    UNIT_MEMPOOL = 2 as i32,
    UNIT_EXEC = 3 as i32,
    UNIT_DB = 4 as i32,
    UNIT_EVENT = 5 as i32,
    UNIT_INDEXER = 6 as i32,
    UNIT_AUTH = 7 as i32,

    // loadgen
    UNIT_LOADGEN = 100 as i32,
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
            6 => Ok(Self::UNIT_INDEXER),
            7 => Ok(Self::UNIT_AUTH),
            100 => Ok(Self::UNIT_LOADGEN),
            _ => Err(()),
        }
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
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(i32)]
pub enum ProcessStatus {
    PROCESS_RUNNING = 1 as i32,
    PROCESS_EXITED = 2 as i32,
    PROCESS_INTERRUPTED = 3 as i32,
}
impl TryFrom<i32> for ProcessStatus {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::PROCESS_RUNNING),
            2 => Ok(Self::PROCESS_EXITED),
            3 => Ok(Self::PROCESS_INTERRUPTED),
            _ => Err(()),
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[repr(i32)]
pub enum ProcessState {
    // case PROCESS_RUNNING:
    //   u32 pid;
    PROCESS_RUNNING { pid: u32 } = ProcessStatus::PROCESS_RUNNING as i32,
    // case PROCESS_EXITED:
    //   i32 exit;
    PROCESS_EXITED { exit: i32 } = ProcessStatus::PROCESS_EXITED as i32,
    // case PROCESS_INTERRUPTED:
    //   void;
    PROCESS_INTERRUPTED = ProcessStatus::PROCESS_INTERRUPTED as i32,
    
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


