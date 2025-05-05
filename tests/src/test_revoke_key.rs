use crate::common::{generate_random_chain_id, get_timestamp, init_program};
use anchor_client::anchor_lang::prelude::System;
use anchor_client::anchor_lang::Id;
use anchor_client::solana_client::rpc_client::RpcClient;
use anchor_client::solana_sdk::native_token::LAMPORTS_PER_SOL;
use anchor_client::solana_sdk::secp256k1_recover::SECP256K1_PUBLIC_KEY_LENGTH;
use anchor_client::solana_sdk::signature::Keypair;
use anchor_client::solana_sdk::signer::Signer;
use anchor_client::{
    solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey},
    Client, Cluster,
};
use keyring_network::common::types::ToHash;
use rand::rngs::OsRng;
use std::str::FromStr;

#[test]
fn revoke_key() {
    let program_id = "GJ5ZVSwDmLDwokctrkdrxfYTRndDtPhso8p7imCGVvch";
    let anchor_rpc_client = RpcClient::new(Cluster::Localnet.url());

    let payer = Keypair::new();
    anchor_rpc_client
        .request_airdrop(&payer.pubkey(), 10000 * LAMPORTS_PER_SOL)
        .unwrap();

    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();

    // Let's fund dummy payer
    let dummy_payer = Keypair::new();
    let rpc = RpcClient::new(Cluster::Localnet.url());
    rpc.request_airdrop(&dummy_payer.pubkey(), 10 * LAMPORTS_PER_SOL)
        .unwrap();

    let mut rng = OsRng::default();
    let chain_id = generate_random_chain_id(&mut rng);
    let (program_state_pubkey, _) = init_program(&program, &payer, chain_id);

    let mut os_rng = OsRng::default();
    let secret_key = libsecp256k1::SecretKey::random(&mut os_rng);
    let public_key = libsecp256k1::PublicKey::from_secret_key(&secret_key);
    let key = public_key.serialize()[1..].to_vec();
    let key_hash = key.to_hash();
    let key_mapping_seeds = [
        b"keyring_program".as_ref(),
        b"_key_mapping".as_ref(),
        key_hash.as_ref(),
    ];
    let (key_mapping_pubkey, _) = Pubkey::find_program_address(&key_mapping_seeds, &program.id());

    let timestamp = get_timestamp(&rpc);
    program
        .request()
        .accounts(keyring_network::accounts::RegisterKey {
            program_state: program_state_pubkey.clone(),
            key_mapping: key_mapping_pubkey.clone(),
            signer: payer.pubkey(),
            system_program: System::id(),
        })
        .args(keyring_network::instruction::RegisterKey {
            key: key.clone(),
            valid_from: timestamp - 1,
            valid_to: timestamp + 20,
        })
        .send()
        .expect("Valid key registration must be successful");

    program
        .request()
        .accounts(keyring_network::accounts::RevokeKey {
            program_state: program_state_pubkey.clone(),
            key_mapping: key_mapping_pubkey.clone(),
            signer: dummy_payer.pubkey(),
            system_program: System::id(),
        })
        .args(keyring_network::instruction::RevokeKey { key: key.clone() })
        .payer(&dummy_payer)
        .send()
        .expect_err("DummyPayer must not be allowed to revoke new key");

    // We cannot revoke unknown key without registering it
    let invalid_key = vec![1; SECP256K1_PUBLIC_KEY_LENGTH];
    let invalid_key_hash = invalid_key.to_hash();
    let invalid_key_mapping_seeds = [
        b"keyring_program".as_ref(),
        b"_key_mapping".as_ref(),
        invalid_key_hash.as_ref(),
    ];
    let (invalid_key_mapping_pubkey, _) =
        Pubkey::find_program_address(&invalid_key_mapping_seeds, &program.id());

    program
        .request()
        .accounts(keyring_network::accounts::RevokeKey {
            program_state: program_state_pubkey.clone(),
            key_mapping: invalid_key_mapping_pubkey.clone(),
            signer: payer.pubkey(),
            system_program: System::id(),
        })
        .args(keyring_network::instruction::RevokeKey { key: invalid_key })
        .send()
        .expect_err("Invalid key cannot be revoked");

    program
        .request()
        .accounts(keyring_network::accounts::RevokeKey {
            program_state: program_state_pubkey.clone(),
            key_mapping: key_mapping_pubkey.clone(),
            signer: payer.pubkey(),
            system_program: System::id(),
        })
        .args(keyring_network::instruction::RevokeKey { key: key.clone() })
        .send()
        .expect("Admin must be allowed to revoke key");
}
