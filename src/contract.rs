// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unexpected_cfgs)]
#![allow(unused_variables)]

use core::option::Option;

use crate::*;

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

#[repr(C)]
#[derive(PartialEq, Debug, Default, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
pub struct ContractDeploymentHashPreimage {
    pub deployer_address: ContractAddress,
    pub nonce: SHA256Hash,
    pub contract_hash: SHA256Hash,
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


