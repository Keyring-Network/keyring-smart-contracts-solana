use crate::common::{generate_random_chain_id, init_program};
use anchor_client::anchor_lang::prelude::System;
use anchor_client::anchor_lang::Id;
use anchor_client::solana_client::rpc_client::RpcClient;
use anchor_client::solana_sdk::native_token::LAMPORTS_PER_SOL;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::signature::Keypair;
use anchor_client::solana_sdk::signer::Signer;
use anchor_client::{solana_sdk::commitment_config::CommitmentConfig, Client, Cluster};
use keyring_network::common::types::{
    Role, BLACKLIST_MANAGER_ROLE, DEFAULT_ADMIN_ROLE, KEY_MANAGER_ROLE, OPERATOR_ROLE,
};
use keyring_network::ID as program_id;
use rand::rngs::OsRng;

#[test]
fn test_manage_roles() {
    let anchor_rpc_client = RpcClient::new(Cluster::Localnet.url());

    let payer = Keypair::new();
    anchor_rpc_client
        .request_airdrop(&payer.pubkey(), 10000 * LAMPORTS_PER_SOL)
        .unwrap();

    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    let program = client.program(program_id).unwrap();

    // Let's fund new admin
    let new_admin = Keypair::new();
    let rpc = program.rpc();
    rpc.request_airdrop(&new_admin.pubkey(), 10 * LAMPORTS_PER_SOL)
        .unwrap();

    let mut rng = OsRng::default();
    let chain_id = generate_random_chain_id(&mut rng);
    let (_, _, default_admin_role_pubkey) = init_program(&program, &payer, chain_id);
    let (default_admin_role_account_for_new_admin, _) = Pubkey::find_program_address(
        &[
            DEFAULT_ADMIN_ROLE.as_ref(),
            new_admin.pubkey().to_bytes().as_ref(),
        ],
        &program.id(),
    );
    let (key_manager_role_account_for_new_admin, _) = Pubkey::find_program_address(
        &[
            KEY_MANAGER_ROLE.as_ref(),
            new_admin.pubkey().to_bytes().as_ref(),
        ],
        &program.id(),
    );
    let (blacklist_manager_role_account_for_new_admin, _) = Pubkey::find_program_address(
        &[
            BLACKLIST_MANAGER_ROLE.as_ref(),
            new_admin.pubkey().to_bytes().as_ref(),
        ],
        &program.id(),
    );
    let (operator_role_account_for_new_admin, _) = Pubkey::find_program_address(
        &[
            OPERATOR_ROLE.as_ref(),
            new_admin.pubkey().to_bytes().as_ref(),
        ],
        &program.id(),
    );

    // Non admin should not be able to manage roles
    program
        .request()
        .accounts(keyring_network::accounts::ManageRole {
            default_admin_role: default_admin_role_account_for_new_admin,
            role: default_admin_role_account_for_new_admin,
            signer: new_admin.pubkey(),
            system_program: System::id(),
        })
        .args(keyring_network::instruction::ManageRoles {
            role: DEFAULT_ADMIN_ROLE,
            user: new_admin.pubkey(),
            has_role: true,
        })
        .payer(&new_admin)
        .send()
        .expect_err("Non admin must not be able to manage roles");

    program
        .request()
        .accounts(keyring_network::accounts::ManageRole {
            default_admin_role: default_admin_role_pubkey,
            role: default_admin_role_account_for_new_admin,
            signer: payer.pubkey(),
            system_program: System::id(),
        })
        .args(keyring_network::instruction::ManageRoles {
            role: DEFAULT_ADMIN_ROLE,
            user: new_admin.pubkey(),
            has_role: true,
        })
        .send()
        .expect("Current admin must be able to grant admin role");

    let role_account_data: Role = program
        .account(default_admin_role_account_for_new_admin.clone())
        .expect("Default admin role account must exist after granting role");
    assert_eq!(role_account_data.has_role, true);

    program
        .request()
        .accounts(keyring_network::accounts::ManageRole {
            default_admin_role: default_admin_role_pubkey,
            role: key_manager_role_account_for_new_admin,
            signer: payer.pubkey(),
            system_program: System::id(),
        })
        .args(keyring_network::instruction::ManageRoles {
            role: KEY_MANAGER_ROLE,
            user: new_admin.pubkey(),
            has_role: true,
        })
        .send()
        .expect("Current admin must be able to grant key manager role");

    let role_account_data: Role = program
        .account(key_manager_role_account_for_new_admin.clone())
        .expect("Key manager role account must exist after granting role");
    assert_eq!(role_account_data.has_role, true);

    program
        .request()
        .accounts(keyring_network::accounts::ManageRole {
            default_admin_role: default_admin_role_pubkey,
            role: blacklist_manager_role_account_for_new_admin,
            signer: payer.pubkey(),
            system_program: System::id(),
        })
        .args(keyring_network::instruction::ManageRoles {
            role: BLACKLIST_MANAGER_ROLE,
            user: new_admin.pubkey(),
            has_role: true,
        })
        .send()
        .expect("Current admin must be able to grant blacklist manager role");

    let role_account_data: Role = program
        .account(blacklist_manager_role_account_for_new_admin.clone())
        .expect("Blacklist manager role account must exist after granting role");
    assert_eq!(role_account_data.has_role, true);

    program
        .request()
        .accounts(keyring_network::accounts::ManageRole {
            default_admin_role: default_admin_role_pubkey,
            role: operator_role_account_for_new_admin,
            signer: payer.pubkey(),
            system_program: System::id(),
        })
        .args(keyring_network::instruction::ManageRoles {
            role: OPERATOR_ROLE,
            user: new_admin.pubkey(),
            has_role: true,
        })
        .send()
        .expect("Current admin must be able to grant operator role");

    let role_account_data: Role = program
        .account(operator_role_account_for_new_admin.clone())
        .expect("Operator role account must exist after granting role");
    assert_eq!(role_account_data.has_role, true);

    ////////////////////////////////////////////////////

    program
        .request()
        .accounts(keyring_network::accounts::ManageRole {
            default_admin_role: default_admin_role_pubkey,
            role: default_admin_role_account_for_new_admin,
            signer: payer.pubkey(),
            system_program: System::id(),
        })
        .args(keyring_network::instruction::ManageRoles {
            role: DEFAULT_ADMIN_ROLE,
            user: new_admin.pubkey(),
            has_role: false,
        })
        .send()
        .expect("Current admin must be able to revoke admin role");

    let role_account_data: Role = program
        .account(default_admin_role_account_for_new_admin.clone())
        .expect("Default admin role account must exist event after revoking role");
    assert_eq!(role_account_data.has_role, false);

    program
        .request()
        .accounts(keyring_network::accounts::ManageRole {
            default_admin_role: default_admin_role_pubkey,
            role: key_manager_role_account_for_new_admin,
            signer: payer.pubkey(),
            system_program: System::id(),
        })
        .args(keyring_network::instruction::ManageRoles {
            role: KEY_MANAGER_ROLE,
            user: new_admin.pubkey(),
            has_role: false,
        })
        .send()
        .expect("Current admin must be able to revoke key manager role");

    let role_account_data: Role = program
        .account(key_manager_role_account_for_new_admin.clone())
        .expect("Key manager role account must exist event after revoking role");
    assert_eq!(role_account_data.has_role, false);

    program
        .request()
        .accounts(keyring_network::accounts::ManageRole {
            default_admin_role: default_admin_role_pubkey,
            role: blacklist_manager_role_account_for_new_admin,
            signer: payer.pubkey(),
            system_program: System::id(),
        })
        .args(keyring_network::instruction::ManageRoles {
            role: BLACKLIST_MANAGER_ROLE,
            user: new_admin.pubkey(),
            has_role: false,
        })
        .send()
        .expect("Current admin must be able to revoke blacklist manager role");

    let role_account_data: Role = program
        .account(blacklist_manager_role_account_for_new_admin.clone())
        .expect("Blacklist manager role account must exist even after revoking role");
    assert_eq!(role_account_data.has_role, false);

    program
        .request()
        .accounts(keyring_network::accounts::ManageRole {
            default_admin_role: default_admin_role_pubkey,
            role: operator_role_account_for_new_admin,
            signer: payer.pubkey(),
            system_program: System::id(),
        })
        .args(keyring_network::instruction::ManageRoles {
            role: OPERATOR_ROLE,
            user: new_admin.pubkey(),
            has_role: false,
        })
        .send()
        .expect("Current admin must be able to revoke operator role");

    let role_account_data: Role = program
        .account(operator_role_account_for_new_admin.clone())
        .expect("Operator role account must exist even after revoking role");
    assert_eq!(role_account_data.has_role, false);
}
