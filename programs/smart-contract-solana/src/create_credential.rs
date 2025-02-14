use crate::common::error::KeyringError;
use crate::common::types::{EntityData, KeyEntry, ProgramState, ToHash, CURRENT_VERSION};
use crate::common::verify_auth_message::verify_auth_message;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::keccak;
use anchor_lang::{system_program, Accounts};

#[event]
pub struct CredentialsCreated {
    key: Vec<u8>,
    policy_id: u64,
    trading_address: Pubkey,
}

#[derive(Accounts)]
#[instruction(key: Vec<u8>, policy_id: u64, trading_address: Pubkey)]
pub struct CreateCredential<'info> {
    #[account(
        mut,
        seeds = [b"keyring_program".as_ref(), b"global_state".as_ref()],
        bump
    )]
    pub program_state: Account<'info, ProgramState>,
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        seeds = [b"keyring_program".as_ref(), b"_key_mapping".as_ref(), &key.to_hash().as_ref()],
        bump
    )]
    pub key_mapping: Account<'info, KeyEntry>,

    #[account(
        init_if_needed,
        payer = signer,
        seeds = [b"keyring_program".as_ref(), b"_entity_mapping".as_ref(), &policy_id.to_le_bytes(), &trading_address.to_bytes()],
        bump,
        space = 8 + EntityData::MAX_SIZE
    )]
    pub entity_mapping: Account<'info, EntityData>,
    pub system_program: Program<'info, System>,
}

pub fn do_create_credential(
    ctx: Context<CreateCredential>,
    key: Vec<u8>,
    policy_id: u64,
    trading_address: Pubkey,
    signature: Vec<u8>,
    valid_from: u64,
    valid_until: u64,
    cost: u64,
    backdoor: Vec<u8>,
) -> Result<()> {
    if cost == 0 {
        return Err(error!(KeyringError::ErrCostParameterZero));
    }

    // Transfer the cost to our PDA
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.signer.to_account_info(),
            to: ctx.accounts.program_state.to_account_info(),
        },
    );
    system_program::transfer(cpi_context, cost)?;

    let trading_address_bytes = trading_address.to_bytes();
    let trading_address_hash = keccak::hash(&trading_address_bytes).to_bytes();
    let truncated_trading_address = trading_address_hash[..20].to_vec();

    if !verify_auth_message(
        key.clone(),
        policy_id,
        truncated_trading_address,
        signature,
        valid_from,
        valid_until,
        cost,
        backdoor,
    )? {
        return Err(error!(KeyringError::ErrInvalidCredentials));
    }

    let clock: Clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp.try_into().unwrap();

    let is_valid = ctx.accounts.key_mapping.is_valid
        && current_timestamp >= ctx.accounts.key_mapping.valid_from
        && current_timestamp <= ctx.accounts.key_mapping.valid_to;
    if !is_valid {
        return Err(error!(KeyringError::ErrInvalidCredentials));
    }

    if valid_until < current_timestamp {
        return Err(error!(KeyringError::ErrInvalidCredentials));
    }

    if ctx.accounts.entity_mapping.blacklisted {
        return Err(error!(KeyringError::ErrInvalidCredentials));
    }
    if valid_until <= ctx.accounts.entity_mapping.exp {
        return Err(error!(KeyringError::ErrInvalidCredentials));
    }
    ctx.accounts.entity_mapping.exp = valid_until;
    ctx.accounts.entity_mapping.version = CURRENT_VERSION;

    emit!(CredentialsCreated {
        key,
        policy_id,
        trading_address
    });

    Ok(())
}
