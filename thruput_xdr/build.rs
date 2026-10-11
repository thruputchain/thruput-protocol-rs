//! Rebuild tracking for files read by `thruput_xdr_macros` at expansion
//! time.
//!
//! `generate_syscall_numbers!` and `generate_syscall_decls!` load
//! `src/syscalls.yaml` (and the docs JSON) from this crate's manifest
//! directory with `std::fs`, which leaves no dependency edge in cargo's
//! dep-info. Without these directives, editing either file does not
//! trigger a rebuild and the generated bindings go stale.

fn main() {
    println!("cargo:rerun-if-changed=src/syscalls.yaml");
    println!("cargo:rerun-if-changed=syscalls_docs.json");
}
