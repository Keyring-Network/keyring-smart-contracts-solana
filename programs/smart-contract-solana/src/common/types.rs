use anchor_lang::account;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::blake3::{hash, Hash};

#[account]
pub struct ProgramState {
    pub admin: Pubkey,
}

impl ProgramState {
    pub const MAX_SIZE: usize = 32;
}

#[account]
pub struct KeyEntry {
    pub is_valid: bool,
    pub valid_from: u64,
    pub valid_to: u64,
}

impl KeyEntry {
    pub const MAX_SIZE: usize = 1 + 8 + 8;
}

pub trait ToHash {
    fn to_hash(&self) -> Hash;
}

impl ToHash for Vec<u8> {
    fn to_hash(&self) -> Hash {
        hash(self.as_slice())
    }
}

#[account]
#[derive(Debug, PartialEq)]
pub struct EntityData {
    pub blacklisted: bool,
    pub exp: u64,
}

impl EntityData {
    pub const MAX_SIZE: usize = 1 + 8;
}

#[derive(borsh::BorshSerialize)]
pub enum AuthMessage {
    V1(AuthMessageV1),
}

#[derive(borsh::BorshSerialize)]
pub struct AuthMessageV1 {
    pub trading_address: Pubkey,
    pub policy_id: u64,
    pub valid_from: u64,
    pub valid_until: u64,
    pub cost: u64,
    pub backdoor: Vec<u8>,
}
