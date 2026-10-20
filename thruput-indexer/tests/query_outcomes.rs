//! Page failures carry only their discriminant, including unknown future tags.
//! These codec checks do not relax the RPC introduction's protocol identity gate.

use thruput_indexer::{
    EventPageResult, IndexerQueryOutcome, TransactionPageResult,
    TransactionRefPageResult,
};

fn check_failure<T>(value: &T, outcome: i32)
where
    T: xdrs::WriteXDR
        + for<'r> xdrs::ReadXDR<'r>
        + PartialEq
        + std::fmt::Debug,
{
    let txs = xdrs::txs::encode(value).unwrap();
    assert_eq!(txs, outcome.to_be_bytes());
    assert_eq!(&xdrs::txs::decode::<T>(&txs).unwrap(), value);

    let cxs = xdrs::cxs::encode(value).unwrap();
    assert_eq!(cxs, xdrs::cxs::encode(&outcome).unwrap());
    assert_eq!(&xdrs::cxs::decode::<T>(&cxs).unwrap(), value);

    // Failures have no page payload. Truncation and trailing data are invalid.
    for len in 0..txs.len() {
        assert!(xdrs::txs::decode::<T>(&txs[..len]).is_err());
    }
    let mut trailing = txs;
    trailing.extend_from_slice(&0_u32.to_be_bytes());
    assert!(xdrs::txs::decode::<T>(&trailing).is_err());
}

#[test]
fn page_failures_preserve_old_new_and_unknown_outcomes() {
    assert_eq!(IndexerQueryOutcome::IndexerQueryOk.0, 0);
    assert_eq!(IndexerQueryOutcome::IndexerQueryNotReady.0, 1);
    assert_eq!(IndexerQueryOutcome::IndexerQueryInternalFailure.0, 2);
    assert_eq!(IndexerQueryOutcome::IndexerQueryInvalidCursor.0, 3);
    assert_eq!(IndexerQueryOutcome::IndexerQueryHistoryUnavailable.0, 4);

    for outcome in [1, 2, 3, 4, 99] {
        check_failure(&EventPageResult::Default { outcome }, outcome);
        check_failure(&TransactionPageResult::Default { outcome }, outcome);
        check_failure(&TransactionRefPageResult::Default { outcome }, outcome);
    }
}

#[test]
fn empty_success_is_distinct_from_cursor_failure() {
    let success = EventPageResult::IndexerQueryOk {
        page: thruput_indexer::EventPage {
            events: Default::default(),
            cursor: Default::default(),
            latest_block: 0.into(),
            oldest_block: 0.into(),
        },
    };
    let bytes = xdrs::txs::encode(&success).unwrap();
    assert_eq!(&bytes[..4], &0_i32.to_be_bytes());
    assert!(bytes.len() > 4);
    assert_eq!(
        xdrs::txs::decode::<EventPageResult>(&bytes).unwrap(),
        success
    );
    assert!(
        xdrs::txs::decode::<EventPageResult>(&0_i32.to_be_bytes()).is_err()
    );
}
