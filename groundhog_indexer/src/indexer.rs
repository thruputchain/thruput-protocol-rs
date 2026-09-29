// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 445ff7bb6fa8c182d33ef0d939ea8c869b7bf4ce

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
    pub tx_hash: TransactionHash,
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

/// Backend disposition of a block-explorer page query.  Distinguishes
/// "the query ran and matched nothing" (IndexerQueryOk with an empty
/// page) from "the indexer could not run the query at all", so a
/// broken or still-opening node is not rendered as an empty chain.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct IndexerQueryOutcome(pub i32);

impl IndexerQueryOutcome {
    pub const IndexerQueryOk: Self = Self(0);

    /// The indexer's database is not open (still starting up, or shut
    /// down).  Transient from the cluster's point of view: retry, or ask
    /// a healthy node.
    pub const IndexerQueryNotReady: Self = Self(1);

    /// Unrecoverable backend failure (corrupt record, storage error).
    /// Details are in the indexer's logs.
    pub const IndexerQueryInternalFailure: Self = Self(2);
}
impl From<i32> for IndexerQueryOutcome {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for IndexerQueryOutcome {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::IndexerQueryOk.0 => {
                write!(f, "IndexerQueryOutcome::IndexerQueryOk")
            },
            x if x == Self::IndexerQueryNotReady.0 => {
                write!(f, "IndexerQueryOutcome::IndexerQueryNotReady")
            },
            x if x == Self::IndexerQueryInternalFailure.0 => {
                write!(f, "IndexerQueryOutcome::IndexerQueryInternalFailure")
            },
            _ => {
                write!(f, "IndexerQueryOutcome({})", self.0)
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
pub enum EventPageResult {
    // case IndexerQueryOk:
    //   EventPage page;
    IndexerQueryOk { page: EventPage } = IndexerQueryOutcome::IndexerQueryOk.0 as i32,
    // default:
    //   void;
    #[xdr(default_variant)] Default { outcome: i32 },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum TransactionPageResult {
    // case IndexerQueryOk:
    //   TransactionPage page;
    IndexerQueryOk { page: TransactionPage } = IndexerQueryOutcome::IndexerQueryOk.0 as i32,
    // default:
    //   void;
    #[xdr(default_variant)] Default { outcome: i32 },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum TransactionRefPageResult {
    // case IndexerQueryOk:
    //   TransactionRefPage page;
    IndexerQueryOk { page: TransactionRefPage } = IndexerQueryOutcome::IndexerQueryOk.0 as i32,
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
    async fn indexer_block_explorer_intro(&self, arg0 : ProtocolCommit) -> Result<IndexerBlockExplorerIntroOutcome, ::rpc_api::ProgramError>;
    
    async fn get_transaction_info(&self, arg0 : TransactionHash) -> Result<TxInfoResult, ::rpc_api::ProgramError>;
        // The canonical consensus value for a retained, indexed height. Unknown,
    // future, and unavailable heights return BlockNotFound; backend failures
    // remain RPC errors.

    async fn get_block_by_height(&self, arg0 : BlockHeight) -> Result<BlockLookupResult, ::rpc_api::ProgramError>;
        // Paginated, filtered events query.  A candidate event matches
    // iff it matches at least one EventFilter in the request (OR
    // across filters, AND within each filter).  Supports the wallet
    // "transfers to OR from me" case in a single call.

    async fn get_events(&self, arg0 : EventQuery) -> Result<EventPageResult, ::rpc_api::ProgramError>;
        // Paginated transactions, whole-chain, in block order.

    async fn get_transactions(&self, arg0 : TransactionQuery) -> Result<TransactionPageResult, ::rpc_api::ProgramError>;
        // Paginated transaction refs whose root invoked address == the query
    // address (account transaction history).  Hydrate via get_transaction_info.

    async fn get_transactions_by_address(&self, arg0 : AddressTransactionQuery) -> Result<TransactionRefPageResult, ::rpc_api::ProgramError>;
        // Paginated transaction refs committed at exactly the requested height.
    // Hydrate via get_transaction_info.

    async fn get_transactions_by_block(&self, arg0 : BlockTransactionQuery) -> Result<TransactionRefPageResult, ::rpc_api::ProgramError>;
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
impl ::rpc_api::RpcHandler for IndexerBlockExplorer_IndexerBlockExplorer1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x4321aaaa, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_api::RpcError> {
        match proc {
            // indexer_block_explorer_intro
            0 => {
                let args: (ProtocolCommit, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.indexer_block_explorer_intro(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // get_transaction_info
            1 => {
                let args: (TransactionHash, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.get_transaction_info(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // get_block_by_height
            2 => {
                let args: (BlockHeight, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.get_block_by_height(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // get_events
            3 => {
                let args: (EventQuery, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.get_events(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // get_transactions
            4 => {
                let args: (TransactionQuery, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.get_transactions(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // get_transactions_by_address
            5 => {
                let args: (AddressTransactionQuery, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.get_transactions_by_address(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // get_transactions_by_block
            6 => {
                let args: (BlockTransactionQuery, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.get_transactions_by_block(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            _ => Err(::rpc_api::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct IndexerBlockExplorer_IndexerBlockExplorer1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>,
}

impl IndexerBlockExplorer_IndexerBlockExplorer1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x4321aaaa, 1)
    }
    pub async fn indexer_block_explorer_intro(&self, arg0 : &ProtocolCommit) -> Result<IndexerBlockExplorerIntroOutcome, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn get_transaction_info(&self, arg0 : &TransactionHash) -> Result<TxInfoResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 1, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn get_block_by_height(&self, arg0 : &BlockHeight) -> Result<BlockLookupResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 2, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn get_events(&self, arg0 : &EventQuery) -> Result<EventPageResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 3, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn get_transactions(&self, arg0 : &TransactionQuery) -> Result<TransactionPageResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 4, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn get_transactions_by_address(&self, arg0 : &AddressTransactionQuery) -> Result<TransactionRefPageResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 5, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn get_transactions_by_block(&self, arg0 : &BlockTransactionQuery) -> Result<TransactionRefPageResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 6, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::rpc_api::RpcSender>> for IndexerBlockExplorer_IndexerBlockExplorer1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
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

/// Canonical committed-block payload returned to the indexer. The durable
/// outbox record stores only `format_version` and `consensus_value`; the two
/// batches live in the block's content-addressed events file, named by
/// `consensus_value.proposal_header.events_hash`, and are reassembled on load.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct IndexerCommittedBlockPayload {
    pub format_version: u32,
    pub consensus_value: ConsensusValue,
    // Both batches are positionally paired; their order is not significant.
    // Each inner Transaction is bound by consensus_value.proposal_header.txs_hash,
    // while the corresponding event log and nondeterministic proposal data are
    // bound together by consensus_value.proposal_header.events_hash. Both roots
    // are recomputed whenever the payload is loaded from disk. Signature
    // wrappers are intentionally outside both consensus commitments and remain
    // trusted internal payload: any wrapper satisfying the transaction's auth
    // claim is equally valid, so which one a block carries is not observable to
    // anything that validates against the header.
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
}

/// Where the indexer stands, as its pull request states it.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct IndexerProgressState(pub i32);

impl IndexerProgressState {
    /// No origin adopted yet. The server answers with the oldest retained
    /// origin, or its first block; never held.
    pub const IndexerProgressUninitialized: Self = Self(0);

    /// Applied everything through `initialized`; asks what comes next. Answered
    /// at once: the next block, `EventIndexerPullCaughtUp` when there is none,
    /// or a conflict. The first pull of every connection is this, since the
    /// caught-up reply is the indexer's readiness signal.
    pub const IndexerProgressInitialized: Self = Self(1);

    /// Same cursor, and already answered caught up on this connection. Asks the
    /// server to hold the request until the next publication and answer with
    /// it, so a block is pulled the moment it is committed rather than when the
    /// indexer next asks. Request-only: an indexer never persists this state.
    pub const IndexerProgressCaughtUp: Self = Self(2);
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
            x if x == Self::IndexerProgressCaughtUp.0 => {
                write!(f, "IndexerProgressState::IndexerProgressCaughtUp")
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
    // case IndexerProgressCaughtUp:
    //   IndexerInitializedProgress initialized;
    IndexerProgressInitialized { initialized: IndexerInitializedProgress } = IndexerProgressState::IndexerProgressInitialized.0 as i32,
    IndexerProgressCaughtUp { initialized: IndexerInitializedProgress } = IndexerProgressState::IndexerProgressCaughtUp.0 as i32,
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

/// The indexer's durable progress is its replay cursor. A lost reply is safe:
/// the next request either repeats the same immutable block or advances from
/// progress already committed by the indexer.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct EventIndexerPullRequest {
    pub protocol: ProtocolCommit,
    pub progress: IndexerProgress,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct EventIndexerPulledBlock {
    pub origin: IndexerOutboxOrigin,
    pub block: IndexerCommittedBlock,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct EventIndexerPullOutcome(pub i32);

impl EventIndexerPullOutcome {
    pub const EventIndexerPullBlock: Self = Self(0);
    pub const EventIndexerPullCaughtUp: Self = Self(1);
    pub const EventIndexerPullNotReady: Self = Self(2);
    pub const EventIndexerPullMismatchedProtocol: Self = Self(3);
    pub const EventIndexerPullProgressConflict: Self = Self(4);
    pub const EventIndexerPullInternalFailure: Self = Self(5);
}
impl From<i32> for EventIndexerPullOutcome {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for EventIndexerPullOutcome {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::EventIndexerPullBlock.0 => {
                write!(f, "EventIndexerPullOutcome::EventIndexerPullBlock")
            },
            x if x == Self::EventIndexerPullCaughtUp.0 => {
                write!(f, "EventIndexerPullOutcome::EventIndexerPullCaughtUp")
            },
            x if x == Self::EventIndexerPullNotReady.0 => {
                write!(f, "EventIndexerPullOutcome::EventIndexerPullNotReady")
            },
            x if x == Self::EventIndexerPullMismatchedProtocol.0 => {
                write!(f, "EventIndexerPullOutcome::EventIndexerPullMismatchedProtocol")
            },
            x if x == Self::EventIndexerPullProgressConflict.0 => {
                write!(f, "EventIndexerPullOutcome::EventIndexerPullProgressConflict")
            },
            x if x == Self::EventIndexerPullInternalFailure.0 => {
                write!(f, "EventIndexerPullOutcome::EventIndexerPullInternalFailure")
            },
            _ => {
                write!(f, "EventIndexerPullOutcome({})", self.0)
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
pub enum EventIndexerPullResult {
    // case EventIndexerPullBlock:
    //   EventIndexerPulledBlock pulled_block;
    EventIndexerPullBlock { pulled_block: EventIndexerPulledBlock } = EventIndexerPullOutcome::EventIndexerPullBlock.0 as i32,
    // case EventIndexerPullCaughtUp:
    //   IndexerOutboxOrigin origin;
    EventIndexerPullCaughtUp { origin: IndexerOutboxOrigin } = EventIndexerPullOutcome::EventIndexerPullCaughtUp.0 as i32,
    // default:
    //   IndexerFailureMessage diagnostic;
    #[xdr(default_variant)] Default { outcome: i32, diagnostic: IndexerFailureMessage },
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct IndexerTxFailureNotice {
    pub tx_hash: TransactionHash,
    pub diagnostic: TxFailureDiagnostic,
}

pub type IndexerTxFailureNoticeBatch = VecM::<IndexerTxFailureNotice, 256>;


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait EventIndexer_EventIndexer1: Send + Sync {    
    async fn event_indexer_pull_next(&self, arg0 : EventIndexerPullRequest) -> Result<EventIndexerPullResult, ::rpc_api::ProgramError>;
    
    async fn event_indexer_pull_tx_failures(&self) -> Result<IndexerTxFailureNoticeBatch, ::rpc_api::ProgramError>;
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
impl ::rpc_api::RpcHandler for EventIndexer_EventIndexer1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x4321aaac, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_api::RpcError> {
        match proc {
            // event_indexer_pull_next
            10 => {
                let args: (EventIndexerPullRequest, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.event_indexer_pull_next(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // event_indexer_pull_tx_failures
            11 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.event_indexer_pull_tx_failures().await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            _ => Err(::rpc_api::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct EventIndexer_EventIndexer1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>,
}

impl EventIndexer_EventIndexer1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x4321aaac, 1)
    }
    pub async fn event_indexer_pull_next(&self, arg0 : &EventIndexerPullRequest) -> Result<EventIndexerPullResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 10, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn event_indexer_pull_tx_failures(&self) -> Result<IndexerTxFailureNoticeBatch, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&()).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 11, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::rpc_api::RpcSender>> for EventIndexer_EventIndexer1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
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

impl EventPageResult {
    pub fn outcome(&self) -> IndexerQueryOutcome {
        match self {
            EventPageResult::Default{ outcome } => (*outcome).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl TransactionPageResult {
    pub fn outcome(&self) -> IndexerQueryOutcome {
        match self {
            TransactionPageResult::Default{ outcome } => (*outcome).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl TransactionRefPageResult {
    pub fn outcome(&self) -> IndexerQueryOutcome {
        match self {
            TransactionRefPageResult::Default{ outcome } => (*outcome).into(),

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

impl EventIndexerPullResult {
    pub fn outcome(&self) -> EventIndexerPullOutcome {
        match self {
            EventIndexerPullResult::Default{ outcome,..} => (*outcome).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


