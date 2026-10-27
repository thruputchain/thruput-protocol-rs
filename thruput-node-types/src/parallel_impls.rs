use crate::ParallelBatch;
use xdr_common::{BytesM, VecM};

impl ParallelBatch {
    /// An empty batch with no segments and no data.
    pub fn empty() -> Self {
        Self {
            segment_tokens: VecM(vec![]), // if VecM can't be const, use a fn
            segment_bytes: VecM(vec![]),
            xdr: BytesM(vec![]),
        }
    }

    /// Total number of items across all segments.
    ///
    /// A `u64`, so the sum of up to `u32::MAX` per-segment `u32` counts
    /// cannot overflow.
    pub fn items(&self) -> u64 {
        self.segment_tokens.iter().map(|&t| u64::from(t)).sum()
    }

    /// Number of segments in this batch.
    ///
    /// Uses the minimum of `segment_tokens` and `segment_bytes` lengths
    /// so the result is always consistent with [`segment()`](Self::segment).
    pub fn segments(&self) -> usize {
        self.segment_tokens.len().min(self.segment_bytes.len())
    }

    /// True if the batch contains no items.
    pub fn is_empty(&self) -> bool {
        self.segment_tokens.iter().all(|&t| t == 0)
    }

    /// Check internal consistency of segment metadata against the
    /// packed data.
    pub fn validate(&self) -> Result<(), BatchValidationError> {
        if self.segment_tokens.len() != self.segment_bytes.len() {
            return Err(BatchValidationError::SegmentCountMismatch {
                tokens: self.segment_tokens.len(),
                bytes: self.segment_bytes.len(),
            });
        }
        let expected: u64 = self.segment_bytes.iter().map(|&b| b as u64).sum();
        let actual = self.xdr.len() as u64;
        if expected != actual {
            return Err(BatchValidationError::ByteLengthMismatch {
                expected,
                actual,
            });
        }
        Ok(())
    }

