use crate::common::{generate_random_chain_id, init_program};
use anchor_client::solana_client::rpc_client::RpcClient;
use anchor_client::solana_sdk::native_token::LAMPORTS_PER_SOL;
use anchor_client::solana_sdk::signature::Keypair;
use anchor_client::solana_sdk::signer::Signer;
use anchor_client::{
    solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey},
    Client, Cluster,
};
use keyring_network::common::types::ProgramState;
use rand::rngs::OsRng;
use std::str::FromStr;

#[test]
fn test_set_admin() {
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
    let new_admin = Keypair::new();
    let rpc = program.rpc();
    rpc.request_airdrop(&new_admin.pubkey(), 10 * LAMPORTS_PER_SOL)
        .unwrap();

    let mut rng = OsRng::default();
    let chain_id = generate_random_chain_id(&mut rng);
    let (program_state_pubkey, _) = init_program(&program, &payer, chain_id);

    // Non admin should not be able to call set_admin
    program
        .request()
        .accounts(keyring_network::accounts::SetAdmin {
            program_state: program_state_pubkey.clone(),
            signer: new_admin.pubkey(),
        })
        .args(keyring_network::instruction::SetAdmin {
            new_admin: new_admin.pubkey(),
        })
        .payer(&new_admin)
        .send()
        .expect_err("non admin must not be able to call set admin");

    program
        .request()
        .accounts(keyring_network::accounts::SetAdmin {
            program_state: program_state_pubkey.clone(),
            signer: payer.pubkey(),
        })
        .args(keyring_network::instruction::SetAdmin {
            new_admin: new_admin.pubkey(),
        })
        .send()
        .expect("Set admin must be called by current admin");

    // Admin must be changed to new admin
    let program_state_data: ProgramState = program
        .account(program_state_pubkey.clone())
        .expect("Program state must exists after initialization");
    assert_eq!(program_state_data.admin, new_admin.pubkey());
}
