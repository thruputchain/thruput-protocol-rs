// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 5351fdbd5f24dc09297b1857e6e8605cbf0ee655

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



/// The type of a storage item.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct StorageItemType(pub i32);

impl StorageItemType {
    pub const kItemTypeBytestring: Self = Self(0);
    pub const kItemTypeNonnegativeInt: Self = Self(1);
    pub const kItemTypeRetrocausalQueue: Self = Self(2);
    pub const kItemTypeReplayCacheData: Self = Self(3);
    pub const kItemTypeAuthData: Self = Self(4);
    pub const kItemTypeDeployedContractHash: Self = Self(5);

    // 
    // This will be removed when we get rid of the old db interface.
    
    #[doc(hidden)]pub const kItemTypeDeleted: Self = Self(-1);
}
impl From<i32> for StorageItemType {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for StorageItemType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::kItemTypeBytestring.0 => {
                write!(f, "StorageItemType::kItemTypeBytestring")
            },
            x if x == Self::kItemTypeNonnegativeInt.0 => {
                write!(f, "StorageItemType::kItemTypeNonnegativeInt")
            },
            x if x == Self::kItemTypeRetrocausalQueue.0 => {
                write!(f, "StorageItemType::kItemTypeRetrocausalQueue")
            },
            x if x == Self::kItemTypeReplayCacheData.0 => {
                write!(f, "StorageItemType::kItemTypeReplayCacheData")
            },
            x if x == Self::kItemTypeAuthData.0 => {
                write!(f, "StorageItemType::kItemTypeAuthData")
            },
            x if x == Self::kItemTypeDeployedContractHash.0 => {
                write!(f, "StorageItemType::kItemTypeDeployedContractHash")
            },
            x if x == Self::kItemTypeDeleted.0 => {
                write!(f, "StorageItemType::kItemTypeDeleted")
            },
            _ => {
                write!(f, "StorageItemType({})", self.0)
            }
        }
    }
}


pub const kBytestringMaxLen : u32 = 1024;

/// An arbitrary value, stored as an opaque string of bytes.
pub type Bytestring = BytesM::<kBytestringMaxLen>;

/// The value at every storage key is one of the following data types.
///
/// [`StorageDelta`]s are modifications that are applied to these StorageItem objects.
///
/// Contracts do not generally directly load these values, but can access them via the typed 
/// system call interface provided by the Groundhog runtime.
/// For example, a contract can call `BytestringGet`, which will return either a Bytestring value or
/// an error if the storage key does not exist or contains a different type.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[repr(i32)]
#[non_exhaustive]
pub enum StorageItem {
    /// An arbitrary value, stored as an opaque string of bytes.
    // case kItemTypeBytestring:
    //   Bytestring bytestring;
    kItemTypeBytestring { bytestring: Bytestring } = StorageItemType::kItemTypeBytestring.0 as i32,
    /// A nonnegative integer, stored as a 128-bit value.
    ///
    /// This can be stored at any [`StorageKey`] of type `ContractData`.
    /// This type is also used to store the free native token balance at 
    /// storage key `FreeNativeTokenBalance`.
    // case kItemTypeNonnegativeInt:
    //   int128_xdr nnint;
    kItemTypeNonnegativeInt { nnint: int128_xdr } = StorageItemType::kItemTypeNonnegativeInt.0 as i32,
    /// A retrocausal multi-producer, single-consumer queue.
    /// Transactions can only insert entries into the queue or consume entries one by one
    /// from the queue, so the whole queue is generally not loaded all at once.
    // case kItemTypeRetrocausalQueue:
    //   VecM<QueueEntry> queue_entries;
    kItemTypeRetrocausalQueue { queue_entries: VecM<QueueEntry> } = StorageItemType::kItemTypeRetrocausalQueue.0 as i32,
    /// A set of [`Uniquifier`]s that have been used by the current address.
    /// Transactions can only insert a Uniquifier, not read the whole set.
    /// 
    /// This is the type stored at storage key `ReplayCache`.
    /// It cannot be stored at any other storage key.
    // case kItemTypeReplayCacheData:
    //   VecM<Uniquifier> active_uniquifiers;
    kItemTypeReplayCacheData { active_uniquifiers: VecM<Uniquifier> } = StorageItemType::kItemTypeReplayCacheData.0 as i32,
    /// The authorization data for the current address.
    ///
    /// This is the type stored at storage key `AddressAuthData`.
    /// It cannot be stored at any other storage key.
    // case kItemTypeAuthData:
    //   AddressAuthData auth_data;
    kItemTypeAuthData { auth_data: AddressAuthData } = StorageItemType::kItemTypeAuthData.0 as i32,
    /// The hash of the deployed contract for the current address.
    ///
    /// This is the type stored at storage key `AddressMetadata`.
    /// It cannot be stored at any other storage key.
    // case kItemTypeDeployedContractHash:
    //   DeployedContractHash deployed_contract_hash;
    kItemTypeDeployedContractHash { deployed_contract_hash: DeployedContractHash } = StorageItemType::kItemTypeDeployedContractHash.0 as i32,
    
}
// 
// This is the type that is actually sent over the network from the database to the executor.
// This type strips out complex types (the replay cache and retrocausal queue) that the
// executor does not need to load.
// The types are filled in, as the executor does need to know the type of an item even 
// without knowing the contents.

