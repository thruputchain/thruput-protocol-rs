#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod common_types;
use common_types::*;

pub mod serialize_xdrs;
pub use serialize_xdrs::*;

#[rustfmt::skip]
pub mod types;
use types::*;

pub mod extras;
use extras::*;

// #[cfg(feature = "alloc")]
#[rustfmt::skip]
pub mod contract;
// #[cfg(feature = "alloc")]
use contract::*;

#[rustfmt::skip]
pub mod ffi_enums;

#[cfg(feature = "alloc")]
#[rustfmt::skip]
pub mod retrocausal_queue;
#[cfg(feature = "alloc")]
use retrocausal_queue::*;

#[cfg(feature = "alloc")]
#[rustfmt::skip]
pub mod transaction;
