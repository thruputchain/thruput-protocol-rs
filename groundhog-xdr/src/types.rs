// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 84fabf8ff739fc6a8d0917c073b267be05fe3db9

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::unnecessary_cast)]

use crate::*;
use core::option::Option;
use core::ops::{Deref, DerefMut};

#[cfg(feature = "std")]
use std::io::{Read, Write};
#[cfg(feature = "xdrs")]
use xdrs::*;
#[cfg(feature = "spec")]
use xdrs::xdr_repr::*;


#[cfg(feature = "serde")]
use ::serde::{{Serialize, Deserialize}};
#[cfg(feature = "serde")]
use ::serde_arrays;


/// Encodes a 128-bit unsigned integer as two 64-bit unsigned integers.
/// Under the standard XDR encoding, this type serializes as a big-endian u128.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct uint128_xdr {
    pub highbits: u64,
    pub lowbits: u64,
}

/// Encodes a 128-bit signed integer as two 64-bit signed integers.
/// Under the standard XDR encoding, this type serializes as a big-endian i128.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct int128_xdr {
    pub highbits: i64,
    pub lowbits: u64,
}

/// uint64_t, Unix time as nanoseconds elapsed since
/// January 1, 1970 UTC
pub type TimePoint = u64;

/// Typedef representing a length of memory in bytes, within
/// a wasm runtime.  This is primarily used as an argument to syscalls,
/// and should not be directly used by most contracts.
// uint32_t
pub type RuntimeMemoryLength = u32;

// TODO(geoff): consider using a smaller type for GasAmount
/// Represents a quantity of gas.  Alias of u64.
// uint64_t
pub type GasAmount = u64;

/// Represents a 512-bit Ed25519 signature.
// 
/// Represents a 256-bit Ed25519 public key.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Ed25519Signature {
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays::hex"))]
    inner: [u8; 64 as usize],
}
impl From<[u8; 64 as usize]> for Ed25519Signature {
    fn from(_inner: [u8; 64 as usize]) -> Self {
        Self { inner: _inner }
    }
}
impl Ed25519Signature {
    pub fn into_inner(self) -> [u8; 64 as usize] {
        self.inner
    }
    pub const fn new(_inner: [u8; 64 as usize]) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for Ed25519Signature {
    type Target = [u8; 64 as usize];

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
impl<'a> From<&'a [u8; 64]> for &'a Ed25519Signature {
    fn from(arr: &[u8; 64]) -> Self {
        let ptr = arr as *const [u8; 64] as *const Ed25519Signature;
        unsafe { &*ptr }
    }
}
#[cfg(feature = "xdrs")]

impl<'r> ReadXDR<'r> for &'r Ed25519Signature {
  fn read_xdr<R: xdrs::Reader<'r>>(r: &mut R) -> xdrs::ReadResult<Self> {
      let arr: &[u8; 64] = xdrs::read_xdr(r)?;
      let val = Self::from(arr);
      Ok(val)
  }
}



// 
/// Represents a 64-byte Ed25519 secret key.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Default, Copy)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Ed25519PublicKey {
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays::hex"))]
    inner: [u8; 32 as usize],
}
impl From<[u8; 32 as usize]> for Ed25519PublicKey {
    fn from(_inner: [u8; 32 as usize]) -> Self {
        Self { inner: _inner }
    }
}
impl Ed25519PublicKey {
    pub fn into_inner(self) -> [u8; 32 as usize] {
        self.inner
    }
    pub const fn new(_inner: [u8; 32 as usize]) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for Ed25519PublicKey {
    type Target = [u8; 32 as usize];

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
impl<'a> From<&'a [u8; 32]> for &'a Ed25519PublicKey {
    fn from(arr: &[u8; 32]) -> Self {
        let ptr = arr as *const [u8; 32] as *const Ed25519PublicKey;
        unsafe { &*ptr }
    }
}
#[cfg(feature = "xdrs")]

impl<'r> ReadXDR<'r> for &'r Ed25519PublicKey {
  fn read_xdr<R: xdrs::Reader<'r>>(r: &mut R) -> xdrs::ReadResult<Self> {
      let arr: &[u8; 32] = xdrs::read_xdr(r)?;
      let val = Self::from(arr);
      Ok(val)
  }
}



// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Ed25519SecretKey {
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays::hex"))]
    inner: [u8; 64 as usize],
}
impl From<[u8; 64 as usize]> for Ed25519SecretKey {
    fn from(_inner: [u8; 64 as usize]) -> Self {
        Self { inner: _inner }
    }
}
impl Ed25519SecretKey {
    pub fn into_inner(self) -> [u8; 64 as usize] {
        self.inner
    }
    pub const fn new(_inner: [u8; 64 as usize]) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for Ed25519SecretKey {
    type Target = [u8; 64 as usize];

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
impl<'a> From<&'a [u8; 64]> for &'a Ed25519SecretKey {
    fn from(arr: &[u8; 64]) -> Self {
        let ptr = arr as *const [u8; 64] as *const Ed25519SecretKey;
        unsafe { &*ptr }
    }
}
#[cfg(feature = "xdrs")]

impl<'r> ReadXDR<'r> for &'r Ed25519SecretKey {
  fn read_xdr<R: xdrs::Reader<'r>>(r: &mut R) -> xdrs::ReadResult<Self> {
      let arr: &[u8; 64] = xdrs::read_xdr(r)?;
      let val = Self::from(arr);
      Ok(val)
  }
}



