import * as anchor from "@coral-xyz/anchor";

import { setup } from "./utils/setup";
import { Config } from "./utils/types";
import { getProgramStatePda, getKeyMappingPda } from "./utils/getPda";

async function registerKey() {
    const config: Config = await setup();

    const keyString = process.env.KEY || "";
    const validFrom = process.env.VALID_FROM || "";
    const validUntil = process.env.VALID_UNTIL || "";

    if (keyString === "" || validFrom === "" || validUntil === "")
        throw new Error("Invalid key registration params");

    console.log("Registering key...");

    const keyArray: number[] = JSON.parse(keyString);
    const key = Buffer.from(keyArray);

    const txSignature = await config.program.methods
        .registerKey(key, new anchor.BN(validFrom), new anchor.BN(validUntil))
        .accounts({
            programState: getProgramStatePda(config.program.programId),
            signer: config.provider.wallet.publicKey,
            keyMapping: getKeyMappingPda(key, config.program.programId),
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

    console.log("Successfully registered key");
    console.log("Transaction signature: ", txSignature);
}

registerKey().catch((err) =>
    console.log("Error registering the key: ", (err as Error).message)
);
