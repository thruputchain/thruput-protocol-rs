use crate::*;

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
        Self::new(ByteArray::new([0; 32]))
    }
}