/// Represents the cryptographic scheme used for a signature or public key.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(i32)]
pub enum CryptographicScheme {
    Ed25519 = 0 as i32,
}
impl TryFrom<i32> for CryptographicScheme {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Ed25519),
            _ => Err(()),
        }
    }
}


/// Union of a Signature type over the various cryptographic schemes.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[repr(i32)]
pub enum Signature {
    // case Ed25519:
    //   Ed25519Signature ed25519;
    Ed25519 { ed25519: Ed25519Signature } = CryptographicScheme::Ed25519 as i32,
    
}

/// Union of a PublicKey type over the various cryptographic schemes.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[repr(i32)]
pub enum PublicKey {
    // case Ed25519:
    //   Ed25519PublicKey ed25519;
    Ed25519 { ed25519: Ed25519PublicKey } = CryptographicScheme::Ed25519 as i32,
    
}

/// Union of a SecretKey type over the various cryptographic schemes.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[repr(i32)]
pub enum SecretKey {
    // case Ed25519:
    //   Ed25519SecretKey ed25519;
    Ed25519 { ed25519: Ed25519SecretKey } = CryptographicScheme::Ed25519 as i32,
    
}

/// An ExtensionPoint is a struct that can be embedded as the first
/// element of other XDR structs in order to allow future upgradeability
/// without breaking serialization of pre-existing objects.
/// This serializes as a 32-bit integer.  In its current state,
/// the only valid byte value is 0, but a struct that embeds this can
/// later replace itself with a union (with the prior struct as variant 0)
/// without breaking backwards compatibility.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[repr(i32)]
pub enum ExtensionPoint {
    // case 0:
    //   void;
    _0 = 0 as i32,
    
}

/// Represents a 256-bit SHA-256 hash.
// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Default, Copy)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct SHA256Hash {
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays::hex"))]
    inner: [u8; 32 as usize],
}
impl From<[u8; 32 as usize]> for SHA256Hash {
    fn from(_inner: [u8; 32 as usize]) -> Self {
        Self { inner: _inner }
    }
}
impl SHA256Hash {
    pub fn into_inner(self) -> [u8; 32 as usize] {
        self.inner
    }
    pub const fn new(_inner: [u8; 32 as usize]) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for SHA256Hash {
    type Target = [u8; 32 as usize];

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
impl<'a> From<&'a [u8; 32]> for &'a SHA256Hash {
    fn from(arr: &[u8; 32]) -> Self {
        let ptr = arr as *const [u8; 32] as *const SHA256Hash;
        unsafe { &*ptr }
    }
}
#[cfg(feature = "xdrs")]

impl<'r> ReadXDR<'r> for &'r SHA256Hash {
  fn read_xdr<R: xdrs::Reader<'r>>(r: &mut R) -> xdrs::ReadResult<Self> {
      let arr: &[u8; 32] = xdrs::read_xdr(r)?;
      let val = Self::from(arr);
      Ok(val)
  }
}



/// Represents a 256-bit contract address.
/// In the groundhog storage model, each contract is deployed at a 
/// 32-byte unique `ContractAddress`.
// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Default, Copy)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ContractAddress {
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays::hex"))]
    inner: [u8; 32 as usize],
}
impl From<[u8; 32 as usize]> for ContractAddress {
    fn from(_inner: [u8; 32 as usize]) -> Self {
        Self { inner: _inner }
    }
}
impl ContractAddress {
    pub fn into_inner(self) -> [u8; 32 as usize] {
        self.inner
    }
    pub const fn new(_inner: [u8; 32 as usize]) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for ContractAddress {
    type Target = [u8; 32 as usize];

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
impl<'a> From<&'a [u8; 32]> for &'a ContractAddress {
    fn from(arr: &[u8; 32]) -> Self {
        let ptr = arr as *const [u8; 32] as *const ContractAddress;
        unsafe { &*ptr }
    }
}
#[cfg(feature = "xdrs")]

impl<'r> ReadXDR<'r> for &'r ContractAddress {
  fn read_xdr<R: xdrs::Reader<'r>>(r: &mut R) -> xdrs::ReadResult<Self> {
      let arr: &[u8; 32] = xdrs::read_xdr(r)?;
      let val = Self::from(arr);
      Ok(val)
  }
}



