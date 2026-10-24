# Storage Item Module

This module defines the storage model for Thruput contracts. It specifies the types of values that can be stored at storage keys, and the types of modifications (deltas) that transactions can make to those values.

## Overview

Every storage key in Thruput can hold one of several distinct value types, represented by the `StorageItem` enum. Transactions don't directly read or write these values - instead, via the typed set of system calls available to a contract,
a transaction accumulates as it executes a set of `StorageDelta` objects that describe modifications to be 
applied if the transaction commits.

These conflict resolution rules are the key idea that makes Thruput unique.
These are the tools that contract developers can use to express what kinds of concurrency are and are not allowed
in their contract.  Thruput does not understand what happens inside a contract's code.  Instead,
contracts tell Thruput, via these typed interfaces, how Thruput should resolve concurrent modifications to their storage items.

## Storage Items

The `StorageItem` enum defines the types of values that can exist at a storage key:

### kItemTypeBytestring

```rust,ignore
StorageItem::kItemTypeBytestring {
    bytestring: BytesM<kBytestringCeilingLen>
}
```

An arbitrary value stored as an opaque string of bytes. The wire type accepts
up to 65,000 bytes (`kBytestringCeilingLen`), while execution enforces the
chain's `kConfigParamMaxBytestringLen` setting, which defaults to 1,024 bytes.
Values that fit the wire type can still exceed the configured execution limit.

This is the most generic storage type, allowing contracts to store any serialized data structure.  Contracts can read and write strings with
the `BytestringGet` and `BytestringSet` system calls, respectively.
This type also gives the Thruput runtime the least information, so Thruput can do almost nothing to resolve
concurrent writes to a bytestring.

**Common uses**: Custom data structures, configuration parameters

### kItemTypeNonnegativeInt

A nonnegative 128-bit integer value.

```rust,ignore
StorageItem::kItemTypeNonnegativeInt {
    nnint: int128_xdr
}
```

This type is used both for general contract data at `StorageKeyType::ContractData` keys and for tracking free native token balances at `StorageKeyType::FreeNativeTokenBalance` keys.

The nonnegative integer type supports efficient parallel updates through addition operations that can be merged without conflicts (see `StorageDelta` below).

**Common uses**: Token balances, counters, or any kind of linear constraint on contract variables (such as borrowing limits in a lending contract)

### kItemTypeRetrocausalQueue

A retrocausal multi-producer, single-consumer queue.

```rust,ignore
StorageItem::kItemTypeRetrocausalQueue {
    queue_entries: VecM<QueueEntry>
}
```

This specialized data structure allows multiple transactions to insert entries (producers) while a single consumer can pop entries one at a time. The queue is typically not loaded all at once - operations work on individual entries.

The common use case for this structure is a contract that, for whatever reason, needs strictly sequential semantics, and
cannot use any of Thruput's other concurrency tools.  Another contract can ``asynchronously invoke'' the contract by inserting a message into the queue (and wait for a response).  Meanwhile, a wrapper contract can consume entries one by one, and invoke the sequential contract.

```rust,ignore
// pseudocode for the wrapper contract
while !queue.empty() {
    let message = queue.pop();
    SequentialContractClient::invoke_method(&message);
}
```

**Common uses**: Message queues, sequential contracts

### kItemTypeReplayCacheData

A set of `Uniquifier` values that have been consumed by the current address.

```rust,ignore
StorageItem::kItemTypeReplayCacheData {
    active_uniquifiers: VecM<Uniquifier>
}
```

This type can only exist at storage key `StorageKeyType::ReplayCache`. Each transaction has an embedded `Uniquifier`, which
is inserted into the set if the transaction commits. The system automatically garbage collects expired uniquifiers.

**Storage key restriction**: Only at `StorageKeyType::ReplayCache`

### kItemTypeAuthData

The authorization data for an address: its public keys and the weight required to authorize a transaction.

```rust,ignore
StorageItem::kItemTypeAuthData {
    auth_data: AddressAuthData
}
```

This type can only exist at storage key `StorageKeyType::AuthData`. 
Through this data item, a contract tells the rest of the Thruput system under what conditions is a transaction allowed to originate
at that contract's address (and i.e. use that address's gas).

**Storage key restriction**: Only at `StorageKeyType::AuthData`

### kItemTypeDeployedContractHash

The hash of the deployed contract code for an address.

```rust,ignore
StorageItem::kItemTypeDeployedContractHash {
    deployed_contract_hash: DeployedContractHash
}
```

This type can only exist at storage key `StorageKeyType::Metadata`. It records what contract code is deployed at an address.

**Storage key restriction**: Only at `StorageKeyType::Metadata`

## Storage Deltas

Transactions don't directly modify storage items. Instead, they accumulate `StorageDelta` objects that describe intended modifications. When a block closes, all deltas from committed transactions are applied atomically with conflict detection.

### Basic Conflict Rules

All deltas follow these fundamental conflict rules:

