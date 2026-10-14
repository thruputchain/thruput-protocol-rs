//! Encodings checked against the cross-language pubkey-account vectors in
//! `protocol/test_vectors/pubkey_auth.json` (see the README beside it).
//!
//! This crate has no hashing, so the checks cover what it produces: the
//! XDR and CXS bytes of keys, account preimages, domain separators and
//! transactions. Hashes and addresses are taken from the file as given.
//! Every transaction in the file, the negative ones included, is
//! well-formed XDR, so each must decode and re-encode byte for byte.

use serde_json::Value;
use thruput_xdr::auth_claim::{AuthClaim, PubkeyAccountPreimage};
use thruput_xdr::preimage::DomainSeparator;
use thruput_xdr::transaction::{SignedTransaction, Transaction};
use thruput_xdr::types::{
    ContractAddress, Ed25519PublicKey, ExtensionPoint, MlDsa65PublicKey,
    PublicKey, PublicKeyHash, Signature,
};

fn vectors() -> Value {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../protocol/test_vectors/pubkey_auth.json"
    );
    let text = std::fs::read_to_string(path).expect("read vectors");
    let v: Value = serde_json::from_str(&text).expect("parse vectors");
    assert_eq!(v["format_version"], 1, "unknown vectors layout");
    v
}

fn s(v: &Value) -> &str {
    v.as_str().expect("string field")
}

fn unhex(v: &Value) -> Vec<u8> {
    let h = s(v);
    assert!(h.len().is_multiple_of(2), "odd hex length");
    (0..h.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&h[i..i + 2], 16).expect("hex"))
        .collect()
}

fn unhex_array<const N: usize>(v: &Value) -> [u8; N] {
    unhex(v).try_into().expect("hex length")
}

fn public_key(key: &Value) -> PublicKey {
    match s(&key["scheme"]) {
        "Ed25519" => PublicKey::Ed25519 {
            ed25519: Ed25519PublicKey::new(unhex_array(&key["public_key"])),
        },
        "MlDsa65" => PublicKey::MlDsa65 {
            mldsa65: MlDsa65PublicKey::new(unhex_array(&key["public_key"])),
        },
        other => panic!("unknown scheme {other}"),
    }
}

/// Decodes a `SignedTransaction`, checks it re-encodes to the same bytes,
/// and checks its `Transaction` part is a prefix of them.
fn decode_signed(xdr: &Value) -> SignedTransaction {
    let bytes = unhex(xdr);
    let stx: SignedTransaction =
        xdrs::txs::decode(&bytes).expect("decode SignedTransaction");
    assert_eq!(xdrs::txs::encode(&stx).unwrap(), bytes, "re-encode");
    let tx_bytes = xdrs::txs::encode(&stx.tx).unwrap();
    assert_eq!(&bytes[..tx_bytes.len()], tx_bytes.as_slice());
    stx
}

#[test]
fn domain_separators() {
    let v = vectors();
    let seps = v["domain_separators"].as_object().unwrap();
    assert!(!seps.is_empty());
    for sep in seps.values() {
        let ds = DomainSeparator {
            ext: ExtensionPoint::_0,
            domain_separator: s(&sep["name"]).as_bytes().try_into().unwrap(),
        };
        assert_eq!(
            xdrs::cxs::encode(&ds).unwrap(),
            unhex(&sep["cxs"]),
            "{}",
            sep["name"]
        );
    }
}

#[test]
fn keys_and_account_preimages() {
    let v = vectors();
    let keys = v["keys"].as_array().unwrap();
    assert!(keys.len() >= 4);
    for key in keys {
        let name = s(&key["name"]);
        let pk = public_key(key);
        let scheme = key["scheme_value"].as_i64().unwrap();
        assert_eq!(scheme, if s(&key["scheme"]) == "Ed25519" { 0 } else { 1 });

        let xdr = unhex(&key["public_key_xdr"]);
        assert_eq!(xdrs::txs::encode(&pk).unwrap(), xdr, "{name} xdr");
        assert_eq!(xdrs::txs::decode::<PublicKey>(&xdr).unwrap(), pk);
        let cxs = unhex(&key["public_key_cxs"]);
        assert_eq!(xdrs::cxs::encode(&pk).unwrap(), cxs, "{name} cxs");
        assert_eq!(xdrs::cxs::decode::<PublicKey>(&cxs).unwrap(), pk);

        let hash = PublicKeyHash::new(unhex_array(&key["public_key_hash"]));
        for account in key["accounts"].as_array().unwrap() {
            let preimage = PubkeyAccountPreimage {
                pubkey_hash: hash,
                nonce: s(&account["nonce"]).parse().unwrap(),
            };
            let cxs = unhex(&account["pubkey_account_preimage_cxs"]);
            assert_eq!(
                xdrs::cxs::encode(&preimage).unwrap(),
                cxs,
                "{name} preimage, nonce {}",
                account["nonce"]
            );
            assert_eq!(
                xdrs::cxs::decode::<PubkeyAccountPreimage>(&cxs).unwrap(),
                preimage
            );
        }
    }
}

