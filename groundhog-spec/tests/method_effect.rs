use groundhog_spec::{ContractMethodEffect, ContractSpecMethod, ContractType};

#[test]
fn method_effect_roundtrips_without_changing_the_signature() {
    for (effect, discriminant) in [
        (ContractMethodEffect::EffectUnknown, 0_i32),
        (ContractMethodEffect::EffectPure, 1),
        (ContractMethodEffect::EffectReadOnly, 2),
        (ContractMethodEffect::EffectStateChanging, 3),
    ] {
        let method = ContractSpecMethod {
            name: "get".try_into().unwrap(),
            args: vec![].try_into().unwrap(),
            return_type: ContractType::U32,
            effect,
        };
        let bytes = xdrs::txs::encode(&method).unwrap();
        assert_eq!(&bytes[bytes.len() - 4..], &discriminant.to_be_bytes());
        assert_eq!(xdrs::txs::decode::<ContractSpecMethod>(&bytes).unwrap(), method);
    }
}
