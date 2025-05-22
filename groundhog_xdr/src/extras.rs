use crate::*;

impl From<SHA256Hash> for StorageKey {
    fn from(hash: SHA256Hash) -> Self {
        Self::kStorageKeyTypeContractData { contract_data_key: hash.into_inner().into() }
    }
}

impl From<[u8; 32]> for StorageKey {
    fn from(array: [u8; 32]) -> Self {
        Self::kStorageKeyTypeContractData { contract_data_key: array.into() }
    }
}

impl From<[u8; 32]> for StorageKeyContractData {
    fn from(array: [u8; 32]) -> Self {
        Self::new(ByteArray::from(array))
    }
}

impl From<[u8; 32]> for ContractAddress {
    fn from(array: [u8; 32]) -> Self {
        Self::new(ByteArray::from(array))
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
        Self::kStorageKeyTypeContractData { contract_data_key: StorageKeyContractData::default() }
    }
}

impl ContractAddress {
    pub const fn zero() -> Self {
        Self::new(ByteArray::new([0; 32]))
    }
}

impl AsRef<[u8]> for SHA256Hash {
    fn as_ref(&self) -> &[u8] {
        let array: &ByteArray<32> = self.as_ref();
        array.as_slice()
    }
}

impl AsRef<[u8]> for ContractAddress {
    fn as_ref(&self) -> &[u8] {
        let array: &ByteArray<32> = self.as_ref();
        array.as_slice()
    }
}

impl AsRef<[u8]> for StorageKeyContractData {
    fn as_ref(&self) -> &[u8] {
        let array: &ByteArray<32> = &self.as_ref();
        array.as_slice()
    }
}
