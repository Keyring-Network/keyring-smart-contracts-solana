import * as anchor from "@coral-xyz/anchor";
import * as idl from "../../target/idl/keyring_network.json";
import { KeyringNetwork } from "../../target/types/keyring_network";

import { Config } from "./types";
import {
    getDefaultAdminRolePda,
    getKeyManagerRolePda,
    getKeyMappingPda,
    getKeyRegistryPda,
    getProgramStatePda,
} from "./getPda";

async function setup(): Promise<Config> {
    const stringKeypair = process.env.KEYPAIR || "";
    const stringProgramId = process.env.PROGRAM_ID || "";
    const rpcUrl = process.env.RPC_URL || "";

    if (stringKeypair === "" || stringProgramId === "" || rpcUrl === "")
        throw new Error("Invalid config");

    const connection = new anchor.web3.Connection(rpcUrl, "confirmed");
    const keypairArray: number[] = JSON.parse(stringKeypair);
    const keypairUint8Array = new Uint8Array(keypairArray);
    const keypair = anchor.web3.Keypair.fromSecretKey(keypairUint8Array);
    const wallet = new anchor.Wallet(keypair);
    const programId = new anchor.web3.PublicKey(stringProgramId);
    const provider = new anchor.AnchorProvider(connection, wallet, {
        commitment: "confirmed",
    });
    const program = new anchor.Program<KeyringNetwork>(
        idl as any,
        programId,
        provider
    );

    const config: Config = {
        provider,
        program,
    };

    const p = anchor.workspace as anchor.Program<KeyringNetwork>;
    const key1 = Buffer.from([22]);
    p.methods
        .revokeKey(key1)
        .accounts({
            keyRegistry: getKeyRegistryPda(config.program.programId),
            signer: config.provider.wallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            keyManagerRole: getKeyManagerRolePda(
                config.program.programId,
                config.provider.wallet.publicKey
            ),
            keyMapping: getKeyMappingPda(key1, config.program.programId),
        })
        .rpc();
    p.methods
        .registerKey(key1, new anchor.BN(1), new anchor.BN(1))
        .accounts({
            keyRegistry: getKeyRegistryPda(config.program.programId),
            signer: config.provider.wallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            keyManagerRole: getKeyManagerRolePda(
                config.program.programId,
                config.provider.wallet.publicKey
            ),
            keyMapping: getKeyMappingPda(key1, config.program.programId),
        })
        .rpc();

    return config;
}

export { setup };
