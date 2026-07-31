// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 442c63216e98dc2fa820f1de01b10774de23a287

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
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct TxInfo {
    // The height of the block the tx was executed in.
    pub height: BlockHeight,
    // The full transaction body (signed tx + nondeterministic data), as
    // forwarded by the event server and stored in the indexer.
    pub tx: ProposedTransaction,
    pub events: TxEventLog,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct TxInfoOutcome(pub i32);

impl TxInfoOutcome {
    pub const TxInfoFound: Self = Self(0);
    pub const TxInfoNotFound: Self = Self(1);

    /// No committed record, but the transaction failed nondeterministically
    /// within the last few blocks and the indexer still holds the diagnostic.
    /// Best-effort and advisory: reports are dropped freely under load, only
    /// the proposing node's indexers hear them, and a later retry of the same
    /// tx may still commit (at which point TxInfoFound wins). Absence of this
    /// outcome proves nothing.
    pub const TxInfoRecentFailure: Self = Self(2);
}
impl From<i32> for TxInfoOutcome {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for TxInfoOutcome {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::TxInfoFound.0 => {
                write!(f, "TxInfoOutcome::TxInfoFound")
            },
            x if x == Self::TxInfoNotFound.0 => {
                write!(f, "TxInfoOutcome::TxInfoNotFound")
            },
            x if x == Self::TxInfoRecentFailure.0 => {
                write!(f, "TxInfoOutcome::TxInfoRecentFailure")
            },
            _ => {
                write!(f, "TxInfoOutcome({})", self.0)
            }
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum TxInfoResult {
    // case TxInfoFound:
    //   TxInfo info;
    TxInfoFound { info: TxInfo } = TxInfoOutcome::TxInfoFound.0 as i32,
    // case TxInfoRecentFailure:
    //   TxFailureDiagnostic recent_failure;
    TxInfoRecentFailure { recent_failure: TxFailureDiagnostic } = TxInfoOutcome::TxInfoRecentFailure.0 as i32,
    // default:
    //   void;
    #[xdr(default_variant)] Default { outcome: i32 },
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct BlockLookupOutcome(pub i32);

impl BlockLookupOutcome {
    pub const BlockFound: Self = Self(0);
    pub const BlockNotFound: Self = Self(1);
}
impl From<i32> for BlockLookupOutcome {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for BlockLookupOutcome {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::BlockFound.0 => {
                write!(f, "BlockLookupOutcome::BlockFound")
            },
            x if x == Self::BlockNotFound.0 => {
                write!(f, "BlockLookupOutcome::BlockNotFound")
            },
            _ => {
                write!(f, "BlockLookupOutcome({})", self.0)
            }
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum BlockLookupResult {
    // case BlockFound:
    //   ConsensusValue value;
    BlockFound { value: ConsensusValue } = BlockLookupOutcome::BlockFound.0 as i32,
    // default:
    //   void;
    #[xdr(default_variant)] Default { outcome: i32 },
}

pub type IdentifiedEventBatch = VecM::<IdentifiedEvent>;

pub type TxInfoBatch = VecM::<TxInfo>;

/// Max contracts per filter and max filters per query.  Matches Soroban
/// getEvents' limit of 5; designed to be low enough that the merge-walk
/// stays cheap even on degenerate inputs.
pub const kMaxContractAddressesPerEventFilter : u32 = 5;

pub const kMaxEventFiltersPerQuery : u32 = 5;

pub type ContractAddressList = VecM::<ContractAddress, kMaxContractAddressesPerEventFilter>;

/// Positional topic filter.  For each topic position (0, 1, 2):
///   - An empty list means "match anything at this position."
///   - A non-empty list means "match if the event's topic at this
///     position is in this list."  OR within a position, AND across
///     positions.
///
/// Partial match: a filter that specifies only topic0 matches events
/// with more topics.  Extra topics on the event past the filter's
/// specified positions are ignored.  (This is the ethereum getLogs /
/// stellar-rpc post-#358 semantics.)
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct EventTopicFilter {
    pub topic0: VecM<Topic>,
    pub topic1: VecM<Topic>,
    pub topic2: VecM<Topic>,
}

/// A single event-filter predicate: events must be emitted by one of
/// `contract_addresses` AND match the topic constraints.  Required
/// non-empty contract_addresses — the indexer has no topic-indexed
/// column family and can't efficiently serve "any contract" scans.
/// (When such an index is added, the gateway relaxes the non-empty
/// check; the XDR stays the same.)
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct EventFilter {
    pub contract_addresses: ContractAddressList,
    pub topics: EventTopicFilter,
}

pub type EventFilterList = VecM::<EventFilter, kMaxEventFiltersPerQuery>;

/// `get_events` request.  A candidate event matches the query iff it
/// matches at least one `EventFilter` in `filters` (OR across filters,
/// AND within each filter).
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct EventQuery {
    pub filters: EventFilterList,
    pub order: IterationOrder,
    pub anchor: BlockAnchor,
    /// Empty on the initial call.  Non-empty to continue where a
    /// previous response left off; `order` and `anchor` in the request
    /// are ignored when a cursor is supplied — the cursor carries them.
    pub cursor: PaginationCursor,
    /// Clamped server-side to [1, 200]; 0 means server default (50).
    pub limit: u32,
}

/// `get_events` response.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct EventPage {
    /// Matched events in the requested order.  May be empty even when
    /// `cursor` is non-empty: the filter was selective and the current
    /// scan window produced no matches, so the client should keep
    /// paginating.  Empty events + empty cursor = stream exhausted.
    pub events: IdentifiedEventBatch,
    /// Empty when the stream has been exhausted in the requested
    /// direction.  Non-empty: pass back in the next call's `cursor`
    /// argument.
    pub cursor: PaginationCursor,
    /// Highest block height the indexer has processed at response time.
    /// Clients compare to the heights on their last-seen events to
    /// decide whether they're caught up.
    pub latest_block: BlockHeight,
    /// Oldest block height the indexer still retains data for at
    /// response time.  Today always 0; once retention/pruning lands
    /// this is the lower bound below which queries return empty
    /// regardless of anchor.
    pub oldest_block: BlockHeight,
}

/// `get_transactions` request.  Whole-chain walk in `order` from `anchor`.
/// There is no address filter here — use `get_transactions_by_address` for
/// per-account (root invoked address) transaction history.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct TransactionQuery {
    pub order: IterationOrder,
    pub anchor: BlockAnchor,
    /// Empty on the initial call.  Non-empty to continue where a previous
    /// response left off; `order` and `anchor` in the request are ignored
    /// when a cursor is supplied — the cursor carries them.
    pub cursor: PaginationCursor,
    /// Clamped server-side to [1, 200]; 0 means server default (50).
    pub limit: u32,
}

/// `get_transactions` response.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct TransactionPage {
    /// Matched transactions in the requested order.  May be empty even
    /// when `cursor` is non-empty: the current scan window produced no
    /// matches, so the client should keep paginating.  Empty
    /// transactions + empty cursor = stream exhausted.
    pub transactions: TxInfoBatch,
    /// Empty when the stream has been exhausted in the requested
    /// direction.  Non-empty: pass back in the next call's `cursor`
    /// argument.
    pub cursor: PaginationCursor,
    /// Highest block height the indexer has processed at response time.
    /// Clients compare to the heights on their last-seen transactions to
    /// decide whether they're caught up.
    pub latest_block: BlockHeight,
    /// Oldest block height the indexer still retains data for at
    /// response time.  Today always 0; once retention/pruning lands
    /// this is the lower bound below which queries return empty
    /// regardless of anchor.
    pub oldest_block: BlockHeight,
}

/// Lightweight reference to a transaction: its hash plus the block it
/// executed in. Returned by transaction-ref queries; hydrate the full body
/// via `get_transaction_info`.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct TransactionRef {
    pub tx_hash: SignedTransactionHash,
    pub height: BlockHeight,
}

pub type TransactionRefBatch = VecM::<TransactionRef>;

/// `get_transactions_by_address` request.  Walked in `order` from `anchor`,
/// returning transactions whose ROOT invocation targets `address` (i.e. the
/// account's transaction history).
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct AddressTransactionQuery {
    pub address: ContractAddress,
    pub order: IterationOrder,
    pub anchor: BlockAnchor,
    /// Empty on the initial call.  Non-empty to continue; `order`/`anchor`
    /// are ignored when a cursor is supplied.
    pub cursor: PaginationCursor,
    /// Clamped server-side to [1, 200]; 0 means server default (50).
    pub limit: u32,
}

/// `get_transactions_by_block` request. Returns transactions committed at
/// exactly `height`, ordered lexicographically by their 32 unsigned hash bytes.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct BlockTransactionQuery {
    pub height: BlockHeight,
    pub order: IterationOrder,
    /// Empty on the initial call. Non-empty to continue at the same height;
    /// `order` is ignored when a cursor is supplied because the cursor carries it.
    /// Malformed, stale, or height-mismatched cursors produce an empty page with
    /// an empty cursor.
    pub cursor: PaginationCursor,
    /// Clamped server-side to [1, 200]; 0 means server default (50).
    pub limit: u32,
}

/// Transaction refs in the query's order. Every successful response reports
/// the indexer's current bounds, including empty responses.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct TransactionRefPage {
    pub refs: TransactionRefBatch,
    pub cursor: PaginationCursor,
    pub latest_block: BlockHeight,
    pub oldest_block: BlockHeight,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct IndexerBlockExplorerIntroOutcome(pub i32);

impl IndexerBlockExplorerIntroOutcome {
    pub const IntroSuccess: Self = Self(0);
    pub const IntroMismatchedProtocol: Self = Self(1);
}
impl From<i32> for IndexerBlockExplorerIntroOutcome {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for IndexerBlockExplorerIntroOutcome {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::IntroSuccess.0 => {
                write!(f, "IndexerBlockExplorerIntroOutcome::IntroSuccess")
            },
            x if x == Self::IntroMismatchedProtocol.0 => {
                write!(f, "IndexerBlockExplorerIntroOutcome::IntroMismatchedProtocol")
            },
            _ => {
                write!(f, "IndexerBlockExplorerIntroOutcome({})", self.0)
            }
        }
    }
}



