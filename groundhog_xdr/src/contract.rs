// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unexpected_cfgs)]
#![allow(unused_variables)]

use crate::*;
use core::option::Option;
use core::ops::{Deref, DerefMut};

#[cfg(feature="std")]
use std::io::{Read, Write};

use ::xdrs::{ReadXDR, WriteXDR};


use ::serde::{Serialize, Deserialize};
use ::serde_arrays;



// clang-format on

pub type ContractBlob = BytesM;


#[derive(Clone, PartialEq, Debug)]
#[repr(i32)]
pub enum ContractType {
    kWasmi = 0,
    kNative = 1,
    kMockTest = 2,
}

#[repr(i32)]
#[derive(PartialEq, Debug, Clone)]
#[derive(ReadXDR, WriteXDR)]
pub enum Contract {
    // case kWasmi:
    //   ContractBlob wasmi_contract;
    kWasmi { wasmi_contract: ContractBlob } = ContractType::kWasmi as i32,
    // case kNative:
    //   u32 native_contract_id;
    kNative { native_contract_id: u32 } = ContractType::kNative as i32,
    // case kMockTest:
    //   u32 mock_contract_id;
    kMockTest { mock_contract_id: u32 } = ContractType::kMockTest as i32,
    // default:
    //   void;
    Default { contract_type: i32 },
}

#[derive(PartialEq, Debug, Clone, ReadXDR, WriteXDR)]
#[repr(C)]
pub struct ContractDeploymentHashPreimage {
    pub deployer_address: ContractAddress,
    pub nonce: SHA256Hash,
    pub contract_hash: SHA256Hash,
}

// This type must be a valid "identifier" as defined in RFC 4506,
// Section 6.2: "An identifier is a letter followed by an optional sequence
// of letters, digits, or underbar ('_')."  It must also obey Section 6.4,
// which lists keywords that may not be used as identifiers.
pub type Identifier = BytesM;

// This type must be a valid version number as defined at
// https://semver.org/.  For example, "0.1.3" or "1.0.0-alpha+001".
pub type SemVer = BytesM;

#[derive(PartialEq, Debug, Clone, ReadXDR, WriteXDR)]
#[repr(C)]
pub struct ContractInterfaceIDPreimage {
    pub name: Identifier,
    pub vers: SemVer,
}

// This type is computed as SHA256(ContractInterfaceIDPreimage)[0..6],
// concatenated with a two-byte "method id" defined by the author of
// the contract interface
// 
#[derive(PartialEq, Debug, Clone, Default, Eq, Hash, Copy, Serialize, Deserialize, ReadXDR, WriteXDR)]
#[repr(transparent)]
pub struct ContractEntrypointID {
    #[serde(with = "serde_arrays")]
    inner: [u8; 8],
}
impl From<[u8; 8]> for ContractEntrypointID {
    fn from(_inner: [u8; 8]) -> Self {
        Self { inner: _inner }
    }
}
impl ContractEntrypointID {
    pub fn into_inner(self) -> [u8; 8] {
        self.inner
    }
    pub const fn new(_inner: [u8; 8]) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for ContractEntrypointID {
    type Target = [u8; 8];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for ContractEntrypointID {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<[u8]> for ContractEntrypointID {
    fn as_ref(&self) -> &[u8] {
        self.deref().as_slice()
    }
}


//
// XDR internals below this point
//

impl Contract {
    pub fn contract_type(&self) -> i32 {
        match self {
            Contract::Default{ contract_type } => *contract_type ,

            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}


