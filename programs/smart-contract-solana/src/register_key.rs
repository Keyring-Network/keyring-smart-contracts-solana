use crate::common::error::KeyringError;
use crate::common::types::{KeyEntry, ProgramState, ToHash};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::secp256k1_recover::SECP256K1_PUBLIC_KEY_LENGTH;
use anchor_lang::Accounts;

#[event]
pub struct KeyRegistered {
    key: Vec<u8>,
    valid_from: u64,
    valid_to: u64,
}

#[derive(Accounts)]
#[instruction(key: Vec<u8>)]
pub struct RegisterKey<'info> {
    #[account(
        seeds = [b"keyring_program".as_ref(), b"global_state".as_ref()],
        bump
    )]
    pub program_state: Account<'info, ProgramState>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init_if_needed,
        payer = signer,
        seeds = [b"keyring_program".as_ref(), b"_key_mapping".as_ref(), &key.to_hash().as_ref()],
        bump,
        space = 8 + KeyEntry::MAX_SIZE
    )]
    pub key_mapping: Account<'info, KeyEntry>,
    pub system_program: Program<'info, System>,
}

pub fn do_register_key(
    ctx: Context<RegisterKey>,
    key: Vec<u8>,
    valid_from: u64,
    valid_to: u64,
) -> Result<()> {
    let signer_key = ctx.accounts.signer.key;

    if !ctx.accounts.program_state.admin.eq(signer_key) {
        return Err(error!(KeyringError::ErrCallerNotAdmin));
    }

    let clock: Clock = Clock::get()?;
    let time_stamp = clock.unix_timestamp;

    if key.len() != SECP256K1_PUBLIC_KEY_LENGTH {
        return Err(error!(KeyringError::ErrInvalidPubkeyLength));
    }

    if valid_to <= valid_from {
        return Err(error!(KeyringError::ErrInvalidKeyRegistrationParams));
    }

    if valid_to < time_stamp as u64 {
        return Err(error!(KeyringError::ErrInvalidKeyRegistrationParams));
    }

    if ctx.accounts.key_mapping.is_valid {
        return Err(error!(KeyringError::ErrKeyAlreadyRegistered));
    }

    *ctx.accounts.key_mapping = KeyEntry {
        is_valid: true,
        valid_from,
        valid_to,
    };

    emit!(KeyRegistered {
        key,
        valid_from,
        valid_to
    });

    Ok(())
}
