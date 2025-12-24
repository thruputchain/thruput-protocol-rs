// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 4ebd3046fbb9768c5b28ea399f4e59957cfef5fe

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

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct GasPermissionConfig {
    pub limit: GasAmount,
    pub accepted_keys: BitMap,
    pub required_weight: Weight,
}

// If an address is initialized as a new address (via the hash of a public key).,
// we will interpret the missing auth data following these sensible defaults.
// we additionally interpret that initial key as key 0 with weight 1.
// TODO(geoff) and TODO(dobog): The initialization workflow (when upgrading an address from just a pk to a full contract)
// should explicitly set the auth data with these defaults (maybe optionally something else),
// so that the default is NOT to accidentally lock the address.

pub const kDefaultNewAddressRequiredWeight : u32 = 1;

pub const kDefaultNewAddressAcceptedKeys : u32 = 0xFFFFFFFF;

pub const kDefaultNewAddressKeyID : u32 = 0;

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct NonceConfig {
    pub accepted_keys: BitMap,
    pub required_weight: Weight,
}

// 
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PublicKeyConfigOpt {
    inner: Option<PublicKeyConfig>,
}
impl From<Option<PublicKeyConfig>> for PublicKeyConfigOpt {
    fn from(_inner: Option<PublicKeyConfig>) -> Self {
        Self { inner: _inner }
    }
}
impl PublicKeyConfigOpt {
    pub fn into_inner(self) -> Option<PublicKeyConfig> {
        self.inner
    }
    pub const fn new(_inner: Option<PublicKeyConfig>) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for PublicKeyConfigOpt {
    type Target = Option<PublicKeyConfig>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for PublicKeyConfigOpt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct AccountAuthData {
    pub public_keys: [PublicKeyConfigOpt; kMaxSigners as usize],
    pub gas_permissions: VecM<GasPermissionConfig, kMaxGasConfigs>,
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


