// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: ecaf232903ce409121e6a42607860a6a0db767e8

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::unnecessary_cast)]

use crate::*;
use core::option::Option;
use core::ops::{Deref, DerefMut};

#[cfg(feature = "std")]
use std::io::{Read, Write};
use xdrs::*;


#[cfg(feature = "serde")]
use ::serde::{{Serialize, Deserialize}};
#[cfg(feature = "serde")]
use ::serde_arrays;



// clang-format on

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct SCPValidationLevel(pub i32);

impl SCPValidationLevel {
    pub const SCP_INVALID: Self = Self(0);
    pub const SCP_MAYBE_VALID: Self = Self(1);
    pub const SCP_FULLY_VALID: Self = Self(2);
}
impl From<i32> for SCPValidationLevel {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for SCPValidationLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::SCP_INVALID.0 => {
                write!(f, "SCPValidationLevel::SCP_INVALID")
            },
            x if x == Self::SCP_MAYBE_VALID.0 => {
                write!(f, "SCPValidationLevel::SCP_MAYBE_VALID")
            },
            x if x == Self::SCP_FULLY_VALID.0 => {
                write!(f, "SCPValidationLevel::SCP_FULLY_VALID")
            },
            _ => {
                write!(f, "SCPValidationLevel({})", self.0)
            }
        }
    }
}



#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait SCP_Handle_SCP_Handle1: Send + Sync {    
    async fn scp_handle_null(&self) -> Result<(), ::rpc_api::ProgramError>;
        // hooks


    async fn scp_handle_emit_envelope(&self, arg0 : SCPEnvelope) -> Result<(), ::rpc_api::ProgramError>;
    
    async fn scp_handle_validate_ballot_value(&self, arg0 : SCPSlotIndex, arg1 : SCPValue) -> Result<SCPValidationLevel, ::rpc_api::ProgramError>;
    
    async fn scp_handle_validate_nomination_value(&self, arg0 : SCPSlotIndex, arg1 : SCPValue) -> Result<SCPValidationLevel, ::rpc_api::ProgramError>;
    
    async fn scp_handle_extract_valid_value(&self, arg0 : SCPSlotIndex, arg1 : SCPValue) -> Result<SCPOptValue, ::rpc_api::ProgramError>;
    
    async fn scp_handle_combine_candidates(&self, arg0 : SCPSlotIndex, arg1 : SCPValues) -> Result<SCPOptValue, ::rpc_api::ProgramError>;
        // notifications


    async fn scp_handle_value_externalized(&self, arg0 : SCPSlotIndex, arg1 : SCPValue) -> Result<(), ::rpc_api::ProgramError>;
    
    async fn scp_handle_nominating_value(&self, arg0 : SCPSlotIndex, arg1 : SCPValue) -> Result<(), ::rpc_api::ProgramError>;
    
    async fn scp_handle_updated_candidate_value(&self, arg0 : SCPSlotIndex, arg1 : SCPValue) -> Result<(), ::rpc_api::ProgramError>;
    
    async fn scp_handle_started_ballot_proto(&self, arg0 : SCPSlotIndex, arg1 : SCPBallot) -> Result<(), ::rpc_api::ProgramError>;
    
    async fn scp_handle_accepted_ballot_prepared(&self, arg0 : SCPSlotIndex, arg1 : SCPBallot) -> Result<(), ::rpc_api::ProgramError>;
    
    async fn scp_handle_confirmed_ballot_prepared(&self, arg0 : SCPSlotIndex, arg1 : SCPBallot) -> Result<(), ::rpc_api::ProgramError>;
    
    async fn scp_handle_accepted_commit(&self, arg0 : SCPSlotIndex, arg1 : SCPBallot) -> Result<(), ::rpc_api::ProgramError>;
    
    async fn scp_handle_ballot_did_hear_from_quorum(&self, arg0 : SCPSlotIndex, arg1 : SCPBallot) -> Result<(), ::rpc_api::ProgramError>;
        // utils

    async fn scp_handle_get_quorum_set(&self, arg0 : SCPHash) -> Result<SCPOptQuorumSet, ::rpc_api::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct SCP_Handle_SCP_Handle1_Dispatcher {
  inner: Box<dyn SCP_Handle_SCP_Handle1>,
}

