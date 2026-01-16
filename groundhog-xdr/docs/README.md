# Groundhog XDR Documentation

This directory contains module-level documentation for the `groundhog-xdr` crate.

## Structure

Each module in `groundhog-xdr` can have a corresponding markdown file in this directory. These markdown files are included in the Rust documentation via `#[doc = include_str!("...")]` attributes in `lib.rs`.

## Adding Documentation

1. **Type-level documentation**: Add `///` comments directly in the `.x` files (e.g., `protocol/types.x`). These are preserved as Rust doc comments in the generated code.

2. **Module-level documentation**: Create a `.md` file in this directory and include it in `lib.rs`:
   ```rust
   #[doc = include_str!("../docs/module_name.md")]
   #[rustfmt::skip]
   pub mod module_name;
   ```

3. **Crate-level documentation**: Add `//!` comments at the top of `lib.rs` or include a README.

## Files

- `types.md` - Documentation for the core types module

## Notes

- Documentation files are written in Markdown and support standard Rust doc comment features
- Links to types use the format `[TypeName](struct.TypeName.html)` or `[TypeName](enum.TypeName.html)`
- These files are included at compile time, so changes require recompilation to appear in `cargo doc`

