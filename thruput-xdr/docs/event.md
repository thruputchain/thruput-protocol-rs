# Event Module

This module defines the event system for Thruput contracts. Events provide a mechanism for contracts to log structured data that can be efficiently indexed, queried, and monitored by external systems.

## Emitting Events: The LogEvent Syscall

Contracts emit events using the LogEvent syscall. The syscall signature is

```rust,ignore
fn LogEvent(
    topic0: Option<&[u8; 32]>,
    topic1: Option<&[u8; 32]>,
    topic2: Option<&[u8; 32]>,
    data: &[u8]
) -> SyscallResult<()>;
```

In practice, contracts should use SDK wrapper functions that handle event serialization and topic construction rather than calling LogEvent directly.

### Example Usage

Here's a conceptual example of emitting a transfer event:

```rust,ignore
// Define your event data (excluding topics)
let event_data = b"some event data";

// Calculate event signature
let event_signature = sha256(b"Transfer");

// Emit the event with topics
// LogEvent will construct serialized_event as event_signature || sender || recipient || event_data
LogEvent(
    Some(&event_signature),     // topic0: event type signature
    Some(&sha256(b"alice")),    // topic1: from address
    Some(&sha256(b"bob")),      // topic2: to address
    event_data                  // remaining data after topics
).unwrap();
```

The resulting `TxEmittedEvent` will have:
- `contract_address`: Your contract's address (automatically set)
- `num_topics`: 3
- `serialized_event`: `event_signature || sender_address || recipient_address || other_event_data`

This event can then be efficiently queried by downstream indexers using:
- Transaction hash (from the `TxEventLog` key)
- Event signature (topic0) - to find all Transfer events
- Sender address (topic1) - to find all transfers from a specific address
- Recipient address (topic2) - to find all transfers to a specific address

### How It Works

When LogEvent is called, it constructs a `TxEmittedEvent` structure where:

1. **Contract calls LogEvent**: Provides up to 3 topics and event data
2. **TxEmittedEvent construction**: The `serialized_event` field is built as `topic0 || topic1 || topic2 || data`
3. **Storage**: The event is appended to the transaction's event log

The `num_topics` field in `TxEmittedEvent` tells downstream systems how many of the leading 32-byte chunks should be treated as indexed topics.

### Topic Indexing Pattern

Topics are purely for downstream indexers - they indicate which fields you want indexed for efficient querying. When you provide topics to LogEvent, you're essentially marking the first N * 32 bytes of your event as "these are the searchable fields."

For example, a transfer event might have:
- `topic0`: Event signature (hash of "Transfer")
- `topic1`: From address (32 bytes)
- `topic2`: To address (32 bytes)
- `data`: The amount and any additional fields

Downstream indexers can then efficiently filter events by "all transfers from address X" without deserializing every event.

### Error Conditions

The `LogEvent` syscall returns an error if:
- **Data too large**: Event data exceeds `kMaxEventLength` (1024 bytes)
- **Invalid topic sequence**: Topics are not provided in a continuous sequence (e.g., topic0 and topic2 without topic1)

## Event Storage

During block execution, the system accumulates events from all transactions. At the end of the block, these events are organized into a merkle-patricia trie and hashed to produce a block-level event root.

### TxEventLog Structure

Events are organized per-transaction using the `TxEventLog` structure:

- **Key**: `tx_hash` - SHA256 hash of the XDR-encoded `Transaction`
- **Value**: Array of `TxEmittedEvent` structures (all events emitted by that transaction)

This organization allows efficient retrieval of all events emitted by a specific transaction.

### TxEmittedEvent Structure

Each individual event contains:
- `contract_address`: The address of the contract that emitted the event
- `num_topics`: Number of topics (0-3) prepended to the data
- `serialized_event`: The complete event data as `topic0 || topic1 || ... || data`

### Block-Level Merkle Trie

Within each block:
1. As transactions execute, their events are collected into `TxEventLog` entries
2. These entries are inserted into a merkle-patricia trie, keyed by `tx_hash`
3. At block close, the trie is finalized and a root hash is computed
4. This root hash becomes part of the block's cryptographic commitment

### IdentifiedEvent

The `IdentifiedEvent` structure is not constructed during the main transaction workflow. Instead, it may be produced by downstream systems (indexers, event explorers, APIs) that cross-reference events across transactions and blocks.

It provides complete identification of a unique event:
- `tx_hash`: The transaction that emitted the event
- `sequence_number`: Distinguishes multiple events from the same transaction (0-indexed)
- `emitted_event`: The complete event data

The combination of `tx_hash` and `sequence_number` uniquely identifies any event in the blockchain's history.

## Contract Developer Guidelines

When designing events for your contracts:

1. **Topics are searchable fields**: Choose which fields to expose as topics (up to 3) based on how you expect indexers to filter events
2. **32-byte topics**: Each topic must be exactly 32 bytes - use addresses, hashes, or hash larger values
3. **Topic ordering**: Topic0 is typically an event signature, topic1 and topic2 are the primary filterable fields
4. **Remaining data**: Pass any additional event data (beyond topics) in the `data` parameter to LogEvent
5. **Total size limit**: The complete event (topics + data) must be under 1024 bytes (`kMaxEventLength`)
