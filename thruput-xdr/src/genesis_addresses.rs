//! The 32-byte [`ContractAddress`] of each address the protocol reserves at
//! genesis: the null address and the contracts placed there.
//!
//! [`GenesisContractAddresses`] numbers these addresses; the address is that
//! number in big-endian order in the last four bytes, with every other byte
//! zero.
//!
//! ```
//! use thruput_xdr::genesis_addresses::ADDRESS_NATIVE_TOKEN;
//!
//! let mut bytes = [0u8; 32];
//! bytes[31] = 1;
//! assert_eq!(*ADDRESS_NATIVE_TOKEN, bytes);
//! ```

use crate::genesis::GenesisContractAddresses;
use crate::types::ContractAddress;

/// Returns the [`ContractAddress`] of the genesis contract `addr` names.
const fn genesis_contract_address(
    addr: GenesisContractAddresses,
) -> ContractAddress {
    let mut bytes = [0u8; 32];
    let id = addr.0.to_be_bytes();
    bytes[28] = id[0];
    bytes[29] = id[1];
    bytes[30] = id[2];
    bytes[31] = id[3];
    ContractAddress::new(bytes)
}

/// The reserved null address: all 32 bytes zero.
pub const ADDRESS_NULL: ContractAddress =
    genesis_contract_address(GenesisContractAddresses::kAddressNull);

/// Address of the precompiled native-token contract.
pub const ADDRESS_NATIVE_TOKEN: ContractAddress =
    genesis_contract_address(GenesisContractAddresses::kAddressNativeToken);

/// Address of the genesis root contract.
pub const ADDRESS_GENESIS_ROOT: ContractAddress =
    genesis_contract_address(GenesisContractAddresses::kAddressGenesisRoot);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addresses_pack_the_genesis_id_into_the_last_four_bytes() {
        assert_eq!(*ADDRESS_NULL, [0u8; 32]);

        let mut native = [0u8; 32];
        native[31] = 1;
        assert_eq!(*ADDRESS_NATIVE_TOKEN, native);

        let mut root = [0u8; 32];
        root[31] = 2;
        assert_eq!(*ADDRESS_GENESIS_ROOT, root);

        assert_eq!(
            *genesis_contract_address(GenesisContractAddresses(0x0102_0304)),
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4,
            ],
        );
    }
}
