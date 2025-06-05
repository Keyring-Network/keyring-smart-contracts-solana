import * as anchor from "@coral-xyz/anchor";
import { Keccak } from "sha3";

const hash = new Keccak(256);

const getProgramStatePda = (programId: anchor.web3.PublicKey) =>
    anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("keyring_program"), Buffer.from("global_state")],
        programId
    )[0];

const getKeyRegistryPda = (programId: anchor.web3.PublicKey) =>
    anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("keyring_program"), Buffer.from("active_keys")],
        programId
    )[0];

const getKeyMappingPda = (
    key: Buffer<ArrayBuffer>,
    programId: anchor.web3.PublicKey
) =>
    anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("keyring_program"),
            Buffer.from("_key_mapping"),
            hash.update(key).digest(),
        ],
        programId
    )[0];

export { getProgramStatePda, getKeyRegistryPda, getKeyMappingPda };
