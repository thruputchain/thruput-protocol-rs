//! Compile-time guard that generated types carry `#[derive(ConstXDRSize)]`.
//!
//! The XDR compiler emits `ConstXDRSize` alongside the other XDR derives; a
//! compiler that left it out would drop it silently. These assertions break
//! the build under the `xdrs` feature if that happens.

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
