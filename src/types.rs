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

#[repr(C)]
#[derive(PartialEq, Debug, Default, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr))]
#[cfg_attr(feature="std", derive(WriteXdr))]
pub struct uint128_xdr {
    pub highbits: u64,
    pub lowbits: u64,
}

#[repr(C)]
#[derive(PartialEq, Debug, Default, Clone)]
pub struct int128_xdr {
    pub highbits: i64,
    pub lowbits: u64,
}

// int64_t, Unix time as nanoseconds elapsed since
// January 1, 1970 UTC
pub type Timestamp = i64;

// uint32_t
pub type RuntimeMemoryOffset = u32;

// uint32_t
pub type RuntimeMemoryLength = u32;

// uint64_t
pub type GasAmount = u64;

// uint32_t
pub type MethodID = u32;

// ed25519 sig len is 512 bits
pub type Ed25519Signature = [u8; 64];

// ed25519 key len is 256 bits
pub type Ed25519PublicKey = [u8; 32];

// ed25519 secret key len is 64 bytes
pub type Ed25519SecretKey = [u8; 64];

#[derive(Clone, PartialEq, Debug)]
#[repr(i32)]
pub enum CryptographicScheme {
    Ed25519 = 0,
}
#[cfg(feature="std")]
impl TryFrom<i32> for CryptographicScheme {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => CryptographicScheme::Ed25519,

            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
      };
      Ok(e)
  }
}

#[repr(i32)]
#[derive(PartialEq, Debug, Clone)]
pub enum Signature {
    // case Ed25519:
    //   Ed25519Signature ed25519;
    Ed25519 { ed25519: Ed25519Signature } = CryptographicScheme::Ed25519 as i32,
    
}

#[repr(i32)]
#[derive(PartialEq, Debug, Clone)]
pub enum PublicKey {
    // case Ed25519:
    //   Ed25519PublicKey ed25519;
    Ed25519 { ed25519: Ed25519PublicKey } = CryptographicScheme::Ed25519 as i32,
    
}

#[repr(i32)]
#[derive(PartialEq, Debug, Clone)]
pub enum SecretKey {
    // case Ed25519:
    //   Ed25519SecretKey ed25519;
    Ed25519 { ed25519: Ed25519SecretKey } = CryptographicScheme::Ed25519 as i32,
    
}

#[repr(i32)]
#[derive(PartialEq, Debug, Clone)]
pub enum ExtensionPoint {
    // case 0:
    //   void;
    _0 = 0 as i32,
    
}

// 256 bit hash
pub type SHA256Hash = [u8; 32];

// 256 bit contract addresses
pub type ContractAddress = [u8; 32];

// arbitrary 32 bytes
pub type StorageKey = [u8; 32];

pub const kMaxLogMessageLen: u32 = 1024;

pub const kMaxReturnBufferLen: u32 = 1024;

// in global storage:
// [contract address + storage key] = 64 bytes
// Contracts are free to set their own conventions for keys.
#[repr(C)]
#[derive(PartialEq, Debug, Default, Clone)]
pub struct AddressAndKey {
    pub address: ContractAddress,
    pub key: StorageKey,
}

#[repr(C)]
#[derive(PartialEq, Debug, Default, Clone)]
pub struct LogEntry {
    pub source_address: ContractAddress,
    pub message: VecM<u8, kMaxLogMessageLen>,
}


//
// XDR internals below this point
//

impl Signature {
    pub fn scheme(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl PublicKey {
    pub fn scheme(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl SecretKey {
    pub fn scheme(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl ExtensionPoint {
    pub fn v(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}


