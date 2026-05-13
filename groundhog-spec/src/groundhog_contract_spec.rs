// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: a238ec3f5700fe0c9d304af7119641d21be8cc63

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



// RFC 4506: string types must declare a max length; `<>` is unbounded (max 2^32-1 bytes).
pub type ContractSpecString = StringM;


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractArrayType {
    // #[rust(box_ptr)]
    pub element: Option<Box<ContractType>>,
    pub len: u32,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractVectorType {
    // #[rust(box_ptr)]
    pub element: Option<Box<ContractType>>,
    pub max_len: u32,
}

/// Mirrors #[repr(i32)] enum ContractType in sdk_rs/groundhog-contract-spec/src/model.rs
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct ContractTypeKind(pub i32);

impl ContractTypeKind {
    pub const Void: Self = Self(0);
    pub const Bool: Self = Self(1);
    pub const U32: Self = Self(2);
    pub const I32: Self = Self(3);
    pub const U64: Self = Self(4);
    pub const I64: Self = Self(5);
    pub const U128: Self = Self(6);
    pub const I128: Self = Self(7);
    pub const String: Self = Self(8);
    pub const Bytes: Self = Self(9);
    pub const Address: Self = Self(10);
    pub const Option: Self = Self(11);
    pub const Vector: Self = Self(12);
    pub const Tuple: Self = Self(13);
    pub const Named: Self = Self(14);
    pub const U8: Self = Self(15);
    pub const Array: Self = Self(16);
    pub const SignedToken: Self = Self(17);
}
impl From<i32> for ContractTypeKind {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for ContractTypeKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::Void.0 => {
                write!(f, "ContractTypeKind::Void")
            },
            x if x == Self::Bool.0 => {
                write!(f, "ContractTypeKind::Bool")
            },
            x if x == Self::U32.0 => {
                write!(f, "ContractTypeKind::U32")
            },
            x if x == Self::I32.0 => {
                write!(f, "ContractTypeKind::I32")
            },
            x if x == Self::U64.0 => {
                write!(f, "ContractTypeKind::U64")
            },
            x if x == Self::I64.0 => {
                write!(f, "ContractTypeKind::I64")
            },
            x if x == Self::U128.0 => {
                write!(f, "ContractTypeKind::U128")
            },
            x if x == Self::I128.0 => {
                write!(f, "ContractTypeKind::I128")
            },
            x if x == Self::String.0 => {
                write!(f, "ContractTypeKind::String")
            },
            x if x == Self::Bytes.0 => {
                write!(f, "ContractTypeKind::Bytes")
            },
            x if x == Self::Address.0 => {
                write!(f, "ContractTypeKind::Address")
            },
            x if x == Self::Option.0 => {
                write!(f, "ContractTypeKind::Option")
            },
            x if x == Self::Vector.0 => {
                write!(f, "ContractTypeKind::Vector")
            },
            x if x == Self::Tuple.0 => {
                write!(f, "ContractTypeKind::Tuple")
            },
            x if x == Self::Named.0 => {
                write!(f, "ContractTypeKind::Named")
            },
            x if x == Self::U8.0 => {
                write!(f, "ContractTypeKind::U8")
            },
            x if x == Self::Array.0 => {
                write!(f, "ContractTypeKind::Array")
            },
            x if x == Self::SignedToken.0 => {
                write!(f, "ContractTypeKind::SignedToken")
            },
            _ => {
                write!(f, "ContractTypeKind({})", self.0)
            }
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum ContractType {
    // case Void:
    //   void;
    Void = ContractTypeKind::Void.0 as i32,
    // case Bool:
    //   void;
    Bool = ContractTypeKind::Bool.0 as i32,
    // case U32:
    //   void;
    U32 = ContractTypeKind::U32.0 as i32,
    // case I32:
    //   void;
    I32 = ContractTypeKind::I32.0 as i32,
    // case U64:
    //   void;
    U64 = ContractTypeKind::U64.0 as i32,
    // case I64:
    //   void;
    I64 = ContractTypeKind::I64.0 as i32,
    // case U128:
    //   void;
    U128 = ContractTypeKind::U128.0 as i32,
    // case I128:
    //   void;
    I128 = ContractTypeKind::I128.0 as i32,
    // case String:
    //   void;
    String = ContractTypeKind::String.0 as i32,
    // case Bytes:
    //   void;
    Bytes = ContractTypeKind::Bytes.0 as i32,
    // case Address:
    //   void;
    Address = ContractTypeKind::Address.0 as i32,
    // #[rust(box_ptr)], #[cpp(box)]
    // case Option:
    //   Box<ContractType> option_inner;
    Option { option_inner: Box<ContractType> } = ContractTypeKind::Option.0 as i32,
    // case Vector:
    //   ContractVectorType vector;
    Vector { vector: ContractVectorType } = ContractTypeKind::Vector.0 as i32,
    // case Tuple:
    //   VecM<ContractType> tuple;
    Tuple { tuple: VecM<ContractType> } = ContractTypeKind::Tuple.0 as i32,
    // case Named:
    //   ContractSpecString named;
    Named { named: ContractSpecString } = ContractTypeKind::Named.0 as i32,
    // case U8:
    //   void;
    U8 = ContractTypeKind::U8.0 as i32,
    // case Array:
    //   ContractArrayType array;
    Array { array: ContractArrayType } = ContractTypeKind::Array.0 as i32,
    // case SignedToken:
    //   ContractSignedTokenPreimage signed_token_preimage;
    SignedToken { signed_token_preimage: ContractSignedTokenPreimage } = ContractTypeKind::SignedToken.0 as i32,
    
}

// ---------------------------------------------------------------------------
// SignedTokenPreimage (Rust: SignedTokenPreimage)
// ---------------------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractSignedTokenPreimage {
    // #[rust(box_ptr)], #[cpp(box)]
    pub preimage_type: Box<ContractType>,
}

// ---------------------------------------------------------------------------
// Exception type wrapper (carries selector metadata alongside the type)
// ---------------------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractExceptionType {
    pub selector: u64,
    pub ty: ContractType,
}

