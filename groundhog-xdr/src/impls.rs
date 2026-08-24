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
    pub const fn zero() -> Self {
        Self::new([0; 32])
    }
}

impl ContractEntrypointID {
    pub const fn from_u64(n: u64) -> Self {
        Self::new(n.to_be_bytes())
    }

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

/// Error returned when an [`AddressAuthData`] configuration cannot
/// authorize any transaction.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum AddressAuthError {
    /// `required_weight` exceeds the total weight available from the
    /// occupied key slots, so no set of signatures can ever reach the
    /// threshold and the address is permanently locked.
    UnreachableRequiredWeight {
        /// The configured `required_weight` threshold.
        required: Weight,
        /// The saturating sum of `max_weight` over all occupied slots.
        available: Weight,
    },
}

impl core::fmt::Display for AddressAuthError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AddressAuthError::UnreachableRequiredWeight {
                required,
                available,
            } => write!(
                f,
                "required weight {required} exceeds the {available} \
                 available from configured keys; the address would be \
                 permanently locked"
            ),
        }
    }
}

impl core::error::Error for AddressAuthError {}

impl AddressAuthData {
    /// Returns the total signing weight available from all occupied
    /// key slots, saturating at [`Weight::MAX`].
    pub fn total_available_weight(&self) -> Weight {
        self.public_keys
            .iter()
            .filter_map(|slot| slot.as_ref())
            .fold(0, |acc: Weight, config| {
                acc.saturating_add(config.max_weight)
            })
    }

    /// Returns whether some set of signatures can reach
    /// `required_weight`, i.e. whether the address is not permanently
    /// locked by its own configuration.
    pub fn is_satisfiable(&self) -> bool {
        self.total_available_weight() >= self.required_weight
    }

    /// Validates that this configuration can authorize transactions.
    ///
    /// Returns [`AddressAuthError::UnreachableRequiredWeight`] when
    /// `required_weight` can never be met by the configured keys.
    /// Storing such a configuration would permanently lock the
    /// address, since auth changes themselves require an authorized
    /// transaction.
    pub fn validate(&self) -> Result<(), AddressAuthError> {
        let available = self.total_available_weight();
        if available < self.required_weight {
            return Err(AddressAuthError::UnreachableRequiredWeight {
                required: self.required_weight,
                available,
            });
        }
        Ok(())
    }

    /// Returns the default configuration for a newly initialized
    /// address controlled by `key`.
    ///
    /// The key is installed at slot [`kDefaultNewAddressKeyID`] with
    /// weight [`kDefaultNewAddressRequiredWeight`], which is also the
    /// required threshold, so the single key can authorize
    /// transactions on its own. (The matching default claim bitmap is
    /// [`kDefaultNewAddressAcceptedKeys`], which accepts every slot.)
    pub fn new_address_default(
        key: PublicKey,
        last_written_height: BlockHeight,
    ) -> Self {
        let mut key = Some(PublicKeyConfig {
            key,
            max_weight: kDefaultNewAddressRequiredWeight,
        });
        let public_keys = core::array::from_fn(|i| {
            PublicKeyConfigOpt::new(if i == kDefaultNewAddressKeyID as usize {
                key.take()
            } else {
                None
            })
        });
        Self {
            public_keys,
            required_weight: kDefaultNewAddressRequiredWeight,
            last_written_height,
        }
    }
}

#[cfg(test)]
mod address_auth_tests {
    use super::*;

    fn pubkey(fill: u8) -> PublicKey {
        PublicKey::Ed25519 {
            ed25519: Ed25519PublicKey::new([fill; 32]),
        }
    }

    fn auth_data_with_weights(
        weights: &[Weight],
        required_weight: Weight,
    ) -> AddressAuthData {
        AddressAuthData {
            public_keys: core::array::from_fn(|i| {
                PublicKeyConfigOpt::new(weights.get(i).map(|&max_weight| {
                    PublicKeyConfig {
                        key: pubkey(i as u8),
                        max_weight,
                    }
                }))
            }),
            required_weight,
            last_written_height: 0.into(),
        }
    }

    #[test]
    fn empty_keys_cannot_satisfy_nonzero_weight() {
        let auth = auth_data_with_weights(&[], 1);
        assert_eq!(auth.total_available_weight(), 0);
        assert!(!auth.is_satisfiable());
        assert_eq!(
            auth.validate(),
            Err(AddressAuthError::UnreachableRequiredWeight {
                required: 1,
                available: 0,
            })
        );

        // A zero threshold is reachable even with no keys.
        let auth = auth_data_with_weights(&[], 0);
        assert!(auth.is_satisfiable());
        assert_eq!(auth.validate(), Ok(()));
    }

    #[test]
    fn weight_sum_saturates_at_max() {
        let auth = auth_data_with_weights(&[Weight::MAX, Weight::MAX, 3], 100);
        assert_eq!(auth.total_available_weight(), Weight::MAX);
        assert!(auth.is_satisfiable());
        assert_eq!(auth.validate(), Ok(()));

        // Saturation must not make an unreachable threshold look
        // reachable: MAX is still >= any required_weight, so the MAX
        // threshold itself remains satisfiable.
        let auth = auth_data_with_weights(&[Weight::MAX], Weight::MAX);
        assert!(auth.is_satisfiable());
    }

    #[test]
    fn insufficient_total_weight_is_reported() {
        let auth = auth_data_with_weights(&[1, 2], 4);
        assert_eq!(auth.total_available_weight(), 3);
        assert_eq!(
            auth.validate(),
            Err(AddressAuthError::UnreachableRequiredWeight {
                required: 4,
                available: 3,
            })
        );
    }

    #[test]
    fn new_address_default_is_satisfiable() {
        let auth = AddressAuthData::new_address_default(pubkey(7), 5.into());
        assert_eq!(auth.required_weight, kDefaultNewAddressRequiredWeight);
        assert_eq!(auth.last_written_height, 5.into());
        let slot = &auth.public_keys[kDefaultNewAddressKeyID as usize];
        assert_eq!(
            slot.as_ref().map(|config| config.max_weight),
            Some(kDefaultNewAddressRequiredWeight)
        );
        let occupied = auth
            .public_keys
            .iter()
            .filter(|slot| slot.is_some())
            .count();
        assert_eq!(occupied, 1);
        assert!(auth.is_satisfiable());
        assert_eq!(auth.validate(), Ok(()));
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
