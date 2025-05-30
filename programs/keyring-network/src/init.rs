use crate::common::error::KeyringError;
use crate::common::types::{ChainId, KeyRegistry, ProgramState, CURRENT_VERSION};
use anchor_lang::prelude::*;
use anchor_lang::Accounts;

#[event]
pub struct Initialized {
    admin: Pubkey,
    chain_id: ChainId,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        seeds = [b"keyring_program".as_ref(), b"global_state".as_ref()],
        bump,
        space = 8 + ProgramState::MAX_SIZE
    )]
    pub program_state: Account<'info, ProgramState>,
    #[account(
        init,
        payer = signer,
        seeds = [b"keyring_program".as_ref(), b"active_keys".as_ref()],
        bump,
        space = 8 + KeyRegistry::MAX_SIZE
    )]
    pub key_registry: Account<'info, KeyRegistry>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn do_initialize(ctx: Context<Initialize>, chain_id: Vec<u8>) -> Result<()> {
    let chain_id = ChainId::new(chain_id).map_err(|_| KeyringError::ErrInvalidChainId)?;

    *ctx.accounts.program_state = ProgramState {
        version: CURRENT_VERSION,
        admin: ctx.accounts.signer.key.clone(),
        chain_id: chain_id.clone(),
    };

    emit!(Initialized {
        admin: ctx.accounts.signer.key.clone(),
        chain_id: chain_id.clone(),
    });
    Ok(())
}
