use crate::common::convert_secp_pubkey_to_address;
use anchor_client::solana_sdk::secp256k1_recover::secp256k1_recover;
use serde::{Deserialize, Serialize};
use keyring_network::common::error::KeyringError;
use keyring_network::common::verify_auth_message::{
    create_signature_payload, split_signature,
};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Secp256k1Vector {
    #[serde(with = "hex::serde")]
    pub trading_address: Vec<u8>,
    pub policy_id: u64,
    pub create_before: u64,
    pub valid_until: u64,
    pub cost: u64,
    #[serde(with = "hex::serde")]
    pub backdoor: Vec<u8>,
    #[serde(with = "hex::serde")]
    pub key: Vec<u8>,
    #[serde(with = "hex::serde")]
    pub signature: Vec<u8>,
    pub expected: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Secp256k1Vectors {
    pub vectors: Vec<Secp256k1Vector>,
}

#[test]
pub fn test_verify_auth_message() {
    let secp256k1_vectors = fs::read("./vectors/secp256k1_vector.json").unwrap();
    let secp256k1_vectors = String::from_utf8_lossy(&secp256k1_vectors);
    let vectors: Secp256k1Vectors = serde_json::from_str(&*secp256k1_vectors).unwrap();

    // Out of range parameter should give an error
    assert_eq!(
        create_signature_payload(
            vectors.vectors[0].trading_address.clone(),
            2u64.pow(24),
            vectors.vectors[0].create_before,
            vectors.vectors[0].valid_until,
            vectors.vectors[0].cost,
            vectors.vectors[0].backdoor.clone(),
        )
        .unwrap_err(),
        KeyringError::ErrAuthMessageParameterOutOfRange.into()
    );

    assert_eq!(
        create_signature_payload(
            vectors.vectors[0].trading_address.clone(),
            vectors.vectors[0].policy_id,
            u32::MAX as u64 + 1,
            vectors.vectors[0].valid_until,
            vectors.vectors[0].cost,
            vectors.vectors[0].backdoor.clone(),
        )
        .unwrap_err(),
        KeyringError::ErrAuthMessageParameterOutOfRange.into()
    );

    assert_eq!(
        create_signature_payload(
            vectors.vectors[0].trading_address.clone(),
            vectors.vectors[0].policy_id,
            vectors.vectors[0].create_before,
            u32::MAX as u64 + 1,
            vectors.vectors[0].cost,
            vectors.vectors[0].backdoor.clone(),
        )
        .unwrap_err(),
        KeyringError::ErrAuthMessageParameterOutOfRange.into()
    );

    for vector in vectors.vectors {
        let message_hash = create_signature_payload(
            vector.trading_address,
            vector.policy_id,
            vector.create_before,
            vector.valid_until,
            vector.cost,
            vector.backdoor,
        )
        .unwrap();
        let (signature, recovery_id) = split_signature(vector.signature).unwrap();
        let maybe_recovered_pubkey =
            secp256k1_recover(message_hash.as_ref(), recovery_id, &signature);
        assert_eq!(maybe_recovered_pubkey.is_ok(), vector.expected);
        if maybe_recovered_pubkey.is_ok() {
            let recovered_pubkey = maybe_recovered_pubkey.unwrap();
            assert_eq!(
                convert_secp_pubkey_to_address(&recovered_pubkey),
                vector.key[1..].to_vec()
            );
        }
    }
}
