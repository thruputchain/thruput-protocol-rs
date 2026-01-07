// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 011eecf37b9bb7cf7f9b0e6ae267ec8935a484b3

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
pub struct HostConfig {
    // HOGX_PUBLIC_IP
    pub public_ip: BytesM,
    // HOGX_PRIVATE_IP
    pub private_ip: BytesM,
    // HOGX_PKG_DIR
    pub pkg_dir: Path,
    // HOGX_APP_DIR; application lifetime: hog db, coordinator storage, event db
    pub app_dir: Path,
    // HOGX_VAR_DIR; host lifetime: configs, logs, core dumps
    pub var_dir: Path,
    // HOGX_TMP_DIR; process lifetime
    pub tmp_dir: Path,
}

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct HostState {
    pub config: HostConfig,
}


//
// XDR internals below this point
//


