use crate::*;
use core::ops::*;

impl From<SHA256Hash> for StorageKey {
    fn from(hash: SHA256Hash) -> Self {
        Self::ContractData {
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

impl From<SHA256Hash> for ContractAddress {
    fn from(hash: SHA256Hash) -> Self {
        hash.into_inner().into()
    }
}

impl BitXor for SHA256Hash {
    type Output = Self;

    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= &rhs.into_inner();

        self
    }
}

impl BitXor<&Self> for SHA256Hash {
    type Output = Self;

    fn bitxor(mut self, rhs: &Self) -> Self::Output {
        let rhs: &[u8; 32] = rhs;

        self ^= rhs;

        self
    }
}

impl BitXorAssign<&[u8; 32]> for SHA256Hash {
    fn bitxor_assign(&mut self, rhs: &[u8; 32]) {
        bitxor_assign_bytes(&mut *self, rhs);
    }
}

impl BitXor for StorageKeyContractData {
    type Output = Self;

    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= &rhs.into_inner();

        self
    }
}

impl BitXor<&Self> for StorageKeyContractData {
    type Output = Self;

    fn bitxor(mut self, rhs: &Self) -> Self::Output {
        let rhs: &[u8; 32] = rhs;

        self ^= rhs;

        self
    }
}

impl BitXorAssign<&[u8; 32]> for StorageKeyContractData {
    fn bitxor_assign(&mut self, rhs: &[u8; 32]) {
        bitxor_assign_bytes(&mut *self, rhs);
    }
}

fn bitxor_assign_bytes<const N: usize>(lhs: &mut [u8; N], rhs: &[u8; N]) {
    let (lchunks, []) = lhs.as_chunks_mut::<4>() else {
        unreachable!()
    };
    let (rchunks, []) = rhs.as_chunks::<4>() else {
        unreachable!()
    };
    lchunks
        .iter_mut()
        .zip(rchunks.iter())
        .for_each(|(lbytes, rbytes)| {
            let l = u32::from_le_bytes(*lbytes);
            let r = u32::from_le_bytes(*rbytes);
            *lbytes = (l ^ r).to_le_bytes();
        });
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
        Self::ContractData {
            contract_data_key: StorageKeyContractData::default(),
        }
    }
}

impl ContractAddress {
    /// Returns the all-zero address, useful as a placeholder where no
    /// real contract address exists yet.
    pub const fn zero() -> Self {
        Self::new([0; 32])
    }
}

// These types carry `#[rust(no_debug)]` in types.x. Wrappers inherit this
// readable identifier form through their derived Debug implementations.
macro_rules! base58_debug {
    ($($t:ty),+ $(,)?) => {$(
        impl core::fmt::Debug for $t {
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                write!(f, "{}", crate::base58::base58_encode(self))
            }
        }
    )+};
}

base58_debug!(ContractAddress, SHA256Hash);

// Ed25519SecretKey also carries `#[rust(no_debug)]`: SecretKey and
// ValidatorPrivateInfo derive Debug, and a derived impl here would print the
// private key into any log that formats them.
impl core::fmt::Debug for Ed25519SecretKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Ed25519SecretKey(<redacted>)")
    }
}

// MlDsa65PublicKey carries `#[rust(no_debug)]` too: a derived impl prints all
// 1952 bytes, also through PublicKey's derived Debug. Its first 8 bytes in
// hex are enough to tell keys apart in a log.
impl core::fmt::Debug for MlDsa65PublicKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("MlDsa65PublicKey(")?;
        for b in &self[..8] {
            write!(f, "{b:02x}")?;
        }
        f.write_str("..)")
    }
}

impl ContractEntrypointID {
    /// Builds an entrypoint ID from its numeric form, storing `n` as
    /// the ID's 8 big-endian bytes.
    pub const fn from_u64(n: u64) -> Self {
        Self::new(n.to_be_bytes())
    }

    /// Returns the numeric form of this ID, reading its 8 bytes as a
    /// big-endian `u64`. Inverse of [`Self::from_u64`].
    pub fn into_u64(self) -> u64 {
        u64::from_be_bytes(self.into_inner())
    }
}

impl From<uint128_xdr> for u128 {
    fn from(u: uint128_xdr) -> Self {
        ((u.highbits as u128) << 64) | (u.lowbits as u128)
    }
}

impl From<u128> for uint128_xdr {
    fn from(n: u128) -> Self {
        uint128_xdr {
            lowbits: (n & 0xffff_ffff_ffff_ffff) as u64,
            highbits: (n >> 64) as u64,
        }
    }
}

impl From<int128_xdr> for i128 {
    fn from(i: int128_xdr) -> Self {
        ((i.highbits as i128) << 64) | (i.lowbits as i128)
    }
}

impl From<i128> for int128_xdr {
    fn from(n: i128) -> Self {
        int128_xdr {
            lowbits: (n & 0xffff_ffff_ffff_ffff) as u64,
            highbits: (n >> 64) as i64,
        }
    }
}

impl Default for StorageLongHash {
    fn default() -> Self {
        Self::new([0; 2048])
    }
}

#[cfg(test)]
mod xor_test {
    use super::*;
    use std::vec::Vec;

    #[test]
    fn sk_bit_xor() {
        let l: Vec<u8> = (0..32).collect();
        let l: &[u8; 32] = l.as_slice().try_into().unwrap();
        let l: StorageKeyContractData = (*l).into();

        let rv: Vec<u8> = (107..139).rev().collect();
        let rs: &[u8; 32] = rv.as_slice().try_into().unwrap();
        let r: StorageKeyContractData = (*rs).into();

        let expected = xor_arrays(&l, &r);

        let x: StorageKeyContractData = l ^ r;
        assert_eq!(*x, expected);

        // Exercise the by-reference operator impl on purpose.
        #[allow(clippy::op_ref)]
        let x: StorageKeyContractData = l ^ &r;
        assert_eq!(*x, expected);

        let mut x: StorageKeyContractData = l;
        x ^= rs;
        assert_eq!(*x, expected);
    }

    fn xor_arrays(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
        let out: Vec<_> =
            a.iter().zip(b.iter()).map(|(a, b)| *a ^ *b).collect();
        let out: &[u8; 32] = out.as_slice().try_into().unwrap();
        *out
    }
}
