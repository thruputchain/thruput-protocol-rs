# Types Module

This module contains the core type definitions for the Thruput protocol, including cryptographic primitives, storage types, and fundamental protocol constants.

## Overview

The `types` module provides the foundational types used throughout the Thruput protocol. These types are generated from the XDR specification in `protocol/types.x` and include:

- **Cryptographic types**: Ed25519 and ML-DSA-65 keys and signatures
- **Storage types**: Contract addresses, storage keys, and hashes
- **Protocol primitives**: Time points, gas amounts, block heights

## Time and Height Arithmetic

Arithmetic on [`TimePoint`] with [`core::time::Duration`], and on
[`BlockHeight`] with a signed number of blocks.

```
use core::time::Duration;
use thruput_xdr::types::{BlockHeight, TimePoint};

let now = TimePoint::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
let deadline = &now + 7 * Duration::from_hours(24);
assert!(now < deadline);
assert_eq!(&deadline - &now, Duration::from_hours(7 * 24));

let height = BlockHeight::new(1_000);
let expiry = &height + 100;
assert_eq!(&expiry - &height, 100);
```

Every `+` and `-` operator on these types checks for overflow in every
build profile and panics when it happens, as `core::time` does and the
integer primitives do not: a deadline that silently wrapped into the past
would be a security bug in a contract. The `checked_*` methods (and, on
`TimePoint`, the `saturating_*` methods) handle the edge without panicking.

Subtracting one timepoint from another saturates to `Duration::ZERO` when
the right-hand side is later, as `std::time::Instant` does; use
[`TimePoint::checked_duration_since`] to detect that case. Subtracting one
height from another is a plain signed difference.
