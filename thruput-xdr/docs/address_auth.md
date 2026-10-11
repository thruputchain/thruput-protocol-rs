# Address Authorization Module

This module defines the authorization configuration stored at each address. This configuration data tells Thruput whether or not a given transaction can begin execution in a given block.

## Overview

The `address_auth` module provides types and structures for managing address authorization, including public key configuration, gas sponsorship permissions, and nonce management. These types are generated from the XDR specification in `protocol/address_auth.x` and include:

- **Public key configuration**: Multi-signature key management with weights
- **Gas permissions**: Configurable gas sponsorship with limits and multi-sig requirements
- **Nonce configuration**: Transaction nonce management rules
- **Address authorization data**: Complete authorization configuration for an address

## Key Concepts

In Thruput, every address can originate transactions and can be configured with multiple signing keys, weighted multi-signature requirements, and gas sponsorship permissions.

Unlike the common blockchain model (as in, for example, Ethereum), in which there is an explicit demarcation between so-called "externally owned addresses" (EOAs) that can initiate transactions but have no associated code, and "contract addresses", which hold invocable code but cannot initiate transactions, Thruput treats every address uniformly: all addresses can both hold a contract and initiate a transaction.

## Public Key Configuration

Each address can have up to `kMaxSigners` public keys, each with an associated weight for multi-signature authorization. When verifying weighted auth claims, weights from signing keys are accumulated and compared against the required threshold.

## Gas Sponsorship

Gas sponsorship permissions allow an address to grant specific keys permission to spend gas on behalf of the address, with configurable limits and multi-signature requirements. This allows separation of concerns (e.g., sponsored gas vs contract owner management).

## Nonce Management

The nonce configuration specifies which keys can increment the nonce and what weight is required. This controls transaction ordering and replay protection.

## Default Initialization

When an address is first created (e.g., from a public key hash), the auth data is initialized with defaults:
- Required weight: 1
- Accepted keys: All key slots initially acceptable (0xFFFFFFFF)
- Initial key ID: 0 (the public key used to derive the address is installed as key 0)

