use core::fmt;

/// Borrowed bytes whose `Debug` and `Display` output is lowercase hex with
/// no prefix or separators, e.g. `deadbeef`.
///
/// Construct it with [`hex_encode`].
pub struct Hex<'a>(&'a [u8]);

impl<'a> fmt::Debug for Hex<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for b in self.0 {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}

/// Identical to the `Debug` form. Raw bytes have no second, friendlier
/// rendering to offer, and a caller embedding them in a `{}` format string
/// wants the same hex a `{:?}` would give.
impl<'a> fmt::Display for Hex<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(self, f)
    }
}

/// Wraps `b` so `{:?}` and `{}` format it as lowercase hex, for readable
/// byte values (hashes, addresses, keys) in logs and error messages
/// without allocating.
pub fn hex_encode<'a, B: AsRef<[u8]>>(b: &'a B) -> Hex<'a> {
    Hex(b.as_ref())
}
