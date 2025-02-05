use crate::common::error::KeyringError;
use crate::common::types::ProgramState;
use anchor_lang::prelude::*;
use anchor_lang::Accounts;

#[event]
pub struct AdminSet {
    new_admin: Pubkey,
    old_admin: Pubkey,
}

#[derive(Accounts)]
pub struct SetAdmin<'info> {
    #[account(
        mut,
        seeds = [b"keyring_program".as_ref(), b"global_state".as_ref()],
        bump
    )]
    pub program_state: Account<'info, ProgramState>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

pub fn do_set_admin(ctx: Context<SetAdmin>, new_admin: Pubkey) -> Result<()> {
    let signer_key = ctx.accounts.signer.key;

    if !ctx.accounts.program_state.admin.eq(signer_key) {
        return Err(error!(KeyringError::ErrCallerNotAdmin));
    }
    ctx.accounts.program_state.admin = new_admin;

    emit!(AdminSet {
        new_admin,
        old_admin: signer_key.clone()
    });

    Ok(())
}
