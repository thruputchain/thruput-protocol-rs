# Storage Item Module

This module defines the storage model for Thruput contracts. It specifies the types of values that can be stored at storage keys, and the types of modifications (deltas) that transactions can make to those values.

## Overview

Every storage key in Thruput can hold one of several distinct value types, represented by the `StorageItem` enum. Transactions don't directly read or write these values - instead, via the typed set of system calls available to a contract,
a transaction accumulates as it executes a set of `StorageDelta` objects that describe modifications to be 
applied if the transaction commits.

These conflict resolution rules are the key idea that makes Thruput unique.
These are the tools that contract developers can use to express what kinds of concurrency are and are not allowed
in their contract.  Thruput does not understand what happens inside of an RV32 contract.  Instead,
contracts tell Thruput, via these typed interfaces, how Thruput should resolve concurrent modifications to their storage items.

## Storage Items

The `StorageItem` enum defines the types of values that can exist at a storage key:

### kItemTypeBytestring

```rust
# use thruput_xdr::{storage_item::*, types::*, retrocausal_queue::*, transaction::*, address_auth::*, contract::*};
# fn shape(item: StorageItem) {
if let StorageItem::kItemTypeBytestring { bytestring } = item {
    let _: Bytestring = bytestring; // BytesM<kBytestringCeilingLen>
}
# }
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

```rust
# use thruput_xdr::{storage_item::*, types::*, retrocausal_queue::*, transaction::*, address_auth::*, contract::*};
# fn shape(item: StorageItem) {
if let StorageItem::kItemTypeNonnegativeInt { nnint } = item {
    let _: int128_xdr = nnint;
}
# }
```

This type is used both for general contract data at `StorageKeyType::ContractData` keys and for tracking free native token balances at `StorageKeyType::FreeNativeTokenBalance` keys.

The nonnegative integer type supports efficient parallel updates through addition operations that can be merged without conflicts (see `StorageDelta` below).

**Common uses**: Token balances, counters, or any kind of linear constraint on contract variables (such as borrowing limits in a lending contract)

### kItemTypeRetrocausalQueue

A retrocausal multi-producer, single-consumer queue.

```rust
# use thruput_xdr::{storage_item::*, types::*, retrocausal_queue::*, transaction::*, address_auth::*, contract::*};
# fn shape(item: StorageItem) {
if let StorageItem::kItemTypeRetrocausalQueue { queue_entries } = item {
    let _: &[QueueEntry] = &queue_entries;
}
# }
```

This specialized data structure allows multiple transactions to insert entries (producers) while a single consumer can pop entries one at a time. The queue is typically not loaded all at once - operations work on individual entries.

The common use case for this structure is a contract that, for whatever reason, needs strictly sequential semantics, and
cannot use any of Thruput's other concurrency tools.  Another contract can ``asynchronously invoke'' the contract by inserting a message into the queue (and wait for a response).  Meanwhile, a wrapper contract can consume entries one by one, and invoke the sequential contract.

```text
// pseudocode for the wrapper contract
while !queue.empty() {
    let message = queue.pop();
    SequentialContractClient::invoke_method(&message);
}
```

**Common uses**: Message queues, sequential contracts

### kItemTypeReplayCacheData

A set of `Uniquifier` values that have been consumed by the current address.

```rust
# use thruput_xdr::{storage_item::*, types::*, retrocausal_queue::*, transaction::*, address_auth::*, contract::*};
# fn shape(item: StorageItem) {
if let StorageItem::kItemTypeReplayCacheData { active_uniquifiers } = item {
    let _: &[Uniquifier] = &active_uniquifiers;
}
# }
```

This type can only exist at storage key type `StorageKeyType::ReplayCache`. Each transaction has an embedded `Uniquifier`, which
is inserted into the set if the transaction commits. The system automatically garbage collects expired uniquifiers.

**Storage key restriction**: Only at `StorageKeyType::ReplayCache`

### kItemTypeAuthData

The authorization data for an address: its public keys, their weights, and the required weight.

```rust
# use thruput_xdr::{storage_item::*, types::*, retrocausal_queue::*, transaction::*, address_auth::*, contract::*};
# fn shape(item: StorageItem) {
if let StorageItem::kItemTypeAuthData { auth_data } = item {
    let _: AddressAuthData = auth_data;
}
# }
```

This type can only exist at storage key type `StorageKeyType::AuthData`.
Through this data item, a contract tells the rest of the Thruput system under what conditions is a transaction allowed to originate
at that contract's address (and i.e. use that address's gas).

**Storage key restriction**: Only at `StorageKeyType::AuthData`

### kItemTypeDeployedContractHash

The hash of the deployed contract code for an address.

```rust
# use thruput_xdr::{storage_item::*, types::*, retrocausal_queue::*, transaction::*, address_auth::*, contract::*};
# fn shape(item: StorageItem) {
if let StorageItem::kItemTypeDeployedContractHash { deployed_contract_hash } = item {
    let _: DeployedContractHash = deployed_contract_hash;
}
# }
```

This type can only exist at storage key type `StorageKeyType::Metadata`. It records what contract code is deployed at an address.

**Storage key restriction**: Only at `StorageKeyType::Metadata`

## Storage Deltas

Transactions don't directly modify storage items. Instead, they accumulate `StorageDelta` objects that describe intended modifications. Conflicts are detected when a transaction commits: two-phase commit locks each item it changes, and a delta that conflicts with one already locked in the block fails that transaction. When the block closes, the surviving deltas are applied.

### Basic Conflict Rules

All deltas follow these fundamental conflict rules:

1. **Type consistency**: Deltas that would create different types at the same storage key conflict
2. **Delete conflicts**: A delete delta conflicts with any delta that would create an item
3. **Type-specific rules**: Each delta type has additional conflict resolution logic (detailed below)

### Delta Types

#### kDeltaTypeBytestringSet

Creates or replaces a bytestring item with the supplied value.

```rust
# use thruput_xdr::{storage_item::*, types::*, retrocausal_queue::*, transaction::*, address_auth::*, contract::*};
# fn shape(delta: StorageDelta) {
if let StorageDelta::kDeltaTypeBytestringSet { bytestring } = delta {
    let _: Bytestring = bytestring;
}
# }
```

**Item type created**: `kItemTypeBytestring`

**Conflicts**:
- Conflicts with any delta that creates a different item type at the same key
- If multiple `BytestringSet` deltas target the same key with different values, they conflict

#### kDeltaTypeNonnegativeIntSetAdd

Creates a nonnegative integer item with a base value plus additions.

```rust
# use thruput_xdr::{storage_item::*, types::*, retrocausal_queue::*, transaction::*, address_auth::*, contract::*};
# fn shape(delta: StorageDelta) {
if let StorageDelta::kDeltaTypeNonnegativeIntSetAdd { nnint } = delta {
    let NonnegativeInt128SetAddArgs { set_value, add_value } = nnint;
    let _: int128_xdr = set_value; // Base value
    let _: int128_xdr = add_value; // Amount to add
}
# }
```

The final value will be: `set_value + sum(all add_values from all deltas in the block)`

**Item type created**: `kItemTypeNonnegativeInt`

**Conflicts**:
- Conflicts with any `NonnegativeIntSetAdd` delta with a different `set_value`
- Conflicts with `NonnegativeIntAdd` if the implicit set_value (the value at the start of the block, or 0) differs
- The final value must not overflow 128 bits and must be at least `min(set_value, 0)`

Note that we explicitly allow `set_value` to be less than zero.  This is useful 
in niche edge cases where some constraint that a contract wishes to maintain has been violated,
due to events outside a contract (such as movements in the price of collateral of a lending contract).

#### kDeltaTypeNonnegativeIntAdd

Adds a value to the existing nonnegative integer (or creates one starting from 0).

```rust
# use thruput_xdr::{storage_item::*, types::*, retrocausal_queue::*, transaction::*, address_auth::*, contract::*};
# fn shape(delta: StorageDelta) {
if let StorageDelta::kDeltaTypeNonnegativeIntAdd { nnint_add } = delta {
    let _: int128_xdr = nnint_add;
}
# }
```

This is equivalent to `NonnegativeIntSetAdd` with `set_value` equal to the item's value at the start of the block (or 0 if none exists). This delta type enables more efficient implementation of common operations like token transfers.

**Item type created**: `kItemTypeNonnegativeInt`

**Conflicts**:
- Same conflict rules as `NonnegativeIntSetAdd`

#### kDeltaTypeDelete

Deletes the item at the storage key.

```rust
# use thruput_xdr::storage_item::*;
let delta = StorageDelta::kDeltaTypeDelete;
```

**Item type created**: None (deletes existing item)

**Conflicts**:
- Conflicts with any delta that creates an item at the same key

#### kDeltaTypeRetrocausalQueue

Inserts entries into or consumes entries from a retrocausal queue.

```rust
# use thruput_xdr::{storage_item::*, types::*, retrocausal_queue::*, transaction::*, address_auth::*, contract::*};
# fn shape(delta: StorageDelta) {
if let StorageDelta::kDeltaTypeRetrocausalQueue { retrocausal_queue } = delta {
    let _: RetrocausalQueueDelta = retrocausal_queue;
}
# }
```

**Item type created**: `kItemTypeRetrocausalQueue`

**Conflicts**:
- Insert operations don't conflict with each other
- Consume operations conflict with other consume operations in the same block
- At most `kRetrocausalQueueBlockEntriesMaxLen` (250) entries can be inserted into one queue per block

#### kDeltaTypeAuthDataChange

Modifies the authorization data for an address.

```rust
# use thruput_xdr::{storage_item::*, types::*, retrocausal_queue::*, transaction::*, address_auth::*, auth_claim::*, contract::*};
# fn shape(delta: StorageDelta) {
if let StorageDelta::kDeltaTypeAuthDataChange { auth_data_change } = delta {
    let AuthDataChangeArgs { auth_data_change_signers, new_required_weight } = auth_data_change;
    let _: &[AuthDataChangeSigner] = &auth_data_change_signers; // at most kMaxSigners
    let _: Option<Weight> = new_required_weight;
}
# }
```

**Item type created**: `kItemTypeAuthData`

**Conflicts**:
- Conflicts with any other auth change delta in the same block
- Only one authorization change per address per block

#### kDeltaTypeDeployContract

Deploys a contract at an address.

```rust
# use thruput_xdr::{storage_item::*, types::*, retrocausal_queue::*, transaction::*, address_auth::*, contract::*};
# fn shape(delta: StorageDelta) {
if let StorageDelta::kDeltaTypeDeployContract { deployed_contract_hash } = delta {
    let _: DeployedContractHash = deployed_contract_hash;
}
# }
```

**Item type created**: `kItemTypeDeployedContractHash`

**Conflicts**:
- Never conflicts with another deployment of the same contract: the address is derived from the contract hash (see `ContractDeploymentHashPreimage`), so two transactions deploying to the same address deploy identical code
- Like every delta, it conflicts with a delta that creates a different item type at the same key

#### kDeltaTypeConsumeUniquifier

Adds a `Uniquifier` to the replay cache to prevent replay attacks.

```rust
# use thruput_xdr::{storage_item::*, types::*, retrocausal_queue::*, transaction::*, address_auth::*, contract::*};
# fn shape(delta: StorageDelta) {
if let StorageDelta::kDeltaTypeConsumeUniquifier { consume_uniquifier_args } = delta {
    let ConsumeUniquifierArgs { uniquifier } = consume_uniquifier_args;
    let _: Uniquifier = uniquifier;
}
# }
```

**Item type created**: `kItemTypeReplayCacheData`

**Conflicts**:
- Conflicts with any other delta attempting to consume the same `Uniquifier`
- This ensures uniquifiers are truly unique

## Storage Delta Flags

Deltas can have flags that modify their behavior:

### StorageDeltaFlags::None

No special flags (default).

### StorageDeltaFlags::ExclusiveLock

Forces the delta to acquire an exclusive lock on the storage key, overriding normal conflict resolution rules. When this flag is set, the delta conflicts with any other delta targeting the same key, regardless of type-specific merge rules. It is not allowed on free native token balances (the transaction conflicts) or on replay cache deltas.

```rust
# use thruput_xdr::storage_item::*;
let flags: StorageDeltaFlagSet = StorageDeltaFlags::ExclusiveLock.0 as u32; // Bitset of flags
```

**Use case**: When a contract needs to ensure no other transaction modifies a key in the same block, even if the modifications would normally be mergeable.
