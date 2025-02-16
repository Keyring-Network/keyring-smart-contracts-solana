use crate::common::error::KeyringError;
use crate::common::types::EntityData;
use anchor_lang::prelude::*;
use anchor_lang::Accounts;

#[event]
pub struct ValidCredentials {
    policy_id: u64,
    trading_address: Pubkey,
}

#[derive(Accounts)]
#[instruction(policy_id: u64, trading_address: Pubkey)]
pub struct CheckCredential<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        seeds = [b"keyring_program".as_ref(), b"_entity_mapping".as_ref(), &policy_id.to_le_bytes(), &trading_address.to_bytes()],
        bump,
    )]
    pub entity_mapping: Account<'info, EntityData>,
}

pub fn do_check_credential(
    ctx: Context<CheckCredential>,
    policy_id: u64,
    trading_address: Pubkey,
) -> Result<()> {
    let clock: Clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp.try_into().unwrap();

    if !ctx.accounts.entity_mapping.blacklisted
        && ctx.accounts.entity_mapping.exp > current_timestamp
    {
        emit!(ValidCredentials {
            policy_id,
            trading_address
        });
        Ok(())
    } else {
        Err(error!(KeyringError::ErrInCheckingCredentials))
    }
}
