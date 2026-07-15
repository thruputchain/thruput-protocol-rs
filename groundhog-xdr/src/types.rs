// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: fcfcf83a6c5f2e7596b7a93113093597f40c45ec

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
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
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
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct int128_xdr {
    pub highbits: i64,
    pub lowbits: u64,
}

/// uint64_t, Unix time as nanoseconds elapsed since
/// January 1, 1970 UTC
// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct TimePoint {
    inner: u64,
}
impl From<u64> for TimePoint {
    fn from(_inner: u64) -> Self {
        Self { inner: _inner }
    }
}
impl TimePoint {
    pub fn into_inner(self) -> u64 {
        self.inner
    }
    pub const fn new(_inner: u64) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for TimePoint {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for TimePoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

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

#[cfg(feature = "schema")]
impl schemars::JsonSchema for Ed25519Signature {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        "Ed25519Signature".into()
    }
    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "string",
            "contentEncoding": "hex",
            "contentMediaType": "application/binary",
            "minLength": 128,
            "maxLength": 128
        })
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

#[cfg(feature = "schema")]
impl schemars::JsonSchema for Ed25519PublicKey {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        "Ed25519PublicKey".into()
    }
    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "string",
            "contentEncoding": "hex",
            "contentMediaType": "application/binary",
            "minLength": 64,
            "maxLength": 64
        })
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

#[cfg(feature = "schema")]
impl schemars::JsonSchema for Ed25519SecretKey {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        "Ed25519SecretKey".into()
    }
    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "string",
            "contentEncoding": "hex",
            "contentMediaType": "application/binary",
            "minLength": 128,
            "maxLength": 128
        })
    }
}



/// Represents the cryptographic scheme used for a signature or public key.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct CryptographicScheme(pub i32);

impl CryptographicScheme {
    pub const Ed25519: Self = Self(0);
}
impl From<i32> for CryptographicScheme {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for CryptographicScheme {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::Ed25519.0 => {
                write!(f, "CryptographicScheme::Ed25519")
            },
            _ => {
                write!(f, "CryptographicScheme({})", self.0)
            }
        }
    }
}


/// Union of a Signature type over the various cryptographic schemes.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[repr(i32)]
#[non_exhaustive]
pub enum Signature {
    // case Ed25519:
    //   Ed25519Signature ed25519;
    Ed25519 { ed25519: Ed25519Signature } = CryptographicScheme::Ed25519.0 as i32,
    
}

/// Union of a PublicKey type over the various cryptographic schemes.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[repr(i32)]
#[non_exhaustive]
pub enum PublicKey {
    // case Ed25519:
    //   Ed25519PublicKey ed25519;
    Ed25519 { ed25519: Ed25519PublicKey } = CryptographicScheme::Ed25519.0 as i32,
    
}

/// Union of a SecretKey type over the various cryptographic schemes.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[repr(i32)]
#[non_exhaustive]
pub enum SecretKey {
    // case Ed25519:
    //   Ed25519SecretKey ed25519;
    Ed25519 { ed25519: Ed25519SecretKey } = CryptographicScheme::Ed25519.0 as i32,
    
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
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[repr(i32)]
#[non_exhaustive]
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

#[cfg(feature = "schema")]
impl schemars::JsonSchema for SHA256Hash {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        "SHA256Hash".into()
    }
    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "string",
            "contentEncoding": "hex",
            "contentMediaType": "application/binary",
            "minLength": 64,
            "maxLength": 64
        })
    }
}



/// Hash of the groundhog database contents.
/// Format is a 2048-byte LtHash16 hash.
// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct DbHash {
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays::hex"))]
    inner: [u8; 2048 as usize],
}
impl From<[u8; 2048 as usize]> for DbHash {
    fn from(_inner: [u8; 2048 as usize]) -> Self {
        Self { inner: _inner }
    }
}
impl DbHash {
    pub fn into_inner(self) -> [u8; 2048 as usize] {
        self.inner
    }
    pub const fn new(_inner: [u8; 2048 as usize]) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for DbHash {
    type Target = [u8; 2048 as usize];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for DbHash {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<[u8]> for DbHash {
    fn as_ref(&self) -> &[u8] {
        self.deref().as_slice()
    }
}
impl<'a> From<&'a [u8; 2048]> for &'a DbHash {
    fn from(arr: &[u8; 2048]) -> Self {
        let ptr = arr as *const [u8; 2048] as *const DbHash;
        unsafe { &*ptr }
    }
}
#[cfg(feature = "xdrs")]
impl<'r> ReadXDR<'r> for &'r DbHash {
  fn read_xdr<R: xdrs::Reader<'r>>(r: &mut R) -> xdrs::ReadResult<Self> {
      let arr: &[u8; 2048] = xdrs::read_xdr(r)?;
      let val = Self::from(arr);
      Ok(val)
  }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for DbHash {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        "DbHash".into()
    }
    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "string",
            "contentEncoding": "hex",
            "contentMediaType": "application/binary",
            "minLength": 4096,
            "maxLength": 4096
        })
    }
}



