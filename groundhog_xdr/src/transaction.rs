// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unexpected_cfgs)]
#![allow(unused_variables)]

use crate::*;
use core::option::Option;
use core::ops::{Deref, DerefMut};
use ::ciborium as cbor;
use cbor::cbor;

#[cfg(feature="std")]
use std::io::{Read, Write};

use ::serde::{Serialize, Deserialize};


// clang-format on

// BCS encoding of the call arguments (as a BCS tuple)
pub type Calldata = VecM<u8, 0xffffffff>;

pub type ReturnBuffer = VecM<u8, 0xffffffff>;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
#[repr(C)]
pub struct Invocation {
    pub invoked_address: ContractAddress,
    pub entrypoint: ContractEntrypointID,
    pub gas_limit: GasAmount,
    pub calldata: Calldata,
}

impl CBOR for Invocation {
    fn to_cbor(&self) -> cbor::Value {
        cbor::Value::Array(vec![
            self.invoked_address.to_cbor(),
            self.entrypoint.to_cbor(),
            cbor!(self.gas_limit).unwrap(),
            cbor::Value::Bytes(self.calldata.to_vec()),
        ])
    }
    fn from_cbor(v: &cbor::Value) -> Self {
        let mut x = v.as_array().unwrap().iter();
        let invoked_address = CBOR::from_cbor(x.next().unwrap());
        let entrypoint = CBOR::from_cbor(x.next().unwrap());
        let gas_limit = u64_from_cbor(x.next().unwrap());
        let calldata = CBOR::from_cbor(x.next().unwrap());

        Invocation {
            invoked_address,
            entrypoint,
            gas_limit,
            calldata,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature="std", derive(ReadXdrEnum,WriteXdrEnum))]
#[repr(i32)]
pub enum InvokeFlag {
    kInvokeFlagNone = 0,
    kInvokeFlagTryInvoke = 0x00000001,
    kInvokeFlagReadOnly = 0x00000002,
}

pub type InvokeFlagSet = i32;

pub type OriginID = [u8; 16];

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
#[repr(C)]
pub struct Transaction {
    pub id: OriginID,
    pub created: Timestamp,
    pub invocation: Invocation,
    pub contracts_to_deploy: VecM<Contract, 0xffffffff>,
}

impl CBOR for Transaction {
    fn to_cbor(&self) -> cbor::Value {
        cbor::Value::Array(vec![
            cbor::Value::Bytes(Vec::from(&self.id)),
            cbor::Value::Integer(cbor::value::Integer::from(self.created)),
            self.invocation.to_cbor(),
            self.contracts_to_deploy.to_cbor(),
        ])
    }

    fn from_cbor(v: &cbor::Value) -> Self {
        match v {
            cbor::Value::Array(vec) => {
                let mut x = vec.iter();
                let id = x.next().unwrap().as_bytes().unwrap().clone().try_into().unwrap();
                let created = i64_from_cbor(x.next().unwrap());
                let invocation = CBOR::from_cbor(x.next().unwrap());
                let contracts_to_deploy = CBOR::from_cbor(x.next().unwrap());

                Transaction {
                    id,
                    created,
                    invocation,
                    contracts_to_deploy,
                }
            },
            _ => panic!("cbor"),
        }
    }
}

pub type SignedTransactionHash = SHA256Hash;

// Deliberately leaving this SignedTransaction object
// (and all the other metadata)
// for a later PR.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
#[repr(C)]
pub struct SignedTransaction {
    pub signature: Signature,
    pub tx: Transaction,
}

impl CBOR for SignedTransaction {
    fn to_cbor(&self) -> cbor::Value {
        cbor::Value::Array(vec![
            self.signature.to_cbor(),
            self.tx.to_cbor(),
        ])
    }
    fn from_cbor(v: &cbor::Value) -> Self {
        let mut x = v.as_array().unwrap().iter();
        let signature = CBOR::from_cbor(x.next().unwrap());
        let tx = CBOR::from_cbor(x.next().unwrap());
        SignedTransaction { signature, tx }
    }
}

// Nondeterministic data attached to a transaction by the proposer.
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
#[repr(C)]
pub struct NondeterministicProposalData {
    // string of nondeterministic bits generated during the course
    // of the transaction.
    pub nondeterministic_bits: VecM<u8, 0xffffffff>,
    // the length of the nondeterministic_bits string, in bits
    // (nondeterminstic_bits.size() returns the rounded-up length in bytes)
    pub nondeterministic_bit_count: u32,
    // sequence of queue entries observed by the proposer
    pub observed_queue_entry_sequence: VecM<QueueEntry, 0xffffffff>,
}

// A signed transaction, with the nondeterministic data from the proposer
// attached to it.
// TODO(petar): Who is the signer? The originator of the transaction or the
// validator proposing it?
// TODO(petar): Should the signature capture the non-deterministic data?
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
#[repr(C)]
pub struct ProposedTransaction {
    pub signed_tx: SignedTransaction,
    pub nondeterministic_proposal_data: NondeterministicProposalData,
}

// Nominations, included in SCP messages, include only the ND data.
// The transactions themselves are disseminated through the overlay.
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
#[repr(C)]
pub struct NominatedTransaction {
    pub nondeterministic_proposal_data: NondeterministicProposalData,
}


//
// XDR internals below this point
//


