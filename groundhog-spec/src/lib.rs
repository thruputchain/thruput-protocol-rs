#![cfg_attr(not(feature = "std"), no_std)]

// Brings types from %#include'd .x files (event.hh) into scope for the
// generated module, which imports them via `use crate::*`.
use groundhog_xdr::event::Topic;

#[rustfmt::skip]
pub mod groundhog_contract_spec;
pub use groundhog_contract_spec::*;
