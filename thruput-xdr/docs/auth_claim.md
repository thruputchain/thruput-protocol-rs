# Authentication Claims Module

This module defines the authorization model for Thruput transactions. Each transaction includes one [`AuthClaim`] that specifies which signatures are required to authorize the transaction's execution.

## Overview

The `auth_claim` module provides types and structures for transaction authorization. These types are generated from the XDR specification in `protocol/auth_claim.x` and include:

- **Auth claim types**: a weighted claim against the source address's registered keys, and a single-key claim for standalone public-key accounts
- **Weight and bitmap types**: Multi-signature weight accumulation and key selection
- **Constants**: `kMaxSigners`, the number of keys an address can register
- **Verification errors**: [`AuthVerifyErrorCode`]

## Workflow

1. A user constructs a Transaction, which includes an AuthClaim.
2. Signatures are added to create a SignedTransaction
3. Thruput verifies that signatures match the auth claim (before execution)
4. During execution, contracts can read the auth claim but NOT the signatures

This design ensures transaction behavior depends only on the auth claim (which is included in what gets signed), not on the signature bytes themselves. This makes transactions signature-scheme agnostic and prevents signature reordering or re-randomization from changing transaction behavior.

## Auth Claim Types

[`AuthClaimType`] has two variants:

- **`kSourceAddressAuthData`**: the transaction is signed by keys registered in the source address's [`AddressAuthData`]. The claim's `allowed_keys` bitmap selects which keys may sign, and the signing keys' weights must reach the larger of the auth data's required weight and the claim's own `required_weight`. Signatures must be listed in strictly increasing `public_key_id` order.
- **`kPubkeySignature`**: a single standalone public key signs the transaction. The claim names the key by its hash (`pubkey_preimage`), and the key itself travels in `SignedTransaction::pubkey_claim_key`. The signature must use key ID 0, and the address must have no registered `AddressAuthData`.

## Key Concepts

- **Weights**: Accumulated from signing keys and compared against a required threshold to determine if an auth claim is satisfied
- **Bitmaps**: Represent sets of key IDs, where each bit position corresponds to a key ID (bit 0 = key 0, bit 1 = key 1, etc.). Only bits below `kMaxSigners` may be set; a bitmap with a higher bit set is `kAuthMalformed`.
- **Key IDs**: Identifiers for public keys within an address's key set, valid range 0 to `kMaxSigners - 1` (0-20)
- **Auth data**: `kSourceAddressAuthData` claims are checked against the [`AddressAuthData`] stored at the source address, which defines the available keys and their weights