#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[repr(i32)]
#[non_exhaustive]
pub enum WireStorageItem {
    // case kItemTypeBytestring:
    //   Bytestring bytestring;
    kItemTypeBytestring { bytestring: Bytestring } = StorageItemType::kItemTypeBytestring.0 as i32,
    // case kItemTypeNonnegativeInt:
    //   int128_xdr nnint;
    kItemTypeNonnegativeInt { nnint: int128_xdr } = StorageItemType::kItemTypeNonnegativeInt.0 as i32,
    // case kItemTypeAuthData:
    //   AddressAuthData auth_data;
    kItemTypeAuthData { auth_data: AddressAuthData } = StorageItemType::kItemTypeAuthData.0 as i32,
    // case kItemTypeRetrocausalQueue:
    //   RetrocausalQueueMetadata metadata;
    kItemTypeRetrocausalQueue { metadata: RetrocausalQueueMetadata } = StorageItemType::kItemTypeRetrocausalQueue.0 as i32,
    // case kItemTypeReplayCacheData:
    //   void;
    kItemTypeReplayCacheData = StorageItemType::kItemTypeReplayCacheData.0 as i32,
    // case kItemTypeDeployedContractHash:
    //   DeployedContractHash deployed_contract_hash;
    kItemTypeDeployedContractHash { deployed_contract_hash: DeployedContractHash } = StorageItemType::kItemTypeDeployedContractHash.0 as i32,
    
}

/// The types of changes accumulated by a transaction that will be applied to the database
/// if the transaction commits.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct StorageDeltaType(pub i32);

