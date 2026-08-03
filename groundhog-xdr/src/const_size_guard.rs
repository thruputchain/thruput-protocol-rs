//! Compile-time guard that generated types carry `#[derive(ConstXDRSize)]`.
//!
//! x23c emits `ConstXDRSize` alongside the other XDR derives; regenerating
//! with an older x23c drops it silently. These assertions break the build
//! under the `xdrs` feature if that happens.

const _: () = {
    const fn assert_const_xdr_size<
        T: xdrs::cxs::ConstMaxSize + xdrs::txs::ConstMaxSize,
    >() {
    }
    assert_const_xdr_size::<crate::types::uint128_xdr>();
    assert_const_xdr_size::<crate::transaction::Transaction>();
    assert_const_xdr_size::<crate::header::ProposalHeader>();
    assert_const_xdr_size::<crate::event::TxEmittedEvent>();
};
