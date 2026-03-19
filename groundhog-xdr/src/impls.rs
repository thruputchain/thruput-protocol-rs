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
        let rhs: &[u8; 32] = &*rhs;

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
        let rhs: &[u8; 32] = &*rhs;

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
            let l = u32::from_le_bytes(lbytes.clone());
            let r = u32::from_le_bytes(rbytes.clone());
            *lbytes = (l ^ r).to_le_bytes();
        });
}

#[cfg(test)]
mod xor_test {
    use super::*;
    use std::ops::Range;
    use std::vec::Vec;

    #[test]
    fn sk_bit_xor() {
        let l: Vec<u8> = Range::from(0..32).into_iter().collect();
        let l: &[u8; 32] = l.as_slice().try_into().unwrap();
        let l: StorageKeyContractData = l.clone().into();

        let rv: Vec<u8> = Range::from(107..139).into_iter().rev().collect();
        let rs: &[u8; 32] = rv.as_slice().try_into().unwrap();
        let r: StorageKeyContractData = rs.clone().into();

        let expected = xor_arrays(&*l, &*r);

        let x: StorageKeyContractData = l ^ r;
        assert_eq!(*x, expected);

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
        out.clone()
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
        Self::ContractData {
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

impl Default for DbHash {
    fn default() -> Self {
        Self::new([0; 2048])
    }
}
