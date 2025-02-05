use crate::common::error::KeyringError;
use crate::common::types::{AuthMessage, AuthMessageV1};
use crate::MESSAGE_PREFIX;
use anchor_lang::prelude::Pubkey;
use anchor_lang::solana_program::keccak;
use anchor_lang::solana_program::keccak::Hash;
use anchor_lang::solana_program::secp256k1_recover::{
    secp256k1_recover, Secp256k1Pubkey, SECP256K1_PUBLIC_KEY_LENGTH, SECP256K1_SIGNATURE_LENGTH,
};
use anchor_lang::{error, AnchorSerialize};

fn split_signature(signature_data: Vec<u8>) -> anchor_lang::Result<(Vec<u8>, u8)> {
    if signature_data.len() != SECP256K1_SIGNATURE_LENGTH + 1 {
        return Err(error!(KeyringError::ErrInvalidSignatureLength));
    }

    let recovery_id = *signature_data
        .last()
        .expect("We already checked that the length is 65 above; qed");

    let signature = signature_data[0..64].to_vec();

    Ok((signature, recovery_id))
}

fn parse_publickey(key: Vec<u8>) -> anchor_lang::Result<Secp256k1Pubkey> {
    if key.len() != SECP256K1_PUBLIC_KEY_LENGTH {
        return Err(error!(KeyringError::ErrInvalidPubkeyLength));
    }

    Ok(Secp256k1Pubkey::new(key.as_slice()))
}

// Verify auth message
pub fn verify_auth_message(
    key: Vec<u8>,
    policy_id: u64,
    trading_address: Pubkey,
    signature_data: Vec<u8>,
    valid_from: u64,
    valid_until: u64,
    cost: u64,
    backdoor: Vec<u8>,
) -> anchor_lang::Result<bool> {
    // Pack auth message
    let provided_signer = parse_publickey(key)?;
    let message_hash = create_signature_payload(
        trading_address,
        policy_id,
        valid_from,
        valid_until,
        cost,
        backdoor,
    )?;
    let (signature, recovery_id) = split_signature(signature_data)?;
    let recovered_pubkey = secp256k1_recover(message_hash.as_ref(), recovery_id, &signature)
        .map_err(|_| error!(KeyringError::ErrInvalidSignature))?;

    Ok(recovered_pubkey.eq(&provided_signer))
}

pub fn create_signature_payload(
    trading_address: Pubkey,
    policy_id: u64,
    valid_from: u64,
    valid_until: u64,
    cost: u64,
    backdoor: Vec<u8>,
) -> anchor_lang::Result<Hash> {
    let packed_message = pack_auth_message(
        trading_address,
        policy_id,
        valid_from,
        valid_until,
        cost,
        backdoor,
    )?;
    Ok(keccak::hash(packed_message.as_slice()))
}

// Packs auth message data
fn pack_auth_message(
    trading_address: Pubkey,
    policy_id: u64,
    valid_from: u64,
    valid_until: u64,
    cost: u64,
    backdoor: Vec<u8>,
) -> anchor_lang::Result<Vec<u8>> {
    let mut serialized_auth_message = vec![];
    let mut buffer = vec![];
    let auth_message = AuthMessage::V1(AuthMessageV1 {
        trading_address,
        policy_id,
        valid_from,
        valid_until,
        cost,
        backdoor,
    });
    auth_message
        .serialize(&mut serialized_auth_message)
        .map_err(|_| error!(KeyringError::ErrUnableToPackAuthMessage))?;
    buffer.append(&mut MESSAGE_PREFIX.to_vec());
    buffer.append(&mut serialized_auth_message);
    Ok(buffer)
}
