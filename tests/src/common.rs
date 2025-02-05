use anchor_client::anchor_lang::prelude::{Clock, Pubkey, System};
use anchor_client::anchor_lang::Id;
use anchor_client::solana_client::rpc_client::RpcClient;
use anchor_client::solana_sdk::signature::{Keypair, Signer};
use anchor_client::solana_sdk::sysvar::clock;
use anchor_client::Program;
use smart_contract_solana::common::types::ProgramState;
use std::thread::sleep;
use std::time::Duration;

pub fn init_program(program: &Program<&Keypair>, payer: &Keypair) -> (Pubkey, ProgramState) {
    // We need to wait a bit for validator to start
    sleep(Duration::from_secs(10));

    let (program_state, _) = Pubkey::find_program_address(
        &[b"keyring_program".as_ref(), b"global_state".as_ref()],
        &program.id(),
    );

    // First initialization should be successful
    program
        .request()
        .accounts(smart_contract_solana::accounts::Initialize {
            program_state: program_state.clone(),
            signer: payer.pubkey(),
            system_program: System::id(),
        })
        .args(smart_contract_solana::instruction::Initialize {})
        .send()
        .expect("First initialization must be successful");

    // Second initialization should return an error
    program
        .request()
        .accounts(smart_contract_solana::accounts::Initialize {
            program_state: program_state.clone(),
            signer: payer.pubkey(),
            system_program: System::id(),
        })
        .args(smart_contract_solana::instruction::Initialize {})
        .send()
        .expect_err("Second initialization should not be successful");

    // We need to check if admin is set to payer
    let program_state_data: ProgramState = program
        .account(program_state.clone())
        .expect("Program state must exists after initialization");
    if program_state_data.admin != payer.pubkey() {
        panic!("Administrative must be payer");
    }

    (program_state, program_state_data)
}

pub fn get_timestamp(rpc: &RpcClient) -> u64 {
    let clock = rpc.get_account(&clock::ID).unwrap();
    let clock_sysvar: Clock = bincode::deserialize(&clock.data).unwrap();
    clock_sysvar.unix_timestamp.try_into().unwrap()
}
