#![no_std]

// Brings types from %#include'd .x files (event.hh) into scope for the
// generated module, which imports them via `use crate::*`.
use thruput_xdr::event::Topic;

#[rustfmt::skip]
pub mod thruput_contract_spec;
pub use thruput_contract_spec::*;

// Hand-written impls on the generated types. `generate.sh` rewrites only
// `thruput_contract_spec.rs`, so these survive regeneration.
mod impls;