/// Each ContractAddress owns a namespace of storage keys.
///
/// Most of the keyspace is made up of ContractData keys, which are indexed
/// by 32-byte (arbitrary) identifiers (the `StorageKeyContractData` type).
/// The other variants here have special purposes, and cannot be freely accessed
/// by contracts.
///
/// Contracts are free to set their own conventions for indexing data within
/// the StorageKeyContractData subspace.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(i32)]
pub enum StorageKeyType {
    /// Contract-controlled data.
    ///
    /// Stores arbitrary data managed by the contract at this address.
    kStorageKeyTypeContractData = 0 as i32,

    /// Native token balance.
    ///
    /// Stores the free balance of the native HOG token for this address.
    kStorageKeyTypeFreeNativeTokenBalance = 1 as i32,

    /// Address authorization data.
    ///
    /// Stores [`AddressAuthData`] - the authorization configuration including
    /// public keys, weights, gas permissions, and nonce rules.
    kStorageKeyTypeAddressAuthData = 2 as i32,

    /// Address metadata.
    ///
    /// Stores [`AddressMetadata`] - metadata about the contract deployed at
    /// this address (hash, size, type).
    kStorageKeyTypeAddressMetadata = 3 as i32,

    /// Replay cache for nonce tracking.
    ///
    /// Stores a set of active [`Uniquifier`] objects
    /// to prevent transaction replay attacks.
    kStorageKeyTypeReplayCache = 4 as i32,
}
impl TryFrom<i32> for StorageKeyType {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::kStorageKeyTypeContractData),
            1 => Ok(Self::kStorageKeyTypeFreeNativeTokenBalance),
            2 => Ok(Self::kStorageKeyTypeAddressAuthData),
            3 => Ok(Self::kStorageKeyTypeAddressMetadata),
            4 => Ok(Self::kStorageKeyTypeReplayCache),
            _ => Err(()),
        }
    }
}


/// Represents an arbitrary 32-byte key into contract storage.
/// This is the key type that contracts directly manipulate
// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Default, Copy)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct StorageKeyContractData {
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays::hex"))]
    inner: [u8; 32 as usize],
}
impl From<[u8; 32 as usize]> for StorageKeyContractData {
    fn from(_inner: [u8; 32 as usize]) -> Self {
        Self { inner: _inner }
    }
}
impl StorageKeyContractData {
    pub fn into_inner(self) -> [u8; 32 as usize] {
        self.inner
    }
    pub const fn new(_inner: [u8; 32 as usize]) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for StorageKeyContractData {
    type Target = [u8; 32 as usize];

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
impl<'a> From<&'a [u8; 32]> for &'a StorageKeyContractData {
    fn from(arr: &[u8; 32]) -> Self {
        let ptr = arr as *const [u8; 32] as *const StorageKeyContractData;
        unsafe { &*ptr }
    }
}
#[cfg(feature = "xdrs")]

impl<'r> ReadXDR<'r> for &'r StorageKeyContractData {
  fn read_xdr<R: xdrs::Reader<'r>>(r: &mut R) -> xdrs::ReadResult<Self> {
      let arr: &[u8; 32] = xdrs::read_xdr(r)?;
      let val = Self::from(arr);
      Ok(val)
  }
}



/// Union of a StorageKey type over the various storage key types.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[repr(i32)]
pub enum StorageKey {
    // case kStorageKeyTypeContractData:
    //   StorageKeyContractData contract_data_key;
    kStorageKeyTypeContractData { contract_data_key: StorageKeyContractData } = StorageKeyType::kStorageKeyTypeContractData as i32,
    // case kStorageKeyTypeFreeNativeTokenBalance:
    //   void;
    kStorageKeyTypeFreeNativeTokenBalance = StorageKeyType::kStorageKeyTypeFreeNativeTokenBalance as i32,
    // case kStorageKeyTypeAddressAuthData:
    //   void;
    kStorageKeyTypeAddressAuthData = StorageKeyType::kStorageKeyTypeAddressAuthData as i32,
    // case kStorageKeyTypeAddressMetadata:
    //   void;
    kStorageKeyTypeAddressMetadata = StorageKeyType::kStorageKeyTypeAddressMetadata as i32,
    // case kStorageKeyTypeReplayCache:
    //   void;
    kStorageKeyTypeReplayCache = StorageKeyType::kStorageKeyTypeReplayCache as i32,
    
}

// 
pub const kTotalHogSupply : u64 = 62_500_000_000;

// 
// Each hog token is divisible into 10^7 units
pub const kHogDivisibility : u32 = 10_000_000;

/// In the global storage, every item is indexed by a unique [contract address + storage key] pair.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct AddressAndKey {
    pub address: ContractAddress,
    pub key: StorageKey,
}

/// BlockHeight is the height of a block, measured as the number of blocks since the genesis of the chain.
/// The first block of transactions is at height 1.
/// It is signed to prevent overflows in
/// calculations.
pub type BlockHeight = i64;


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


