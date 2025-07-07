use anchor_lang::account;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    keccak::{hash, Hash},
    secp256k1_recover::SECP256K1_PUBLIC_KEY_LENGTH,
};

pub const CURRENT_VERSION: u8 = 1;
pub const DEFAULT_ADMIN_ROLE: [u8; 32] = [0; 32];
pub const KEY_MANAGER_ROLE: [u8; 32] = [
    27, 30, 232, 100, 197, 54, 57, 215, 70, 43, 119, 63, 124, 139, 76, 234, 20, 166, 174, 54, 21,
    116, 59, 147, 115, 163, 165, 135, 151, 112, 113, 28,
];
pub const BLACKLIST_MANAGER_ROLE: [u8; 32] = [
    249, 136, 228, 251, 98, 184, 225, 79, 72, 32, 254, 208, 49, 146, 48, 109, 223, 77, 125, 191,
    162, 21, 89, 91, 161, 198, 186, 75, 118, 179, 105, 238,
];
pub const OPERATOR_ROLE: [u8; 32] = [
    151, 102, 112, 112, 197, 78, 241, 130, 176, 245, 133, 139, 3, 75, 234, 193, 182, 243, 8, 154,
    162, 211, 24, 139, 177, 232, 146, 159, 79, 169, 185, 41,
];

#[account]
pub struct ProgramState {
    pub version: u8,
    pub chain_id: ChainId,
}

impl ProgramState {
    pub const MAX_SIZE: usize = 32 + 41 + 1;
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

pub const MAX_ACTIVE_KEYS: u8 = 10;

#[account]
#[derive(Debug, PartialEq)]
pub struct KeyRegistry {
    pub active_keys: Vec<Vec<u8>>,
}

impl KeyRegistry {
    pub const MAX_SIZE: usize = SECP256K1_PUBLIC_KEY_LENGTH * MAX_ACTIVE_KEYS as usize;
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

#[account]
#[derive(Debug, PartialEq)]
pub struct Role {
    pub has_role: bool,
}

impl Role {
    pub const MAX_SIZE: usize = 8;
}

pub const CHAIN_ID_MAX_SIZE: usize = 41;
pub const CHAIN_ID_MIN_SIZE: usize = 4;

#[derive(Debug, PartialEq)]
pub enum ChainIdConversionError {
    InputExceedsMaxSize { expected: usize, actual: usize },
    InputLessThanMinSize { expected: usize, actual: usize },
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct ChainId {
    pub chain_id: [u8; CHAIN_ID_MAX_SIZE],
}

impl ChainId {
    pub fn new(chain_id: Vec<u8>) -> std::result::Result<ChainId, ChainIdConversionError> {
        if chain_id.len() > CHAIN_ID_MAX_SIZE {
            return Err(ChainIdConversionError::InputExceedsMaxSize {
                expected: CHAIN_ID_MAX_SIZE,
                actual: chain_id.len(),
            });
        }

        if chain_id.len() < CHAIN_ID_MIN_SIZE {
            return Err(ChainIdConversionError::InputLessThanMinSize {
                expected: CHAIN_ID_MIN_SIZE,
                actual: chain_id.len(),
            });
        }

        let mut constructed_chain_id = [0; CHAIN_ID_MAX_SIZE];
        for (i, elem) in chain_id.iter().enumerate() {
            constructed_chain_id[i] = *elem;
        }

        Ok(ChainId {
            chain_id: constructed_chain_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::common::types::{
        ChainId, ChainIdConversionError, CHAIN_ID_MAX_SIZE, CHAIN_ID_MIN_SIZE,
    };

    #[test]
    fn test_chain_id() {
        assert!(ChainId::new(vec![1u8; 32]).is_ok());
        assert_eq!(
            ChainId::new(vec![1u8; 45]),
            Err(ChainIdConversionError::InputExceedsMaxSize {
                expected: CHAIN_ID_MAX_SIZE,
                actual: 45
            })
        );
        assert_eq!(
            ChainId::new(vec![1u8; 4]),
            Err(ChainIdConversionError::InputLessThanMinSize {
                expected: CHAIN_ID_MIN_SIZE,
                actual: 4
            })
        );
    }
}
