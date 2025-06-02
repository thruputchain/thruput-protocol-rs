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


use ::serde::{Serialize, Deserialize};
use ::serde_arrays;


#[derive(PartialEq, Debug, Default, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
#[repr(C)]
pub struct uint128_xdr {
    pub highbits: u64,
    pub lowbits: u64,
}

#[derive(PartialEq, Debug, Default, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
#[repr(C)]
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

// TODO(geoff): consider using a smaller type for GasAmount
// uint64_t
pub type GasAmount = u64;

// uint64_t
pub type MethodID = u64;

// ed25519 sig len is 512 bits
// 
// ed25519 key len is 256 bits
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
#[repr(transparent)]
pub struct Ed25519Signature {
    #[serde(with = "serde_arrays")]
    inner: [u8; 64],
}
impl From<[u8; 64]> for Ed25519Signature {
    fn from(_inner: [u8; 64]) -> Self {
        Self { inner: _inner }
    }
}
impl Ed25519Signature {
    pub fn into_inner(self) -> [u8; 64] {
        self.inner
    }
    pub const fn new(_inner: [u8; 64]) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for Ed25519Signature {
    type Target = [u8; 64];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for Ed25519Signature {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<[u8]> for Ed25519Signature {
    fn as_ref(&self) -> &[u8] {
        self.deref().as_slice()
    }
}

// 
// ed25519 secret key len is 64 bytes
#[derive(PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
#[repr(transparent)]
pub struct Ed25519PublicKey {
    #[serde(with = "serde_arrays")]
    inner: [u8; 32],
}
impl From<[u8; 32]> for Ed25519PublicKey {
    fn from(_inner: [u8; 32]) -> Self {
        Self { inner: _inner }
    }
}
impl Ed25519PublicKey {
    pub fn into_inner(self) -> [u8; 32] {
        self.inner
    }
    pub const fn new(_inner: [u8; 32]) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for Ed25519PublicKey {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for Ed25519PublicKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<[u8]> for Ed25519PublicKey {
    fn as_ref(&self) -> &[u8] {
        self.deref().as_slice()
    }
}

// 
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
#[repr(transparent)]
pub struct Ed25519SecretKey {
    #[serde(with = "serde_arrays")]
    inner: [u8; 64],
}
impl From<[u8; 64]> for Ed25519SecretKey {
    fn from(_inner: [u8; 64]) -> Self {
        Self { inner: _inner }
    }
}
impl Ed25519SecretKey {
    pub fn into_inner(self) -> [u8; 64] {
        self.inner
    }
    pub const fn new(_inner: [u8; 64]) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for Ed25519SecretKey {
    type Target = [u8; 64];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for Ed25519SecretKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<[u8]> for Ed25519SecretKey {
    fn as_ref(&self) -> &[u8] {
        self.deref().as_slice()
    }
}


#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature="std", derive(ReadXdrEnum,WriteXdrEnum))]
#[repr(i32)]
pub enum CryptographicScheme {
    Ed25519 = 0,
}

#[repr(i32)]
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
pub enum Signature {
    // case Ed25519:
    //   Ed25519Signature ed25519;
    Ed25519 { ed25519: Ed25519Signature } = CryptographicScheme::Ed25519 as i32,
    
}

#[repr(i32)]
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
pub enum PublicKey {
    // case Ed25519:
    //   Ed25519PublicKey ed25519;
    Ed25519 { ed25519: Ed25519PublicKey } = CryptographicScheme::Ed25519 as i32,
    
}

#[repr(i32)]
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
pub enum SecretKey {
    // case Ed25519:
    //   Ed25519SecretKey ed25519;
    Ed25519 { ed25519: Ed25519SecretKey } = CryptographicScheme::Ed25519 as i32,
    
}

#[repr(i32)]
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
pub enum ExtensionPoint {
    // case 0:
    //   void;
    _0 = 0 as i32,
    
}

// 256 bit hash
// 
// 256 bit contract address
#[derive(PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
#[repr(transparent)]
pub struct SHA256Hash {
    #[serde(with = "serde_arrays")]
    inner: [u8; 32],
}
impl From<[u8; 32]> for SHA256Hash {
    fn from(_inner: [u8; 32]) -> Self {
        Self { inner: _inner }
    }
}
impl SHA256Hash {
    pub fn into_inner(self) -> [u8; 32] {
        self.inner
    }
    pub const fn new(_inner: [u8; 32]) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for SHA256Hash {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for SHA256Hash {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<[u8]> for SHA256Hash {
    fn as_ref(&self) -> &[u8] {
        self.deref().as_slice()
    }
}

// 
// arbitrary 32-byte key into contract storage
// This is the key type that contracts directly manipulate
#[derive(PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
#[repr(transparent)]
pub struct ContractAddress {
    #[serde(with = "serde_arrays")]
    inner: [u8; 32],
}
impl From<[u8; 32]> for ContractAddress {
    fn from(_inner: [u8; 32]) -> Self {
        Self { inner: _inner }
    }
}
impl ContractAddress {
    pub fn into_inner(self) -> [u8; 32] {
        self.inner
    }
    pub const fn new(_inner: [u8; 32]) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for ContractAddress {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for ContractAddress {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<[u8]> for ContractAddress {
    fn as_ref(&self) -> &[u8] {
        self.deref().as_slice()
    }
}

// 
#[derive(PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
#[repr(transparent)]
pub struct StorageKeyContractData {
    #[serde(with = "serde_arrays")]
    inner: [u8; 32],
}
impl From<[u8; 32]> for StorageKeyContractData {
    fn from(_inner: [u8; 32]) -> Self {
        Self { inner: _inner }
    }
}
impl StorageKeyContractData {
    pub fn into_inner(self) -> [u8; 32] {
        self.inner
    }
    pub const fn new(_inner: [u8; 32]) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for StorageKeyContractData {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for StorageKeyContractData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<[u8]> for StorageKeyContractData {
    fn as_ref(&self) -> &[u8] {
        self.deref().as_slice()
    }
}


#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature="std", derive(ReadXdrEnum,WriteXdrEnum))]
#[repr(i32)]
pub enum StorageKeyType {
    kStorageKeyTypeContractData = 0,
    kStorageKeyTypeFreeNativeTokenBalance = 1,
}

#[repr(i32)]
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
pub enum StorageKey {
    // case kStorageKeyTypeContractData:
    //   StorageKeyContractData contract_data_key;
    kStorageKeyTypeContractData { contract_data_key: StorageKeyContractData } = StorageKeyType::kStorageKeyTypeContractData as i32,
    // case kStorageKeyTypeFreeNativeTokenBalance:
    //   void;
    kStorageKeyTypeFreeNativeTokenBalance = StorageKeyType::kStorageKeyTypeFreeNativeTokenBalance as i32,
    
}

// 
pub const kMaxLogMessageLen : u32 = 1024;

// 
pub const kMaxReturnBufferLen : u32 = 1024;

// 
pub const kTotalHogSupply : u64 = 62_500_000_000;

// 
// Each hog token is divisible into 10^7 units
pub const kHogDivisibility : u32 = 10_000_000;

pub type ReplayNonce = u64;

// in global storage:
// [contract address + storage key] = 64 bytes
// Contracts are free to set their own conventions for keys.
#[derive(PartialEq, Debug, Default, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
#[repr(C)]
pub struct AddressAndKey {
    pub address: ContractAddress,
    pub key: StorageKey,
}

#[derive(PartialEq, Debug, Default, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
#[repr(C)]
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

impl StorageKey {
    pub fn key_type(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}


