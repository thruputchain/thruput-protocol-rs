// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 6ab835b07c234abb4b0b50b61d687904727a4f32

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
#[cfg(feature = "xdrs")]
use xdrs::*;


#[cfg(feature = "serde")]
use ::serde::{{Serialize, Deserialize}};



// clang-format on

/// Names one consensus-critical configuration parameter.
///
/// Append-only: an id, once assigned, means that parameter forever, and a
/// retired parameter keeps its id rather than freeing it for reuse.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct ConfigParamID(pub i32);

impl ConfigParamID {
    pub const kConfigParamMaxSHA256Buffers: Self = Self(0);
    pub const kConfigParamMaxCrossInvokeDepth: Self = Self(1);
    pub const kConfigParamMaxNewContractsPerTx: Self = Self(2);
    pub const kConfigParamMaxBytestringLen: Self = Self(3);
    pub const kConfigParamMaxContractSizeBytes: Self = Self(4);
    pub const kConfigParamMaxSavepointDepth: Self = Self(5);
    pub const kConfigParamMaxSHA256BufferBytes: Self = Self(6);
    pub const kConfigParamMaxAuthTokenDepth: Self = Self(7);
    pub const kConfigParamMaxRv32CodeBytes: Self = Self(8);
    pub const kConfigParamMaxRv32MemoryBytes: Self = Self(9);
    pub const kConfigParamMaxAuthTokens: Self = Self(10);
    pub const kConfigParamMaxCalldataLen: Self = Self(11);
    pub const kConfigParamMaxReturnBufferLen: Self = Self(12);
    pub const kConfigParamMaxTxGas: Self = Self(13);
}
impl From<i32> for ConfigParamID {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for ConfigParamID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::kConfigParamMaxSHA256Buffers.0 => {
                write!(f, "ConfigParamID::kConfigParamMaxSHA256Buffers")
            },
            x if x == Self::kConfigParamMaxCrossInvokeDepth.0 => {
                write!(f, "ConfigParamID::kConfigParamMaxCrossInvokeDepth")
            },
            x if x == Self::kConfigParamMaxNewContractsPerTx.0 => {
                write!(f, "ConfigParamID::kConfigParamMaxNewContractsPerTx")
            },
            x if x == Self::kConfigParamMaxBytestringLen.0 => {
                write!(f, "ConfigParamID::kConfigParamMaxBytestringLen")
            },
            x if x == Self::kConfigParamMaxContractSizeBytes.0 => {
                write!(f, "ConfigParamID::kConfigParamMaxContractSizeBytes")
            },
            x if x == Self::kConfigParamMaxSavepointDepth.0 => {
                write!(f, "ConfigParamID::kConfigParamMaxSavepointDepth")
            },
            x if x == Self::kConfigParamMaxSHA256BufferBytes.0 => {
                write!(f, "ConfigParamID::kConfigParamMaxSHA256BufferBytes")
            },
            x if x == Self::kConfigParamMaxAuthTokenDepth.0 => {
                write!(f, "ConfigParamID::kConfigParamMaxAuthTokenDepth")
            },
            x if x == Self::kConfigParamMaxRv32CodeBytes.0 => {
                write!(f, "ConfigParamID::kConfigParamMaxRv32CodeBytes")
            },
            x if x == Self::kConfigParamMaxRv32MemoryBytes.0 => {
                write!(f, "ConfigParamID::kConfigParamMaxRv32MemoryBytes")
            },
            x if x == Self::kConfigParamMaxAuthTokens.0 => {
                write!(f, "ConfigParamID::kConfigParamMaxAuthTokens")
            },
            x if x == Self::kConfigParamMaxCalldataLen.0 => {
                write!(f, "ConfigParamID::kConfigParamMaxCalldataLen")
            },
            x if x == Self::kConfigParamMaxReturnBufferLen.0 => {
                write!(f, "ConfigParamID::kConfigParamMaxReturnBufferLen")
            },
            x if x == Self::kConfigParamMaxTxGas.0 => {
                write!(f, "ConfigParamID::kConfigParamMaxTxGas")
            },
            _ => {
                write!(f, "ConfigParamID({})", self.0)
            }
        }
    }
}