1. **Type consistency**: Deltas that would create different types at the same storage key conflict
2. **Delete conflicts**: A delete delta conflicts with any delta that would create an item
3. **Type-specific rules**: Each delta type has additional conflict resolution logic (detailed below)

### Delta Types

#### kDeltaTypeBytestringSet

Creates or replaces a bytestring item with the supplied value.

```rust,ignore
StorageDelta::kDeltaTypeBytestringSet {
    bytestring: Bytestring
}
```

**Item type created**: `kItemTypeBytestring`

**Conflicts**:
- Conflicts with any delta that creates a different item type at the same key
- If multiple `BytestringSet` deltas target the same key with different values, they conflict

#### kDeltaTypeNonnegativeIntSetAdd

Creates a nonnegative integer item with a base value plus additions.

```rust,ignore
StorageDelta::kDeltaTypeNonnegativeIntSetAdd {
    nnint: NonnegativeInt128SetAddArgs {
        set_value: int128_xdr,   // Base value
        add_value: int128_xdr,   // Amount to add
    }
}
```

The final value will be: `set_value + sum(all add_values from all deltas in the block)`

**Item type created**: `kItemTypeNonnegativeInt`

**Conflicts**:
- Conflicts with any `NonnegativeIntSetAdd` delta with a different `set_value`
- Conflicts with `NonnegativeIntAdd` if the implicit set_value (current value or 0) differs
- The final value must not overflow 128 bits and must be at least `min(set_value, 0)`

Note that we explicitly allow `set_value` to be less than zero.  This is useful 
in niche edge cases where some constraint that a contract wishes to maintain has been violated,
due to events outside a contract (such as movements in the price of collateral of a lending contract).

#### kDeltaTypeNonnegativeIntAdd

Adds a value to the existing nonnegative integer (or creates one starting from 0).

```rust,ignore
StorageDelta::kDeltaTypeNonnegativeIntAdd {
    nnint_add: int128_xdr
}
```

This is equivalent to `NonnegativeIntSetAdd` with `set_value` equal to the current item value (or 0 if none exists). This delta type enables more efficient implementation of common operations like token transfers.

**Item type created**: `kItemTypeNonnegativeInt`

**Conflicts**:
- Same conflict rules as `NonnegativeIntSetAdd`

#### kDeltaTypeDelete

Deletes the item at the storage key.

```rust,ignore
StorageDelta::kDeltaTypeDelete
```

**Item type created**: None (deletes existing item)

**Conflicts**:
- Conflicts with any delta that creates an item at the same key

#### kDeltaTypeRetrocausalQueue

Inserts entries into or consumes entries from a retrocausal queue.

```rust,ignore
StorageDelta::kDeltaTypeRetrocausalQueue {
    retrocausal_queue: RetrocausalQueueDelta
}
```

**Item type created**: `kItemTypeRetrocausalQueue`

**Conflicts**:
- Insert operations don't conflict with each other
- Consume operations conflict with other consume operations in the same block
- Note: Future versions may impose limits on insertions per block

#### kDeltaTypeAuthDataChange

Modifies the authorization data for an address.

```rust,ignore
StorageDelta::kDeltaTypeAuthDataChange {
    auth_data_change: AuthDataChangeArgs {
        auth_data_change_signers: VecM<AuthDataChangeSigner, kMaxSigners>,
        new_required_weight: Option<Weight>,
    }
}
```

**Item type created**: `kItemTypeAuthData`

**Conflicts**:
- Conflicts with any other auth change delta in the same block
- Only one authorization change per address per block

#### kDeltaTypeDeployContract

Deploys a contract at an address.

```rust,ignore
StorageDelta::kDeltaTypeDeployContract {
    deployed_contract_hash: DeployedContractHash
}
```

**Item type created**: `kItemTypeDeployedContractHash`

**Conflicts**:
- No conflicts! If two transactions deploy to the same address, the semantics of `ContractDeploymentHashPreimage` ensure they're deploying identical code

#### kDeltaTypeConsumeUniquifier

Adds a `Uniquifier` to the replay cache to prevent replay attacks.

```rust,ignore
StorageDelta::kDeltaTypeConsumeUniquifier {
    consume_uniquifier_args: ConsumeUniquifierArgs {
        uniquifier: Uniquifier
    }
}
```

**Item type created**: `kItemTypeReplayCacheData`

**Conflicts**:
- Conflicts with any other delta attempting to consume the same `Uniquifier`
- This ensures uniquifiers are truly unique

## Storage Delta Flags

Deltas can have flags that modify their behavior:

### `StorageDeltaFlags::None`

No special flags (default).

### `StorageDeltaFlags::ExclusiveLock`

Forces the delta to acquire an exclusive lock on the storage key, overriding normal conflict resolution rules. When this flag is set, the delta conflicts with any other delta targeting the same key, regardless of type-specific merge rules. Not every key type supports it; setting it on one that does not fails the commit with `ExclusiveLockDenied`.

```rust,ignore
pub type StorageDeltaFlagSet = u32;  // Bitset of flags
```

**Use case**: When a contract needs to ensure no other transaction modifies a key in the same block, even if the modifications would normally be mergeable.
