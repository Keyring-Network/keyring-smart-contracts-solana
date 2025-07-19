import * as anchor from "@coral-xyz/anchor";

import { setup } from "./utils/setup";
import { Config } from "./utils/types";
import {
    getBlacklistManagerRolePda,
    getEntityMappingPda,
} from "./utils/getPda";

async function manageBlacklist() {
    const config: Config = await setup();
    const policyIdString = process.env.POLICY_ID || "";
    const userString = process.env.USER || "";
    const blacklistString = process.env.BLACKLIST || "";

    if (
        policyIdString === "" ||
        userString === "" ||
        blacklistString === "" ||
        (blacklistString != "true" && blacklistString != "false")
    )
        throw new Error("Invalid params");

    const policyId = new anchor.BN(policyIdString);
    const user = new anchor.web3.PublicKey(userString);
    let blacklist = blacklistString === "true" ? true : false;

    console.log("Managing blacklist...");

    let txSignature: string;
    if (blacklist) {
        txSignature = await config.program.methods
            .blacklistEntity(policyId, user, blacklist)
            .accounts({
                blacklistManagerRole: getBlacklistManagerRolePda(
                    config.program.programId,
                    user
                ),
                entityMapping: getEntityMappingPda(
                    policyId,
                    user,
                    config.program.programId
                ),
                signer: config.provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .rpc();
    } else {
        txSignature = await config.program.methods
            .unblacklistEntity(policyId, user, blacklist)
            .accounts({
                blacklistManagerRole: getBlacklistManagerRolePda(
                    config.program.programId,
                    user
                ),
                entityMapping: getEntityMappingPda(
                    policyId,
                    user,
                    config.program.programId
                ),
                signer: config.provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .rpc();
    }

    console.log("Successfully managed blakclist");
    console.log("Transaction signature: ", txSignature);
}

manageBlacklist();
