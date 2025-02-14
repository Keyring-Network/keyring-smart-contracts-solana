use anchor_lang::account;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::keccak::{hash, Hash};

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
