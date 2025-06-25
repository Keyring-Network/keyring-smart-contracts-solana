import * as anchor from "@coral-xyz/anchor";

import { setup } from "./utils/setup";
import { Config } from "./utils/types";
import {
    getProgramStatePda,
    getKeyMappingPda,
    getKeyRegistryPda,
    getKeyManagerRolePda,
} from "./utils/getPda";

async function rotateKey() {
    const config: Config = await setup();

    const previousKeyString = process.env.PREVIOUS_KEY || "";
    const keyString = process.env.KEY || "";
    const validFrom = process.env.VALID_FROM || "";
    const validUntil = process.env.VALID_UNTIL || "";

    if (keyString === "" || validFrom === "" || validUntil === "")
        throw new Error("Invalid key rotation params");

    console.log("Rotating key...");

    const revokeKey = previousKeyString === "" ? false : true;
    let previousKeyArray: number[];
    let previousKey: Buffer<ArrayBufferLike>;
    if (revokeKey) {
        previousKeyArray = JSON.parse(previousKeyString);
        previousKey = Buffer.from(previousKeyArray);
    }
    const keyArray: number[] = JSON.parse(keyString);
    const key = Buffer.from(keyArray);

    let txSignature1: string;
    if (revokeKey) {
        txSignature1 = await config.program.methods
            .revokeKey(previousKey)
            .accounts({
                keyRegistry: getKeyRegistryPda(config.program.programId),
                signer: config.provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
                keyManagerRole: getKeyManagerRolePda(
                    config.program.programId,
                    config.provider.wallet.publicKey
                ),
                keyMapping: getKeyMappingPda(
                    previousKey,
                    config.program.programId
                ),
            })
            .rpc();
    }
    const txSignature2 = config.program.methods
        .registerKey(key, new anchor.BN(validFrom), new anchor.BN(validUntil))
        .accounts({
            keyRegistry: getKeyRegistryPda(config.program.programId),
            signer: config.provider.wallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            keyManagerRole: getKeyManagerRolePda(
                config.program.programId,
                config.provider.wallet.publicKey
            ),
            keyMapping: getKeyMappingPda(key, config.program.programId),
        })
        .rpc();

    console.log("Successfully rotated key");
    if (revokeKey) {
        console.log(
            "Key rotation transaction signatures: ",
            txSignature1,
            txSignature2
        );
    } else {
        console.log("Register key transaction signature: ", txSignature2);
    }
}

rotateKey();
