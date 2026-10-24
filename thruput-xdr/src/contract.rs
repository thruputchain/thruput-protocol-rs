// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 0c2c59fb8ebeb0d6d37a1dad9cf88c4f22187ce6

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


#[cfg(feature = "serde")]
use ::serde::{{Serialize, Deserialize}};



//==============================================================================
// Contracts and Contract Deployment
//
// This module defines the contract abstraction in Groundhog. Contracts are
// code that can be deployed to addresses and invoked to execute logic.
//
// Key concepts:
// - Contracts can be executable blobs, native precompiled code, or test mocks
// - Contract addresses are deterministically derived from deployment parameters
// - Contract methods are identified by 8-byte entrypoint IDs
// - Entrypoint IDs are derived from interface names and versions
//==============================================================================

//------------------------------------------------------------------------------
// Contract Types and Code
//------------------------------------------------------------------------------

/// The largest contract code blob a chain admits when its configuration does
/// not state `kConfigParamMaxContractSizeBytes`. Includes the artifact header,
/// code, and initial memory.
pub const kDefaultMaxContractSizeBytes : u32 = 524288;

/// An opaque blob of executable code.
///
/// This is the raw binary that will be executed as an RV32 contract.
///
/// Unbounded on the wire: the size a chain admits is a configuration
/// parameter (`kConfigParamMaxContractSizeBytes`), enforced where a
/// transaction attaches a contract rather than where one is decoded.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractBlob {
    pub data: BytesM,
}

/// The execution environment for a contract.
///
/// Discriminant 0 is retired. It selected the Wasmi runtime, which was
/// removed once RV32 became the only deployable execution environment.
/// Do not reuse the value: old encodings that carry it must fail to parse
/// rather than silently decode as a live contract type.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct ContractType(pub i32);

impl ContractType {
    /// Precompiled native contract implemented natively by the node.
    ///
    /// Used for system contracts like the native token.
    /// Cheaper to invoke than a deployed RV32 contract, but users cannot
    /// deploy new ones.
    pub const kNative: Self = Self(1);

    /// Mock contract for local testing.
    ///
    /// Never deployed on-chain; exists only in the test harness.
    pub const kMockTest: Self = Self(2);

    /// RiscV contract executed in the RV32 VM.
    ///
    /// This is the standard contract type for user-deployed contracts.
    pub const kRv32: Self = Self(3);
}
impl From<i32> for ContractType {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for ContractType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::kNative.0 => {
                write!(f, "ContractType::kNative")
            },
            x if x == Self::kMockTest.0 => {
                write!(f, "ContractType::kMockTest")
            },
            x if x == Self::kRv32.0 => {
                write!(f, "ContractType::kRv32")
            },
            _ => {
                write!(f, "ContractType({})", self.0)
            }
        }
    }
}


/// A contract that can be deployed to an address and invoked.
///
/// Contracts are the fundamental unit of code execution in Groundhog.
/// Once deployed to a `ContractAddress`, they can be invoked by transactions
/// to execute their methods (entrypoints).
///
/// The three contract types serve different purposes:
/// - `kRv32`: User-deployed contracts (most common)
/// - `kNative`: System contracts with privileged access
/// - `kMockTest`: Test contracts for local development
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[repr(i32)]
#[non_exhaustive]
pub enum Contract {
    /// Identifier for which native contract this is.
    ///
    /// Native contracts are hard-coded in the runtime.
    // case kNative:
    //   HogexecPrecompiledNativeContracts native_contract_id;
    kNative { native_contract_id: HogexecPrecompiledNativeContracts } = ContractType::kNative.0 as i32,
    /// Identifier for the mock contract in the test harness.
    // case kMockTest:
    //   i32 mock_contract_id;
    kMockTest { mock_contract_id: i32 } = ContractType::kMockTest.0 as i32,
    // case kRv32:
    //   ContractBlob rv32_contract;
    kRv32 { rv32_contract: ContractBlob } = ContractType::kRv32.0 as i32,
    
}

//------------------------------------------------------------------------------
// Contract Deployment and Address Derivation
//------------------------------------------------------------------------------

