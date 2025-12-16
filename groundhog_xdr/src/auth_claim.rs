// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unexpected_cfgs)]
#![allow(unused_variables)]
#![allow(clippy::unnecessary_cast)]

use crate::*;
use core::option::Option;
use core::ops::{Deref, DerefMut};

#[cfg(feature="std")]
use std::io::{Read, Write};

#[cfg(feature="xdrs")]
use xdrs::*;
#[cfg(feature="spec")]
use xdrs::xdr_repr::*;


#[cfg(not(feature="xdrs"))]
use xdr_common::*;



#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(i32)]
pub enum AuthClaimType {
    kInvokedAddressSignature = 0 as i32,
    kInvokedAddressSignedWeight = 1 as i32,
    kOtherAddressSignature = 2 as i32,
    kOtherAddressSignedWeight = 3 as i32,
}
pub use AuthClaimType::*;

impl TryFrom<i32> for AuthClaimType {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::kInvokedAddressSignature),
            1 => Ok(Self::kInvokedAddressSignedWeight),
            2 => Ok(Self::kOtherAddressSignature),
            3 => Ok(Self::kOtherAddressSignedWeight),
            _ => Err(()),
        }
    }
}


// between 0 and 31, i.e. 1 byte in CXS encoding
// This is why MAX_SIGNERS <= 31
pub type KeyID = u32;

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct AuthClaim_invoked_address_signed_weight {
    pub accepted_keys: BitMap,
    pub required_weight: Weight,
}

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct AuthClaim_other_address_signature_id {
    pub address: ContractAddress,
    pub key_id: KeyID,
}

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct AuthClaim_other_address_signed_weight {
    pub address: ContractAddress,
    pub accepted_keys: BitMap,
    pub required_weight: Weight,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[repr(i32)]
pub enum AuthClaim {
    // case kInvokedAddressSignature:
    //   KeyID invoked_address_signature_id;
    kInvokedAddressSignature { invoked_address_signature_id: KeyID } = AuthClaimType::kInvokedAddressSignature as i32,
    // case kInvokedAddressSignedWeight:
    //   invoked_address_signed_weight invoked_address_signed_weight;
    kInvokedAddressSignedWeight { invoked_address_signed_weight: AuthClaim_invoked_address_signed_weight } = AuthClaimType::kInvokedAddressSignedWeight as i32,
    // case kOtherAddressSignature:
    //   other_address_signature_id other_address_signature_id;
    kOtherAddressSignature { other_address_signature_id: AuthClaim_other_address_signature_id } = AuthClaimType::kOtherAddressSignature as i32,
    // case kOtherAddressSignedWeight:
    //   other_address_signed_weight other_address_signed_weight;
    kOtherAddressSignedWeight { other_address_signed_weight: AuthClaim_other_address_signed_weight } = AuthClaimType::kOtherAddressSignedWeight as i32,
    
}

pub const kMaxAuthClaims : u32 = 10;

pub type AuthClaimData = VecM::<AuthClaim, kMaxAuthClaims>;


//
// XDR internals below this point
//

impl AuthClaim {
    pub fn claim_type(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}


