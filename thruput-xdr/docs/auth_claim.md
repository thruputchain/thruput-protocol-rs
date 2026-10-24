# Authentication Claims Module

This module defines the authorization model for Thruput transactions. Each transaction includes one `AuthClaim` that specifies which signatures are required to authorize the transaction's execution.

## Overview

The `auth_claim` module provides types and structures for transaction authorization. These types are generated from the XDR specification in `protocol/auth_claim.x` and include:

- **Auth claim types**: Signatures from the keys in the source address's auth data, or from a standalone public key
- **Weight, bitmap, and key ID types**: Multi-signature weight accumulation and key selection
- **Constants**: `kMaxSigners`, the number of key slots an address has
- **Verification error codes**: `AuthVerifyErrorCode`, which distinguishes malformed claims, failed signatures, and claims that do not match an address's auth data

## Workflow

1. A user constructs a Transaction, which includes an AuthClaim.
2. Signatures are added to create a SignedTransaction
3. Thruput verifies that signatures match the auth claim (before execution)
4. During execution, contracts can read the auth claim (with the `GetAuthClaim` syscall) but NOT the signatures

This design ensures transaction behavior depends only on the auth claim (which is included in what gets signed), not on the signature bytes themselves. This makes transactions signature-scheme agnostic and prevents signature reordering or re-randomization from changing transaction behavior.

## Auth Claim Types

`AuthClaim` has two variants:

- **`kSourceAddressAuthData`**: The transaction is signed by keys from the `AddressAuthData` stored at the source address. The claim's `allowed_keys` bitmap says which of those keys may sign, and the signatures must be listed in strictly increasing `public_key_id` order. The signing keys' weights must sum to at least the larger of the auth data's `required_weight` and the claim's own `required_weight`, so a claim can raise the address's threshold but never lower it.
- **`kPubkeySignature`**: A single standalone public key signs the transaction. This is for addresses with no `AddressAuthData`, whose address is derived from a [`PubkeyAccountPreimage`] (a public key hash and a nonce). The claim names the key by its hash; the key itself travels outside the signed transaction, as `SignedTransaction::pubkey_claim_key`, so the claim that contracts read is the same size for every signature scheme.

## Key Concepts

- **Weights**: Accumulated from signing keys and compared against a required threshold to determine if a `kSourceAddressAuthData` claim is satisfied
- **Bitmaps**: Represent sets of key IDs, where each bit position corresponds to a key ID (bit 0 = key 0, bit 1 = key 1, etc.). Only bits below `kMaxSigners` may be set; a bitmap with a higher bit set is `kAuthMalformed`.
- **Key IDs**: Identifiers for public keys within an address's key set, valid range 0 to `kMaxSigners - 1` (0-20)
