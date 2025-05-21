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
use ::serde::{Serialize, Deserialize};
use ::serde_arrays;

pub use crate::byte_array::*;

#[cfg(feature="std")]
use std::io::{Read, Write};




// clang-format on

pub type ContractBlob = VecM<u8, 0xffffffff>;


#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature="std", derive(ReadXdrEnum,WriteXdrEnum))]
#[repr(i32)]
pub enum ContractType {
    kWasmi = 0,
    kNative = 1,
    kMockTest = 2,
}

#[repr(i32)]
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
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
    default { contract_type: i32 },
}

#[derive(PartialEq, Debug, Default, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
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
pub type Identifier = BytesM<0xffffffff>;

// This type must be a valid version number as defined at
// https://semver.org/.  For example, "0.1.3" or "1.0.0-alpha+001".
pub type SemVer = BytesM<0xffffffff>;

#[derive(PartialEq, Debug, Default, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
#[repr(C)]
pub struct ContractInterfaceIDPreimage {
    pub name: Identifier,
    pub vers: SemVer,
}

#[derive(PartialEq, Debug, Default, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
#[repr(C)]
pub struct ContractEntrypointID {
    // This field is computed as SHA256(ContractInterfaceIDPreimage)[0..6]
    pub contract_interface_id: ByteArray<6>,
    // The meaning of this field is defined by the contract-interface author
    pub contract_method_id: ByteArray<2>,
}


//
// XDR internals below this point
//

impl Contract {
    pub fn contract_type(&self) -> i32 {
        match self {
            Contract::default{ contract_type } => *contract_type ,

            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}


