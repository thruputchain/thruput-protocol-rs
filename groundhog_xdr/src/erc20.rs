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

#[cfg(feature="std")]
use std::io::{Read, Write};




#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature="std", derive(ReadXdrEnum,WriteXdrEnum))]
#[repr(i32)]
pub enum ResultType {
    Ok = 0,
    Err = 1,
}

#[repr(i32)]
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
pub enum Result<T, E> {
    // case Ok:
    //   T val;
    Ok { val: T } = ResultType::Ok as i32,
    // case Err:
    //   E err;
    Err { err: E } = ResultType::Err as i32,
    
}


#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature="std", derive(ReadXdrEnum,WriteXdrEnum))]
#[repr(i32)]
pub enum ContractErrorType {
    // "Storage-layer error"
    StorageErr = 0,

    // "System error"
    SystemErr = 1,
}

#[repr(i32)]
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
pub enum ContractError {
    // case StorageErr:
    //   void;
    StorageErr = ContractErrorType::StorageErr as i32,
    // case SystemErr:
    //   void;
    SystemErr = ContractErrorType::SystemErr as i32,
    
}


#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature="std", derive(ReadXdrEnum,WriteXdrEnum))]
#[repr(i32)]
pub enum ErrorType {
    // Contract already initialized
    AlreadyInitialized = 0,

    // Caller must be contract owner
    MustBeContractOwner = 1,

    // Internal contract error
    ContractErr = 2,
}

#[repr(i32)]
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
pub enum Error {
    // case AlreadyInitialized:
    //   void;
    AlreadyInitialized = ErrorType::AlreadyInitialized as i32,
    // case MustBeContractOwner:
    //   void;
    MustBeContractOwner = ErrorType::MustBeContractOwner as i32,
    // case ContractErr:
    //   ContractError contract_error;
    ContractErr { contract_error: ContractError } = ErrorType::ContractErr as i32,
    
}

#[repr(i32)]
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
pub enum Res<T> {
    // case Ok:
    //   T val;
    Ok { val: T } = ResultType::Ok as i32,
    // case Err:
    //   Error err;
    Err { err: Error } = ResultType::Err as i32,
    
}

#[derive(PartialEq, Debug, Default, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
#[repr(C)]
pub struct Void {
    // temporary: ReadXdr/WriteXdr derive-macros don't support unnamed fields
    pub not_void: i32,
}

pub type Amount = uint128_xdr;

pub type Difference = int128_xdr;



//
// XDR internals below this point
//

impl<T, E> Result<T, E> {
    pub fn is_err(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl ContractError {
    pub fn err_type(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl Error {
    pub fn err_type(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl<T> Res<T> {
    pub fn is_err(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}


