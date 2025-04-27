use crate::*;

impl From<SHA256Hash> for StorageKey {
    fn from(hash: SHA256Hash) -> Self {
        hash.into_inner().into()
    }
}

impl From<[u8; 32]> for StorageKey {
    fn from(array: [u8; 32]) -> Self {
        Self::from(ByteArray::from(array))
    }
}

impl From<[u8; 32]> for ContractAddress {
    fn from(array: [u8; 32]) -> Self {
        Self::from(ByteArray::from(array))
    }
}
