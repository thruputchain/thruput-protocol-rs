# Transaction Module

This module defines the transaction model for Thruput. Transactions are the fundamental unit of state modification in the system.

## Overview

The `transaction` module provides types and structures for constructing, signing, and executing transactions. These types are generated from the XDR specification in `protocol/transaction.x` and include:

- **Invocations**: Contract method calls with parameters and gas limits
- **Transactions**: Complete transactions with auth claims, uniquifiers, and deployments
- **Signatures**: Decorated signatures with key identifiers
- **Nondeterministic data**: Execution data for deterministic replay in parallel execution

## Key Concepts

- Each transaction invokes a contract method with specified parameters
- Transactions include authorization claims that must be satisfied by signatures
- Transactions can deploy new contracts and sponsor gas
- Thruput executes transactions in parallel, using 2PC for conflict resolution
- Proposers attach nondeterministic data to ensure deterministic re-execution by validators

## Transaction Lifecycle

1. User constructs a [`Transaction`] with invocation details, auth claims, etc.
2. Signatures are added to create a [`SignedTransaction`]
3. Transaction is submitted to the network
4. A proposer executes the transaction and captures nondeterministic data
5. Proposer creates a [`ProposedTransaction`] and includes it in a block proposal
6. Validators re-execute using the nondeterministic data to verify correctness

## Parallel Execution and Nondeterminism

Thruput's key innovation is parallel transaction execution. Unlike sequential blockchains, Thruput executes transactions without observing other transactions' writes. After execution, a two-phase commit process commits writes to the database. If there's a conflict, the entire transaction aborts.

The [`InvokeFlag::TryInvoke`] flag enables early conflict detection: when a try_invoke frame finishes, it performs two-phase commit lock acquisition immediately. If any lock fails, the subinvocation frame reverts (but the caller can handle this gracefully).

Since two-phase commit results are nondeterministic, they are captured in [`NondeterministicProposalData`] to ensure validators can deterministically replay the same execution path as the proposer.

## Invocation Flags

The Invoke syscall supports several flags that modify subinvocation behavior:

- **[`InvokeFlag::TryInvoke`]**: Enable early conflict detection for parallel execution
- **[`InvokeFlag::ReadOnly`]**: Prevent all state modifications in the subinvocation
- **[`InvokeFlag::DelegateCall`]**: Execute target contract's code in caller's context (not yet implemented)

Note: [`InvokeFlagSet`] is NOT part of the [`Invocation`] struct itself, but is instead a parameter to the Invoke syscall. The root transaction invocation always has no flags set.

## Encoding

Call arguments ([`Calldata`]) and return values ([`ReturnBuffer`]) are opaque byte arrays. Contracts can encode them arbitrarily, but the SDK default uses XDR serialization per [RFC 4506](https://datatracker.ietf.org/doc/html/rfc4506).
