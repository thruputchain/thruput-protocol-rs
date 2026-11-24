//! `Copy` for the protocol newtypes that wrap a single `Copy` value.
//!
//! The generated sources derive `Clone` alone for these. Each is one
//! integer or one hash, so a struct that holds one can derive `Copy` too,
//! and an operand need not be borrowed or cloned.
//!
//! List every newtype over an integer or over [`SHA256Hash`] here. The
//! fixed-size byte-array newtypes, `SHA256Hash` among them, already get
//! `Copy` from the generator.

#[cfg(test)]
use crate::types::SHA256Hash;
use crate::types::{
    BlockHeight, ConfigHash, ConsensusValueHash, EventsHash, NetworkHash,
    ProposalHeaderHash, Protocol, StorageShortHash, TimePoint, TimeSpan,
    TxsHash,
};

// Over an integer.
impl Copy for BlockHeight {}
impl Copy for Protocol {}
impl Copy for TimePoint {}
impl Copy for TimeSpan {}

// Over `SHA256Hash`.
impl Copy for ConfigHash {}
impl Copy for ConsensusValueHash {}
impl Copy for EventsHash {}
impl Copy for NetworkHash {}
impl Copy for ProposalHeaderHash {}
impl Copy for StorageShortHash {}
impl Copy for TxsHash {}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_copy<T: Copy>() {}

    #[test]
    fn integer_newtypes_are_copy() {
        assert_copy::<BlockHeight>();
        assert_copy::<Protocol>();
        assert_copy::<TimePoint>();
        assert_copy::<TimeSpan>();
    }

    #[test]
    fn hash_newtypes_are_copy() {
        assert_copy::<SHA256Hash>();
        assert_copy::<ConfigHash>();
        assert_copy::<ConsensusValueHash>();
        assert_copy::<EventsHash>();
        assert_copy::<NetworkHash>();
        assert_copy::<ProposalHeaderHash>();
        assert_copy::<StorageShortHash>();
        assert_copy::<TxsHash>();
    }
}
