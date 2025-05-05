use anchor_lang::error_code;

#[error_code]
pub enum KeyringError {
    #[msg("The caller is not administrator")]
    ErrCallerNotAdmin,
    #[msg("Invalid parameters passed in key registration")]
    ErrInvalidKeyRegistrationParams,
    #[msg("Invalid credentials passed")]
    ErrInvalidCredentials,
    #[msg("Unable to pack auth message")]
    ErrUnableToPackAuthMessage,
    #[msg("Invalid signature length")]
    ErrInvalidSignatureLength,
    #[msg("Invalid recovery id")]
    ErrInvalidRecoveryID,
    #[msg("Invalid pubkey length")]
    ErrInvalidPubkeyLength,
    #[msg("Invalid signature")]
    ErrInvalidSignature,
    #[msg("Key is already registered")]
    ErrKeyAlreadyRegistered,
    #[msg("Cost parameter is zero")]
    ErrCostParameterZero,
    #[msg("error while checking credentials")]
    ErrInCheckingCredentials,
    #[msg("Error auth message parameter out of range")]
    ErrAuthMessageParameterOutOfRange,
    #[msg("Invalid chain id")]
    ErrInvalidChainId,
}
