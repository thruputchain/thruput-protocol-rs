#![no_std]
#![allow(non_snake_case)]
#![allow(unused_imports)]

#[cfg(any(test, feature = "std"))]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(not(feature = "xdrs"))]
pub use xdr_common::*;

pub mod hex;
use hex::*;

/// Core type definitions for the Groundhog protocol.
#[doc = include_str!("../docs/types.md")]
#[rustfmt::skip]
pub mod types;
use types::*;

#[doc(hidden)]
pub mod impls;
use impls::*;

/// Contract deployment, execution, and interface management.
#[doc = include_str!("../docs/contract.md")]
#[rustfmt::skip]
pub mod contract;
use contract::*;

#[rustfmt::skip]
pub mod genesis;
use genesis::*;

#[rustfmt::skip]
pub mod retrocausal_queue;
use retrocausal_queue::*;

/// Syscall error codes returned by system calls.
#[rustfmt::skip]
pub mod syscall_error;
use syscall_error::*;

#[rustfmt::skip]
pub mod storage_cost;
use storage_cost::*;

/// Transaction construction, signing, and parallel execution with nondeterministic replay.
#[doc = include_str!("../docs/transaction.md")]
#[rustfmt::skip]
pub mod transaction;
use transaction::*;

/// Event emission, indexing, and storage for contract-emitted logs.
#[doc = include_str!("../docs/event.md")]
#[rustfmt::skip]
pub mod event;
use event::*;

#[rustfmt::skip]
pub mod standard_asset_events;
use standard_asset_events::*;

/// Transaction authorization claims and signature verification.
#[doc = include_str!("../docs/auth_claim.md")]
#[rustfmt::skip]
pub mod auth_claim;
use auth_claim::*;

#[rustfmt::skip]
pub mod exec_result;
use exec_result::*;

/// Address authorization configuration and multi-signature management.
#[doc = include_str!("../docs/address_auth.md")]
#[rustfmt::skip]
pub mod address_auth;
use address_auth::*;

/// System call bindings and syscall numbers (SyscallNo enum).
#[cfg(feature = "syscalls")]
pub mod syscalls;

#[doc = include_str!("../docs/storage_item.md")]
#[rustfmt::skip]
pub mod storage_item;
use storage_item::*;