#[test]
fn signing() {
    let v = vectors();
    let keys = v["keys"].as_array().unwrap();
    let ds_tx = unhex(&v["domain_separators"]["Transaction"]["cxs"]);
    let signing = v["signing"].as_array().unwrap();
    assert!(signing.len() >= 4);
    for vec in signing {
        let name = s(&vec["name"]);
        let key = keys.iter().find(|k| k["name"] == vec["key"]).unwrap();
        let fields = &vec["transaction"];

        let tx_xdr = unhex(&vec["transaction_xdr"]);
        let tx: Transaction = xdrs::txs::decode(&tx_xdr).expect(name);
        assert_eq!(xdrs::txs::encode(&tx).unwrap(), tx_xdr, "{name}");
        assert_eq!(
            tx.invocation.invoked_address,
            ContractAddress::new(unhex_array(&fields["invoked_address"]))
        );
        assert_eq!(
            tx.auth_claim,
            AuthClaim::kPubkeySignature {
                pubkey_preimage: PubkeyAccountPreimage {
                    pubkey_hash: PublicKeyHash::new(unhex_array(
                        &fields["pubkey_hash"]
                    )),
                    nonce: s(&fields["account_nonce"]).parse().unwrap(),
                },
            }
        );

        // The signed bytes: domain separator, network hash, xdr(Transaction).
        let mut preimage = ds_tx.clone();
        preimage.extend(unhex(&vec["network_hash"]));
        preimage.extend(&tx_xdr);
        assert_eq!(preimage, unhex(&vec["signing_preimage"]), "{name}");

        let stx = decode_signed(&vec["signed_transaction_xdr"]);
        assert_eq!(stx.tx, tx);
        assert_eq!(stx.pubkey_claim_key, Some(public_key(key)), "{name}");
        assert_eq!(stx.signatures.len(), 1);
        assert_eq!(stx.signatures[0].public_key_id, 0);
        let sig = unhex(&vec["signature"]);
        match &stx.signatures[0].signature {
            Signature::Ed25519 { ed25519 } => {
                assert_eq!(s(&key["scheme"]), "Ed25519");
                assert_eq!(ed25519.as_ref(), sig.as_slice());
            }
            Signature::MlDsa65 { mldsa65 } => {
                assert_eq!(s(&key["scheme"]), "MlDsa65");
                assert_eq!(mldsa65.as_ref(), sig.as_slice());
            }
            other => panic!("{name}: unexpected signature {other:?}"),
        }
    }
}

/// Re-signing replaces the signature but leaves the signed `Transaction`,
/// and so its hash, unchanged.
#[test]
fn resigning() {
    let v = vectors();
    let signing = v["signing"].as_array().unwrap();
    let resigning = v["resigning"].as_array().unwrap();
    assert!(!resigning.is_empty());
    for vec in resigning {
        let base = signing.iter().find(|b| b["name"] == vec["of"]).unwrap();
        let stx = decode_signed(&vec["signed_transaction_xdr"]);
        let base_stx = decode_signed(&base["signed_transaction_xdr"]);
        assert_eq!(stx.tx, base_stx.tx);
        assert_eq!(stx.pubkey_claim_key, base_stx.pubkey_claim_key);
        assert_ne!(stx.signatures, base_stx.signatures);
        assert_eq!(vec["transaction_hash"], base["transaction_hash"]);
    }
}

/// The negatives break auth rules, not the encoding: each decodes and
/// re-encodes unchanged, leaving rejection to the verifier.
#[test]
fn negatives_are_well_formed_xdr() {
    let v = vectors();
    let negative = v["negative"].as_array().unwrap();
    assert!(!negative.is_empty());
    for vec in negative {
        decode_signed(&vec["signed_transaction_xdr"]);
    }
}