/// Hash of a groundhog block of transactions
// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct BlockHash {
    inner: SHA256Hash,
}
impl From<SHA256Hash> for BlockHash {
    fn from(_inner: SHA256Hash) -> Self {
        Self { inner: _inner }
    }
}
impl BlockHash {
    pub fn into_inner(self) -> SHA256Hash {
        self.inner
    }
    pub const fn new(_inner: SHA256Hash) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for BlockHash {
    type Target = SHA256Hash;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for BlockHash {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
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

#[cfg(feature = "schema")]
impl schemars::JsonSchema for ContractAddress {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        "ContractAddress".into()
    }
    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "string",
            "contentEncoding": "hex",
            "contentMediaType": "application/binary",
            "minLength": 64,
            "maxLength": 64
        })
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
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct StorageKeyType(pub i32);

impl StorageKeyType {
    /// Contract-controlled data.
    ///
    /// Stores arbitrary data managed by the contract at this address.
    pub const ContractData: Self = Self(0);

    /// Native token balance.
    ///
    /// Stores the free balance of the native HOG token for this address.
    pub const FreeNativeTokenBalance: Self = Self(1);

    /// Address authorization data.
    ///
    /// Stores [`AddressAuthData`] - the authorization configuration including
    /// public keys, weights, gas permissions, and nonce rules.
    pub const AuthData: Self = Self(2);

    /// Address metadata.
    ///
    /// Stores [`AddressMetadata`] - metadata about the contract deployed at
    /// this address (hash, size, type).
    pub const Metadata: Self = Self(3);

    /// Replay cache for nonce tracking.
    ///
    /// Stores a set of active [`Uniquifier`] objects
    /// to prevent transaction replay attacks.
    pub const ReplayCache: Self = Self(4);
}
impl From<i32> for StorageKeyType {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for StorageKeyType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::ContractData.0 => {
                write!(f, "StorageKeyType::ContractData")
            },
            x if x == Self::FreeNativeTokenBalance.0 => {
                write!(f, "StorageKeyType::FreeNativeTokenBalance")
            },
            x if x == Self::AuthData.0 => {
                write!(f, "StorageKeyType::AuthData")
            },
            x if x == Self::Metadata.0 => {
                write!(f, "StorageKeyType::Metadata")
            },
            x if x == Self::ReplayCache.0 => {
                write!(f, "StorageKeyType::ReplayCache")
            },
            _ => {
                write!(f, "StorageKeyType({})", self.0)
            }
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

#[cfg(feature = "schema")]
impl schemars::JsonSchema for StorageKeyContractData {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        "StorageKeyContractData".into()
    }
    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "string",
            "contentEncoding": "hex",
            "contentMediaType": "application/binary",
            "minLength": 64,
            "maxLength": 64
        })
    }
}



/// Union of a StorageKey type over the various storage key types.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[repr(i32)]
#[non_exhaustive]
pub enum StorageKey {
    // case ContractData:
    //   StorageKeyContractData contract_data_key;
    ContractData { contract_data_key: StorageKeyContractData } = StorageKeyType::ContractData.0 as i32,
    // case FreeNativeTokenBalance:
    //   void;
    FreeNativeTokenBalance = StorageKeyType::FreeNativeTokenBalance.0 as i32,
    // case AuthData:
    //   void;
    AuthData = StorageKeyType::AuthData.0 as i32,
    // case Metadata:
    //   void;
    Metadata = StorageKeyType::Metadata.0 as i32,
    // case ReplayCache:
    //   void;
    ReplayCache = StorageKeyType::ReplayCache.0 as i32,
    
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
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct AddressAndKey {
    pub address: ContractAddress,
    pub key: StorageKey,
}

/// BlockHeight is the height of a block, measured as the number of blocks since the genesis of the chain.
/// The first block of transactions is at height 0.
/// It is signed to prevent overflows in
/// calculations.
// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct BlockHeight {
    inner: i64,
}
impl From<i64> for BlockHeight {
    fn from(_inner: i64) -> Self {
        Self { inner: _inner }
    }
}
impl BlockHeight {
    pub fn into_inner(self) -> i64 {
        self.inner
    }
    pub const fn new(_inner: i64) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for BlockHeight {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for BlockHeight {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}


//
// XDR internals below this point
//

impl Signature {
    pub fn scheme(&self) -> CryptographicScheme {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl PublicKey {
    pub fn scheme(&self) -> CryptographicScheme {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl SecretKey {
    pub fn scheme(&self) -> CryptographicScheme {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
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
    pub fn key_type(&self) -> StorageKeyType {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


