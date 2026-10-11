# Authentication Claims Module

This module defines the authorization model for Thruput transactions. Each transaction includes a set of AuthClaims that specify which signatures are required to authorize the transaction's execution.

## Overview

The `auth_claim` module provides types and structures for transaction authorization. These types are generated from the XDR specification in `protocol/auth_claim.x` and include:

- **Auth claim types**: Single-signature and multi-signature authorization patterns
- **Cross-contract authorization**: Support for signatures from other contract addresses
- **Weight and bitmap types**: Multi-signature weight accumulation and key selection
- **Constants**: Limits for signers, gas configs, and auth claims per transaction

## Workflow

1. A user constructs a Transaction, which includes some number of AuthClaims.
2. Signatures are added to create a SignedTransaction
3. Thruput verifies that signatures match the auth claims (before execution)
4. During execution, contracts can read the auth claims but NOT the signatures

This design ensures transaction behavior depends only on the auth claims (which are included in what gets signed), not on the signature bytes themselves. This makes transactions signature-scheme agnostic and prevents signature reordering or re-randomization from changing transaction behavior.

## Authorization Patterns

The authentication model supports:

- **Single-signature authorization**: One specific key must sign
- **Multi-signature authorization**: Weighted threshold of keys must sign
- **Cross-contract authorization**: Signatures from other contract addresses

## Auth Claim Types

Claims are categorized by:
1. Whether they reference the invoked contract or another contract
2. Whether they require a single signature or weighted multi-sig

The four variants support different authorization patterns:

- **`kInvokedAddressSignature`**: Single key from invoked contract (most common)
- **`kInvokedAddressSignedWeight`**: Multi-sig from invoked contract (for high-security operations)
- **`kOtherAddressSignature`**: Single key from other contract (for delegation)
- **`kOtherAddressSignedWeight`**: Multi-sig from other contract (for complex delegated operations)

## Key Concepts

- **Weights**: Accumulated from signing keys and compared against a required threshold to determine if an auth claim is satisfied
- **Bitmaps**: Represent sets of key IDs, where each bit position corresponds to a key ID (bit 0 = key 0, bit 1 = key 1, etc.)
- **Key IDs**: Identifiers for public keys within an address's key set, valid range 0-31
- **Auth claims reference**: AddressAuthData stored at each contract address, which defines the available keys and their weights

