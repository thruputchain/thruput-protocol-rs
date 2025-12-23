// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 3e16fec198246fef8b0deaab118d9f65fe1c6a9e

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



#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct PublicKeyConfig {
    // union multiplexing over signature scheme (ed25519,...)
    pub key: PublicKey,
    pub max_weight: Weight,
}

// If we use the bitmap approach below, should be less than the width of
// the bitmap.
// This constant is intended to match the MAX_SIGNERS constant in the Stellar protocol,
// which is 20.  We add 1 to account for the primary key on a Stellar account.
pub const MAX_SIGNERS : u32 = 21;

// Could be 1; Slightly useful to have multiple for sponsored gas,
// i.e. separating sponsor ops vs contract owner management
pub const MAX_GAS_CONFIGS : u32 = 5;

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct GasPermissionConfig {
    pub limit: GasAmount,
    pub accepted_keys: BitMap,
    pub required_weight: Weight,
}

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct NonceConfig {
    pub accepted_keys: BitMap,
    pub required_weight: Weight,
}

pub type PublicKeyConfigOpt = Option::<PublicKeyConfig>;

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct AccountAuthData {
    pub public_keys: [PublicKeyConfigOpt; MAX_SIGNERS],
    pub gas_permissions: VecM<GasPermissionConfig, MAX_GAS_CONFIGS>,
    pub nonce_config: NonceConfig,
    // TODO(geoff): this can be removed once hogdb has a way to track this directly
    // on each db item.
    pub last_written_height: BlockHeight,
}

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct DeployedContractHash {
    pub hash: SHA256Hash,
    // This field is technically redundant, but allows us to compute (and
    // update) storage costs without (re)loading the entire contract from
    // storage
    pub contract_size: u32,
    // This field is used as an annotation hint in the executor, to determine which 
    // type of runtime to instantiate.
    // Again, is redundant with the stored contract, but allows us to avoid loading the
    // whole contract from storage.
    pub contract_type: ContractType,
}

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct AddressMetadata {
    pub deployed_contract: DeployedContractHash,
}


//
// XDR internals below this point
//