/// One configuration parameter and the value in force for it.
///
/// Serves both uses of that pairing: `NetworkConfigV0` states a chain's
/// configuration as a list of these, and is meant to carry one per parameter
/// being changed by a consensus upgrade, where everything a block does not
/// name keeps the value it already has. No upgrade mechanism exists yet:
/// `ConsensusValue::upgrades` is opaque, and the node leaves it empty. Each arm
/// carries its own type, so a future parameter need not be an `unsigned int`.
///
/// There is no `default` arm on purpose: a node that cannot name a parameter
/// cannot know what value is in force for it, so an unrecognized id must fail
/// to decode rather than decode to something the node then ignores.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[repr(i32)]
#[non_exhaustive]
pub enum ConfigParam {
    /// Concurrently open incremental SHA-256 buffers permitted per invocation
    /// frame (`SHA256Update`, `SHA256Copy`, `SHA256Finalize`). 32 is the value
    /// in force for a configuration that does not state one.
    // case kConfigParamMaxSHA256Buffers:
    //   u32 max_sha256_buffers;
    kConfigParamMaxSHA256Buffers { max_sha256_buffers: u32 } = ConfigParamID::kConfigParamMaxSHA256Buffers.0 as i32,
    /// Invocation frames one transaction may have open at once, counting every
    /// invoke rather than only the savepoint-pushing ones that
    /// `kConfigParamMaxSavepointDepth` bounds. Each open frame holds a live contract
    /// runtime, so this is what stops a recursive call chain from growing until
    /// the executor runs out of room for it. 16 is the value in force for a
    /// configuration that does not state one.
    // case kConfigParamMaxCrossInvokeDepth:
    //   u32 max_cross_invoke_depth;
    kConfigParamMaxCrossInvokeDepth { max_cross_invoke_depth: u32 } = ConfigParamID::kConfigParamMaxCrossInvokeDepth.0 as i32,
    /// Contracts one transaction may attach in `contracts_to_deploy`.
    ///
    /// Counted over what is attached rather than what a contract goes on to
    /// deploy: the executor holds every attached contract for the whole
    /// transaction, and the ones deployed are a subset chosen during execution.
    /// Attaching the same contract twice is rejected outright, so the count is
    /// over distinct contracts either way. 4 is the value in force for a
    /// configuration that does not state one.
    ///
    /// Together with `kConfigParamMaxContractSizeBytes` this is what bounds the
    /// transaction's final commit batch, which the executor sends to storage as
    /// a single message. A chain must set the pair low enough that the largest
    /// batch it admits still fits that message; the node rejects a genesis
    /// that does not.
    // case kConfigParamMaxNewContractsPerTx:
    //   u32 max_new_contracts_per_tx;
    kConfigParamMaxNewContractsPerTx { max_new_contracts_per_tx: u32 } = ConfigParamID::kConfigParamMaxNewContractsPerTx.0 as i32,
    /// Bytes one bytestring storage item may hold, checked where a contract
    /// writes one rather than where one is read, so that lowering it leaves
    /// existing items readable instead of stranding state.
    ///
    /// `kBytestringCeilingLen` bounds this. A chain must also keep it low
    /// enough that its own storage engine handles the largest item it thereby
    /// admits; the ceiling does not guarantee that.
    // case kConfigParamMaxBytestringLen:
    //   u32 max_bytestring_len;
    kConfigParamMaxBytestringLen { max_bytestring_len: u32 } = ConfigParamID::kConfigParamMaxBytestringLen.0 as i32,
    /// Bytes one attached contract's code blob (`ContractBlob.data`) may hold,
    /// checked before any attached contract is verified or executed. It is
    /// the same quantity `contract_deployment_cost_per_byte` charges for.
    ///
    /// `kDefaultMaxContractSizeBytes` is the value in force for a
    /// configuration that does not state one. The wire format does not bound
    /// `ContractBlob`, so nothing here caps what a chain may state: a chain
    /// must keep it low enough that its own storage engine stores the largest
    /// contract it thereby admits, that every attached contract still fits a
    /// transaction (`kMaxBytesSignedTransaction`), and that the commit batch
    /// `kConfigParamMaxNewContractsPerTx` bounds fits one storage message.
    /// The node rejects a genesis that does not.
    // case kConfigParamMaxContractSizeBytes:
    //   u32 max_contract_size_bytes;
    kConfigParamMaxContractSizeBytes { max_contract_size_bytes: u32 } = ConfigParamID::kConfigParamMaxContractSizeBytes.0 as i32,
    /// Storage savepoints one transaction may hold at once: one per successful
    /// `TryInvoke` whose writes are still held (finished siblings included),
    /// plus the one the root commit takes. A `TryInvoke` whose child would go
    /// past the limit still runs, then fails deterministically, as one that
    /// loses a lock conflict does, and its writes are discarded.
    /// `kMaxSavepointDepth` (1024) is the value in force for a configuration
    /// that does not state one.
    ///
    /// Every open savepoint is a live nested VM on one executor thread, so a
    /// chain must keep this within what its executors' stacks hold; the
    /// node rejects a genesis above its build ceiling.
    // case kConfigParamMaxSavepointDepth:
    //   u32 max_savepoint_depth;
    kConfigParamMaxSavepointDepth { max_savepoint_depth: u32 } = ConfigParamID::kConfigParamMaxSavepointDepth.0 as i32,
    /// Legacy cumulative SHA-256 byte limit. Retained for wire compatibility
    /// and configuration hashing; no longer enforced. SHA256Update checks gas
    /// before processing input and keeps constant-size incremental state.
    // case kConfigParamMaxSHA256BufferBytes:
    //   u32 max_sha256_buffer_bytes;
    kConfigParamMaxSHA256BufferBytes { max_sha256_buffer_bytes: u32 } = ConfigParamID::kConfigParamMaxSHA256BufferBytes.0 as i32,
    /// Levels of `AuthTokenNode` nesting an auth token may carry, counted from
    /// its root, checked where a token is minted or forwarded into a view.
    /// `kAuthTokenMaxDepth` is the value in force for a configuration that
    /// does not state one. The wire type bounds a node's payload and its
    /// direct children (`kAuthTokenMaxLen`, `kAuthTokenMaxChildNodes`) but not
    /// the depth, so this is what bounds a token's total size.
    // case kConfigParamMaxAuthTokenDepth:
    //   u32 max_auth_token_depth;
    kConfigParamMaxAuthTokenDepth { max_auth_token_depth: u32 } = ConfigParamID::kConfigParamMaxAuthTokenDepth.0 as i32,
    /// Bytes of code (`code_size` in the artifact header) an RV32 artifact may
    /// carry to be admitted as an attached contract.
    /// `kDefaultMaxRv32CodeBytes` (262144) is the value in force for a
    /// configuration that does not state one.
    ///
    /// Checked where a contract is attached, not where a deployed one is
    /// instantiated, so lowering it leaves deployed contracts runnable. The
    /// artifact format itself does not bound the field; the node rejects a
    /// genesis above what its VM can translate.
    // case kConfigParamMaxRv32CodeBytes:
    //   u32 max_rv32_code_bytes;
    kConfigParamMaxRv32CodeBytes { max_rv32_code_bytes: u32 } = ConfigParamID::kConfigParamMaxRv32CodeBytes.0 as i32,
    /// Bytes of guest data memory (`memory_bytes` in the artifact header) an
    /// RV32 artifact may declare to be admitted as an attached contract.
    /// `kDefaultMaxRv32MemoryBytes` (65536) is the value in force for a
    /// configuration that does not state one.
    ///
    /// Checked at attachment like `kConfigParamMaxRv32CodeBytes`. Every
    /// invocation provisions and seeds the declared memory outside gas, so a
    /// chain raising this accepts that per-call cost; the node rejects a
    /// genesis above what its VM provisions.
    // case kConfigParamMaxRv32MemoryBytes:
    //   u32 max_rv32_memory_bytes;
    kConfigParamMaxRv32MemoryBytes { max_rv32_memory_bytes: u32 } = ConfigParamID::kConfigParamMaxRv32MemoryBytes.0 as i32,
    /// Auth token ids one call may carry in a list: `Invoke`'s
    /// `owned_auth_token_ids_list` and `Return`'s `returned_auth_token_ids`,
    /// each read as a `[u32]` span.
    ///
    /// Checked against the span's length before the ids are read, so a list
    /// past the bound is refused without being copied and what a rejected call
    /// costs does not scale with the memory that held it. The ids in a list are
    /// distinct and owned by the frame naming them, so this bounds what one
    /// frame hands another in a single call rather than what a frame may hold.
    ///
    /// Neither wire type bounds its list, and no value was in force before this
    /// parameter: a list was limited only by the memory holding it. A
    /// configuration omitting this one is not continuing an older rule: the
    /// value in force is 64.
    // case kConfigParamMaxAuthTokens:
    //   u32 max_auth_tokens;
    kConfigParamMaxAuthTokens { max_auth_tokens: u32 } = ConfigParamID::kConfigParamMaxAuthTokens.0 as i32,
    /// Bytes of calldata one invocation may be given, whether a transaction
    /// supplies it in `Invocation::calldata` or a contract supplies it in
    /// `Invoke`'s `calldata` span.
    ///
    /// Checked before the bytes are copied, so an oversized call is refused
    /// rather than materialized. `Calldata` is `opaque<>`, so the wire type
    /// bounds neither form and this is what bounds them in execution.
    /// `kConfigParamMaxReturnBufferLen` bounds the other direction the same
    /// way.
    ///
    /// The executor holds a frame's calldata for the life of that frame and
    /// every open frame holds its own, so a chain must keep this within what
    /// `kConfigParamMaxCrossInvokeDepth` frames of it leave room for. No value
    /// was in force before this parameter; for a configuration that does not
    /// state one, the value in force is 65536.
    // case kConfigParamMaxCalldataLen:
    //   u32 max_calldata_len;
    kConfigParamMaxCalldataLen { max_calldata_len: u32 } = ConfigParamID::kConfigParamMaxCalldataLen.0 as i32,
    /// Bytes one invocation frame may return, whether to a calling frame or,
    /// from the root frame, into `TxEventLog::root_output`.
    ///
    /// Checked where the Return syscall writes a buffer, which is the only
    /// place one is built. A root frame's buffer is committed to by the events
    /// root, so what a chain states here it stores and serves for every
    /// transaction; together with the event bounds it is what keeps a
    /// `TxEventLog` inside the RPC transport message limit.
    ///
    /// `ReturnBuffer` bounded itself at 1024 bytes before this parameter, so a
    /// configuration omitting this one is continuing an older rule rather than
    /// lifting it: the value in force for a configuration that does not state
    /// one is 1024.
    // case kConfigParamMaxReturnBufferLen:
    //   u32 max_return_buffer_len;
    kConfigParamMaxReturnBufferLen { max_return_buffer_len: u32 } = ConfigParamID::kConfigParamMaxReturnBufferLen.0 as i32,
    /// Gas one transaction may budget: the most a root `Invocation::gas_limit`
    /// may state.
    ///
    /// Checked before any attached contract is verified or any body runs, so
    /// a transaction over the bound is refused outright rather than clamped:
    /// the budget a transaction signs is the one it executes under or none.
    /// Subinvocations are already capped at their caller's remaining gas, so
    /// bounding the root bounds every frame.
    ///
    /// Native-token availability bounded a transaction's budget before this
    /// parameter, and nothing else did. A configuration omitting this one does
    /// not continue that rule: the value in force is `1,000,000,000`. A
    /// `GasAmount` rather than an `unsigned int` so that a chain may state any
    /// budget a transaction can.
    // case kConfigParamMaxTxGas:
    //   GasAmount max_tx_gas;
    kConfigParamMaxTxGas { max_tx_gas: GasAmount } = ConfigParamID::kConfigParamMaxTxGas.0 as i32,
    
}

