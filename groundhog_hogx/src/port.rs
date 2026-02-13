// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 3a6c032b7e60f6306f2d9ddf20916b59acba429b

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


// Common definitions of default ports used within hogd

pub const kExecutorMempoolServerPort : u32 = 8040;

pub const kAuthDbServerPort : u32 = 8050;

pub const kAuthMempoolServerPort : u32 = 8051;

pub const kIndexerEventServerPort : u32 = 8060;

pub const kIndexerBlockExplorerServerPort : u32 = 8061;

pub const kMempoolCoordinatorServerPort : u32 = 8070;

pub const kMempoolExecutorServerPort : u32 = 8071;

pub const kMempoolDbServerPort : u32 = 8072;

pub const kMempoolEventServerPort : u32 = 8073;

pub const kMempoolAuthServerPort : u32 = 8074;

pub const kDbExecutorServerPort : u32 = 8080;

pub const kDbMempoolServerPort : u32 = 8081;

pub const kDbAuthServerPort : u32 = 8082;

pub const kEventExecutorServerPort : u32 = 8090;

pub const kEventMempoolServerPort : u32 = 8091;

pub const kEventIndexerServerPort : u32 = 8092;

pub const kCIControlServerPort : u32 = 11111;


//
// XDR internals below this point
//


