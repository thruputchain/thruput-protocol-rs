// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 7c6215466921ff17dc1f31705e23af0f52ce2c08

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

#[derive(schemars::JsonSchema)]
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
#[cfg(feature = "std")]
impl core::fmt::Display for SystemType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::SYSTEM_VALIDATOR.0 => {
                write!(f, "SystemType::SYSTEM_VALIDATOR")
            },
            x if x == Self::SYSTEM_LOADGEN.0 => {
                write!(f, "SystemType::SYSTEM_LOADGEN")
            },
            _ => {
                write!(f, "SystemType({})", self.0)
            }
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
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
#[cfg(feature = "std")]
impl core::fmt::Display for UnitType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::UNIT_COORDINATOR.0 => {
                write!(f, "UnitType::UNIT_COORDINATOR")
            },
            x if x == Self::UNIT_MEMPOOL.0 => {
                write!(f, "UnitType::UNIT_MEMPOOL")
            },
            x if x == Self::UNIT_EXEC.0 => {
                write!(f, "UnitType::UNIT_EXEC")
            },
            x if x == Self::UNIT_DB.0 => {
                write!(f, "UnitType::UNIT_DB")
            },
            x if x == Self::UNIT_EVENT.0 => {
                write!(f, "UnitType::UNIT_EVENT")
            },
            x if x == Self::UNIT_INDEXER.0 => {
                write!(f, "UnitType::UNIT_INDEXER")
            },
            x if x == Self::UNIT_AUTH.0 => {
                write!(f, "UnitType::UNIT_AUTH")
            },
            x if x == Self::UNIT_LOADGEN.0 => {
                write!(f, "UnitType::UNIT_LOADGEN")
            },
            _ => {
                write!(f, "UnitType({})", self.0)
            }
        }
    }
}


pub type ChainID = StringM;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct Context {
    pub assembly_id: AssemblyID,
    pub chain_id: ChainID,
    pub uniquifier: StringM,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct UnitRole {
    pub unit_type: UnitType,
    pub unit_shard: StringM,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct UnitID {
    pub ctx: Context,
    pub host_id: HostID,
    pub unit_role: UnitRole,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct UnitBuild {
    pub pkg: StringM,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct UnitAgreement {
    pub meta: StringMap,
    pub endpoint: SocketAddressMap,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
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

#[derive(schemars::JsonSchema)]
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

#[derive(schemars::JsonSchema)]
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
#[cfg(feature = "std")]
impl core::fmt::Display for ProcessStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::PROCESS_RUNNING.0 => {
                write!(f, "ProcessStatus::PROCESS_RUNNING")
            },
            x if x == Self::PROCESS_EXITED.0 => {
                write!(f, "ProcessStatus::PROCESS_EXITED")
            },
            x if x == Self::PROCESS_INTERRUPTED.0 => {
                write!(f, "ProcessStatus::PROCESS_INTERRUPTED")
            },
            _ => {
                write!(f, "ProcessStatus({})", self.0)
            }
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
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

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ProcessTcpInfo {
    pub listening: VecM<SocketAddress>,
    pub connections: VecM<SocketConnection>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
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
    pub fn status(&self) -> ProcessStatus {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


