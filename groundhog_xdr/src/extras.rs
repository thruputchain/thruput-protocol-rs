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
        Self::new(hash.into_inner())
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
