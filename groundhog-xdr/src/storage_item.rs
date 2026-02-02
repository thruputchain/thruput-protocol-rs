// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 4cdeb628f95f47a7054bbf893ab281e195473e83

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

#[cfg(feature = "std")]
use std::io::{Read, Write};

#[cfg(feature = "xdrs")]
use xdrs::*;
#[cfg(feature = "spec")]
use xdrs::xdr_repr::*;


#[cfg(feature = "serde")]
use ::serde::{Serialize, Deserialize};
#[cfg(feature = "serde")]
use ::serde_arrays;



/// The type of a storage item.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(i32)]
pub enum StorageItemType {
    kItemTypeBytestring = 0 as i32,
    kItemTypeNonnegativeInt = 1 as i32,
    kItemTypeRetrocausalQueue = 2 as i32,
    kItemTypeReplayCacheData = 3 as i32,
    kItemTypeAuthData = 4 as i32,
    kItemTypeDeployedContractHash = 5 as i32,

    // 
    // This will be removed when we get rid of the old db interface.
    
    #[doc(hidden)]kItemTypeDeleted = -1 as i32,
}
impl TryFrom<i32> for StorageItemType {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::kItemTypeBytestring),
            1 => Ok(Self::kItemTypeNonnegativeInt),
            2 => Ok(Self::kItemTypeRetrocausalQueue),
            3 => Ok(Self::kItemTypeReplayCacheData),
            4 => Ok(Self::kItemTypeAuthData),
            5 => Ok(Self::kItemTypeDeployedContractHash),
            -1 => Ok(Self::kItemTypeDeleted),
            _ => Err(()),
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
#[repr(i32)]
pub enum StorageItem {
    /// An arbitrary value, stored as an opaque string of bytes.
    // case kItemTypeBytestring:
    //   Bytestring bytestring;
    kItemTypeBytestring { bytestring: Bytestring } = StorageItemType::kItemTypeBytestring as i32,
    /// A nonnegative integer, stored as a 128-bit value.
    ///
    /// This can be stored at any [`StorageKey`] of type `kStorageKeyTypeContractData`.
    /// This type is also used to store the free native token balance at 
    /// storage key `kStorageKeyTypeFreeNativeTokenBalance`.
    // case kItemTypeNonnegativeInt:
    //   int128_xdr nnint;
    kItemTypeNonnegativeInt { nnint: int128_xdr } = StorageItemType::kItemTypeNonnegativeInt as i32,
    /// A retrocausal multi-producer, single-consumer queue.
    /// Tranactions can only insert entries into the queue or consume entries one by one
    /// from the queue, so the whole queue is generally not loaded all at once.
    // case kItemTypeRetrocausalQueue:
    //   VecM<QueueEntry> queue_entries;
    kItemTypeRetrocausalQueue { queue_entries: VecM<QueueEntry> } = StorageItemType::kItemTypeRetrocausalQueue as i32,
    /// A set of [`Uniquifier`]s that have been used by the current address.
    /// Transactions can only insert a Uniquifier, not read the whole set.
    /// 
    /// This is the type stored at storage key `kStorageKeyTypeReplayCache`.
    /// It cannot be stored at any other storage key.
    // case kItemTypeReplayCacheData:
    //   VecM<Uniquifier> active_uniquifiers;
    kItemTypeReplayCacheData { active_uniquifiers: VecM<Uniquifier> } = StorageItemType::kItemTypeReplayCacheData as i32,
    /// The authorization data for the current address.
    ///
    /// This is the type stored at storage key `kStorageKeyTypeAddressAuthData`.
    /// It cannot be stored at any other storage key.
    // case kItemTypeAuthData:
    //   AddressAuthData auth_data;
    kItemTypeAuthData { auth_data: AddressAuthData } = StorageItemType::kItemTypeAuthData as i32,
    /// The hash of the deployed contract for the current address.
    ///
    /// This is the type stored at storage key `kStorageKeyTypeAddressMetadata`.
    /// It cannot be stored at any other storage key.
    // case kItemTypeDeployedContractHash:
    //   DeployedContractHash deployed_contract_hash;
    kItemTypeDeployedContractHash { deployed_contract_hash: DeployedContractHash } = StorageItemType::kItemTypeDeployedContractHash as i32,
    
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
#[repr(i32)]
pub enum WireStorageItem {
    // case kItemTypeBytestring:
    //   Bytestring bytestring;
    kItemTypeBytestring { bytestring: Bytestring } = StorageItemType::kItemTypeBytestring as i32,
    // case kItemTypeNonnegativeInt:
    //   int128_xdr nnint;
    kItemTypeNonnegativeInt { nnint: int128_xdr } = StorageItemType::kItemTypeNonnegativeInt as i32,
    // case kItemTypeAuthData:
    //   AddressAuthData auth_data;
    kItemTypeAuthData { auth_data: AddressAuthData } = StorageItemType::kItemTypeAuthData as i32,
    // case kItemTypeRetrocausalQueue:
    //   void;
    kItemTypeRetrocausalQueue = StorageItemType::kItemTypeRetrocausalQueue as i32,
    // case kItemTypeReplayCacheData:
    //   void;
    kItemTypeReplayCacheData = StorageItemType::kItemTypeReplayCacheData as i32,
    // case kItemTypeDeployedContractHash:
    //   DeployedContractHash deployed_contract_hash;
    kItemTypeDeployedContractHash { deployed_contract_hash: DeployedContractHash } = StorageItemType::kItemTypeDeployedContractHash as i32,
    
}

/// The types of changes accumulated by a transaction that will be applied to the database
/// if the transaction commits.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(i32)]
pub enum StorageDeltaType {
    kDeltaTypeBytestringSet = 0 as i32,
    kDeltaTypeNonnegativeIntSetAdd = 1 as i32,
    kDeltaTypeDelete = 2 as i32,
    kDeltaTypeRetrocausalQueue = 3 as i32,
    kDeltaTypeNonnegativeIntAdd = 5 as i32,
    kDeltaTypeAuthDataChange = 6 as i32,
    kDeltaTypeDeployContract = 7 as i32,
    kDeltaTypeConsumeUniquifier = 8 as i32,
}
impl TryFrom<i32> for StorageDeltaType {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::kDeltaTypeBytestringSet),
            1 => Ok(Self::kDeltaTypeNonnegativeIntSetAdd),
            2 => Ok(Self::kDeltaTypeDelete),
            3 => Ok(Self::kDeltaTypeRetrocausalQueue),
            5 => Ok(Self::kDeltaTypeNonnegativeIntAdd),
            6 => Ok(Self::kDeltaTypeAuthDataChange),
            7 => Ok(Self::kDeltaTypeDeployContract),
            8 => Ok(Self::kDeltaTypeConsumeUniquifier),
            _ => Err(()),
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct AuthDataChangeSigner {
    pub public_key_id: u32,
    pub public_key: PublicKeyConfigOpt,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct GasConfigChange {
    pub new_gas_permission_configs: VecM<GasPermissionConfig, kMaxGasConfigs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct AuthDataChangeArgs {
    pub auth_data_change_signers: VecM<AuthDataChangeSigner>,
    pub new_nonce_config: Option<NonceConfig>,
    pub new_gas_config_change: Option<GasConfigChange>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct NonnegativeInt128SetAddArgs {
    pub set_value: int128_xdr,
    pub add_value: int128_xdr,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
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
#[repr(i32)]
pub enum StorageDelta {
    /// Creates a bytestring item, with the supplied value.
    ///
    /// Item Type: `kItemTypeBytestring`
    ///
    /// Conflicts: 
    ///   - Will conflict with any other delta that creates a *different* item at the same storage key.
    // case kDeltaTypeBytestringSet:
    //   Bytestring bytestring;
    kDeltaTypeBytestringSet { bytestring: Bytestring } = StorageDeltaType::kDeltaTypeBytestringSet as i32,
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
    kDeltaTypeNonnegativeIntSetAdd { nnint: NonnegativeInt128SetAddArgs } = StorageDeltaType::kDeltaTypeNonnegativeIntSetAdd as i32,
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
    kDeltaTypeNonnegativeIntAdd { nnint_add: int128_xdr } = StorageDeltaType::kDeltaTypeNonnegativeIntAdd as i32,
    /// Deletes the item at the storage key.
    ///
    /// Item Type: None (deletes the item at the storage key)
    ///
    /// Conflicts: 
    ///   - Will conflict with any other delta that creates a *different* item at the same storage key.
    // case kDeltaTypeDelete:
    //   void;
    kDeltaTypeDelete = StorageDeltaType::kDeltaTypeDelete as i32,
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
    kDeltaTypeRetrocausalQueue { retrocausal_queue: RetrocausalQueueDelta } = StorageDeltaType::kDeltaTypeRetrocausalQueue as i32,
    /// Changes the authorization data for the current address.
    ///
    /// Item Type: `kItemTypeAuthData`
    ///
    /// Conflicts: 
    ///   - Will conflict with any other auth change delta in the same block.
    // case kDeltaTypeAuthDataChange:
    //   AuthDataChangeArgs auth_data_change;
    kDeltaTypeAuthDataChange { auth_data_change: AuthDataChangeArgs } = StorageDeltaType::kDeltaTypeAuthDataChange as i32,
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
    kDeltaTypeDeployContract { deployed_contract_hash: DeployedContractHash } = StorageDeltaType::kDeltaTypeDeployContract as i32,
    /// Add a [`Uniquifier`] to the replay cache.
    ///
    /// Item Type: kItemTypeReplayCacheData 
    ///
    /// Conflicts: 
    ///   - Will conflict with any other attempt to consume the same [`Uniquifier`].
    // case kDeltaTypeConsumeUniquifier:
    //   ConsumeUniquifierArgs consume_uniquifier_args;
    kDeltaTypeConsumeUniquifier { consume_uniquifier_args: ConsumeUniquifierArgs } = StorageDeltaType::kDeltaTypeConsumeUniquifier as i32,
    
}

/// Maximum heights with an unexpired consumed uniquifier in a cache.
pub const kReplayCacheHeightsMaxLen : u32 = 4000;

/// Maximum unexpired consumed uniquifiers allowed per height in a cache.
pub const kReplayCacheNoncesMaxLen : u32 = 8000;
// 
// Information about the number of nonces stored at each height of a
// ReplayCache. Tracked in the ReplayCacheMetadata so nonce GC can
// determine the change cost after some heights are deleted without
// having to load the nonces at that height.

#[doc(hidden)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
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
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ReplayCacheData {
    // Sorted list of nonces for a particular height of a ReplayCache.
    pub nonces: VecM<i64, kReplayCacheNoncesMaxLen>,
}

/// Flags that can be set on a storage delta to modify its behavior.
/// Multiple flags can be combined using bitwise OR.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(i32)]
pub enum StorageDeltaFlags {
    kStorageDeltaFlagNone = 0 as i32,

    /// Forces the delta to acquire an exclusive lock on the item at the storage key,
    /// regardless of any other conflict resolution rules.
    kStorageDeltaFlagExclusiveLock = 0x00000001 as i32,
}
impl TryFrom<i32> for StorageDeltaFlags {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::kStorageDeltaFlagNone),
            0x00000001 => Ok(Self::kStorageDeltaFlagExclusiveLock),
            _ => Err(()),
        }
    }
}


/// A bitset of StorageDeltaFlags values.
pub type StorageDeltaFlagSet = i32;


//
// XDR internals below this point
//

impl StorageItem {
    pub fn type_(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl WireStorageItem {
    pub fn type_(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl StorageDelta {
    pub fn type_(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}