// ---------------------------------------------------------------------------
// Fields, methods, interfaces
// ---------------------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractField {
    pub name: ContractSpecString,
    pub ty: ContractType,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractSpecMethod {
    pub name: ContractSpecString,
    pub args: VecM<ContractField>,
    pub return_type: ContractType,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractSpecInterface {
    pub name: ContractSpecString,
    pub vers: ContractSpecString,
    pub throws: VecM<ContractExceptionType>,
    pub methods: VecM<ContractSpecMethod>,
}

// ---------------------------------------------------------------------------
// User-defined types (Rust: ContractTypeDef and components)
// ---------------------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractStructDef {
    pub name: ContractSpecString,
    pub fields: VecM<ContractField>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractEnumVariant {
    pub name: ContractSpecString,
    pub value: i32,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractEnumDef {
    pub name: ContractSpecString,
    pub variants: VecM<ContractEnumVariant>,
}

/// Mirrors #[repr(i32)] enum ContractUnionCaseType in sdk_rs/groundhog-contract-spec/src/model.rs
/// (goxdr requires enum tags unique across the namespace; names prefixed for uniqueness.)
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct ContractUnionCaseKind(pub i32);

impl ContractUnionCaseKind {
    pub const UnionCaseVoid: Self = Self(0);
    pub const UnionCaseTuple: Self = Self(1);
    pub const UnionCaseStruct: Self = Self(2);
}
impl From<i32> for ContractUnionCaseKind {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for ContractUnionCaseKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::UnionCaseVoid.0 => {
                write!(f, "ContractUnionCaseKind::UnionCaseVoid")
            },
            x if x == Self::UnionCaseTuple.0 => {
                write!(f, "ContractUnionCaseKind::UnionCaseTuple")
            },
            x if x == Self::UnionCaseStruct.0 => {
                write!(f, "ContractUnionCaseKind::UnionCaseStruct")
            },
            _ => {
                write!(f, "ContractUnionCaseKind({})", self.0)
            }
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum ContractUnionCaseType {
    // case UnionCaseVoid:
    //   void;
    UnionCaseVoid = ContractUnionCaseKind::UnionCaseVoid.0 as i32,
    // case UnionCaseTuple:
    //   VecM<ContractType> tuple;
    UnionCaseTuple { tuple: VecM<ContractType> } = ContractUnionCaseKind::UnionCaseTuple.0 as i32,
    // case UnionCaseStruct:
    //   VecM<ContractField> struct_fields;
    UnionCaseStruct { struct_fields: VecM<ContractField> } = ContractUnionCaseKind::UnionCaseStruct.0 as i32,
    
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractUnionCase {
    pub name: ContractSpecString,
    pub discriminant: i32,
    pub case_type: ContractUnionCaseType,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractUnionDef {
    pub name: ContractSpecString,
    pub cases: VecM<ContractUnionCase>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct ContractTypeDefKind(pub i32);

impl ContractTypeDefKind {
    pub const TypeDefStruct: Self = Self(0);
    pub const TypeDefEnum: Self = Self(1);
    pub const TypeDefUnion: Self = Self(2);
}
impl From<i32> for ContractTypeDefKind {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for ContractTypeDefKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::TypeDefStruct.0 => {
                write!(f, "ContractTypeDefKind::TypeDefStruct")
            },
            x if x == Self::TypeDefEnum.0 => {
                write!(f, "ContractTypeDefKind::TypeDefEnum")
            },
            x if x == Self::TypeDefUnion.0 => {
                write!(f, "ContractTypeDefKind::TypeDefUnion")
            },
            _ => {
                write!(f, "ContractTypeDefKind({})", self.0)
            }
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum ContractTypeDef {
    // case TypeDefStruct:
    //   ContractStructDef struct_def;
    TypeDefStruct { struct_def: ContractStructDef } = ContractTypeDefKind::TypeDefStruct.0 as i32,
    // case TypeDefEnum:
    //   ContractEnumDef enum_def;
    TypeDefEnum { enum_def: ContractEnumDef } = ContractTypeDefKind::TypeDefEnum.0 as i32,
    // case TypeDefUnion:
    //   ContractUnionDef union_def;
    TypeDefUnion { union_def: ContractUnionDef } = ContractTypeDefKind::TypeDefUnion.0 as i32,
    
}

// ---------------------------------------------------------------------------
// Full spec (Rust: ContractSpec)
// ---------------------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractSpec {
    pub interfaces: VecM<ContractSpecInterface>,
    pub type_defs: VecM<ContractTypeDef>,
}

// ---------------------------------------------------------------------------
// Spec fragments (Rust: SpecFragment and components)
// ---------------------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SpecInterfaceFragment {
    pub interface_name: ContractSpecString,
    pub interface_version: ContractSpecString,
    pub throws: VecM<ContractExceptionType>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SpecMethodFragment {
    pub interface_name: ContractSpecString,
    pub interface_version: ContractSpecString,
    pub method: ContractSpecMethod,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct SpecFragmentKind(pub i32);

impl SpecFragmentKind {
    pub const SpecInterface: Self = Self(0);
    pub const SpecMethod: Self = Self(1);
    pub const SpecTypeDef: Self = Self(2);
    pub const ExportedSpecTypeDef: Self = Self(3);
}
impl From<i32> for SpecFragmentKind {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for SpecFragmentKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::SpecInterface.0 => {
                write!(f, "SpecFragmentKind::SpecInterface")
            },
            x if x == Self::SpecMethod.0 => {
                write!(f, "SpecFragmentKind::SpecMethod")
            },
            x if x == Self::SpecTypeDef.0 => {
                write!(f, "SpecFragmentKind::SpecTypeDef")
            },
            x if x == Self::ExportedSpecTypeDef.0 => {
                write!(f, "SpecFragmentKind::ExportedSpecTypeDef")
            },
            _ => {
                write!(f, "SpecFragmentKind({})", self.0)
            }
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum SpecFragment {
    // case SpecInterface:
    //   SpecInterfaceFragment interface_fragment;
    SpecInterface { interface_fragment: SpecInterfaceFragment } = SpecFragmentKind::SpecInterface.0 as i32,
    // case SpecMethod:
    //   SpecMethodFragment method_fragment;
    SpecMethod { method_fragment: SpecMethodFragment } = SpecFragmentKind::SpecMethod.0 as i32,
    // case SpecTypeDef:
    // case ExportedSpecTypeDef:
    //   ContractTypeDef type_def;
    SpecTypeDef { type_def: ContractTypeDef } = SpecFragmentKind::SpecTypeDef.0 as i32,
    ExportedSpecTypeDef { type_def: ContractTypeDef } = SpecFragmentKind::ExportedSpecTypeDef.0 as i32,
    
}


//
// XDR internals below this point
//

impl ContractType {
    pub fn v(&self) -> ContractTypeKind {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl ContractUnionCaseType {
    pub fn v(&self) -> ContractUnionCaseKind {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl ContractTypeDef {
    pub fn v(&self) -> ContractTypeDefKind {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl SpecFragment {
    pub fn v(&self) -> SpecFragmentKind {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


