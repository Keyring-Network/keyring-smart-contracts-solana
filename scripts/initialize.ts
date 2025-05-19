import * as anchor from "@coral-xyz/anchor";

import { setup } from "./utils/setup";
import { Config } from "./utils/types";
import { getProgramStatePda } from "./utils/getPda";

async function initialize() {
    const config: Config = await setup();

    const solanaChainId = 1915121141;
    const bufferSolanaChainId = new anchor.BN(solanaChainId).toBuffer("be", 4);

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

initialize();