#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait IndexerBlockExplorer_IndexerBlockExplorer1: Send + Sync {    
    async fn indexer_block_explorer_intro(&self, arg0 : ProtocolCommit) -> Result<IndexerBlockExplorerIntroOutcome, ::rpc_driver::ProgramError>;
    
    async fn get_transaction_info(&self, arg0 : SignedTransactionHash) -> Result<TxInfoResult, ::rpc_driver::ProgramError>;
        // The canonical consensus value for a retained, indexed height. Unknown,
    // future, and unavailable heights return BlockNotFound; backend failures
    // remain RPC errors.

    async fn get_block_by_height(&self, arg0 : BlockHeight) -> Result<BlockLookupResult, ::rpc_driver::ProgramError>;
        // Paginated, filtered events query.  A candidate event matches
    // iff it matches at least one EventFilter in the request (OR
    // across filters, AND within each filter).  Supports the wallet
    // "transfers to OR from me" case in a single call.

    async fn get_events(&self, arg0 : EventQuery) -> Result<EventPage, ::rpc_driver::ProgramError>;
        // Paginated transactions, whole-chain, in block order.

    async fn get_transactions(&self, arg0 : TransactionQuery) -> Result<TransactionPage, ::rpc_driver::ProgramError>;
        // Paginated transaction refs whose root invoked address == the query
    // address (account transaction history).  Hydrate via get_transaction_info.

    async fn get_transactions_by_address(&self, arg0 : AddressTransactionQuery) -> Result<TransactionRefPage, ::rpc_driver::ProgramError>;
        // Paginated transaction refs committed at exactly the requested height.
    // Hydrate via get_transaction_info.

    async fn get_transactions_by_block(&self, arg0 : BlockTransactionQuery) -> Result<TransactionRefPage, ::rpc_driver::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct IndexerBlockExplorer_IndexerBlockExplorer1_Dispatcher {
  inner: Box<dyn IndexerBlockExplorer_IndexerBlockExplorer1>,
}

