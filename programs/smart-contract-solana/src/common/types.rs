use anchor_lang::account;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::blake3::{hash, Hash};

pub const CURRENT_VERSION: u8 = 1;

#[account]
pub struct ProgramState {
    pub version: u8,
    pub admin: Pubkey,
}

impl ProgramState {
    pub const MAX_SIZE: usize = 32 + 1;
}

#[account]
pub struct KeyEntry {
    pub version: u8,
    pub is_valid: bool,
    pub valid_from: u64,
    pub valid_to: u64,
}

impl KeyEntry {
    pub const MAX_SIZE: usize = 1 + 8 + 8 + 1;
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
    pub version: u8,
    pub blacklisted: bool,
    pub exp: u64,
}

impl EntityData {
    pub const MAX_SIZE: usize = 1 + 8 + 1;
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
