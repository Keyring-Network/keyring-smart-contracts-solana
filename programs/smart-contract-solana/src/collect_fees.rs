use crate::common::error::KeyringError;
use crate::common::types::ProgramState;
use anchor_lang::prelude::*;
use anchor_lang::Accounts;

#[event]
pub struct FeesCollected {
    amount: u64,
    receiver: Pubkey,
}

#[derive(Accounts)]
pub struct CollectFees<'info> {
    /// CHECK: We are using AccountInfo here as receiver account is only
    /// used to receive the lamports and does not play any other role.
    #[account(mut)]
    pub receiver_account: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"keyring_program".as_ref(), b"global_state".as_ref()],
        bump
    )]
    pub program_state: Account<'info, ProgramState>,
}

pub fn do_collect_fees(ctx: Context<CollectFees>) -> Result<()> {
    let signer_key = ctx.accounts.signer.key;

    if !ctx.accounts.program_state.admin.eq(signer_key) {
        return Err(error!(KeyringError::ErrCallerNotAdmin));
    }

    let rent_sysvar = Rent::get()?;
    // We added 8 bytes for discriminator
    let min_amount_for_rent_exempt = rent_sysvar.minimum_balance(8 + ProgramState::MAX_SIZE);

    let program_balance = ctx.accounts.program_state.get_lamports();
    let amount_to_transfer = program_balance.saturating_sub(min_amount_for_rent_exempt);

    if amount_to_transfer != 0 {
        ctx.accounts
            .program_state
            .sub_lamports(amount_to_transfer)?;
        ctx.accounts
            .receiver_account
            .add_lamports(amount_to_transfer)?;
    }

    emit!(FeesCollected {
        amount: amount_to_transfer,
        receiver: ctx.accounts.receiver_account.key().clone()
    });

    Ok(())
}
