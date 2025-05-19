use crate::common::error::KeyringError;
use crate::common::types::ChainId;
use anchor_lang::solana_program::keccak;
use anchor_lang::solana_program::keccak::Hash;
use anchor_lang::solana_program::secp256k1_recover::{
    secp256k1_recover, Secp256k1Pubkey, SECP256K1_PUBLIC_KEY_LENGTH, SECP256K1_SIGNATURE_LENGTH,
};
use anchor_lang::{error, Result};

pub const ETH_SIGNED_MESSAGE_PREFIX: &[u8] = b"\x19Ethereum Signed Message:\n32";

pub fn split_signature(signature_data: Vec<u8>) -> Result<(Vec<u8>, u8)> {
    if signature_data.len() != SECP256K1_SIGNATURE_LENGTH + 1 {
        return Err(error!(KeyringError::ErrInvalidSignatureLength));
    }

    let mut recovery_id = *signature_data
        .last()
        .expect("We already checked that the length is 65 above; qed");

    // We expect recovery id similar to ethereum rpc
    if recovery_id >= 27 && recovery_id < 27 + 4 {
        recovery_id = recovery_id - 27;
    } else {
        return Err(error!(KeyringError::ErrInvalidRecoveryID));
    }

    let signature = signature_data[0..64].to_vec();

    Ok((signature, recovery_id))
}

fn parse_publickey(key: Vec<u8>) -> Result<Secp256k1Pubkey> {
    if key.len() != SECP256K1_PUBLIC_KEY_LENGTH {
        return Err(error!(KeyringError::ErrInvalidPubkeyLength));
    }

    Ok(Secp256k1Pubkey::new(key.as_slice()))
}

// Verify auth message
pub fn verify_auth_message(
    key: Vec<u8>,
    policy_id: u64,
    trading_address: Vec<u8>,
    signature_data: Vec<u8>,
    chain_id: ChainId,
    valid_until: u64,
    cost: u64,
    backdoor: Vec<u8>,
) -> Result<bool> {
    // Pack auth message
    let provided_signer = parse_publickey(key)?;
    let message_hash = create_signature_payload(
        trading_address,
        policy_id,
        chain_id,
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
    trading_address: Vec<u8>,
    policy_id: u64,
    chain_id: ChainId,
    valid_until: u64,
    cost: u64,
    backdoor: Vec<u8>,
) -> Result<Hash> {
    let packed_message = pack_auth_message(
        trading_address,
        policy_id,
        chain_id,
        valid_until,
        cost,
        backdoor,
    )?;

    let message_hash = keccak::hash(packed_message.as_slice());
    let eth_signed_message_hash = convert_to_eth_signed_message_hash(message_hash);

    Ok(eth_signed_message_hash)
}

pub fn convert_to_eth_signed_message_hash(message_hash: Hash) -> Hash {
    let mut buffer = vec![];
    buffer.extend_from_slice(&ETH_SIGNED_MESSAGE_PREFIX);
    buffer.extend_from_slice(message_hash.as_ref());
    keccak::hash(buffer.as_slice())
}

// Packs auth message data
// This function mimics the exact same behaviour as this from solditiy:
// return abi.encodePacked(
//    tradingAddress,
//    uint8(0),
//    uint24(policyId),
//    uint32(validFrom),
//    uint32(validUntil),
//    uint160(cost),
//    backdoor
// );
// See full code here: https://github.com/Keyring-Network/keyring-smart-contracts/blob/master/src/lib/RsaMessagePacking.sol#L18
pub fn pack_auth_message(
    trading_address: Vec<u8>,
    policy_id: u64,
    chain_id: ChainId,
    valid_until: u64,
    cost: u64,
    backdoor: Vec<u8>,
) -> Result<Vec<u8>> {
    let mut packed = vec![];

    let reserved_byte = 0u8;

    if policy_id > 2u64.pow(24) - 1 {
        return Err(error!(KeyringError::ErrAuthMessageParameterOutOfRange));
    }
    let policy_id_in_bytes = policy_id.to_be_bytes();
    let encoded_policy_id =
        policy_id_in_bytes[policy_id_in_bytes.len() - 3..policy_id_in_bytes.len()].to_vec();

    if valid_until > u32::MAX as u64 {
        return Err(error!(KeyringError::ErrAuthMessageParameterOutOfRange));
    }
    let encoded_valid_until = (valid_until as u32).to_be_bytes().to_vec();
    let encoded_cost = (cost as u128).to_be_bytes().to_vec();

    packed.extend_from_slice(&trading_address.as_slice());
    packed.push(reserved_byte);
    packed.extend_from_slice(&encoded_policy_id.as_slice());
    packed.extend_from_slice(&chain_id.chain_id[0..4]);
    packed.extend_from_slice(&encoded_valid_until.as_slice());
    packed.extend_from_slice(vec![0u8; 4].as_slice());
    packed.extend_from_slice(&encoded_cost.as_slice());
    packed.extend_from_slice(backdoor.as_slice());

    Ok(packed)
}