/// Hash preimage for deriving deployed contract addresses.
///
/// When a contract deploys another contract, the new contract's address is
/// deterministically computed as:
///
/// ```text
/// ContractAddress = SHA256(ContractDeploymentHashPreimage)
/// ```
///
/// Under XDR encoding, this struct serializes as the concatenation of its
/// fields: `deployer_address || nonce || contract_hash`.
///
/// This is Groundhog's analogue of Ethereum's `CREATE2` opcode, allowing
/// deterministic address derivation. The deployer controls the nonce to
/// create addresses at will.
///
/// # Example
///
/// ```text
/// Contract A address:  0x1234...
/// Contract B hash:     0xabcd...   (= SHA256(XDR(Contract B)))
/// Deployment nonce:    0x5678...
/// Result: SHA256(0x1234... || 0x5678... || 0xabcd...) = Contract B address
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractDeploymentHashPreimage {
    /// The address of the contract performing the deployment.
    pub deployer_address: ContractAddress,
    /// Arbitrary 32-byte value chosen by the deployer.
    ///
    /// Allows creating multiple instances of the same contract code
    /// at different addresses.
    pub nonce: SHA256Hash,
    /// SHA-256 hash of the XDR-encoded `Contract` being deployed.
    ///
    /// This hashes the entire union, not the bare code blob:
    ///
    /// ```text
    /// contract_hash = SHA256(XDR(Contract))
    /// ```
    ///
    /// The encoding covers the 4-byte `ContractType` discriminant followed by
    /// the selected arm, so identical code deployed under a different contract
    /// type yields a different hash. For `kRv32` the arm is a `ContractBlob`,
    /// which is itself a 4-byte length prefix followed by the code padded to a
    /// 4-byte boundary -- hashing the raw code alone does not reproduce this
    /// value.
    ///
    /// Native and mock contracts are hashed the same way, over a union whose
    /// arm holds a contract identifier instead of a blob. They do not use
    /// separately predefined hash constants.
    pub contract_hash: SHA256Hash,
}

//------------------------------------------------------------------------------
// Contract Interfaces and Entrypoint Derivation
//------------------------------------------------------------------------------

/// 8-byte identifier for a contract method (entrypoint).
///
/// Entrypoint IDs uniquely identify which method to invoke on a contract.
/// A contract may use any 8 bytes; the protocol attaches no meaning to them.
/// By default, the SDK derives them from the interface name, the major
/// version of the interface, and the method name:
///
/// ```text
/// SHA256(name || "::" || major || "::" || method)[0..8]
/// ```
///
/// `major` is the semver compatibility prefix of the interface version: the
/// leading components up to and including the first nonzero one (`"1"` for
/// `"1.4.2"`, `"0.2"` for `"0.2.7"`, `"0.0.3"` for `"0.0.3"`). Versions that
/// share it therefore share entrypoint IDs, and the `"::"` separators keep
/// distinct (name, major, method) triples from concatenating to the same
/// preimage.
///
/// # Example
///
/// For an `"ERC20"` interface at version `"1.0.0"`, `transfer` is
/// `SHA256("ERC20::1::transfer")[0..8]`.
///
/// Interface IDs use the same prefix under a domain tag,
/// `SHA256("groundhog-interface::" || name || "::" || major)[0..8]`, so they
/// never coincide with an entrypoint ID.
// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Default, Copy)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ContractEntrypointID {
    #[cfg_attr(feature = "serde", serde(with = "xdr_common::serde_hex"))]
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
impl<'r: 'a, 'a> ReadXDR<'r> for &'a ContractEntrypointID {
  fn read_xdr<R: xdrs::Reader<'r>>(r: &mut R) -> xdrs::ReadResult<Self> {
      let arr: &[u8; 8] = xdrs::read_xdr(r)?;
      let val = Self::from(arr);
      Ok(val)
  }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for ContractEntrypointID {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        "ContractEntrypointID".into()
    }
    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "string",
            "contentEncoding": "hex",
            "contentMediaType": "application/binary",
            "minLength": 16,
            "maxLength": 16
        })
    }
}



//------------------------------------------------------------------------------
// Deployed Contract Metadata
//------------------------------------------------------------------------------

/// Hash and metadata for a contract deployed at an address.
///
/// This allows the system to locate and instantiate the contract code
/// without loading the full contract from storage.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct DeployedContractHash {
    /// SHA-256 hash of the XDR-encoded `Contract` deployed at this address.
    ///
    /// Computed as `SHA256(XDR(Contract))`, identically to the `contract_hash`
    /// field of `ContractDeploymentHashPreimage`.
    pub hash: SHA256Hash,
    /// Size of the contract code in bytes.
    ///
    /// Always 0 for `kNative` and `kMockTest`, which carry no deployable blob.
    ///
    /// Optimization: allows computing and updating storage costs without
    /// loading the full contract from storage.
    pub contract_size: u32,
    /// The type of contract: RV32, native, or mock.
    ///
    /// Optimization: allows the executor to determine which runtime to
    /// instantiate without a database lookup. When the contract module is
    /// already cached, this avoids loading the contract bytecode entirely.
    pub contract_type: ContractType,
}



//
// XDR internals below this point
//

impl Contract {
    pub fn contract_type(&self) -> ContractType {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