impl StorageDeltaType {
    pub const kDeltaTypeBytestringSet: Self = Self(0);
    pub const kDeltaTypeNonnegativeIntSetAdd: Self = Self(1);
    pub const kDeltaTypeDelete: Self = Self(2);
    pub const kDeltaTypeRetrocausalQueue: Self = Self(3);
    pub const kDeltaTypeNonnegativeIntAdd: Self = Self(5);
    pub const kDeltaTypeAuthDataChange: Self = Self(6);
    pub const kDeltaTypeDeployContract: Self = Self(7);
    pub const kDeltaTypeConsumeUniquifier: Self = Self(8);
}
impl From<i32> for StorageDeltaType {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for StorageDeltaType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::kDeltaTypeBytestringSet.0 => {
                write!(f, "StorageDeltaType::kDeltaTypeBytestringSet")
            },
            x if x == Self::kDeltaTypeNonnegativeIntSetAdd.0 => {
                write!(f, "StorageDeltaType::kDeltaTypeNonnegativeIntSetAdd")
            },
            x if x == Self::kDeltaTypeDelete.0 => {
                write!(f, "StorageDeltaType::kDeltaTypeDelete")
            },
            x if x == Self::kDeltaTypeRetrocausalQueue.0 => {
                write!(f, "StorageDeltaType::kDeltaTypeRetrocausalQueue")
            },
            x if x == Self::kDeltaTypeNonnegativeIntAdd.0 => {
                write!(f, "StorageDeltaType::kDeltaTypeNonnegativeIntAdd")
            },
            x if x == Self::kDeltaTypeAuthDataChange.0 => {
                write!(f, "StorageDeltaType::kDeltaTypeAuthDataChange")
            },
            x if x == Self::kDeltaTypeDeployContract.0 => {
                write!(f, "StorageDeltaType::kDeltaTypeDeployContract")
            },
            x if x == Self::kDeltaTypeConsumeUniquifier.0 => {
                write!(f, "StorageDeltaType::kDeltaTypeConsumeUniquifier")
            },
            _ => {
                write!(f, "StorageDeltaType({})", self.0)
            }
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct AuthDataChangeSigner {
    pub public_key_id: u32,
    pub public_key: PublicKeyConfigOpt,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct AuthDataChangeArgs {
    pub auth_data_change_signers: VecM<AuthDataChangeSigner>,
    pub new_required_weight: Option<Weight>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct NonnegativeInt128SetAddArgs {
    pub set_value: int128_xdr,
    pub add_value: int128_xdr,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ConsumeUniquifierArgs {
    pub uniquifier: Uniquifier,
}

/// One change (of possibly many) accumulated by a transaction to be applied to the database
/// if the transaction commits.
///
/// Each operation (other than `kDeltaTypeDelete`) will create an item of one specific type
/// at the storage key it is applied to.  `kDeltaTypeDelete` will delete the item.
/// These writes overwrite an existing item at the storage key, if any.
/// Two deltas conflict with each other if they would create different types of items
/// at the same storage key (or if one would delete an item that the other would create).
///
/// Beyond that, each subtype of storage delta has its own conflict resolution rules.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[repr(i32)]
#[non_exhaustive]
pub enum StorageDelta {
    /// Creates a bytestring item, with the supplied value.
    ///
    /// Item Type: `kItemTypeBytestring`
    ///
    /// Conflicts: 
    ///   - Will conflict with any other delta that creates a *different* item at the same storage key.
    // case kDeltaTypeBytestringSet:
    //   Bytestring bytestring;
    kDeltaTypeBytestringSet { bytestring: Bytestring } = StorageDeltaType::kDeltaTypeBytestringSet.0 as i32,
    /// Creates a nonnegative integer item.
    /// The value of the int will be equal to the `set_value`, plus the sum of all the `add_value`s supplied
    /// by this delta and all NonnegativeInt128SetAdd and NonnegativeInt128Add deltas in the same block.
    ///
    /// Item Type: `kItemTypeNonnegativeInt`
    ///
    /// Conflicts: 
    ///   - Will conflict with any NonnegativeIntSetAdd delta with a different `set_value`.
    ///   - NonnegativeIntAdd deltas are equivalent to a NonnegativeIntSetAdd delta with a `set_value` equal
    ///     to the value of the item at the start of the block (if it is a nonnegative integer) or 0 otherwise.
    ///     If that implicit set_value is not the same as the supplied `set_value`, the deltas will conflict.
    ///   - At the end of a block, the value will be `set_value` plus the sum of all `add_values` supplied by this
    ///     delta and other deltas from the same block.  Groundhog maintains the property that this expression
    ///     does not overflow a 128-bit integer, and that that value is always at least `min(set_value, 0)`.
    // case kDeltaTypeNonnegativeIntSetAdd:
    //   NonnegativeInt128SetAddArgs nnint;
    kDeltaTypeNonnegativeIntSetAdd { nnint: NonnegativeInt128SetAddArgs } = StorageDeltaType::kDeltaTypeNonnegativeIntSetAdd.0 as i32,
    /// Adds the supplied value to the nonnegative integer item at the storage key.
    /// This type is equivalent to a NonnegativeIntSetAdd delta with a `set_value` 
    /// equal to the value of the item at the start of the block (if it is a nonnegative integer) or 0 otherwise.
    /// This delta type enables a more efficient implementation of common nonnegative integer use cases (like token balances).
    ///
    /// Item Type: `kItemTypeNonnegativeInt`
    ///
    /// Conflicts: 
    ///   - Same as NonnegativeIntSetAdd deltas.
    // case kDeltaTypeNonnegativeIntAdd:
    //   int128_xdr nnint_add;
    kDeltaTypeNonnegativeIntAdd { nnint_add: int128_xdr } = StorageDeltaType::kDeltaTypeNonnegativeIntAdd.0 as i32,
    /// Deletes the item at the storage key.
    ///
    /// Item Type: None (deletes the item at the storage key)
    ///
    /// Conflicts: 
    ///   - Will conflict with any other delta that creates a *different* item at the same storage key.
    // case kDeltaTypeDelete:
    //   void;
    kDeltaTypeDelete = StorageDeltaType::kDeltaTypeDelete.0 as i32,
    /// Consumes or inserts entries into the retrocausal queue.
    ///
    /// Item Type: `kItemTypeRetrocausalQueue`
    ///
    /// Conflicts: 
    ///   - Insertions do not conflict with each other.
    ///     TODO(geoff): we may impose limits on the number of entries that can be inserted in one block.
    ///   - Consuming entries conflicts with other consumption deltas in the same block.
    // case kDeltaTypeRetrocausalQueue:
    //   RetrocausalQueueDelta retrocausal_queue;
    kDeltaTypeRetrocausalQueue { retrocausal_queue: RetrocausalQueueDelta } = StorageDeltaType::kDeltaTypeRetrocausalQueue.0 as i32,
    /// Changes the authorization data for the current address.
    ///
    /// Item Type: `kItemTypeAuthData`
    ///
    /// Conflicts: 
    ///   - Will conflict with any other auth change delta in the same block.
    // case kDeltaTypeAuthDataChange:
    //   AuthDataChangeArgs auth_data_change;
    kDeltaTypeAuthDataChange { auth_data_change: AuthDataChangeArgs } = StorageDeltaType::kDeltaTypeAuthDataChange.0 as i32,
    /// Deploys a new contract for the current address.
    /// 
    /// Item Type: `kItemTypeDeployedContractHash`
    ///
    /// Conflicts: 
    ///   - None.  The semantics of how contract deployment addresses are calculated (see [`ContractDeploymentHashPreimage`])
    ///     if two transactions deploy a contract to the same address, 
    ///     then both transactions are deploying the exact same contract.
    // case kDeltaTypeDeployContract:
    //   DeployedContractHash deployed_contract_hash;
    kDeltaTypeDeployContract { deployed_contract_hash: DeployedContractHash } = StorageDeltaType::kDeltaTypeDeployContract.0 as i32,
    /// Add a [`Uniquifier`] to the replay cache.
    ///
    /// Item Type: kItemTypeReplayCacheData 
    ///
    /// Conflicts: 
    ///   - Will conflict with any other attempt to consume the same [`Uniquifier`].
    // case kDeltaTypeConsumeUniquifier:
    //   ConsumeUniquifierArgs consume_uniquifier_args;
    kDeltaTypeConsumeUniquifier { consume_uniquifier_args: ConsumeUniquifierArgs } = StorageDeltaType::kDeltaTypeConsumeUniquifier.0 as i32,
    
}

/// Maximum heights with an unexpired consumed uniquifier in a cache.
pub const kReplayCacheHeightsMaxLen : u32 = 4000;

/// Maximum unexpired consumed uniquifiers allowed per height in a cache.
pub const kReplayCacheNoncesMaxLen : u32 = 8000;

/// Maximum number of entries in a single RetrocausalQueue block.
/// Each entry is at most 256 bytes, so 1000 entries is ~256KB.
pub const kRetrocausalQueueBlockEntriesMaxLen : u32 = 1000;
// 
// Information about the number of nonces stored at each height of a
// ReplayCache. Tracked in the ReplayCacheMetadata so nonce GC can
// determine the change cost after some heights are deleted without
// having to load the nonces at that height.

#[doc(hidden)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ReplayCacheHeightCount {
    pub height: BlockHeight,
    pub nonce_count: u32,
}
// 

#[doc(hidden)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ReplayCacheMetadata {
    // Total count of all nonces in this ReplayCache across all heights.
    pub size: i64,
    // Sorted list of heights with active nonces and count at each.
    pub heights: VecM<ReplayCacheHeightCount, kReplayCacheNoncesMaxLen>,
}
// 

#[doc(hidden)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ReplayCacheData {
    // Sorted list of nonces for a particular height of a ReplayCache.
    pub nonces: VecM<u64, kReplayCacheNoncesMaxLen>,
}

/// Flags that can be set on a storage delta to modify its behavior.
/// Multiple flags can be combined using bitwise OR.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct StorageDeltaFlags(pub i32);

impl StorageDeltaFlags {
    pub const None: Self = Self(0);

