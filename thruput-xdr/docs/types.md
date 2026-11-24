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

[`TimePoint`] and [`BlockHeight`] are `Copy` and support `+` and `-`: a
timepoint against a [`TimeSpan`], and a height against
a signed number of blocks. The [`time`](crate::time) module documents the
arithmetic, its overflow rules and `TimeSpan`.
