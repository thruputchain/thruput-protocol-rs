# Address Authorization Module

This module defines the authorization configuration stored at each address. This configuration data tells Thruput whether or not a given transaction can begin execution in a given block.

## Overview

The `address_auth` module provides types and structures for managing address authorization. These types are generated from the XDR specification in `protocol/address_auth.x` and include:

- **Public key configuration**: Multi-signature key management with weights
- **Address authorization data**: [`AddressAuthData`], the complete authorization configuration for an address: its keys, their weights, and the weight a transaction needs

## Key Concepts

In Thruput, every address can originate transactions and can be configured with multiple signing keys and a weighted multi-signature requirement.

Unlike the common blockchain model (as in, for example, Ethereum), in which there is an explicit demarcation between so-called "externally owned addresses" (EOAs) that can initiate transactions but have no associated code, and "contract addresses", which hold invocable code but cannot initiate transactions, Thruput treats every address uniformly: all addresses can both hold a contract and initiate a transaction.

## Public Key Configuration

Each address can have up to `kMaxSigners` public keys, each with an associated weight for multi-signature authorization. A null entry marks an unoccupied key slot. When verifying a `kSourceAddressAuthData` auth claim, weights from signing keys are accumulated and compared against the required threshold.

Replay protection does not live here: every transaction carries a [`Uniquifier`], which the address's replay cache records.

## Default Initialization

An address has no `AddressAuthData` until a change registers a key or writes a threshold for it. Until then, the only claim that can authorize a transaction from it is `kPubkeySignature`, for an address derived from a standalone public key; that claim's single signature uses key ID 0 (`kDefaultNewAddressKeyID`). The first change creates the auth data with required weight 1 (`kDefaultNewAddressRequiredWeight`) unless it writes a threshold of its own.
