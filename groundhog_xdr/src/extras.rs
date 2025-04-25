use crate::*;

impl From<SHA256Hash> for StorageKey {
    fn from(hash: SHA256Hash) -> Self {
        hash.into_inner().into()
    }
}
