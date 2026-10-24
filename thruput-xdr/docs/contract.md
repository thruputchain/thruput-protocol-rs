# Contract Module

This module defines the contract abstraction in Thruput. Contracts are code that can be deployed to addresses and invoked to execute logic.

## Overview

The `contract` module provides types and structures for contract deployment, execution, and management. These types are generated from the XDR specification in `protocol/contract.x` and include:

- **Contract types**: RV32 blobs, native precompiled code, and test mocks
- **Deployment**: Deterministic address derivation and deployment metadata
- **Interfaces**: Contract interface identification and entrypoint derivation
- **Metadata**: Deployed contract information and address metadata

## Key Concepts

- Contracts can be RV32 blobs, native precompiled code, or test mocks
- Contract addresses are deterministically derived from deployment parameters
- Contract methods are identified by 8-byte entrypoint IDs
- Entrypoint IDs are derived from interface names and versions

## Contract Types

Thruput supports three types of contracts:

- **`kRv32`**: RISC-V contracts executed in the RV32 VM. This is the standard contract type for user-deployed contracts.
- **`kNative`**: Precompiled native contracts implemented directly in Rust. Used for system contracts like the native token. More efficient than RV32 but cannot be deployed by users.
- **`kMockTest`**: Mock contracts for local testing. Never deployed on-chain; exists only in the test harness.

A fourth discriminant, `kWasmi`, is a legacy WebAssembly contract type. WebAssembly
execution has been removed in favor of RV32; the discriminant remains reserved in
the XDR only so existing encodings stay parseable, and it must not be used for new
deployments. It will be dropped from `protocol/contract.x` once the on-chain
migration completes.

## Contract Deployment

When a contract deploys another contract, the new contract's address is deterministically computed as:

```text
ContractAddress = SHA256(ContractDeploymentHashPreimage)
```

Under XDR encoding, the `ContractDeploymentHashPreimage` struct serializes as the concatenation of its fields: `deployer_address || nonce || contract_hash`.

This is Thruput's analogue of Ethereum's `CREATE2` opcode, allowing deterministic address derivation. The deployer controls the nonce to create addresses at will.

## Contract Interfaces and Entrypoints

Contract entrypoints can be arbitrary 8-byte identifiers. By default, the SDK for Thruput contracts follows a scheme for allocating those identifiers:

1. Contract interfaces define a set of methods (entrypoints) with a name and version
2. The first 6 bytes of `SHA256(ContractInterfaceIDPreimage)` serve as a namespace prefix for all methods in that interface
3. The last 2 bytes identify the specific method within the interface

This two-level scheme provides:
- Interface-level namespace (6 bytes) - prevents collisions between interfaces
- Method-level identifier (2 bytes) - identifies specific method within interface

By default, the SDK lexicographically sorts method names and allocates the last 2 bytes based on that order (first method gets 0x0000, second gets 0x0001, etc.).