    /// Byte slice for segment `i`. Returns None if out of bounds
    /// or if the offset falls outside xdr.
    pub fn segment(&self, i: usize) -> Option<&[u8]> {
        if i >= self.segments() {
            return None;
        }
        // Sum in u64 and convert checked: on a 32-bit target the offset of a
        // corrupt batch can exceed usize.
        let offset: u64 =
            self.segment_bytes[..i].iter().map(|&b| u64::from(b)).sum();
        let offset = usize::try_from(offset).ok()?;
        let len = usize::try_from(self.segment_bytes[i]).ok()?;
        self.xdr.0.get(offset..offset.checked_add(len)?)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BatchValidationError {
    /// `segment_tokens` and `segment_bytes` have different lengths.
    SegmentCountMismatch { tokens: usize, bytes: usize },
    /// Sum of `segment_bytes` doesn't match `xdr` length.
    ByteLengthMismatch { expected: u64, actual: u64 },
}

#[cfg(feature = "std")]
impl std::fmt::Display for BatchValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SegmentCountMismatch { tokens, bytes } => {
                write!(
                    f,
                    "segment count mismatch: segment_tokens has {tokens} entries, segment_bytes has {bytes}"
                )
            }
            Self::ByteLengthMismatch { expected, actual } => {
                write!(
                    f,
                    "byte length mismatch: segment_bytes sum to {expected}, xdr is {actual} bytes"
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ParallelNondeterministicProposalDataBatch,
        ParallelSignedTransactionBatch, ParallelSignedTransactionHashBatch,
    };
    use xdr_common::{BytesM, VecM};

    /// Helper: build a batch with the given segments.
    /// Each segment is a `(token_count, payload)` pair.
    fn make_batch(segments: &[(u32, &[u8])]) -> ParallelBatch {
        let mut segment_tokens = Vec::new();
        let mut segment_bytes = Vec::new();
        let mut xdr = Vec::new();
        for &(tokens, payload) in segments {
            segment_tokens.push(tokens);
            segment_bytes.push(payload.len() as u32);
            xdr.extend_from_slice(payload);
        }
        ParallelBatch {
            segment_tokens: VecM(segment_tokens),
            segment_bytes: VecM(segment_bytes),
            xdr: BytesM(xdr),
        }
    }

    // ── Validation ──────────────────────────────────────────────────

    #[test]
    fn validate_empty_batch() {
        let batch = ParallelBatch::empty();
        assert!(batch.validate().is_ok());
    }

    #[test]
    fn validate_single_segment() {
        let batch = make_batch(&[(3, &[0xAA; 10])]);
        assert!(batch.validate().is_ok());
    }

    #[test]
    fn validate_multi_segment() {
        let batch = make_batch(&[(1, &[1, 2, 3]), (2, &[4, 5]), (5, &[6])]);
        assert!(batch.validate().is_ok());
    }

    #[test]
    fn validate_segment_count_mismatch() {
        let batch = ParallelBatch {
            segment_tokens: VecM(vec![1, 2]),
            segment_bytes: VecM(vec![4]),
            xdr: BytesM(vec![0; 4]),
        };
        assert_eq!(
            batch.validate(),
            Err(BatchValidationError::SegmentCountMismatch {
                tokens: 2,
                bytes: 1,
            })
        );
    }

    #[test]
    fn segments_and_segment_consistent_on_mismatched_vecs() {
        // segment_tokens longer than segment_bytes
        let batch = ParallelBatch {
            segment_tokens: VecM(vec![1, 2, 3]),
            segment_bytes: VecM(vec![4, 5]),
            xdr: BytesM(vec![0; 9]),
        };
        assert!(batch.validate().is_err());
        // segments() should be capped to the shorter vec
        assert_eq!(batch.segments(), 2);
        assert!(batch.segment(0).is_some());
        assert!(batch.segment(1).is_some());
        assert_eq!(batch.segment(2), None);

        // segment_bytes longer than segment_tokens
        let batch = ParallelBatch {
            segment_tokens: VecM(vec![1]),
            segment_bytes: VecM(vec![3, 4]),
            xdr: BytesM(vec![0; 7]),
        };
        assert!(batch.validate().is_err());
        assert_eq!(batch.segments(), 1);
        assert!(batch.segment(0).is_some());
        assert_eq!(batch.segment(1), None);
    }

    #[test]
    fn validate_byte_length_too_short() {
        let batch = ParallelBatch {
            segment_tokens: VecM(vec![1]),
            segment_bytes: VecM(vec![10]),
            xdr: BytesM(vec![0; 5]), // only 5, expected 10
        };
        assert_eq!(
            batch.validate(),
            Err(BatchValidationError::ByteLengthMismatch {
                expected: 10,
                actual: 5,
            })
        );
    }

    #[test]
    fn validate_byte_length_too_long() {
        let batch = ParallelBatch {
            segment_tokens: VecM(vec![1]),
            segment_bytes: VecM(vec![3]),
            xdr: BytesM(vec![0; 8]), // 8 > 3
        };
        assert_eq!(
            batch.validate(),
            Err(BatchValidationError::ByteLengthMismatch {
                expected: 3,
                actual: 8,
            })
        );
    }

    // ── Utility methods ─────────────────────────────────────────────

    #[test]
    fn empty_batch_properties() {
        let batch = ParallelBatch::empty();
        assert_eq!(batch.items(), 0);
        assert_eq!(batch.segments(), 0);
        assert!(batch.is_empty());
        assert_eq!(batch.segment(0), None);
    }

    #[test]
    fn items_sums_across_segments() {
        let batch = make_batch(&[(3, &[0; 2]), (7, &[0; 4]), (1, &[0; 1])]);
        assert_eq!(batch.items(), 11);
        assert_eq!(batch.segments(), 3);
        assert!(!batch.is_empty());
    }

    #[test]
    fn segment_returns_correct_slices() {
        let batch = make_batch(&[
            (1, &[0xAA, 0xBB]),
            (1, &[0xCC]),
            (1, &[0xDD, 0xEE, 0xFF]),
        ]);
        assert_eq!(batch.segment(0), Some([0xAA, 0xBB].as_slice()));
        assert_eq!(batch.segment(1), Some([0xCC].as_slice()));
        assert_eq!(batch.segment(2), Some([0xDD, 0xEE, 0xFF].as_slice()));
        assert_eq!(batch.segment(3), None);
    }

    #[test]
    fn segment_out_of_bounds() {
        let batch = make_batch(&[(1, &[0x01])]);
        assert_eq!(batch.segment(1), None);
        assert_eq!(batch.segment(100), None);
    }

    // ── XDR round-trip (CXS) ───────────────────────────────────────

    #[test]
    fn xdr_cxs_roundtrip_empty() {
        let batch = ParallelBatch::empty();
        let encoded = xdrs::cxs::encode(&batch).unwrap();
        let decoded: ParallelBatch = xdrs::cxs::decode(&encoded).unwrap();
        assert_eq!(batch, decoded);
    }

    #[test]
    fn xdr_cxs_roundtrip_populated() {
        let batch = make_batch(&[(3, &[1, 2, 3, 4, 5]), (2, &[6, 7, 8])]);
        let encoded = xdrs::cxs::encode(&batch).unwrap();
        let decoded: ParallelBatch = xdrs::cxs::decode(&encoded).unwrap();
        assert_eq!(batch, decoded);
    }

    #[test]
    fn xdr_txs_roundtrip_empty() {
        let batch = ParallelBatch::empty();
        let encoded = xdrs::txs::encode(&batch).unwrap();
        let decoded: ParallelBatch = xdrs::txs::decode(&encoded).unwrap();
        assert_eq!(batch, decoded);
    }

    #[test]
    fn xdr_txs_roundtrip_populated() {
        let batch = make_batch(&[(10, &[0xFF; 32]), (5, &[0x42; 16])]);
        let encoded = xdrs::txs::encode(&batch).unwrap();
        let decoded: ParallelBatch = xdrs::txs::decode(&encoded).unwrap();
        assert_eq!(batch, decoded);
    }

    // ── XDR round-trip for newtype wrappers ─────────────────────────

    #[test]
    fn xdr_roundtrip_signed_transaction_batch() {
        let inner = make_batch(&[(2, &[0xAA, 0xBB, 0xCC])]);
        let batch = ParallelSignedTransactionBatch::from(inner.clone());
        let encoded = xdrs::cxs::encode(&batch).unwrap();
        let decoded: ParallelSignedTransactionBatch =
            xdrs::cxs::decode(&encoded).unwrap();
        assert_eq!(*decoded, inner);
    }

    #[test]
    fn xdr_roundtrip_signed_transaction_hash_batch() {
        let inner = make_batch(&[(1, &[0xDE, 0xAD])]);
        let batch = ParallelSignedTransactionHashBatch::from(inner.clone());
        let encoded = xdrs::txs::encode(&batch).unwrap();
        let decoded: ParallelSignedTransactionHashBatch =
            xdrs::txs::decode(&encoded).unwrap();
        assert_eq!(*decoded, inner);
    }

    #[test]
    fn xdr_roundtrip_nondeterministic_proposal_data_batch() {
        let inner = make_batch(&[(4, &[1; 8]), (3, &[2; 6])]);
        let batch =
            ParallelNondeterministicProposalDataBatch::from(inner.clone());
        let encoded = xdrs::cxs::encode(&batch).unwrap();
        let decoded: ParallelNondeterministicProposalDataBatch =
            xdrs::cxs::decode(&encoded).unwrap();
        assert_eq!(*decoded, inner);
    }

    // ── Serde JSON round-trip ───────────────────────────────────────

    #[cfg(feature = "serde")]
    #[test]
    fn serde_json_roundtrip_empty() {
        let batch = ParallelBatch::empty();
        let json = serde_json::to_string(&batch).unwrap();
        let decoded: ParallelBatch = serde_json::from_str(&json).unwrap();
        assert_eq!(batch, decoded);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_json_roundtrip_populated() {
        let batch = make_batch(&[(3, &[1, 2, 3, 4, 5]), (2, &[6, 7, 8])]);
        let json = serde_json::to_string(&batch).unwrap();
        let decoded: ParallelBatch = serde_json::from_str(&json).unwrap();
        assert_eq!(batch, decoded);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_json_roundtrip_newtypes() {
        let inner = make_batch(&[(1, &[0xFF; 4])]);

        let tx = ParallelSignedTransactionBatch::from(inner.clone());
        let json = serde_json::to_string(&tx).unwrap();
        let decoded: ParallelSignedTransactionBatch =
            serde_json::from_str(&json).unwrap();
        assert_eq!(*decoded, inner);

        let hash = ParallelSignedTransactionHashBatch::from(inner.clone());
        let json = serde_json::to_string(&hash).unwrap();
        let decoded: ParallelSignedTransactionHashBatch =
            serde_json::from_str(&json).unwrap();
        assert_eq!(*decoded, inner);

        let nd =
            ParallelNondeterministicProposalDataBatch::from(inner.clone());
        let json = serde_json::to_string(&nd).unwrap();
        let decoded: ParallelNondeterministicProposalDataBatch =
            serde_json::from_str(&json).unwrap();
        assert_eq!(*decoded, inner);
    }

    // ── Newtype conversions ─────────────────────────────────────────

    #[test]
    fn newtype_from_and_into_inner() {
        let inner = make_batch(&[(5, &[0x01; 10])]);
        let tx = ParallelSignedTransactionBatch::from(inner.clone());
        assert_eq!(tx.into_inner(), inner);

        let hash = ParallelSignedTransactionHashBatch::new(inner.clone());
        assert_eq!(hash.into_inner(), inner);

        let nd =
            ParallelNondeterministicProposalDataBatch::from(inner.clone());
        assert_eq!(nd.into_inner(), inner);
    }

    #[test]
    fn newtype_deref_provides_validation() {
        let inner = make_batch(&[(2, &[0xAB; 6])]);
        let tx = ParallelSignedTransactionBatch::from(inner);
        // Deref allows calling ParallelBatch methods directly
        assert!(tx.validate().is_ok());
        assert_eq!(tx.items(), 2);
        assert_eq!(tx.segments(), 1);
        assert_eq!(tx.segment(0), Some([0xAB; 6].as_slice()));
    }

    #[test]
    fn items_does_not_overflow_u32() {
        let batch = ParallelBatch {
            segment_tokens: VecM(vec![u32::MAX, u32::MAX]),
            segment_bytes: VecM(vec![0, 0]),
            xdr: BytesM(vec![]),
        };
        assert_eq!(batch.items(), 2 * u64::from(u32::MAX));
        assert!(!batch.is_empty());
    }

    #[test]
    fn is_empty_with_zero_token_segments() {
        let batch = make_batch(&[(0, &[]), (0, &[])]);
        assert!(batch.is_empty());
        assert_eq!(batch.items(), 0);
    }

    #[test]
    fn segment_with_corrupt_lengths_is_none() {
        // segment_bytes claim far more data than xdr holds; the offset of
        // the last segment exceeds u32 and must not panic or wrap.
        let batch = ParallelBatch {
            segment_tokens: VecM(vec![1, 1, 1]),
            segment_bytes: VecM(vec![u32::MAX, u32::MAX, 1]),
            xdr: BytesM(vec![0; 4]),
        };
        assert!(batch.validate().is_err());
        assert_eq!(batch.segment(0), None);
        assert_eq!(batch.segment(2), None);
    }
}
