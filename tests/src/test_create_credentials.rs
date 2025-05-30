use crate::common::{
    convert_pubkey_to_address, generate_random_chain_id, get_timestamp, init_program,
};
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
use keyring_network::common::types::{ChainId, EntityData, ToHash, CURRENT_VERSION};
use keyring_network::common::verify_auth_message::create_signature_payload;
use keyring_network::ID as program_id;
use libsecp256k1::{sign, Message};
use rand::rngs::OsRng;

#[test]
fn create_credentials() {
    let anchor_rpc_client = RpcClient::new(Cluster::Localnet.url());

    let payer = Keypair::new();
    anchor_rpc_client
        .request_airdrop(&payer.pubkey(), 10000 * LAMPORTS_PER_SOL)
        .unwrap();

    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    let program = client.program(program_id).unwrap();

    // Let's fund dummy payer
    let dummy_payer = Keypair::new();
    let rpc = program.rpc();
    rpc.request_airdrop(&dummy_payer.pubkey(), 10 * LAMPORTS_PER_SOL)
        .unwrap();

    let mut rng = OsRng::default();
    let chain_id = generate_random_chain_id(&mut rng);
    let (program_state_pubkey, _) = init_program(&program, &payer, chain_id.clone());

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

    let timestamp = get_timestamp(&rpc);
    program
        .request()
        .accounts(keyring_network::accounts::RegisterKey {
            program_state: program_state_pubkey.clone(),
            key_registry: key_registry.clone(),
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

    let policy_id: u64 = 1;
    let trading_address = Pubkey::new_unique();
    let valid_until = timestamp + 20;
    let cost = 1;
    let backdoor = vec![2; 20];
    let entity_mapping_seeds = [
        b"keyring_program".as_ref(),
        b"_entity_mapping".as_ref(),
        &policy_id.to_le_bytes(),
        &trading_address.to_bytes(),
    ];
    let (entity_mapping_pubkey, _) =
        Pubkey::find_program_address(&entity_mapping_seeds, &program.id());

    let packed_message = create_signature_payload(
        convert_pubkey_to_address(&trading_address),
        policy_id,
        ChainId::new(chain_id.clone()).unwrap(),
        valid_until,
        cost,
        backdoor.clone(),
    )
    .unwrap();
    let message = Message::parse_slice(packed_message.as_ref()).unwrap();
    let (signature, recovery_id) = sign(&message, &secret_key);
    let serialized_recovery_id = recovery_id.serialize() + 27u8;
    let mut serialized_signature = signature.serialize().to_vec();
    serialized_signature.push(serialized_recovery_id);

    // Modify any element from auth message can lead to failure
    let valid_until = timestamp + 40;

    program
        .request()
        .accounts(keyring_network::accounts::CreateCredential {
            program_state: program_state_pubkey.clone(),
            key_mapping: key_mapping_pubkey.clone(),
            signer: payer.pubkey(),
            entity_mapping: entity_mapping_pubkey.clone(),
            system_program: System::id(),
        })
        .args(keyring_network::instruction::CreateCredential {
            key: key.clone(),
            policy_id,
            trading_address,
            signature: serialized_signature.clone(),
            valid_until,
            cost,
            backdoor: backdoor.clone(),
        })
        .send()
        .expect_err("Invalid signature must not succeed");

    let timestamp = get_timestamp(&rpc);
    let policy_id: u64 = 1;
    let trading_address = Pubkey::new_unique();
    let valid_until = timestamp - 10;
    let cost = 1;
    let backdoor = vec![2; 20];
    let entity_mapping_seeds = [
        b"keyring_program".as_ref(),
        b"_entity_mapping".as_ref(),
        &policy_id.to_le_bytes(),
        &trading_address.to_bytes(),
    ];
    let (entity_mapping_pubkey, _) =
        Pubkey::find_program_address(&entity_mapping_seeds, &program.id());

    let packed_message = create_signature_payload(
        convert_pubkey_to_address(&trading_address),
        policy_id,
        ChainId::new(chain_id.clone()).unwrap(),
        valid_until,
        cost,
        backdoor.clone(),
    )
    .unwrap();
    let message = Message::parse_slice(packed_message.as_ref()).unwrap();
    let (signature, recovery_id) = sign(&message, &secret_key);
    let serialized_recovery_id = recovery_id.serialize() + 27u8;
    let mut serialized_signature = signature.serialize().to_vec();
    serialized_signature.push(serialized_recovery_id);

    program
        .request()
        .accounts(keyring_network::accounts::CreateCredential {
            program_state: program_state_pubkey.clone(),
            key_mapping: key_mapping_pubkey.clone(),
            signer: payer.pubkey(),
            entity_mapping: entity_mapping_pubkey.clone(),
            system_program: System::id(),
        })
        .args(keyring_network::instruction::CreateCredential {
            key: key.clone(),
            policy_id,
            trading_address,
            signature: serialized_signature.clone(),
            valid_until,
            cost,
            backdoor: backdoor.clone(),
        })
        .send()
        .expect_err("Invalid valid_until must not be accepted by the program.");

    let timestamp = get_timestamp(&rpc);
    let policy_id: u64 = 1;
    let trading_address = Pubkey::new_unique();
    let valid_until = timestamp + 1000;
    let cost = 100 * LAMPORTS_PER_SOL + rpc.get_balance(&payer.pubkey()).unwrap();
    let backdoor = vec![2; 20];
    let entity_mapping_seeds = [
        b"keyring_program".as_ref(),
        b"_entity_mapping".as_ref(),
        &policy_id.to_le_bytes(),
        &trading_address.to_bytes(),
    ];
    let (entity_mapping_pubkey, _) =
        Pubkey::find_program_address(&entity_mapping_seeds, &program.id());

    let packed_message = create_signature_payload(
        convert_pubkey_to_address(&trading_address),
        policy_id,
        ChainId::new(chain_id.clone()).unwrap(),
        valid_until,
        cost,
        backdoor.clone(),
    )
    .unwrap();
    let message = Message::parse_slice(packed_message.as_ref()).unwrap();
    let (signature, recovery_id) = sign(&message, &secret_key);
    let serialized_recovery_id = recovery_id.serialize() + 27u8;
    let mut serialized_signature = signature.serialize().to_vec();
    serialized_signature.push(serialized_recovery_id);

    program
        .request()
        .accounts(keyring_network::accounts::CreateCredential {
            program_state: program_state_pubkey.clone(),
            key_mapping: key_mapping_pubkey.clone(),
            signer: payer.pubkey(),
            entity_mapping: entity_mapping_pubkey.clone(),
            system_program: System::id(),
        })
        .args(keyring_network::instruction::CreateCredential {
            key: key.clone(),
            policy_id,
            trading_address,
            signature: serialized_signature.clone(),
            valid_until,
            cost,
            backdoor: backdoor.clone(),
        })
        .send()
        .expect_err("Without sufficient balance tx cannot succeed.");

    // If we use different chain_id then one in program state create_credentials will not work
    let cost = 21 * LAMPORTS_PER_SOL;
    let dummy_chain_id = generate_random_chain_id(&mut rng);
    let packed_message = create_signature_payload(
        convert_pubkey_to_address(&trading_address),
        policy_id,
        ChainId::new(dummy_chain_id.clone()).unwrap(),
        valid_until,
        cost,
        backdoor.clone(),
    )
    .unwrap();
    let message = Message::parse_slice(packed_message.as_ref()).unwrap();
    let (signature, recovery_id) = sign(&message, &secret_key);
    let serialized_recovery_id = recovery_id.serialize() + 27u8;
    let mut serialized_signature = signature.serialize().to_vec();
    serialized_signature.push(serialized_recovery_id);

    program
        .request()
        .accounts(keyring_network::accounts::CreateCredential {
            program_state: program_state_pubkey.clone(),
            key_mapping: key_mapping_pubkey.clone(),
            signer: payer.pubkey(),
            entity_mapping: entity_mapping_pubkey.clone(),
            system_program: System::id(),
        })
        .args(keyring_network::instruction::CreateCredential {
            key: key.clone(),
            policy_id,
            trading_address,
            signature: serialized_signature.clone(),
            valid_until,
            cost,
            backdoor: backdoor.clone(),
        })
        .send()
        .expect_err("Dummy chain id should not be accepted");

    let program_state_before_balance = rpc.get_balance(&program_state_pubkey).unwrap();

    let timestamp = get_timestamp(&rpc);
    let policy_id: u64 = 1;
    let trading_address = Pubkey::new_unique();
    let valid_until = timestamp + 1000;
    let cost = 21 * LAMPORTS_PER_SOL;
    let backdoor = vec![2; 20];
    let entity_mapping_seeds = [
        b"keyring_program".as_ref(),
        b"_entity_mapping".as_ref(),
        &policy_id.to_le_bytes(),
        &trading_address.to_bytes(),
    ];
    let (entity_mapping_pubkey, _) =
        Pubkey::find_program_address(&entity_mapping_seeds, &program.id());

    let packed_message = create_signature_payload(
        convert_pubkey_to_address(&trading_address),
        policy_id,
        ChainId::new(chain_id.clone()).unwrap(),
        valid_until,
        cost,
        backdoor.clone(),
    )
    .unwrap();
    let message = Message::parse_slice(packed_message.as_ref()).unwrap();
    let (signature, recovery_id) = sign(&message, &secret_key);
    let serialized_recovery_id = recovery_id.serialize() + 27u8;
    let mut serialized_signature = signature.serialize().to_vec();
    serialized_signature.push(serialized_recovery_id);

    program
        .request()
        .accounts(keyring_network::accounts::CreateCredential {
            program_state: program_state_pubkey.clone(),
            key_mapping: key_mapping_pubkey.clone(),
            signer: payer.pubkey(),
            entity_mapping: entity_mapping_pubkey.clone(),
            system_program: System::id(),
        })
        .args(keyring_network::instruction::CreateCredential {
            key: key.clone(),
            policy_id,
            trading_address,
            signature: serialized_signature.clone(),
            valid_until,
            cost,
            backdoor: backdoor.clone(),
        })
        .send()
        .expect("Valid create credentials request must succeed.");

    let program_state_after_balance = rpc.get_balance(&program_state_pubkey).unwrap();
    assert_eq!(
        program_state_after_balance - program_state_before_balance,
        cost
    );

    let entity_data: EntityData = program.account(entity_mapping_pubkey).unwrap();
    assert_eq!(
        entity_data,
        EntityData {
            version: CURRENT_VERSION,
            blacklisted: false,
            exp: valid_until,
        }
    );

    // We can modify same entity again
    let program_state_before_balance = rpc.get_balance(&program_state_pubkey).unwrap();
    let timestamp = get_timestamp(&rpc);
    let policy_id: u64 = 1;
    let trading_address = Pubkey::new_unique();
    let valid_until = timestamp + 10000;
    let cost = 5 * LAMPORTS_PER_SOL;
    let backdoor = vec![3; 24];
    let entity_mapping_seeds = [
        b"keyring_program".as_ref(),
        b"_entity_mapping".as_ref(),
        &policy_id.to_le_bytes(),
        &trading_address.to_bytes(),
    ];
    let (entity_mapping_pubkey, _) =
        Pubkey::find_program_address(&entity_mapping_seeds, &program.id());

    let packed_message = create_signature_payload(
        convert_pubkey_to_address(&trading_address),
        policy_id,
        ChainId::new(chain_id.clone()).unwrap(),
        valid_until,
        cost,
        backdoor.clone(),
    )
    .unwrap();
    let message = Message::parse_slice(packed_message.as_ref()).unwrap();
    let (signature, recovery_id) = sign(&message, &secret_key);
    let serialized_recovery_id = recovery_id.serialize() + 27u8;
    let mut serialized_signature = signature.serialize().to_vec();
    serialized_signature.push(serialized_recovery_id);

    program
        .request()
        .accounts(keyring_network::accounts::CreateCredential {
            program_state: program_state_pubkey.clone(),
            key_mapping: key_mapping_pubkey.clone(),
            signer: payer.pubkey(),
            entity_mapping: entity_mapping_pubkey.clone(),
            system_program: System::id(),
        })
        .args(keyring_network::instruction::CreateCredential {
            key: key.clone(),
            policy_id,
            trading_address,
            signature: serialized_signature.clone(),
            valid_until,
            cost,
            backdoor: backdoor.clone(),
        })
        .send()
        .expect("Valid create credentials request must succeed.");

    let program_state_after_balance = rpc.get_balance(&program_state_pubkey).unwrap();
    assert_eq!(
        program_state_after_balance - program_state_before_balance,
        cost
    );

    let entity_data: EntityData = program.account(entity_mapping_pubkey).unwrap();
    assert_eq!(
        entity_data,
        EntityData {
            version: CURRENT_VERSION,
            blacklisted: false,
            exp: valid_until,
        }
    );

    // Blacklisted entity cannot be used to create credentials
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

    program
        .request()
        .accounts(keyring_network::accounts::CreateCredential {
            program_state: program_state_pubkey.clone(),
            key_mapping: key_mapping_pubkey.clone(),
            signer: payer.pubkey(),
            entity_mapping: entity_mapping_pubkey.clone(),
            system_program: System::id(),
        })
        .args(keyring_network::instruction::CreateCredential {
            key: key.clone(),
            policy_id,
            trading_address,
            signature: serialized_signature.clone(),
            valid_until,
            cost,
            backdoor: backdoor.clone(),
        })
        .send()
        .expect_err("Blacklisted entity must not be used to create credentials");

    // Once we unblacklist the entity it could be used once again
    program
        .request()
        .accounts(keyring_network::accounts::UnblacklistEntity {
            program_state: program_state_pubkey.clone(),
            signer: payer.pubkey(),
            entity_mapping: entity_mapping_pubkey.clone(),
            system_program: System::id(),
        })
        .args(keyring_network::instruction::UnblacklistEntity {
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
            blacklisted: false,
            exp: 0,
        }
    );

    program
        .request()
        .accounts(keyring_network::accounts::CreateCredential {
            program_state: program_state_pubkey.clone(),
            key_mapping: key_mapping_pubkey.clone(),
            signer: payer.pubkey(),
            entity_mapping: entity_mapping_pubkey.clone(),
            system_program: System::id(),
        })
        .args(keyring_network::instruction::CreateCredential {
            key: key.clone(),
            policy_id,
            trading_address,
            signature: serialized_signature.clone(),
            valid_until,
            cost,
            backdoor: backdoor.clone(),
        })
        .send()
        .expect("Unblacklisted entity can be used to create credentials");

    // If we revoke the key, we will not be able to create credentials based on it.
    program
        .request()
        .accounts(keyring_network::accounts::RevokeKey {
            program_state: program_state_pubkey.clone(),
            key_registry: key_registry.clone(),
            key_mapping: key_mapping_pubkey.clone(),
            signer: payer.pubkey(),
            system_program: System::id(),
        })
        .args(keyring_network::instruction::RevokeKey { key: key.clone() })
        .send()
        .expect("Admin must be allowed to revoke key");

    let timestamp = get_timestamp(&rpc);
    let policy_id: u64 = 1;
    let trading_address = Pubkey::new_unique();
    let valid_until = timestamp + 10000;
    let cost = 5 * LAMPORTS_PER_SOL;
    let backdoor = vec![3; 24];
    let entity_mapping_seeds = [
        b"keyring_program".as_ref(),
        b"_entity_mapping".as_ref(),
        &policy_id.to_le_bytes(),
        &trading_address.to_bytes(),
    ];
    let (entity_mapping_pubkey, _) =
        Pubkey::find_program_address(&entity_mapping_seeds, &program.id());

    let packed_message = create_signature_payload(
        convert_pubkey_to_address(&trading_address),
        policy_id,
        ChainId::new(chain_id.clone()).unwrap(),
        valid_until,
        cost,
        backdoor.clone(),
    )
    .unwrap();
    let message = Message::parse_slice(packed_message.as_ref()).unwrap();
    let (signature, recovery_id) = sign(&message, &secret_key);
    let serialized_recovery_id = recovery_id.serialize() + 27u8;
    let mut serialized_signature = signature.serialize().to_vec();
    serialized_signature.push(serialized_recovery_id);

    program
        .request()
        .accounts(keyring_network::accounts::CreateCredential {
            program_state: program_state_pubkey.clone(),
            key_mapping: key_mapping_pubkey.clone(),
            signer: payer.pubkey(),
            entity_mapping: entity_mapping_pubkey.clone(),
            system_program: System::id(),
        })
        .args(keyring_network::instruction::CreateCredential {
            key: key.clone(),
            policy_id,
            trading_address,
            signature: serialized_signature.clone(),
            valid_until,
            cost,
            backdoor: backdoor.clone(),
        })
        .send()
        .expect_err("Revoked key cannot be used to create credentials");
}
