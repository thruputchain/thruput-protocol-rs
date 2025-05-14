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

impl Default for Signature {
    fn default() -> Self {
        Signature::Ed25519 {
            ed25519: Ed25519Signature::default(),
        }
    }
}

impl ContractAddress {
    pub const fn zero() -> Self {
        Self::new(ByteArray::new([0; 32]))
    }
}

impl AsRef<[u8]> for SHA256Hash {
    fn as_ref(&self) -> &[u8] {
        let array: &ByteArray<32> = self;
        array.as_slice()
    }
}

impl AsRef<[u8]> for ContractAddress {
    fn as_ref(&self) -> &[u8] {
        let array: &ByteArray<32> = self;
        array.as_slice()
    }
}

impl AsRef<[u8]> for StorageKey {
    fn as_ref(&self) -> &[u8] {
        let array: &ByteArray<32> = self;
        array.as_slice()
    }
}
