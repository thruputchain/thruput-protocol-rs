// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: b2a6e4e0e501d7921c5a048f18026f92d40f8acc

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



//==============================================================================
// Contracts and Contract Deployment
//
// This module defines the contract abstraction in Groundhog. Contracts are
// code that can be deployed to addresses and invoked to execute logic.
//
// Key concepts:
// - Contracts can be WebAssembly blobs, native precompiled code, or test mocks
// - Contract addresses are deterministically derived from deployment parameters
// - Contract methods are identified by 8-byte entrypoint IDs
// - Entrypoint IDs are derived from interface names and versions
//==============================================================================

//------------------------------------------------------------------------------
// Contract Types and Code
//------------------------------------------------------------------------------

/// An opaque blob of WebAssembly code.
///
/// This is the raw WASM binary that will be executed by the Wasmi runtime.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractBlob {
    pub wasm: BytesM,
}

/// The execution environment for a contract.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(i32)]
pub enum ContractType {
    /// WebAssembly contract executed in the Wasmi runtime.
    ///
    /// This is the standard contract type for user-deployed contracts.
    kWasmi = 0 as i32,

    /// Precompiled native contract implemented directly in Rust.
    ///
    /// Used for system contracts like the native HogToken.
    /// More efficient than WASM but cannot be deployed by users.
    kNative = 1 as i32,

    /// Mock contract for local testing.
    ///
    /// Never deployed on-chain; exists only in the test harness.
    kMockTest = 2 as i32,
}
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


/// A contract that can be deployed to an address and invoked.
///
/// Contracts are the fundamental unit of code execution in Groundhog.
/// Once deployed to a `ContractAddress`, they can be invoked by transactions
/// to execute their methods (entrypoints).
///
/// The three contract types serve different purposes:
/// - `kWasmi`: User-deployed contracts (most common)
/// - `kNative`: System contracts with privileged access
/// - `kMockTest`: Test contracts for local development
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
    //   /// Identifier for which native contract this is.
    //   ///
    //   /// Native contracts are hard-coded in the runtime.
    //   i32 native_contract_id;
    kNative { native_contract_id: i32 } = ContractType::kNative as i32,
    // case kMockTest:
    //   /// Identifier for the mock contract in the test harness.
    //   i32 mock_contract_id;
    kMockTest { mock_contract_id: i32 } = ContractType::kMockTest as i32,
    
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
/// Contract B hash:     0xabcd...
/// Deployment nonce:    0x5678...
/// Result: SHA256(0x1234... || 0x5678... || 0xabcd...) = Contract B address
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
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
    /// SHA-256 hash of the contract code being deployed.
    ///
    /// For WASM contracts, this is `SHA256(wasm_blob)`.
    /// For native contracts, this is a predefined hash.
    pub contract_hash: SHA256Hash,
}

//------------------------------------------------------------------------------
// Contract Interfaces and Entrypoint Derivation
//------------------------------------------------------------------------------

/// Valid identifier string following RFC 4506.
///
/// Must be a letter followed by an optional sequence of letters, digits,
/// or underscores. Cannot be an XDR keyword (e.g., `"struct"`, `"union"`, `"enum"`).
///
/// # Examples
///
/// - `"transfer"`
/// - `"approve"`
/// - `"get_balance"`
pub type Identifier = StringM;

/// Semantic version string following semver.org specification.
///
/// Format: `MAJOR.MINOR.PATCH[-PRERELEASE][+BUILD]`
///
/// # Examples
///
/// - `"0.1.3"`
/// - `"1.0.0"`
/// - `"2.1.4-alpha"`
/// - `"1.0.0-alpha+001"`
pub type SemVer = StringM;

/// Hash preimage for deriving contract interface identifiers.
///
/// Contract entrypoints can be arbitrary 8 byte identifiers (the [`ContractEntrypointID`] type).
///  By default, the SDK
/// for Groundhog contracts follows the following scheme for allocating those 
/// identifiers, although this scheme is not a strict requirement.
///
/// Contract interfaces define a set of methods (entrypoints) with a name
/// and version. The first 6 bytes of `SHA256(ContractInterfaceIDPreimage)`
/// serve as a namespace prefix for all methods in that interface.
///
/// This allows:
/// - Interface versioning (same methods, different versions)
/// - Collision resistance (different interfaces, same method names)
/// - Stable method IDs across implementations
///
/// # Example
///
/// An interface `"ERC20"` version `"1.0.0"` will have a unique 6-byte
/// prefix, and all methods (`transfer`, `approve`, etc.) will share that prefix.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractInterfaceIDPreimage {
    /// Name of the contract interface.
    ///
    /// Should be descriptive and unique (e.g., `"ERC20"`, `"Governance"`, `"Oracle"`).
    pub name: Identifier,
    /// Semantic version of the interface.
    ///
    /// Allows evolving interfaces while maintaining compatibility.
    pub vers: SemVer,
}

/// 8-byte identifier for a contract method (entrypoint).
///
/// Entrypoint IDs uniquely identify which method to invoke on a contract.
/// By default, the SDK computes these as follows.
///
/// ```text
/// Bytes 0-5: SHA256(ContractInterfaceIDPreimage)[0..6]
/// Bytes 6-7: Method-specific ID chosen by interface author
/// ```
///
/// This two-level scheme provides:
/// 1. Interface-level namespace (6 bytes) - prevents collisions between interfaces
/// 2. Method-level identifier (2 bytes) - identifies specific method within interface
///
/// By default, the SDK lexicographically sorts method names and allocates the
/// last 2 bytes based on that order (first method gets 0x0000, second gets 0x0001, etc.).
///
/// # Example
///
/// For `"ERC20"` v1.0.0 interface:
///
/// ```text
/// SHA256("ERC20" || "1.0.0")[0..6] = 0xabcdef012345 (6-byte prefix)
///
/// Methods sorted lexicographically:
/// approve  -> method ID = 0x0000 -> Full entrypoint = 0xabcdef0123450000
/// transfer -> method ID = 0x0001 -> Full entrypoint = 0xabcdef0123450001
/// ```
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


//------------------------------------------------------------------------------
// Deployed Contract Metadata
//------------------------------------------------------------------------------

/// Hash and metadata for a contract deployed at an address.
///
/// This allows the system to locate and instantiate the contract code
/// without loading the full contract from storage.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct DeployedContractHash {
    /// SHA-256 hash of the deployed contract code.
    pub hash: SHA256Hash,
    /// Size of the contract in bytes.
    ///
    /// Optimization: allows computing and updating storage costs without
    /// loading the full contract from storage.
    pub contract_size: u32,
    /// The type of contract (e.g., Wasm, native, etc.).
    ///
    /// Optimization: allows the executor to determine which runtime to
    /// instantiate without a database lookup. When the contract module is
    /// already cached, this avoids loading the contract bytecode entirely.
    pub contract_type: ContractType,
}

/// Metadata associated with an address.
///
/// Currently only stores information about the deployed contract, if any.
/// This is the type stored at storage key `kStorageKeyTypeAddressMetadata`.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct AddressMetadata {
    /// Information about the contract deployed at this address.
    pub deployed_contract: DeployedContractHash,
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


