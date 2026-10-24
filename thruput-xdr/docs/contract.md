# Contract Module

This module defines the contract abstraction in Thruput. Contracts are code that can be deployed to addresses and invoked to execute logic.

## Overview

The `contract` module provides types and structures for contract deployment, execution, and management. These types are generated from the XDR specification in `protocol/contract.x` and include:

- **Contract types**: RV32 blobs, native precompiled code, and test mocks
- **Deployment**: Deterministic address derivation and deployment metadata
- **Entrypoints**: 8-byte contract method identifiers and how the SDK derives them
- **Metadata**: Deployed contract information

## Key Concepts

- Contracts can be RV32 blobs, native precompiled code, or test mocks
- Contract addresses are deterministically derived from deployment parameters
- Contract methods are identified by 8-byte entrypoint IDs
- Entrypoint IDs are derived from interface names and versions

## Contract Types

Thruput supports three types of contracts:

- **`kRv32`**: RISC-V contracts executed in the RV32 VM. This is the standard contract type for user-deployed contracts.
- **`kNative`**: Precompiled native contracts implemented natively by the node. Used for system contracts like the native token. More efficient than RV32 but cannot be deployed by users.
- **`kMockTest`**: Mock contracts for local testing. Never deployed on-chain; exists only in the test harness.

Discriminant 0 is retired. It selected the Wasmi (WebAssembly) runtime, which was
removed once RV32 became the only deployable execution environment. The value is
not reused: encodings that carry it fail to decode rather than silently decoding
as a live contract type.

## Contract Deployment

When a contract deploys another contract, the new contract's address is deterministically computed as:

```text
ContractAddress = SHA256(ContractDeploymentHashPreimage)
```

Under XDR encoding, the `ContractDeploymentHashPreimage` struct serializes as the concatenation of its fields: `deployer_address || nonce || contract_hash`.

This is Thruput's analogue of Ethereum's `CREATE2` opcode, allowing deterministic address derivation. The deployer controls the nonce to create addresses at will.

## Contract Interfaces and Entrypoints

Contract entrypoints can be arbitrary 8-byte identifiers. By default, the SDK for Thruput contracts derives each one from the interface name, the interface's major version, and the method name:

```text
ContractEntrypointID = SHA256(name || "::" || major || "::" || method)[0..8]
```

`major` is the semver compatibility prefix of the interface version: the leading components up to and including the first nonzero one (`"1"` for `"1.4.2"`, `"0.2"` for `"0.2.7"`, `"0.0.3"` for `"0.0.3"`), so compatible versions share entrypoint IDs. For example, `transfer` on an `"ERC20"` interface at version `"1.0.0"` is `SHA256("ERC20::1::transfer")[0..8]`. [`ContractEntrypointID::from_u64`] and [`ContractEntrypointID::into_u64`] convert between the bytes and the big-endian `u64` the `Invoke` syscall takes.

Interface IDs use the same prefix under a domain tag, `SHA256("groundhog-interface::" || name || "::" || major)[0..8]`, so they never coincide with an entrypoint ID.

