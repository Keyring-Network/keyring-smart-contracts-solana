use crate::common::{generate_random_chain_id, get_timestamp, init_program};
use anchor_client::anchor_lang::prelude::System;
use anchor_client::anchor_lang::Id;
use anchor_client::solana_client::rpc_client::RpcClient;
use anchor_client::solana_sdk::native_token::LAMPORTS_PER_SOL;
use anchor_client::solana_sdk::signature::Keypair;
use anchor_client::solana_sdk::signer::Signer;
use anchor_client::{
    solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey},
    Client, Cluster,
};
use keyring_network::common::types::{KeyRegistry, ToHash, KEY_MANAGER_ROLE};
use keyring_network::ID as program_id;
use rand::rngs::OsRng;

#[test]
fn register_key() {
    let anchor_rpc_client = RpcClient::new(Cluster::Localnet.url());

    let payer = Keypair::new();
    anchor_rpc_client
        .request_airdrop(&payer.pubkey(), 10000 * LAMPORTS_PER_SOL)
        .unwrap();

    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    let program = client.program(program_id).unwrap();

    // Let's fund dummy payer
    let dummy_payer = Keypair::new();
    let rpc = RpcClient::new(Cluster::Localnet.url());
    rpc.request_airdrop(&dummy_payer.pubkey(), 10 * LAMPORTS_PER_SOL)
        .unwrap();

    let mut rng = OsRng::default();
    let chain_id = generate_random_chain_id(&mut rng);
    let (_, _, default_admin_role_pubkey) = init_program(&program, &payer, chain_id);

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
    let (key_registry, _) = Pubkey::find_program_address(
        &[b"keyring_program".as_ref(), b"active_keys".as_ref()],
        &program.id(),
    );
    let (key_manager_role_account_for_admin, _) = Pubkey::find_program_address(
        &[
            KEY_MANAGER_ROLE.as_ref(),
            payer.pubkey().to_bytes().as_ref(),
        ],
        &program.id(),
    );
    let (key_manager_role_account_for_dummy_payer, _) = Pubkey::find_program_address(
        &[
            KEY_MANAGER_ROLE.as_ref(),
            dummy_payer.pubkey().to_bytes().as_ref(),
        ],
        &program.id(),
    );

    program
        .request()
        .accounts(keyring_network::accounts::ManageRole {
            default_admin_role: default_admin_role_pubkey,
            role: key_manager_role_account_for_admin,
            signer: payer.pubkey(),
            system_program: System::id(),
        })
        .args(keyring_network::instruction::ManageRoles {
            role: KEY_MANAGER_ROLE,
            user: payer.pubkey(),
            has_role: true,
        })
        .send()
        .expect("Current admin must be able to grant key manager role");

    let timestamp = get_timestamp(&rpc);
    program
        .request()
        .accounts(keyring_network::accounts::RegisterKey {
            key_registry: key_registry.clone(),
            key_mapping: key_mapping_pubkey.clone(),
            signer: dummy_payer.pubkey(),
            key_manager_role: key_manager_role_account_for_dummy_payer,
            system_program: System::id(),
        })
        .args(keyring_network::instruction::RegisterKey {
            key: key.clone(),
            valid_from: timestamp - 1,
            valid_to: timestamp + 20,
        })
        .payer(&dummy_payer)
        .send()
        .expect_err("DummyPayer must not be allowed to register new key");

    let timestamp = get_timestamp(&rpc);
    program
        .request()
        .accounts(keyring_network::accounts::RegisterKey {
            key_registry: key_registry.clone(),
            key_mapping: key_mapping_pubkey.clone(),
            signer: payer.pubkey(),
            key_manager_role: key_manager_role_account_for_admin,
            system_program: System::id(),
        })
        .args(keyring_network::instruction::RegisterKey {
            key: key.clone(),
            valid_from: timestamp + 20,
            valid_to: timestamp + 20,
        })
        .send()
        .expect_err("invalid valid_from should be rejected by program");

    let timestamp = get_timestamp(&rpc);
    program
        .request()
        .accounts(keyring_network::accounts::RegisterKey {
            key_registry: key_registry.clone(),
            key_mapping: key_mapping_pubkey.clone(),
            signer: payer.pubkey(),
            key_manager_role: key_manager_role_account_for_admin,
            system_program: System::id(),
        })
        .args(keyring_network::instruction::RegisterKey {
            key: key.clone(),
            valid_from: timestamp - 10,
            valid_to: timestamp - 1,
        })
        .send()
        .expect_err("invalid valid_to should be rejected by program");

    let invalid_key = vec![1, 2, 3];
    let invalid_key_hash = invalid_key.to_hash();
    let invalid_key_mapping_seeds = [
        b"keyring_program".as_ref(),
        b"_key_mapping".as_ref(),
        invalid_key_hash.as_ref(),
    ];
    let (invalid_key_mapping_pubkey, _) =
        Pubkey::find_program_address(&invalid_key_mapping_seeds, &program.id());

    let timestamp = get_timestamp(&rpc);
    program
        .request()
        .accounts(keyring_network::accounts::RegisterKey {
            key_registry: key_registry.clone(),
            key_mapping: invalid_key_mapping_pubkey.clone(),
            signer: payer.pubkey(),
            key_manager_role: key_manager_role_account_for_admin,
            system_program: System::id(),
        })
        .args(keyring_network::instruction::RegisterKey {
            key: invalid_key,
            valid_from: timestamp - 1,
            valid_to: timestamp + 20,
        })
        .send()
        .expect_err("Invalid key must be rejected");

    let timestamp = get_timestamp(&rpc);
    program
        .request()
        .accounts(keyring_network::accounts::RegisterKey {
            key_registry: key_registry.clone(),
            key_mapping: key_mapping_pubkey.clone(),
            signer: payer.pubkey(),
            key_manager_role: key_manager_role_account_for_admin,
            system_program: System::id(),
        })
        .args(keyring_network::instruction::RegisterKey {
            key: key.clone(),
            valid_from: timestamp - 1,
            valid_to: timestamp + 20,
        })
        .send()
        .expect("Valid key registration must be successful");

    let timestamp = get_timestamp(&rpc);
    program
        .request()
        .accounts(keyring_network::accounts::RegisterKey {
            key_registry: key_registry.clone(),
            key_mapping: key_mapping_pubkey.clone(),
            signer: payer.pubkey(),
            key_manager_role: key_manager_role_account_for_admin,
            system_program: System::id(),
        })
        .args(keyring_network::instruction::RegisterKey {
            key: key.clone(),
            valid_from: timestamp - 1,
            valid_to: timestamp + 20,
        })
        .send()
        .expect_err("Same key cannot be registered twice without revoking");

    let key_registry_account: KeyRegistry = program.account(key_registry).unwrap();
    assert_eq!(
        key_registry_account.active_keys.first().unwrap().clone(),
        key.clone()
    );
    assert_eq!(key_registry_account.active_keys.len(), 1);
}
