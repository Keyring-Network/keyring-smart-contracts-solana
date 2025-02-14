mod blacklist_entity;
mod check_credentials;
mod collect_fees;
pub mod common;
mod create_credential;
mod init;
mod register_key;
mod revoke_key;
mod set_admin;
mod unblacklist_entity;

use anchor_lang::prelude::*;
use blacklist_entity::*;
use check_credentials::*;
use collect_fees::*;
use create_credential::*;
use init::*;
use register_key::*;
use revoke_key::*;
use set_admin::*;
use unblacklist_entity::*;

declare_id!("9tDMCGD9wDGE9ZEqGRteg9sR9kVEm7wxqdHZnHDdC3qw");

#[program]
pub mod smart_contract_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        do_initialize(ctx)
    }

    pub fn set_admin(ctx: Context<SetAdmin>, new_admin: Pubkey) -> Result<()> {
        do_set_admin(ctx, new_admin)
    }

    pub fn register_key(
        ctx: Context<RegisterKey>,
        key: Vec<u8>,
        valid_from: u64,
        valid_to: u64,
    ) -> Result<()> {
        do_register_key(ctx, key, valid_from, valid_to)
    }

    pub fn revoke_key(ctx: Context<RevokeKey>, key: Vec<u8>) -> Result<()> {
        do_revoke_key(ctx, key)
    }

    pub fn blacklist_entity(
        ctx: Context<BlacklistEntity>,
        policy_id: u64,
        trading_address: Pubkey,
    ) -> Result<()> {
        do_blacklist_entity(ctx, policy_id, trading_address)
    }

    pub fn unblacklist_entity(
        ctx: Context<UnblacklistEntity>,
        policy_id: u64,
        trading_address: Pubkey,
    ) -> Result<()> {
        do_unblacklist_entity(ctx, policy_id, trading_address)
    }

    pub fn collect_fees(ctx: Context<CollectFees>) -> Result<()> {
        do_collect_fees(ctx)
    }

    pub fn create_credential(
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
        do_create_credential(
            ctx,
            key,
            policy_id,
            trading_address,
            signature,
            valid_from,
            valid_until,
            cost,
            backdoor,
        )
    }

    pub fn check_credential(
        ctx: Context<CheckCredential>,
        policy_id: u64,
        trading_address: Pubkey,
    ) -> Result<()> {
        do_check_credential(ctx, policy_id, trading_address)
    }
}
