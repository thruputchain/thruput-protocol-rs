use groundhog_spec::ContractType;

fn assert_roundtrip(
    value: ContractType,
    expected_txs: &[u8],
    expected_cxs: &[u8],
) {
    let txs = xdrs::txs::encode(&value).expect("txs encode");
    assert_eq!(txs, expected_txs);
    let decoded = xdrs::txs::decode::<ContractType>(&txs).expect("txs decode");
    assert_eq!(decoded, value);

    let cxs = xdrs::cxs::encode(&value).expect("cxs encode");
    assert_eq!(cxs, expected_cxs);
    let decoded = xdrs::cxs::decode::<ContractType>(&cxs).expect("cxs decode");
    assert_eq!(decoded, value);
}

#[test]
fn u256_contract_type_roundtrips() {
    assert_roundtrip(ContractType::U256, &[0, 0, 0, 18], &[18]);
}

#[test]
fn i256_contract_type_roundtrips() {
    assert_roundtrip(ContractType::I256, &[0, 0, 0, 19], &[19]);
}
