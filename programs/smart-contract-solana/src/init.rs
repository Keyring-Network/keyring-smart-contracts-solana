use crate::common::types::{ProgramState, CURRENT_VERSION};
use anchor_lang::prelude::*;
use anchor_lang::Accounts;

#[event]
pub struct Initialized {
    admin: Pubkey,
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
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn do_initialize(ctx: Context<Initialize>) -> Result<()> {
    *ctx.accounts.program_state = ProgramState {
        version: CURRENT_VERSION,
        admin: ctx.accounts.signer.key.clone(),
    };

    emit!(Initialized {
        admin: ctx.accounts.signer.key.clone()
    });
    Ok(())
}
