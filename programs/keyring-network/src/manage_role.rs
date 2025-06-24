use crate::common::error::KeyringError;
use crate::common::types::Role;
use crate::common::types::DEFAULT_ADMIN_ROLE;
use anchor_lang::prelude::*;
use anchor_lang::Accounts;

#[event]
pub struct RoleManaged {
    role: [u8; 32],
    user: Pubkey,
    has_role: bool,
}

#[derive(Accounts)]
#[instruction(role_identifier: [u8; 32], user: Pubkey, has_role: bool)]
pub struct ManageRole<'info> {
    #[account(
        seeds = [DEFAULT_ADMIN_ROLE.as_ref(), signer.key().to_bytes().as_ref()],
        bump
    )]
    pub default_admin_role: Account<'info, Role>,
    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + Role::MAX_SIZE,
        seeds = [role_identifier.as_ref(), user.to_bytes().as_ref()],
        bump
    )]
    role: Account<'info, Role>,
    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn do_manage_role(
    ctx: Context<ManageRole>,
    role_identifier: [u8; 32],
    user: Pubkey,
    has_role: bool,
) -> Result<()> {
    if !ctx.accounts.default_admin_role.has_role {
        return Err(error!(KeyringError::ErrCallerDoesNotHaveRole));
    }

    ctx.accounts.role.has_role = has_role;

    emit!(RoleManaged {
        role: role_identifier,
        user,
        has_role
    });

    Ok(())
}
