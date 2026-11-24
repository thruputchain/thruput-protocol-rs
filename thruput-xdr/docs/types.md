# Types Module

This module contains the core type definitions for the Thruput protocol, including cryptographic primitives, storage types, and fundamental protocol constants.

## Overview

The `types` module provides the foundational types used throughout the Thruput protocol. These types are generated from the XDR specification in `protocol/types.x` and include:

- **Cryptographic types**: Ed25519 and ML-DSA-65 keys and signatures
- **Storage types**: Contract addresses, storage keys, and hashes
- **Protocol primitives**: Time points, gas amounts, block heights

## Contract Addresses

Contract code written with `thruput-sdk` should hold the address of a
contract it calls as `thruput_sdk::Address<C>`. That type names the generated
client, or tuple of clients, used to call the contract, so the interface is
stated once, at the declaration.

If you really need an address untyped, [`ContractAddress`] is the bare
32 bytes: for an address that is never called, such as an owner or a
recipient, for one whose interface is not known, and for code outside a
contract. The two encode identically.

## Time and Height Arithmetic

Arithmetic on [`TimePoint`] with [`TimeSpan`], the length of time between
two timepoints, and on [`BlockHeight`] with a signed number of blocks. All
three are `Copy`.

```
use thruput_xdr::types::{BlockHeight, TimePoint, TimeSpan};

let now = TimePoint::UNIX_EPOCH + TimeSpan::from_secs(1_700_000_000);
let deadline = now + TimeSpan::from_days(7);
assert!(now < deadline);
assert_eq!(deadline - now, TimeSpan::from_hours(7 * 24));

let height = BlockHeight::new(1_000);
let expiry = height + 100;
assert_eq!(expiry - height, 100);
```

A `TimeSpan` counts in the unit of `TimePoint` and encodes as a timepoint
does. Build one with [`TimeSpan::from_secs`] and its siblings, so that code
does not depend on the unit. [`TimeSpan::from_duration`] and `Duration::from`
convert from and to [`core::time::Duration`].

Every `+`, `-` and `*` operator on these types checks for overflow in every
build profile and panics when it happens, as `core::time` does and the
integer primitives do not: a deadline that silently wrapped into the past
would be a security bug in a contract. The `checked_*` methods (and, on
`TimePoint`, the `saturating_*` methods) handle the edge without panicking.

Subtracting one timepoint from another saturates to [`TimeSpan::ZERO`] when
the right-hand side is later, as `std::time::Instant` does; use
[`TimePoint::checked_duration_since`] to detect that case. Subtracting one
height from another is a plain signed difference.
