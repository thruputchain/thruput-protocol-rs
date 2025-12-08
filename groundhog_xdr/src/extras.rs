use crate::*;
use core::ops::Deref;

impl From<SHA256Hash> for StorageKey {
    fn from(hash: SHA256Hash) -> Self {
        Self::kStorageKeyTypeContractData {
            contract_data_key: hash.into_inner().into(),
        }
    }
}

impl From<SHA256Hash> for StorageKeyContractData {
    fn from(hash: SHA256Hash) -> Self {
        hash.into_inner().into()
    }
}

impl From<ContractAddress> for StorageKeyContractData {
    fn from(addr: ContractAddress) -> Self {
        addr.into_inner().into()
    }
}

// arrays with size above 32 don't implement Default, so we can't derive it
impl Default for Ed25519Signature {
    fn default() -> Self {
        Self::from([0; 64])
    }
}

impl Default for Signature {
    fn default() -> Self {
        Signature::Ed25519 {
            ed25519: Ed25519Signature::default(),
        }
    }
}

impl Default for StorageKey {
    fn default() -> Self {
        Self::kStorageKeyTypeContractData {
            contract_data_key: StorageKeyContractData::default(),
        }
    }
}

impl ContractAddress {
    pub const fn zero() -> Self {
        Self::new([0; 32])
    }
}

impl ContractEntrypointID {
    pub const fn from_u64(n: u64) -> Self {
        Self::new(n.to_be_bytes())
    }
}

impl From<uint128_xdr> for u128 {
    fn from(u: uint128_xdr) -> Self {
        ((u.highbits as u128) << 64) | (u.lowbits as u128)
    }
}

impl Into<uint128_xdr> for u128 {
    fn into(self: u128) -> uint128_xdr {
        uint128_xdr {
            lowbits: (self & 0xffff_ffff_ffff_ffff) as u64,
            highbits: (self >> 64) as u64,
        }
    }
}

impl From<int128_xdr> for i128 {
    fn from(i: int128_xdr) -> Self {
        ((i.highbits as i128) << 64) | (i.lowbits as i128)
    }
}

impl Into<int128_xdr> for i128 {
    fn into(self: i128) -> int128_xdr {
        int128_xdr {
            lowbits: (self & 0xffff_ffff_ffff_ffff) as u64,
            highbits: (self >> 64) as i64,
        }
    }
}
