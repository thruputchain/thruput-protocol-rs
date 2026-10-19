//! Allocation-free Bitcoin/Solana base58 conversion for identifier bytes.
//!
//! These helpers do not select an encoding based on byte length. Public API
//! adapters select identifier fields; opaque payloads continue to use hex.
//! StrKey input policy belongs to higher-level clients.

use core::fmt;

/// A conversion failure. Neither formatting nor debug output includes input.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// The output buffer cannot hold the encoded text or decoded bytes.
    BufferTooSmall,
    /// The input contains a character outside the case-sensitive alphabet.
    InvalidEncoding,
    /// Decoding succeeded but the identifier has the wrong byte length.
    InvalidLength {
        /// Required byte length.
        expected: usize,
        /// Decoded byte length.
        actual: usize,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BufferTooSmall => {
                f.write_str("base58 output buffer is too small")
            }
            Self::InvalidEncoding => f.write_str("invalid base58 encoding"),
            Self::InvalidLength { expected, actual } => {
                write!(f, "expected {expected} bytes, received {actual}")
            }
        }
    }
}

impl core::error::Error for Error {}

/// Conservative text bound for a decoded byte limit; saturates on overflow.
///
/// A value within this bound can still exceed the decoded limit. Empty bytes
/// encode as empty text. The ratio 1.38 exceeds log(256) / log(58).
pub const fn max_encoded_len(max_bytes: usize) -> usize {
    if max_bytes == 0 {
        return 0;
    }
    (max_bytes / 100)
        .saturating_mul(138)
        .saturating_add((max_bytes % 100) * 138 / 100)
        .saturating_add(1)
}

/// Encode bytes into the start of `output`, returning the written length.
///
/// Preserves leading zeros and adds no prefix. A buffer of
/// [`max_encoded_len(bytes.len())`](max_encoded_len) bytes is sufficient.
/// On error, the output buffer may have been modified.
pub fn encode_into(bytes: &[u8], output: &mut [u8]) -> Result<usize, Error> {
    bs58::encode(bytes)
        .onto(output)
        .map_err(|_| Error::BufferTooSmall)
}

/// Decode into the start of `output`, returning the written byte length.
///
/// Rejects whitespace and invalid characters without echoing the input. The
/// buffer length bounds decoded size and is used to reject oversized text
/// before conversion. Conversion has quadratic cost; use a field-appropriate
/// buffer size for untrusted input. On error, the buffer may be modified.
pub fn decode_into(text: &str, output: &mut [u8]) -> Result<usize, Error> {
    if text.len() > max_encoded_len(output.len()) {
        return Err(Error::BufferTooSmall);
    }
    bs58::decode(text)
        .onto(output)
        .map_err(|error| match error {
            bs58::decode::Error::BufferTooSmall => Error::BufferTooSmall,
            _ => Error::InvalidEncoding,
        })
}

/// Decode an identifier of exactly `N` bytes, with no heap allocation.
///
/// ```
/// use thruput_xdr::base58::decode_fixed;
/// assert_eq!(decode_fixed::<4>("119p")?, [0, 0, 1, 255]);
/// # Ok::<(), thruput_xdr::base58::Error>(())
/// ```
pub fn decode_fixed<const N: usize>(text: &str) -> Result<[u8; N], Error> {
    let mut bytes = [0; N];
    let actual = decode_into(text, &mut bytes)?;
    if actual != N {
        return Err(Error::InvalidLength {
            expected: N,
            actual,
        });
    }
    Ok(bytes)
}

/// Borrowed fixed-size bytes formatted as unprefixed base58.
///
/// Construct with [`base58_encode`]. Formatting uses `2 * N` bytes of stack
/// scratch space and no heap allocation; intended for small identifiers.
pub struct Base58<'a, const N: usize>(&'a [u8; N]);

impl<const N: usize> fmt::Display for Base58<'_, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Two characters per byte is sufficient even for a single byte.
        // Nested arrays keep the buffer size tied to N on stable Rust.
        let mut buffer = [[0u8; 2]; N];
        let buffer = buffer.as_flattened_mut();
        let len = encode_into(self.0, buffer).map_err(|_| fmt::Error)?;
        let text =
            core::str::from_utf8(&buffer[..len]).map_err(|_| fmt::Error)?;
        f.write_str(text)
    }
}

impl<const N: usize> fmt::Debug for Base58<'_, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/// Wrap fixed-size identifier bytes for allocation-free base58 formatting.
///
/// ```
/// use thruput_xdr::base58::base58_encode;
/// assert_eq!(format!("{}", base58_encode(&[0, 0, 1, 255])), "119p");
/// ```
pub fn base58_encode<const N: usize>(bytes: &[u8; N]) -> Base58<'_, N> {
    Base58(bytes)
}

#[cfg(test)]
mod tests;
