//! The protocol's chunked RPCs, as a Rust server receives them.
//!
//! The database publishes a committed block's balance deltas and auth-data
//! changes as runs of [`NativeTokensTransferredChunk`] and [`KeyChangeChunk`] records,
//! because a block can touch more accounts than one RPC record carries.
//! Every record of a run carries the same `chunk_count`, each `chunk_index`
//! in `[0, chunk_count)` is sent exactly once, and a run always has at least
//! one record, so an empty message is a run of one empty record. The records
//! of a run may be sent concurrently and over several connections, so they
//! can arrive in any order. The `KeyChangeChunk` runs of several heights can
//! interleave; the database sends one height's `NativeTokensTransferredChunk` run
//! only once the previous one is acknowledged.
//!
//! The [`ChunkRecord`] impls here take the two records apart into height,
//! [`ChunkPosition`] and items. A server implementing `SchedDb1`, `AuthDB1`
//! or `MempoolAuth1` (over which the auth server forwards `KeyChangeChunk`
//! runs to the mempool) hands every record it receives to a
//! `thruput_rpc_driver::ChunkRunAssembler`, which returns the height's whole message
//! once every chunk of its run is in, with the parts read as one sequence.
//! This crate depends on `thruput_rpc_api` alone, so the assembler is not
//! named here:
//!
//! ```
//! use thruput_rpc_driver::{ChunkRunAssembler, ProgramError};
//! use thruput_node_rpc::{Delta, NativeTokensTransferredChunk};
//! use thruput_xdr::types::BlockHeight;
//! use xdrs::VecM;
//!
//! struct Mempool {
//!     transfers: ChunkRunAssembler<BlockHeight, VecM<Delta>>,
//! }
//!
//! impl Mempool {
//!     fn scheddb_native_tokens_transferred_chunk(
//!         &self,
//!         chunk: NativeTokensTransferredChunk,
//!     ) -> Result<(), ProgramError> {
//!         // A chunk that breaks its run's framing is a sender bug; the
//!         // error converts into the ProgramError a handler returns.
//!         if let Some(run) = self.transfers.push_record(chunk)? {
//!             let block: &BlockHeight = &run.key;
//!             for delta in &run.parts {
//!                 let _: &Delta = delta;
//!                 // apply the block's delta ...
//!             }
//!         }
//!         Ok(())
//!     }
//! }
//! ```

use thruput_rpc_api::{ChunkPosition, ChunkRecord};
use thruput_xdr::types::BlockHeight;
use xdrs::VecM;

use crate::{
    Delta, KeyChangeBatch, KeyChangeChunk, NativeTokensTransferredChunk,
};

impl ChunkRecord for NativeTokensTransferredChunk {
    type Key = BlockHeight;
    type Part = VecM<Delta>;

    fn into_chunk(self) -> (BlockHeight, ChunkPosition, VecM<Delta>) {
        (
            self.block,
            ChunkPosition::new(self.chunk_index, self.chunk_count),
            self.deltas,
        )
    }
}

impl ChunkRecord for KeyChangeChunk {
    type Key = BlockHeight;
    type Part = KeyChangeBatch;

    fn into_chunk(self) -> (BlockHeight, ChunkPosition, KeyChangeBatch) {
        (
            self.height,
            ChunkPosition::new(self.chunk_index, self.chunk_count),
            self.changes,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::KeyChange;
    use thruput_xdr::types::{ContractAddress, int128_xdr};

    fn delta(lowbits: u64) -> Delta {
        Delta {
            address: ContractAddress::default(),
            native_token_delta: int128_xdr {
                highbits: 0,
                lowbits,
            },
        }
    }

    #[test]
    fn a_transfer_chunk_comes_apart_into_block_position_and_deltas() {
        // The record a handler receives is what the dispatcher decoded from
        // the wire, so go through the wire form.
        let chunk = NativeTokensTransferredChunk {
            block: BlockHeight::new(42),
            chunk_index: 1,
            chunk_count: 3,
            deltas: VecM(vec![delta(1), delta(2)]),
        };
        let bytes = xdrs::txs::encode(&chunk).expect("encodes");
        let decoded: NativeTokensTransferredChunk =
            xdrs::txs::decode(&bytes).expect("decodes");
        let (block, position, deltas) = decoded.into_chunk();
        assert_eq!(block, BlockHeight::new(42));
        assert_eq!(position, ChunkPosition::new(1, 3));
        assert_eq!(deltas, VecM(vec![delta(1), delta(2)]));
    }

    #[test]
    fn a_key_change_chunk_comes_apart_into_height_position_and_changes() {
        let change = KeyChange {
            address: ContractAddress::default(),
            auth_data: None,
        };
        let chunk = KeyChangeChunk {
            height: BlockHeight::new(5),
            chunk_index: 0,
            chunk_count: 1,
            changes: VecM(vec![change.clone()]),
        };
        let (height, position, changes) = chunk.into_chunk();
        assert_eq!(height, BlockHeight::new(5));
        assert_eq!(position, ChunkPosition::new(0, 1));
        assert_eq!(changes, VecM(vec![change]));
    }
}
