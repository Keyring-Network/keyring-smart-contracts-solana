use crate::common::error::KeyringError;
use crate::common::types::{KeyEntry, KeyRegistry, ProgramState, ToHash};
use anchor_lang::prelude::*;
use anchor_lang::Accounts;

#[event]
pub struct KeyRevoked {
    key: Vec<u8>,
}

#[derive(Accounts)]
#[instruction(key: Vec<u8>)]
pub struct RevokeKey<'info> {
    #[account(
        seeds = [b"keyring_program".as_ref(), b"global_state".as_ref()],
        bump
    )]
    pub program_state: Account<'info, ProgramState>,
    #[account(
        mut,
        seeds = [b"keyring_program".as_ref(), b"active_keys".as_ref()],
        bump,
    )]
    pub key_registry: Account<'info, KeyRegistry>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"keyring_program".as_ref(), b"_key_mapping".as_ref(), &key.to_hash().as_ref()],
        bump
    )]
    pub key_mapping: Account<'info, KeyEntry>,
    pub system_program: Program<'info, System>,
}

pub fn do_revoke_key(ctx: Context<RevokeKey>, key: Vec<u8>) -> Result<()> {
    let signer_key = ctx.accounts.signer.key;

    if !ctx.accounts.program_state.admin.eq(signer_key) {
        return Err(error!(KeyringError::ErrCallerNotAdmin));
    }

    ctx.accounts.key_mapping.is_valid = false;

    let active_keys = &mut ctx.accounts.key_registry.active_keys;
    if let Some(index) = active_keys.iter().position(|x| x.eq(&key)) {
        active_keys.swap_remove(index);
    }

    emit!(KeyRevoked { key });

    Ok(())
}
