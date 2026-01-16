#![no_std]
#![allow(non_snake_case)]
#![allow(unused_imports)]

#[cfg(any(test, feature = "std"))]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

/// Core type definitions for the Groundhog protocol.
#[doc = include_str!("../docs/types.md")]
#[rustfmt::skip]
pub mod types;
use types::*;

pub mod impls;
use impls::*;

#[rustfmt::skip]
pub mod contract;
use contract::*;

#[rustfmt::skip]
pub mod contract_error;
use contract_error::*;

#[rustfmt::skip]
pub mod ffi_enums;

#[rustfmt::skip]
pub mod genesis;
use genesis::*;

#[rustfmt::skip]
pub mod retrocausal_queue;
use retrocausal_queue::*;

#[rustfmt::skip]
pub mod syscall_error;
use syscall_error::*;

#[rustfmt::skip]
pub mod storage_cost;
use storage_cost::*;

#[rustfmt::skip]
pub mod storage_delta_flags;
use storage_delta_flags::*;

#[rustfmt::skip]
pub mod transaction;
use transaction::*;

#[rustfmt::skip]
pub mod event;
use event::*;

#[rustfmt::skip]
pub mod standard_asset_events;
use standard_asset_events::*;

#[rustfmt::skip]
pub mod auth_claim;
use auth_claim::*;

#[rustfmt::skip]
pub mod exec_result;
use exec_result::*;

#[rustfmt::skip]
pub mod account;
use account::*;

#[cfg(feature = "syscalls")]
pub mod syscalls;
