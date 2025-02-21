use crate::common::init_program;
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
use keyring_network::common::types::{EntityData, CURRENT_VERSION};
use std::str::FromStr;

#[test]
fn test_blacklist_entity() {
    let program_id = "GJ5ZVSwDmLDwokctrkdrxfYTRndDtPhso8p7imCGVvch";
    let anchor_rpc_client = RpcClient::new(Cluster::Localnet.url());

    let payer = Keypair::new();
    anchor_rpc_client
        .request_airdrop(&payer.pubkey(), 10000 * LAMPORTS_PER_SOL)
        .unwrap();

    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();

    // Let's fund new admin
    let dummy_payer = Keypair::new();
    let rpc = program.rpc();
    rpc.request_airdrop(&dummy_payer.pubkey(), 10 * LAMPORTS_PER_SOL)
        .unwrap();

    let (program_state_pubkey, _) = init_program(&program, &payer);

    let policy_id: u64 = 1;
    let trading_address = Pubkey::new_unique();
    let entity_mapping_seeds = [
        b"keyring_program".as_ref(),
        b"_entity_mapping".as_ref(),
        &policy_id.to_le_bytes(),
        &trading_address.to_bytes(),
    ];
    let (entity_mapping_pubkey, _) =
        Pubkey::find_program_address(&entity_mapping_seeds, &program.id());

    // Non admin should not be able to call blacklist entity
    program
        .request()
        .accounts(keyring_network::accounts::BlacklistEntity {
            program_state: program_state_pubkey.clone(),
            signer: dummy_payer.pubkey(),
            entity_mapping: entity_mapping_pubkey.clone(),
            system_program: System::id(),
        })
        .args(keyring_network::instruction::BlacklistEntity {
            policy_id,
            trading_address,
        })
        .payer(&dummy_payer)
        .send()
        .expect_err("Non-admin should not be able to blacklist entity");

    program
        .request()
        .accounts(keyring_network::accounts::BlacklistEntity {
            program_state: program_state_pubkey.clone(),
            signer: payer.pubkey(),
            entity_mapping: entity_mapping_pubkey.clone(),
            system_program: System::id(),
        })
        .args(keyring_network::instruction::BlacklistEntity {
            policy_id,
            trading_address,
        })
        .send()
        .expect("Admin should be able to blacklist entity");

    let entity_data: EntityData = program.account(entity_mapping_pubkey).unwrap();
    assert_eq!(
        entity_data,
        EntityData {
            version: CURRENT_VERSION,
            blacklisted: true,
            exp: 0,
        }
    );

    // No error must be thrown when we blacklist already blacklisted entity
    program
        .request()
        .accounts(keyring_network::accounts::BlacklistEntity {
            program_state: program_state_pubkey.clone(),
            signer: payer.pubkey(),
            entity_mapping: entity_mapping_pubkey.clone(),
            system_program: System::id(),
        })
        .args(keyring_network::instruction::BlacklistEntity {
            policy_id,
            trading_address,
        })
        .send()
        .expect("Admin should be able to blacklist entity");

    let entity_data: EntityData = program.account(entity_mapping_pubkey).unwrap();
    assert_eq!(
        entity_data,
        EntityData {
            version: CURRENT_VERSION,
            blacklisted: true,
            exp: 0,
        }
    );
}
