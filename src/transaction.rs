// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unexpected_cfgs)]
#![allow(unused_variables)]

use core::option::Option;

use crate::*;

#[cfg(feature="std")]
use std::io::{Read, Write};




// clang-format on
pub type Calldata = VecM<u8, 0xffffffff>;

pub type ReturnBuffer = VecM<u8, 0xffffffff>;

#[repr(C)]
#[derive(PartialEq, Debug, Default, Clone)]
pub struct Invocation {
    pub invoked_address: ContractAddress,
    pub method: u32,
    pub gas_limit: GasAmount,
    pub calldata: Calldata,
}

#[repr(C)]
#[derive(PartialEq, Debug, Default, Clone)]
pub struct Transaction {
    pub invocation: Invocation,
    pub contracts_to_deploy: VecM<Contract, 0xffffffff>,
}

// Deliberately leaving this SignedTransaction object
// (and all the other metadata)
// for a later PR.
#[repr(C)]
#[derive(PartialEq, Debug, Default, Clone)]
pub struct SignedTransaction {
    pub tx: Transaction,
}

// Nondeterministic data attached to a transaction by the proposer.
#[repr(C)]
#[derive(PartialEq, Debug, Default, Clone)]
pub struct NondeterministicProposalData {
    // string of nondeterministic bits generated during the course
    // of the transaction.
    pub nondeterministic_bits: VecM<u8, 0xffffffff>,
    // the length of the nondeterministic_bits string, in bits
    // (nondeterminstic_bits.size() returns the rounded-up length in bytes)
    pub nondeterministic_bit_count: u32,
}

// A signed transaction, with the nondeterministic data from the proposer
// attached to it.
#[repr(C)]
#[derive(PartialEq, Debug, Default, Clone)]
pub struct ProposedTransaction {
    pub signed_tx: SignedTransaction,
    pub nondeterministic_proposal_data: NondeterministicProposalData,
}


//
// XDR internals below this point
//


