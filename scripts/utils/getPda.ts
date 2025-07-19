import * as anchor from "@coral-xyz/anchor";
import { Keccak } from "sha3";

import {
    DEFAULT_ADMIN_ROLE,
    KEY_MANAGER_ROLE,
    BLACKLIST_MANAGER_ROLE,
    OPERATOR_ROLE,
} from "./constants";

const hash = new Keccak(256);

const getProgramStatePda = (programId: anchor.web3.PublicKey) =>
    anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("keyring_program"), Buffer.from("global_state")],
        programId
    )[0];

const getDefaultAdminRolePda = (
    programId: anchor.web3.PublicKey,
    user: anchor.web3.PublicKey
) =>
    anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(DEFAULT_ADMIN_ROLE), user.toBuffer()],
        programId
    )[0];

const getKeyManagerRolePda = (
    programId: anchor.web3.PublicKey,
    user: anchor.web3.PublicKey
) =>
    anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(KEY_MANAGER_ROLE), user.toBuffer()],
        programId
    )[0];

const getBlacklistManagerRolePda = (
    programId: anchor.web3.PublicKey,
    user: anchor.web3.PublicKey
) =>
    anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(BLACKLIST_MANAGER_ROLE), user.toBuffer()],
        programId
    )[0];

const getOperatorRolePda = (
    programId: anchor.web3.PublicKey,
    user: anchor.web3.PublicKey
) =>
    anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(OPERATOR_ROLE), user.toBuffer()],
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

const getEntityMappingPda = (
    policyId: anchor.BN,
    user: anchor.web3.PublicKey,
    programId: anchor.web3.PublicKey
) =>
    anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("keyring_program"),
            Buffer.from("_entity_mapping"),
            policyId.toBuffer("le"),
            user.toBuffer(),
        ],
        programId
    )[0];

export {
    getProgramStatePda,
    getDefaultAdminRolePda,
    getKeyManagerRolePda,
    getBlacklistManagerRolePda,
    getOperatorRolePda,
    getKeyRegistryPda,
    getKeyMappingPda,
    getEntityMappingPda,
};
