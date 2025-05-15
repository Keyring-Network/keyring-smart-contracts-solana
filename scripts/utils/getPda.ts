import * as anchor from "@coral-xyz/anchor";

const getProgramStatePda = (programId: anchor.web3.PublicKey) =>
    anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("keyring_program"), Buffer.from("global_state")],
        programId
    )[0];

export { getProgramStatePda };
