use crate::common::error::KeyringError;
use crate::common::types::{EntityData, ProgramState, CURRENT_VERSION};
use anchor_lang::prelude::*;
use anchor_lang::Accounts;

#[event]
pub struct UnBlackListedEntity {
    policy_id: u64,
    trading_address: Pubkey,
}

#[derive(Accounts)]
#[instruction(policy_id: u64, trading_address: Pubkey)]
pub struct UnblacklistEntity<'info> {
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
        seeds = [b"keyring_program".as_ref(), b"_entity_mapping".as_ref(), &policy_id.to_le_bytes(), &trading_address.to_bytes()],
        bump,
        space = 8 + EntityData::MAX_SIZE
    )]
    pub entity_mapping: Account<'info, EntityData>,
    pub system_program: Program<'info, System>,
}

pub fn do_unblacklist_entity(
    ctx: Context<UnblacklistEntity>,
    policy_id: u64,
    trading_address: Pubkey,
) -> Result<()> {
    let signer_key = ctx.accounts.signer.key;

    if !ctx.accounts.program_state.admin.eq(signer_key) {
        return Err(error!(KeyringError::ErrCallerNotAdmin));
    }

    if !ctx.accounts.entity_mapping.blacklisted {
        ctx.accounts.entity_mapping.version = CURRENT_VERSION;
        return Ok(());
    }

    *ctx.accounts.entity_mapping = EntityData {
        version: CURRENT_VERSION,
        blacklisted: false,
        exp: 0,
    };

    emit!(UnBlackListedEntity {
        policy_id,
        trading_address
    });

    Ok(())
}
