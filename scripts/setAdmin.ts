import * as anchor from "@coral-xyz/anchor";

import { setup } from "./utils/setup";
import { Config } from "./utils/types";
import { getProgramStatePda } from "./utils/getPda";

async function setAdmin() {
    const config: Config = await setup();
    const newAdminString = process.env.NEW_ADMIN || "";

    if (newAdminString === "") throw new Error("Invalid admin");

    const newAdmin = new anchor.web3.PublicKey(newAdminString);

    console.log("Setting new admin...");

    const txSignature = await config.program.methods
        .setAdmin(newAdmin)
        .accounts({
            programState: getProgramStatePda(config.program.programId),
            signer: config.provider.wallet.publicKey,
        })
        .rpc();

    console.log("Successfully set new admin");
    console.log("Transaction signature: ", txSignature);
}

setAdmin().catch((err) =>
    console.log("Error setting new admin: ", (err as Error).message)
);
