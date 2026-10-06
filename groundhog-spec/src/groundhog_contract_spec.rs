// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 4eba524354bda9d7db1efa2845da43ce40dd1ab9

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

use xdrs::*;


#[cfg(feature = "serde")]
use ::serde::{{Serialize, Deserialize}};



// RFC 4506: string types must declare a max length; `<>` is unbounded (max 2^32-1 bytes).
pub type ContractSpecString = StringM;

// ---------------------------------------------------------------------------
// Spec ids
// ---------------------------------------------------------------------------

/// The identity of a user-defined type or event.
///
/// Every spec-named item — a `#[contract_type]`, a `#[contract_exception]`, a
/// `#[derive(Event)]` — is named by one of these, and every reference to one
/// resolves through it. A bare identifier cannot tell two of them apart: two
/// crates, two modules of one crate, or two major versions of one crate may
/// each declare a `Transfer`, and a spec naming both could not say which is
/// which.
///
/// The segments are carried apart rather than joined into
/// `CRATE@MAJOR::module::path::Name`, because the joined form cannot be split
/// back. `name = "..."` on a spec-named item substitutes its value verbatim —
/// a value carrying `::` spells its own segments and nothing rejects it — so
/// the boundary between the module path and the declared name is not
/// recoverable from the string. Consumers that need a display name or a
/// target-language identifier read `declared` directly; how an id is shortened
/// for display stays each codegen's business.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractSpecId {
    // Declaring crate, as Cargo names the package (`CARGO_PKG_NAME`, so
    // hyphenated). This is the spelling event signatures already hash and the
    // spelling RPC reports as an exception's crate.
    pub crate_name: ContractSpecString,
    // Identity version: the leftmost non-zero component of the declaring
    // crate's semver ("0.1" before 1.0, "1" after), or of the item's own
    // `version = "..."` where it pins one. Semver-compatible releases share an
    // id; only a breaking release changes it.
    pub major: ContractSpecString,
    // `module_path!()` at the declaration, verbatim -- so it leads with the
    // crate as a Rust ident (underscored), unlike `crate_name` above. Kept
    // whole because that is the one string the compiler hands the macro; the
    // generator splits nothing to fill this in. Compiler-generated, so a
    // consumer that wants the segments apart can split it safely, which is not
    // true of `declared`.
    pub module_path: ContractSpecString,
    // The declared name. Substituted verbatim from `name = "..."` where an item
    // pins one, so it may itself contain `::` -- which is why it has its own
    // field rather than being the last `::` separated segment of a joined id.
    pub declared: ContractSpecString,
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractArrayType {
    // #[rust(box_ptr)]
    pub element: Box<ContractType>,
    pub len: u32,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractVectorType {
    // #[rust(box_ptr)]
    pub element: Box<ContractType>,
    pub max_len: u32,
}

/// Mirrors #[repr(i32)] enum ContractType in sdk_rs/groundhog-contract-spec/src/model.rs
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
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

    // Contract values are exactly 32 bytes, most-significant byte first.
    // Event topics use the same bytes directly, without hashing.
    pub const U256: Self = Self(18);

    // Contract values are exactly 32 bytes, most-significant byte first, in
    // two's-complement representation. Event topics use the same bytes directly.
    pub const I256: Self = Self(19);
}
impl From<i32> for ContractTypeKind {
    fn from(v: i32) -> Self {
        Self(v)
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
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
    //   ContractSpecId named;
    Named { named: ContractSpecId } = ContractTypeKind::Named.0 as i32,
    // case U8:
    //   void;
    U8 = ContractTypeKind::U8.0 as i32,
    // case Array:
    //   ContractArrayType array;
    Array { array: ContractArrayType } = ContractTypeKind::Array.0 as i32,
    // case SignedToken:
    //   ContractSignedTokenPreimage signed_token_preimage;
    SignedToken { signed_token_preimage: ContractSignedTokenPreimage } = ContractTypeKind::SignedToken.0 as i32,
    // case U256:
    //   void;
    U256 = ContractTypeKind::U256.0 as i32,
    // case I256:
    //   void;
    I256 = ContractTypeKind::I256.0 as i32,
    
}

// ---------------------------------------------------------------------------
// SignedTokenPreimage (Rust: SignedTokenPreimage)
// ---------------------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractSignedTokenPreimage {
    // #[rust(box_ptr)], #[cpp(box)]
    pub preimage_type: Option<Box<ContractType>>,
}

// ---------------------------------------------------------------------------
// Exception type wrapper (carries selector metadata alongside the type)
// ---------------------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
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
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractField {
    pub name: ContractSpecString,
    pub ty: ContractType,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractSpecMethod {
    pub name: ContractSpecString,
    pub args: VecM<ContractField>,
    pub return_type: ContractType,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
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
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractStructDef {
    pub name: ContractSpecId,
    pub fields: VecM<ContractField>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractEnumVariant {
    pub name: ContractSpecString,
    pub value: i32,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractEnumDef {
    pub name: ContractSpecId,
    pub variants: VecM<ContractEnumVariant>,
}

/// Mirrors #[repr(i32)] enum ContractUnionCaseType in sdk_rs/groundhog-contract-spec/src/model.rs
/// (goxdr requires enum tags unique across the namespace; names prefixed for uniqueness.)
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
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


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
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
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractUnionCase {
    pub name: ContractSpecString,
    pub discriminant: i32,
    pub case_type: ContractUnionCaseType,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractUnionDef {
    pub name: ContractSpecId,
    pub cases: VecM<ContractUnionCase>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
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


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
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
// Event definitions (Rust: ContractEventDef)
// ---------------------------------------------------------------------------

/// Describes an event emitted via the LogEvent syscall (see event.x).
/// Topic 0 is always the event signature:
///   sha256(MAJOR_VERSION :: CRATE_NAME :: TYPE_NAME)
/// Topic fields and payload fields are listed separately, each in
/// declaration order. Topic values that are not already 32 bytes wide are
/// XDR-encoded and sha256-hashed; payload fields are XDR-encoded in order
/// to form the event data.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractEventDef {
    pub name: ContractSpecId,
    // topic 0
    pub signature: Topic,
    // topics 1.. ; at most kMaxTopics-1 entries
    pub topics: VecM<ContractField>,
    // payload, XDR-encoded in order
    pub fields: VecM<ContractField>,
}

// ---------------------------------------------------------------------------
// Full spec (Rust: ContractSpec)
// ---------------------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractSpec {
    pub interfaces: VecM<ContractSpecInterface>,
    pub type_defs: VecM<ContractTypeDef>,
    pub events: VecM<ContractEventDef>,
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


