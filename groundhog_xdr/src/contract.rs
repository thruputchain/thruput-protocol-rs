// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 

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
#[cfg(feature="xdrs")]
use xdrs::xdr_repr::*;


#[cfg(not(feature="xdrs"))]
use xdr_common::*;



// clang-format on

pub type ContractBlob = BytesM;

#[cfg_attr(feature = "xdrs", derive(XDR, ReadXDR, WriteXDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[derive(Copy)]
#[repr(i32)]
pub enum ContractType {
    kWasmi = 0 as i32,
    kNative = 1 as i32,
    kMockTest = 2 as i32,
}pub use ContractType::*;


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "xdrs", derive(XDR, ReadXDR, WriteXDR))]
#[repr(i32)]
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "xdrs", derive(XDR, ReadXDR, WriteXDR))]
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "xdrs", derive(XDR, ReadXDR, WriteXDR))]
#[repr(C)]
pub struct ContractInterfaceIDPreimage {
    pub name: Identifier,
    pub vers: SemVer,
}

// This type is computed as SHA256(ContractInterfaceIDPreimage)[0..6],
// concatenated with a two-byte "method id" defined by the author of
// the contract interface
// 
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[derive(Default, Copy)]
#[cfg_attr(feature = "xdrs", derive(XDR, ReadXDR, WriteXDR))]
#[repr(transparent)]
pub struct ContractEntrypointID {
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
impl<'a> From<&'a [u8; 8]> for &'a ContractEntrypointID {
    fn from(arr: &[u8; 8]) -> Self {
        let ptr = arr as *const [u8; 8] as *const ContractEntrypointID;
        unsafe { &*ptr }
    }
}
#[cfg(feature = "xdrs")]
impl<'r> ReadXDR<'r> for &'r ContractEntrypointID {
    fn read_xdr<R: xdrs::Reader<'r>>(r: &mut R) -> xdrs::ReadResult<Self> {
        let arr: &[u8; 8] = xdrs::read_xdr(r)?;
        let val = Self::from(arr);
        Ok(val)
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


