// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 7205267353bfdfe0f30403515b2c0aa330a72344

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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct HostConfig {
    // HOGX_PUBLIC_IP
    pub public_ip: StringM,
    // HOGX_PRIVATE_IP
    pub private_ip: StringM,
    pub agent_port: u32,
    // HOGX_PKG_DIR
    pub pkg_dir: Path,
    // HOGX_APP_DIR; application lifetime: hog db, coordinator storage, event db
    pub app_dir: Path,
    // HOGX_VAR_DIR; host lifetime: configs, logs, core dumps
    pub var_dir: Path,
    // HOGX_TMP_DIR; process lifetime
    pub tmp_dir: Path,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct HostState {
    pub config: HostConfig,
}


//
// XDR internals below this point
//


