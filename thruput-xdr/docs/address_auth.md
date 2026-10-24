# Address Authorization Module

This module defines the authorization configuration stored at each address. This configuration data tells Thruput whether or not a given transaction can begin execution in a given block.

## Overview

The `address_auth` module provides the types for an address's authorization configuration. These types are generated from the XDR specification in `protocol/address_auth.x` and include:

- **Public key configuration**: Multi-signature key management with weights
- **Address authorization data**: Complete authorization configuration for an address: its keys and the minimum weight required to authorize a transaction

## Key Concepts

In Thruput, every address can originate transactions and can be configured with multiple signing keys and weighted multi-signature requirements.

Unlike the common blockchain model (as in, for example, Ethereum), in which there is an explicit demarcation between so-called "externally owned addresses" (EOAs) that can initiate transactions but have no associated code, and "contract addresses", which hold invocable code but cannot initiate transactions, Thruput treats every address uniformly: all addresses can both hold a contract and initiate a transaction.

## Public Key Configuration

Each address can have up to `kMaxSigners` public keys, each with an associated weight for multi-signature authorization. A null entry marks an unoccupied key slot. When verifying a `kSourceAddressAuthData` auth claim, weights from signing keys are accumulated and compared against the required threshold.

## Required Weight

`required_weight` is the minimum total weight the signing keys must reach to authorize a transaction from the address. An auth claim can raise this threshold for its own transaction but never lower it: the signing weight must reach the larger of the auth data's `required_weight` and the claim's.

## Replay Protection

Replay protection is not part of the auth data. Every transaction carries a `Uniquifier` (an expiry height and a nonce), which is recorded in the replay cache at the invoked contract's address when the transaction commits.

## Addresses Without Auth Data

An address has no `AddressAuthData` until an auth-data change registers a key or writes a threshold for it. The row that change creates requires a weight of `kDefaultNewAddressRequiredWeight` (1) unless the change writes a threshold of its own.

An address authorized by a `kPubkeySignature` claim has no `AddressAuthData` at all. That claim's single signature carries key ID `kDefaultNewAddressKeyID` (0), but nothing is installed under that ID.
