import * as anchor from "@coral-xyz/anchor";

import { setup } from "./utils/setup";
import { Config } from "./utils/types";
import { getProgramStatePda } from "./utils/getPda";

async function initialize() {
    const config: Config = await setup();
    const solanaChainId = 1915121141;
    const bufferSolanaChainId = Buffer.alloc(4);
    bufferSolanaChainId.writeUInt32BE(solanaChainId, 0);

    console.log("Initializing program...");

    const txSignature = await config.program.methods
        .initialize(bufferSolanaChainId)
        .accounts({
            programState: getProgramStatePda(config.program.programId),
            signer: config.provider.wallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

    console.log("Successfully initialized program");
    console.log("Transaction signature: ", txSignature);
}

initialize().catch((err) =>
    console.log("Error initializing the program: ", (err as Error).message)
);
