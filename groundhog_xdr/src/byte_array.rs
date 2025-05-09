/// A wrapper type that allows arrays of arbitrary size
/// to be serialized with serde
use core::ops::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone)]
#[repr(transparent)]
pub struct ByteArray<const N: usize> {
    #[serde(with = "serde_arrays")]
    array: [u8; N],
}

impl<const N: usize> Default for ByteArray<N> {
    fn default() -> Self {
        Self { array: [0; N] }
    }
}

impl<const N: usize> From<[u8; N]> for ByteArray<N> {
    fn from(array: [u8; N]) -> Self {
        Self { array }
    }
}

impl<const N: usize> Deref for ByteArray<N> {
    type Target = [u8; N];

    fn deref(&self) -> &Self::Target {
        &self.array
    }
}

impl<const N: usize> DerefMut for ByteArray<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.array
    }
}

impl<const N: usize> AsRef<[u8]> for ByteArray<N> {
    fn as_ref(&self) -> &[u8] {
        &self.array
    }
}

impl<const N: usize> AsMut<[u8]> for ByteArray<N> {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.array
    }
}

impl<const N: usize> ByteArray<N> {
    pub const fn new(array: [u8; N]) -> Self {
        Self { array }
    }

    pub fn into_array(self) -> [u8; N] {
        self.array
    }

    pub fn as_array(&self) -> &[u8; N] {
        &self.array
    }
}

impl<const N: usize> core::fmt::Display for ByteArray<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for b in self.array {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}