/// The parameters a chain states, as a list rather than fixed fields. A new
/// parameter is then an id and a [`ConfigParam`] arm, with no new version here
/// and no change to the layout of anything embedding one.
///
/// `params` MUST be a prefix of the id space: ids run 0, 1, 2, ... in order,
/// with no gaps. The encoding is hashed into `ProposalHeader::config_hash`, so
/// permitting one arrangement per configuration is what gives it one hash. A
/// list that is unordered, repeats an id, or skips one must fail validation
/// rather than resolve to one of the values. The rule also bounds the list: a
/// prefix cannot be longer than the number of parameters that exist.
///
/// Ascending order alone would leave interior gaps legal, and a gap is what
/// makes the encoding ambiguous: a parameter at its default value could be
/// written down or left out. A prefix settles which -- below the list's length
/// a parameter is stated, past it it is not -- and makes resolving one an
/// index rather than a search.
///
/// A parameter absent from the list takes the value the protocol defines for
/// it. Absence says nothing about why: the list may be short because the
/// configuration predates the parameter, or because its creator declined to
/// state one it knew, and nothing on the wire tells those apart. A creator
/// SHOULD state every parameter it knows; a verifier cannot check that.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct NetworkConfigV0 {
    pub params: VecM<ConfigParam>,
}

