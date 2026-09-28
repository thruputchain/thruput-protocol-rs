use core::fmt;

use crate::{ContractSpecId, ContractSpecString};

/// `module_path::declared (crate_name@major)`: an id the way a person reading
/// a diagnostic wants it.
///
/// Not the joined `CRATE@MAJOR::module::path::Name` form: a reader wants to
/// see which segment is which, and `module_path` already leads with the crate,
/// so the crate and major trail in parentheses rather than doubling up in the
/// path.
///
/// Only for messages. The rendering is one-way -- `declared` may itself carry
/// `::`, so where the module path ends cannot be recovered from the text --
/// and ids are compared and keyed whole, never by this.
///
/// Lossy: nothing on the wire makes a segment UTF-8, and a diagnostic is the
/// wrong place to fail, so an invalid sequence renders as U+FFFD, exactly as
/// `String::from_utf8_lossy` has it. Segments are decoded rather than written
/// through `ContractSpecString`'s own `Display`, which hex-escapes every
/// non-ASCII byte, because a Rust identifier may be non-ASCII. Validate the
/// segments first wherever their bytes matter.
impl fmt::Display for ContractSpecId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_lossy(f, &self.module_path)?;
        f.write_str("::")?;
        write_lossy(f, &self.declared)?;
        f.write_str(" (")?;
        write_lossy(f, &self.crate_name)?;
        f.write_str("@")?;
        write_lossy(f, &self.major)?;
        f.write_str(")")
    }
}

/// Write `s` as text the way `String::from_utf8_lossy` would, without
/// allocating.
fn write_lossy(
    f: &mut fmt::Formatter<'_>,
    s: &ContractSpecString,
) -> fmt::Result {
    for chunk in s.as_slice().utf8_chunks() {
        f.write_str(chunk.valid())?;
        if !chunk.invalid().is_empty() {
            f.write_str("\u{FFFD}")?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use std::format;
    use std::string::{String, ToString};

    fn s(bytes: &[u8]) -> ContractSpecString {
        ContractSpecString::try_from(bytes).unwrap()
    }

    fn id(declared: &[u8]) -> ContractSpecId {
        ContractSpecId {
            crate_name: s(b"my-token"),
            major: s(b"0.1"),
            module_path: s(b"my_token::types"),
            declared: s(declared),
        }
    }

    #[test]
    fn renders_path_then_crate_and_major() {
        assert_eq!(
            id(b"Meta").to_string(),
            "my_token::types::Meta (my-token@0.1)"
        );
        // A `::` in `declared` passes through verbatim, which is what makes
        // the rendering one-way.
        assert_eq!(
            id(b"A::B").to_string(),
            "my_token::types::A::B (my-token@0.1)"
        );
        assert_eq!(
            id("Café".as_bytes()).to_string(),
            "my_token::types::Café (my-token@0.1)"
        );
    }

    #[test]
    fn invalid_utf8_renders_as_from_utf8_lossy_does() {
        let cases: [&[u8]; 5] = [
            b"Bad\xffName",
            b"\xff\xfe",
            b"cut\xe2\x82",
            b"\xe2\x82\xac",
            b"",
        ];
        for declared in cases {
            assert_eq!(
                id(declared).to_string(),
                format!(
                    "my_token::types::{} (my-token@0.1)",
                    String::from_utf8_lossy(declared)
                ),
                "declared = {declared:?}",
            );
        }
    }
}
