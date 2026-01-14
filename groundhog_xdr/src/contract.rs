// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: b276e3d2f1e9a04f8d23893cd6efa71dc03a2e47

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

#[cfg(feature = "serde")]
use ::serde::{Serialize, Deserialize};
#[cfg(feature = "serde")]
use ::serde_arrays;



/// An opaque blob of WebAssembly code.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractBlob {
    pub wasm: BytesM,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(i32)]
pub enum ContractType {
    kWasmi = 0 as i32,
    kNative = 1 as i32,
    kMockTest = 2 as i32,
}
pub use ContractType::*;

impl TryFrom<i32> for ContractType {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::kWasmi),
            1 => Ok(Self::kNative),
            2 => Ok(Self::kMockTest),
            _ => Err(()),
        }
    }
}


/// A `Contract` is something that can be deployed and invoked onchain.
/// A contract can be deployed to a specific `ContractAddress`,
/// and then invoked at that address.
///
/// Groundhog currently supports three types of contracts:
/// - `kWasmi`: A contract that is implemented as a WASM blob.
/// - `kNative`: A contract that is implemented as a precompiled native contract.   
///   The primary example of a native contract is the one that implements the native token of the chain.
/// - `kMockTest`: A `kMockTest` contract is is never deployed on chain, 
///   but rather is created as part of the test harness when testing new contracts locally.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[repr(i32)]
pub enum Contract {
    // case kWasmi:
    //   ContractBlob wasmi_contract;
    kWasmi { wasmi_contract: ContractBlob } = ContractType::kWasmi as i32,
    // case kNative:
    //   i32 native_contract_id;
    kNative { native_contract_id: i32 } = ContractType::kNative as i32,
    // case kMockTest:
    //   i32 mock_contract_id;
    kMockTest { mock_contract_id: i32 } = ContractType::kMockTest as i32,
    
}

/// When one contract deploys another, the address of the resulting contract is the hash of this struct.
/// The nonce is an arbitrary value supplied by the calling contract.
/// Informally, this is the groundhog analogue of the Ethereum virtual machine's `CREATE2` opcode.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractDeploymentHashPreimage {
    pub deployer_address: ContractAddress,
    pub nonce: SHA256Hash,
    pub contract_hash: SHA256Hash,
}

/// This type must be a valid "identifier" as defined in RFC 4506,
/// Section 6.2: "An identifier is a letter followed by an optional sequence
/// of letters, digits, or underbar ('_')."  It must also obey Section 6.4,
/// which lists keywords that may not be used as identifiers.
pub type Identifier = StringM;

/// This type must be a valid version number as defined at
/// <https://semver.org/>.  For example, "0.1.3" or "1.0.0-alpha+001".
pub type SemVer = StringM;

/// The hash of this type is used to compute the first 6 bytes of contract interface method IDs.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractInterfaceIDPreimage {
    pub name: Identifier,
    pub vers: SemVer,
}

/// This is an 8-byte opaque type that is computed as SHA256(ContractInterfaceIDPreimage)[0..6],
/// concatenated with a two-byte "method id" defined by the author of
/// the contract interface
// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Default, Copy)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ContractEntrypointID {
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays"))]
    inner: [u8; 8 as usize],
}
impl From<[u8; 8 as usize]> for ContractEntrypointID {
    fn from(_inner: [u8; 8 as usize]) -> Self {
        Self { inner: _inner }
    }
}
impl ContractEntrypointID {
    pub fn into_inner(self) -> [u8; 8 as usize] {
        self.inner
    }
    pub const fn new(_inner: [u8; 8 as usize]) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for ContractEntrypointID {
    type Target = [u8; 8 as usize];

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
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}