impl IndexerBlockExplorer_IndexerBlockExplorer1_Dispatcher {
  pub fn new(inner: Box<dyn IndexerBlockExplorer_IndexerBlockExplorer1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_driver::handler::RpcHandler for IndexerBlockExplorer_IndexerBlockExplorer1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x4321aaaa, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_driver::RpcError> {
        match proc {
            // indexer_block_explorer_intro
            0 => {
                let args: (ProtocolCommit, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.indexer_block_explorer_intro(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // get_transaction_info
            1 => {
                let args: (SignedTransactionHash, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.get_transaction_info(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // get_block_by_height
            2 => {
                let args: (BlockHeight, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.get_block_by_height(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // get_events
            3 => {
                let args: (EventQuery, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.get_events(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // get_transactions
            4 => {
                let args: (TransactionQuery, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.get_transactions(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // get_transactions_by_address
            5 => {
                let args: (AddressTransactionQuery, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.get_transactions_by_address(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // get_transactions_by_block
            6 => {
                let args: (BlockTransactionQuery, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.get_transactions_by_block(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            _ => Err(::rpc_driver::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct IndexerBlockExplorer_IndexerBlockExplorer1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>,
}

impl IndexerBlockExplorer_IndexerBlockExplorer1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x4321aaaa, 1)
    }
    pub async fn indexer_block_explorer_intro(&self, arg0 : &ProtocolCommit) -> Result<IndexerBlockExplorerIntroOutcome, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn get_transaction_info(&self, arg0 : &SignedTransactionHash) -> Result<TxInfoResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 1, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn get_block_by_height(&self, arg0 : &BlockHeight) -> Result<BlockLookupResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 2, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn get_events(&self, arg0 : &EventQuery) -> Result<EventPage, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 3, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn get_transactions(&self, arg0 : &TransactionQuery) -> Result<TransactionPage, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 4, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn get_transactions_by_address(&self, arg0 : &AddressTransactionQuery) -> Result<TransactionRefPage, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 5, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn get_transactions_by_block(&self, arg0 : &BlockTransactionQuery) -> Result<TransactionRefPage, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 6, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::rpc_driver::RpcSender>> for IndexerBlockExplorer_IndexerBlockExplorer1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>) -> Self {
        Self::new(handle)
    }
}


pub type TxEventLogBatch = VecM::<TxEventLog>;

/// Immutable identity of one protocol epoch in the event service's durable
/// indexer outbox. The first committed block in the epoch is `first_height`
/// and links to `first_prev_hash`.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct IndexerOutboxOrigin {
    pub protocol: ProtocolCommit,
    pub epoch_sequence: u64,
    pub first_height: BlockHeight,
    pub first_prev_hash: ConsensusValueHash,
}

/// Canonical durable outbox payload. This exact XDR value is hashed for
/// `payload_digest`, stored in the private outbox record, and delivered to the
/// indexer so both sides use one content identity.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct IndexerCommittedBlockPayload {
    pub format_version: u32,
    pub consensus_value: ConsensusValue,
    pub proposal_txs: ProposalTxs,
    pub event_logs: TxEventLogBatch,
    pub tx_bodies: ProposedTransactionBatch,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct IndexerCommittedBlockIdentity {
    pub consensus_hash: ConsensusValueHash,
    pub payload_digest: SHA256Hash,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct IndexerCommittedBlock {
    pub payload: IndexerCommittedBlockPayload,
    pub identity: IndexerCommittedBlockIdentity,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct IndexerInitializedProgress {
    pub origin: IndexerOutboxOrigin,
    pub contiguous_height: BlockHeight,
    pub last_consensus_hash: ConsensusValueHash,
    pub last_payload_digest: SHA256Hash,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct IndexerProgressState(pub i32);

impl IndexerProgressState {
    pub const IndexerProgressUninitialized: Self = Self(0);
    pub const IndexerProgressInitialized: Self = Self(1);
}
impl From<i32> for IndexerProgressState {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for IndexerProgressState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::IndexerProgressUninitialized.0 => {
                write!(f, "IndexerProgressState::IndexerProgressUninitialized")
            },
            x if x == Self::IndexerProgressInitialized.0 => {
                write!(f, "IndexerProgressState::IndexerProgressInitialized")
            },
            _ => {
                write!(f, "IndexerProgressState({})", self.0)
            }
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum IndexerProgress {
    // case IndexerProgressInitialized:
    //   IndexerInitializedProgress initialized;
    IndexerProgressInitialized { initialized: IndexerInitializedProgress } = IndexerProgressState::IndexerProgressInitialized.0 as i32,
    // default:
    //   void;
    #[xdr(default_variant)] Default { state: i32 },
}

pub type IndexerFailureMessage = StringM::<1024>;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct IndexerEpochOutcome(pub i32);

impl IndexerEpochOutcome {
    pub const IndexerEpochInitialized: Self = Self(0);
    pub const IndexerEpochUnchanged: Self = Self(1);
    pub const IndexerEpochConflict: Self = Self(2);
    pub const IndexerEpochInternalFailure: Self = Self(3);
}
impl From<i32> for IndexerEpochOutcome {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for IndexerEpochOutcome {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::IndexerEpochInitialized.0 => {
                write!(f, "IndexerEpochOutcome::IndexerEpochInitialized")
            },
            x if x == Self::IndexerEpochUnchanged.0 => {
                write!(f, "IndexerEpochOutcome::IndexerEpochUnchanged")
            },
            x if x == Self::IndexerEpochConflict.0 => {
                write!(f, "IndexerEpochOutcome::IndexerEpochConflict")
            },
            x if x == Self::IndexerEpochInternalFailure.0 => {
                write!(f, "IndexerEpochOutcome::IndexerEpochInternalFailure")
            },
            _ => {
                write!(f, "IndexerEpochOutcome({})", self.0)
            }
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum IndexerEpochResult {
    // case IndexerEpochInitialized:
    // case IndexerEpochUnchanged:
    //   IndexerInitializedProgress progress;
    IndexerEpochInitialized { progress: IndexerInitializedProgress } = IndexerEpochOutcome::IndexerEpochInitialized.0 as i32,
    IndexerEpochUnchanged { progress: IndexerInitializedProgress } = IndexerEpochOutcome::IndexerEpochUnchanged.0 as i32,
    // default:
    //   IndexerFailureMessage diagnostic;
    #[xdr(default_variant)] Default { outcome: i32, diagnostic: IndexerFailureMessage },
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct IndexerHeightGap {
    pub expected_height: BlockHeight,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct IndexerBlockApplyOutcome(pub i32);

impl IndexerBlockApplyOutcome {
    pub const IndexerBlockApplied: Self = Self(0);
    pub const IndexerBlockDuplicate: Self = Self(1);
    pub const IndexerBlockGap: Self = Self(2);
    pub const IndexerBlockOriginMismatch: Self = Self(3);
    pub const IndexerBlockContentConflict: Self = Self(4);
    pub const IndexerBlockInternalFailure: Self = Self(5);
}
impl From<i32> for IndexerBlockApplyOutcome {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for IndexerBlockApplyOutcome {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::IndexerBlockApplied.0 => {
                write!(f, "IndexerBlockApplyOutcome::IndexerBlockApplied")
            },
            x if x == Self::IndexerBlockDuplicate.0 => {
                write!(f, "IndexerBlockApplyOutcome::IndexerBlockDuplicate")
            },
            x if x == Self::IndexerBlockGap.0 => {
                write!(f, "IndexerBlockApplyOutcome::IndexerBlockGap")
            },
            x if x == Self::IndexerBlockOriginMismatch.0 => {
                write!(f, "IndexerBlockApplyOutcome::IndexerBlockOriginMismatch")
            },
            x if x == Self::IndexerBlockContentConflict.0 => {
                write!(f, "IndexerBlockApplyOutcome::IndexerBlockContentConflict")
            },
            x if x == Self::IndexerBlockInternalFailure.0 => {
                write!(f, "IndexerBlockApplyOutcome::IndexerBlockInternalFailure")
            },
            _ => {
                write!(f, "IndexerBlockApplyOutcome({})", self.0)
            }
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum IndexerBlockApplyResult {
    // case IndexerBlockApplied:
    // case IndexerBlockDuplicate:
    //   IndexerInitializedProgress progress;
    IndexerBlockApplied { progress: IndexerInitializedProgress } = IndexerBlockApplyOutcome::IndexerBlockApplied.0 as i32,
    IndexerBlockDuplicate { progress: IndexerInitializedProgress } = IndexerBlockApplyOutcome::IndexerBlockDuplicate.0 as i32,
    // case IndexerBlockGap:
    //   IndexerHeightGap gap;
    IndexerBlockGap { gap: IndexerHeightGap } = IndexerBlockApplyOutcome::IndexerBlockGap.0 as i32,
    // default:
    //   IndexerFailureMessage diagnostic;
    #[xdr(default_variant)] Default { outcome: i32, diagnostic: IndexerFailureMessage },
}


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait IndexerEvent_IndexerEvent1: Send + Sync {    
    async fn indexer_event_get_progress(&self) -> Result<IndexerProgress, ::rpc_driver::ProgramError>;
    
    async fn indexer_event_initialize_epoch(&self, arg0 : IndexerOutboxOrigin) -> Result<IndexerEpochResult, ::rpc_driver::ProgramError>;
    
    async fn indexer_event_apply_committed_block(&self, arg0 : IndexerOutboxOrigin, arg1 : IndexerCommittedBlock) -> Result<IndexerBlockApplyResult, ::rpc_driver::ProgramError>;
        // Diagnostics remain best-effort and are independent of durable block
    // acknowledgement.

    async fn indexer_event_report_tx_failure(&self, arg0 : SignedTransactionHash, arg1 : TxFailureDiagnostic) -> Result<(), ::rpc_driver::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct IndexerEvent_IndexerEvent1_Dispatcher {
  inner: Box<dyn IndexerEvent_IndexerEvent1>,
}

impl IndexerEvent_IndexerEvent1_Dispatcher {
  pub fn new(inner: Box<dyn IndexerEvent_IndexerEvent1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_driver::handler::RpcHandler for IndexerEvent_IndexerEvent1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x4321aaab, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_driver::RpcError> {
        match proc {
            // indexer_event_get_progress
            10 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.indexer_event_get_progress().await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // indexer_event_initialize_epoch
            11 => {
                let args: (IndexerOutboxOrigin, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.indexer_event_initialize_epoch(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // indexer_event_apply_committed_block
            12 => {
                let args: (IndexerOutboxOrigin, IndexerCommittedBlock, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.indexer_event_apply_committed_block(args.0, args.1).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // indexer_event_report_tx_failure
            13 => {
                let args: (SignedTransactionHash, TxFailureDiagnostic, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.indexer_event_report_tx_failure(args.0, args.1).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            _ => Err(::rpc_driver::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct IndexerEvent_IndexerEvent1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>,
}

impl IndexerEvent_IndexerEvent1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x4321aaab, 1)
    }
    pub async fn indexer_event_get_progress(&self) -> Result<IndexerProgress, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&()).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 10, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn indexer_event_initialize_epoch(&self, arg0 : &IndexerOutboxOrigin) -> Result<IndexerEpochResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 11, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn indexer_event_apply_committed_block(&self, arg0 : &IndexerOutboxOrigin, arg1 : &IndexerCommittedBlock) -> Result<IndexerBlockApplyResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0, &arg1)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 12, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn indexer_event_report_tx_failure(&self, arg0 : &SignedTransactionHash, arg1 : &TxFailureDiagnostic) -> Result<(), ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0, &arg1)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 13, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::rpc_driver::RpcSender>> for IndexerEvent_IndexerEvent1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>) -> Self {
        Self::new(handle)
    }
}


/// Registration is bound to one indexer process generation and the durable
/// progress observed immediately before registration.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct IndexerCard {
    pub protocol: ProtocolCommit,
    pub generation: UPID,
    // Absent for the first durable indexer session. A new session may replace
    // only the event slot named by this predecessor. Exact-generation retries
    // remain heartbeats, including retries after an accepted response was lost.
    pub predecessor_generation: Option<UPID>,
    pub address_for_event: ServerAddress,
    pub progress: IndexerProgress,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct EventIndexerRegistrationOutcome(pub i32);

impl EventIndexerRegistrationOutcome {
    pub const EventRegistrationAccepted: Self = Self(0);
    pub const EventRegistrationMismatchedProtocol: Self = Self(1);
    pub const EventRegistrationConnectFailed: Self = Self(2);
    pub const EventRegistrationOriginUnavailable: Self = Self(3);
    pub const EventRegistrationProgressConflict: Self = Self(4);
    pub const EventRegistrationReplayFailed: Self = Self(5);

    // This process generation previously owned the slot but was superseded by a
    // later registration. It cannot reclaim ownership by retrying.
    pub const EventRegistrationStaleGeneration: Self = Self(6);
}
impl From<i32> for EventIndexerRegistrationOutcome {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for EventIndexerRegistrationOutcome {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::EventRegistrationAccepted.0 => {
                write!(f, "EventIndexerRegistrationOutcome::EventRegistrationAccepted")
            },
            x if x == Self::EventRegistrationMismatchedProtocol.0 => {
                write!(f, "EventIndexerRegistrationOutcome::EventRegistrationMismatchedProtocol")
            },
            x if x == Self::EventRegistrationConnectFailed.0 => {
                write!(f, "EventIndexerRegistrationOutcome::EventRegistrationConnectFailed")
            },
            x if x == Self::EventRegistrationOriginUnavailable.0 => {
                write!(f, "EventIndexerRegistrationOutcome::EventRegistrationOriginUnavailable")
            },
            x if x == Self::EventRegistrationProgressConflict.0 => {
                write!(f, "EventIndexerRegistrationOutcome::EventRegistrationProgressConflict")
            },
            x if x == Self::EventRegistrationReplayFailed.0 => {
                write!(f, "EventIndexerRegistrationOutcome::EventRegistrationReplayFailed")
            },
            x if x == Self::EventRegistrationStaleGeneration.0 => {
                write!(f, "EventIndexerRegistrationOutcome::EventRegistrationStaleGeneration")
            },
            _ => {
                write!(f, "EventIndexerRegistrationOutcome({})", self.0)
            }
        }
    }
}



#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait EventIndexer_EventIndexer1: Send + Sync {    
    async fn event_indexer_register(&self, arg0 : IndexerCard) -> Result<EventIndexerRegistrationOutcome, ::rpc_driver::ProgramError>;
    
    async fn event_indexer_probe(&self, arg0 : UPID) -> Result<EventIndexerRegistrationOutcome, ::rpc_driver::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct EventIndexer_EventIndexer1_Dispatcher {
  inner: Box<dyn EventIndexer_EventIndexer1>,
}

impl EventIndexer_EventIndexer1_Dispatcher {
  pub fn new(inner: Box<dyn EventIndexer_EventIndexer1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_driver::handler::RpcHandler for EventIndexer_EventIndexer1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x4321aaac, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_driver::RpcError> {
        match proc {
            // event_indexer_register
            10 => {
                let args: (IndexerCard, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.event_indexer_register(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // event_indexer_probe
            11 => {
                let args: (UPID, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.event_indexer_probe(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            _ => Err(::rpc_driver::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct EventIndexer_EventIndexer1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>,
}

impl EventIndexer_EventIndexer1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x4321aaac, 1)
    }
    pub async fn event_indexer_register(&self, arg0 : &IndexerCard) -> Result<EventIndexerRegistrationOutcome, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 10, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn event_indexer_probe(&self, arg0 : &UPID) -> Result<EventIndexerRegistrationOutcome, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 11, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::rpc_driver::RpcSender>> for EventIndexer_EventIndexer1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>) -> Self {
        Self::new(handle)
    }
}



//
// XDR internals below this point
//

impl TxInfoResult {
    pub fn outcome(&self) -> TxInfoOutcome {
        match self {
            TxInfoResult::Default{ outcome } => (*outcome).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl BlockLookupResult {
    pub fn outcome(&self) -> BlockLookupOutcome {
        match self {
            BlockLookupResult::Default{ outcome } => (*outcome).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl IndexerProgress {
    pub fn state(&self) -> IndexerProgressState {
        match self {
            IndexerProgress::Default{ state } => (*state).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl IndexerEpochResult {
    pub fn outcome(&self) -> IndexerEpochOutcome {
        match self {
            IndexerEpochResult::Default{ outcome,..} => (*outcome).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl IndexerBlockApplyResult {
    pub fn outcome(&self) -> IndexerBlockApplyOutcome {
        match self {
            IndexerBlockApplyResult::Default{ outcome,..} => (*outcome).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