    /// Forces the delta to acquire an exclusive lock on the item at the storage key,
    /// regardless of any other conflict resolution rules.
    pub const ExclusiveLock: Self = Self(0x00000001);
}
impl From<i32> for StorageDeltaFlags {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for StorageDeltaFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::None.0 => {
                write!(f, "StorageDeltaFlags::None")
            },
            x if x == Self::ExclusiveLock.0 => {
                write!(f, "StorageDeltaFlags::ExclusiveLock")
            },
            _ => {
                write!(f, "StorageDeltaFlags({})", self.0)
            }
        }
    }
}


/// A bitset of StorageDeltaFlags values.
pub type StorageDeltaFlagSet = i32;

/// The combined unit of storage modification sent to the database from the executor.
/// A delta with a set of flags, applied to a specific entry in the database.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct FlaggedStorageDelta {
    // TODO(geoff): The addrkey field is likely repetitive when sending 
    // many deltas to the database.
    // possibly we can readjust this data structure, if raw bandwidth becomes
    // important.
    pub addrkey: AddressAndKey,
    pub delta: StorageDelta,
    pub flags: StorageDeltaFlagSet,
}


//
// XDR internals below this point
//

impl StorageItem {
    pub fn r#type(&self) -> StorageItemType {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl WireStorageItem {
    pub fn r#type(&self) -> StorageItemType {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl StorageDelta {
    pub fn r#type(&self) -> StorageDeltaType {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