/// The consensus-critical execution parameters a block executes under.
///
/// A chain states its configuration at genesis. A parameter the
/// configuration does not state takes the value its [`ConfigParam`] arm
/// documents (see [`NetworkConfigV0`]).
///
/// One version, because `v0` states its parameters as a list: a new parameter
/// is an id and a [`ConfigParam`] arm, not a new arm here. The discriminant is
/// kept so that if a change ever does need one, it is a new case rather than a
/// silent reinterpretation of these bytes. A node meeting a version it cannot
/// name must fail to decode rather than guess what is in force.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[repr(i32)]
#[non_exhaustive]
pub enum NetworkConfig {
    // case 0:
    //   NetworkConfigV0 v0;
    _0 { v0: NetworkConfigV0 } = 0 as i32,
    
}

/// The network configuration committed to by ProposalHeader::config_hash.
/// New consensus-critical configuration belongs in this preimage.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ConfigHashPreimage {
    pub storage_cost_model: StorageCostModelXdr,
    pub network_config: NetworkConfig,
    /// The contract a root invocation of an undeployed address resolves to, as
    /// `GenesisConfiguration` states it and every `DbTip` carries it forward.
    ///
    /// It selects executing code, so a node running under a different one from
    /// its peers executes a different chain. Nothing else commits to it: a
    /// node's copy travels in the tip, which implementations are free to keep
    /// outside the state the storage hash covers, and some do. Committing to
    /// it here is what binds the value a node executes under to the one
    /// consensus agreed, rather than to whatever its own storage returns.
    pub default_wallet: DeployedContractHash,
}


//
// XDR internals below this point
//

impl ConfigParam {
    #[allow(clippy::match_single_binding)]
    pub fn param(&self) -> ConfigParamID {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl NetworkConfig {
    #[allow(clippy::match_single_binding)]
    pub fn v(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}