impl SCP_Handle_SCP_Handle1_Dispatcher {
  pub fn new(inner: Box<dyn SCP_Handle_SCP_Handle1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_api::RpcHandler for SCP_Handle_SCP_Handle1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x9aa92002, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_api::RpcError> {
        match proc {
            // scp_handle_null
            0 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_handle_null().await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // scp_handle_emit_envelope
            10 => {
                let args: (SCPEnvelope, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_handle_emit_envelope(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // scp_handle_validate_ballot_value
            20 => {
                let args: (SCPSlotIndex, SCPValue, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_handle_validate_ballot_value(args.0, args.1).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // scp_handle_validate_nomination_value
            30 => {
                let args: (SCPSlotIndex, SCPValue, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_handle_validate_nomination_value(args.0, args.1).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // scp_handle_extract_valid_value
            40 => {
                let args: (SCPSlotIndex, SCPValue, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_handle_extract_valid_value(args.0, args.1).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // scp_handle_combine_candidates
            50 => {
                let args: (SCPSlotIndex, SCPValues, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_handle_combine_candidates(args.0, args.1).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // scp_handle_value_externalized
            100 => {
                let args: (SCPSlotIndex, SCPValue, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_handle_value_externalized(args.0, args.1).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // scp_handle_nominating_value
            200 => {
                let args: (SCPSlotIndex, SCPValue, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_handle_nominating_value(args.0, args.1).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // scp_handle_updated_candidate_value
            210 => {
                let args: (SCPSlotIndex, SCPValue, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_handle_updated_candidate_value(args.0, args.1).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // scp_handle_started_ballot_proto
            310 => {
                let args: (SCPSlotIndex, SCPBallot, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_handle_started_ballot_proto(args.0, args.1).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // scp_handle_accepted_ballot_prepared
            320 => {
                let args: (SCPSlotIndex, SCPBallot, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_handle_accepted_ballot_prepared(args.0, args.1).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // scp_handle_confirmed_ballot_prepared
            330 => {
                let args: (SCPSlotIndex, SCPBallot, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_handle_confirmed_ballot_prepared(args.0, args.1).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // scp_handle_accepted_commit
            340 => {
                let args: (SCPSlotIndex, SCPBallot, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_handle_accepted_commit(args.0, args.1).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // scp_handle_ballot_did_hear_from_quorum
            350 => {
                let args: (SCPSlotIndex, SCPBallot, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_handle_ballot_did_hear_from_quorum(args.0, args.1).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // scp_handle_get_quorum_set
            1000 => {
                let args: (SCPHash, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_handle_get_quorum_set(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            _ => Err(::rpc_api::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct SCP_Handle_SCP_Handle1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>,
}

impl SCP_Handle_SCP_Handle1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x9aa92002, 1)
    }
    pub async fn scp_handle_null(&self) -> Result<(), ::rpc_api::RpcError> {
         let data = xdrs::txs::encode(&()).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn scp_handle_emit_envelope(&self, arg0 : &SCPEnvelope) -> Result<(), ::rpc_api::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 10, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn scp_handle_validate_ballot_value(&self, arg0 : &SCPSlotIndex, arg1 : &SCPValue) -> Result<SCPValidationLevel, ::rpc_api::RpcError> {
         let data = xdrs::txs::encode(&(&arg0, &arg1)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 20, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn scp_handle_validate_nomination_value(&self, arg0 : &SCPSlotIndex, arg1 : &SCPValue) -> Result<SCPValidationLevel, ::rpc_api::RpcError> {
         let data = xdrs::txs::encode(&(&arg0, &arg1)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 30, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn scp_handle_extract_valid_value(&self, arg0 : &SCPSlotIndex, arg1 : &SCPValue) -> Result<SCPOptValue, ::rpc_api::RpcError> {
         let data = xdrs::txs::encode(&(&arg0, &arg1)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 40, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn scp_handle_combine_candidates(&self, arg0 : &SCPSlotIndex, arg1 : &SCPValues) -> Result<SCPOptValue, ::rpc_api::RpcError> {
         let data = xdrs::txs::encode(&(&arg0, &arg1)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 50, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn scp_handle_value_externalized(&self, arg0 : &SCPSlotIndex, arg1 : &SCPValue) -> Result<(), ::rpc_api::RpcError> {
         let data = xdrs::txs::encode(&(&arg0, &arg1)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 100, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn scp_handle_nominating_value(&self, arg0 : &SCPSlotIndex, arg1 : &SCPValue) -> Result<(), ::rpc_api::RpcError> {
         let data = xdrs::txs::encode(&(&arg0, &arg1)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 200, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn scp_handle_updated_candidate_value(&self, arg0 : &SCPSlotIndex, arg1 : &SCPValue) -> Result<(), ::rpc_api::RpcError> {
         let data = xdrs::txs::encode(&(&arg0, &arg1)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 210, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn scp_handle_started_ballot_proto(&self, arg0 : &SCPSlotIndex, arg1 : &SCPBallot) -> Result<(), ::rpc_api::RpcError> {
         let data = xdrs::txs::encode(&(&arg0, &arg1)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 310, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn scp_handle_accepted_ballot_prepared(&self, arg0 : &SCPSlotIndex, arg1 : &SCPBallot) -> Result<(), ::rpc_api::RpcError> {
         let data = xdrs::txs::encode(&(&arg0, &arg1)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 320, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn scp_handle_confirmed_ballot_prepared(&self, arg0 : &SCPSlotIndex, arg1 : &SCPBallot) -> Result<(), ::rpc_api::RpcError> {
         let data = xdrs::txs::encode(&(&arg0, &arg1)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 330, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn scp_handle_accepted_commit(&self, arg0 : &SCPSlotIndex, arg1 : &SCPBallot) -> Result<(), ::rpc_api::RpcError> {
         let data = xdrs::txs::encode(&(&arg0, &arg1)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 340, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn scp_handle_ballot_did_hear_from_quorum(&self, arg0 : &SCPSlotIndex, arg1 : &SCPBallot) -> Result<(), ::rpc_api::RpcError> {
         let data = xdrs::txs::encode(&(&arg0, &arg1)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 350, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn scp_handle_get_quorum_set(&self, arg0 : &SCPHash) -> Result<SCPOptQuorumSet, ::rpc_api::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 1000, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::rpc_api::RpcSender>> for SCP_Handle_SCP_Handle1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self::new(handle)
    }
}



//
// XDR internals below this point
//


