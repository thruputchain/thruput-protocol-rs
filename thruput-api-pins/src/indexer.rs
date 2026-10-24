//! Signature pins for `thruput_indexer` (generated: src/indexer.rs).

use thruput_indexer::{
    AddressTransactionQuery, BlockLookupResult, BlockTransactionQuery,
    EventIndexer_EventIndexer1, EventIndexer_EventIndexer1_Client,
    EventIndexerPullRequest, EventIndexerPullResult, EventPageResult,
    EventQuery, IndexerBlockExplorer_IndexerBlockExplorer1,
    IndexerBlockExplorer_IndexerBlockExplorer1_Client,
    IndexerBlockExplorerIntroOutcome, IndexerTxFailureNoticeBatch,
    TransactionPageResult, TransactionQuery, TransactionRefPageResult,
    TxInfoResult,
};
use thruput_rpc_api::{ProgramError, RpcError};
use thruput_xdr::network::ProtocolCommit;
use thruput_xdr::transaction::TransactionHash;
use thruput_xdr::types::BlockHeight;
use xdrs::EncodesAs;

pub mod indexer_block_explorer {
    use super::*;
    type S = IndexerBlockExplorer_IndexerBlockExplorer1_Client;

    pub async fn client_indexer_block_explorer_intro(
        c: &S,
        arg0: &(impl EncodesAs<ProtocolCommit> + ?Sized),
    ) -> Result<IndexerBlockExplorerIntroOutcome, RpcError> {
        c.indexer_block_explorer_intro(arg0).await
    }
    pub async fn client_get_transaction_info(
        c: &S,
        arg0: &(impl EncodesAs<TransactionHash> + ?Sized),
    ) -> Result<TxInfoResult, RpcError> {
        c.get_transaction_info(arg0).await
    }
    pub async fn client_get_block_by_height(
        c: &S,
        arg0: &(impl EncodesAs<BlockHeight> + ?Sized),
    ) -> Result<BlockLookupResult, RpcError> {
        c.get_block_by_height(arg0).await
    }
    pub async fn client_get_events(
        c: &S,
        arg0: &(impl EncodesAs<EventQuery> + ?Sized),
    ) -> Result<EventPageResult, RpcError> {
        c.get_events(arg0).await
    }
    pub async fn client_get_transactions(
        c: &S,
        arg0: &(impl EncodesAs<TransactionQuery> + ?Sized),
    ) -> Result<TransactionPageResult, RpcError> {
        c.get_transactions(arg0).await
    }
    pub async fn client_get_transactions_by_address(
        c: &S,
        arg0: &(impl EncodesAs<AddressTransactionQuery> + ?Sized),
    ) -> Result<TransactionRefPageResult, RpcError> {
        c.get_transactions_by_address(arg0).await
    }
    pub async fn client_get_transactions_by_block(
        c: &S,
        arg0: &(impl EncodesAs<BlockTransactionQuery> + ?Sized),
    ) -> Result<TransactionRefPageResult, RpcError> {
        c.get_transactions_by_block(arg0).await
    }

    pub async fn trait_indexer_block_explorer_intro<
        T: IndexerBlockExplorer_IndexerBlockExplorer1,
    >(
        t: &T,
        arg0: ProtocolCommit,
    ) -> Result<IndexerBlockExplorerIntroOutcome, ProgramError> {
        t.indexer_block_explorer_intro(arg0).await
    }
    pub async fn trait_get_transaction_info<
        T: IndexerBlockExplorer_IndexerBlockExplorer1,
    >(
        t: &T,
        arg0: TransactionHash,
    ) -> Result<TxInfoResult, ProgramError> {
        t.get_transaction_info(arg0).await
    }
    pub async fn trait_get_block_by_height<
        T: IndexerBlockExplorer_IndexerBlockExplorer1,
    >(
        t: &T,
        arg0: BlockHeight,
    ) -> Result<BlockLookupResult, ProgramError> {
        t.get_block_by_height(arg0).await
    }
    pub async fn trait_get_events<
        T: IndexerBlockExplorer_IndexerBlockExplorer1,
    >(
        t: &T,
        arg0: EventQuery,
    ) -> Result<EventPageResult, ProgramError> {
        t.get_events(arg0).await
    }
    pub async fn trait_get_transactions<
        T: IndexerBlockExplorer_IndexerBlockExplorer1,
    >(
        t: &T,
        arg0: TransactionQuery,
    ) -> Result<TransactionPageResult, ProgramError> {
        t.get_transactions(arg0).await
    }
    pub async fn trait_get_transactions_by_address<
        T: IndexerBlockExplorer_IndexerBlockExplorer1,
    >(
        t: &T,
        arg0: AddressTransactionQuery,
    ) -> Result<TransactionRefPageResult, ProgramError> {
        t.get_transactions_by_address(arg0).await
    }
    pub async fn trait_get_transactions_by_block<
        T: IndexerBlockExplorer_IndexerBlockExplorer1,
    >(
        t: &T,
        arg0: BlockTransactionQuery,
    ) -> Result<TransactionRefPageResult, ProgramError> {
        t.get_transactions_by_block(arg0).await
    }
}

pub mod event_indexer {
    use super::*;
    type S = EventIndexer_EventIndexer1_Client;

    pub async fn client_event_indexer_pull_next(
        c: &S,
        arg0: &(impl EncodesAs<EventIndexerPullRequest> + ?Sized),
    ) -> Result<EventIndexerPullResult, RpcError> {
        c.event_indexer_pull_next(arg0).await
    }
    pub async fn client_event_indexer_pull_tx_failures(
        c: &S,
    ) -> Result<IndexerTxFailureNoticeBatch, RpcError> {
        c.event_indexer_pull_tx_failures().await
    }

    pub async fn trait_event_indexer_pull_next<
        T: EventIndexer_EventIndexer1,
    >(
        t: &T,
        arg0: EventIndexerPullRequest,
    ) -> Result<EventIndexerPullResult, ProgramError> {
        t.event_indexer_pull_next(arg0).await
    }
    pub async fn trait_event_indexer_pull_tx_failures<
        T: EventIndexer_EventIndexer1,
    >(
        t: &T,
    ) -> Result<IndexerTxFailureNoticeBatch, ProgramError> {
        t.event_indexer_pull_tx_failures().await
    }
}
