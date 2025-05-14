use anchor_lang::account;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::keccak::{hash, Hash};

pub const CURRENT_VERSION: u8 = 1;

#[account]
pub struct ProgramState {
    pub version: u8,
    pub admin: Pubkey,
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

/// Max size of chain id as per CAIP2 spec
pub const CHAIN_ID_MAX_SIZE: usize = 41;
/// Min size of chain id as per CAIP2 spec
pub const CHAIN_ID_MIN_SIZE: usize = 5;

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
