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
    pub fn items(&self) -> u32 {
        self.segment_tokens.iter().sum()
    }

    /// Number of segments in this batch.
    pub fn segments(&self) -> usize {
        self.segment_tokens.len()
    }

    /// True if the batch contains no items.
    pub fn is_empty(&self) -> bool {
        self.items() == 0
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
        if i >= self.segment_bytes.len() {
            return None;
        }
        let offset: usize =
            self.segment_bytes[..i].iter().map(|&b| b as usize).sum();
        let len = self.segment_bytes[i] as usize;
        self.xdr.0.get(offset..offset + len)
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
