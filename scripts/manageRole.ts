import * as anchor from "@coral-xyz/anchor";

import { setup } from "./utils/setup";
import { Config } from "./utils/types";
import {
    BLACKLIST_MANAGER_ROLE,
    DEFAULT_ADMIN_ROLE,
    KEY_MANAGER_ROLE,
    OPERATOR_ROLE,
} from "./utils/constants";
import {
    getBlacklistManagerRolePda,
    getDefaultAdminRolePda,
    getKeyManagerRolePda,
    getOperatorRolePda,
} from "./utils/getPda";

async function manageRole() {
    const config: Config = await setup();
    const userString = process.env.USER || "";
    const roleString = process.env.ROLE || "";
    const hasRoleString = process.env.HAS_ROLE || "";

    if (
        userString === "" ||
        roleString === "" ||
        hasRoleString === "" ||
        (hasRoleString != "true" && hasRoleString != "false")
    )
        throw new Error("Invalid params");

    const user = new anchor.web3.PublicKey(userString);
    let role: Buffer<ArrayBufferLike>;
    let roleAccountPda: anchor.web3.PublicKey;
    let hasRole = hasRoleString === "true" ? true : false;

    switch (roleString) {
        case "DEFAULT_ADMIN_ROLE":
            role = Buffer.from(DEFAULT_ADMIN_ROLE);
            roleAccountPda = getDefaultAdminRolePda(
                config.program.programId,
                user
            );
            break;
        case "KEY_MANAGER_ROLE":
            role = Buffer.from(KEY_MANAGER_ROLE);
            roleAccountPda = getKeyManagerRolePda(
                config.program.programId,
                user
            );
            break;
        case "BLACKLIST_MANAGER_ROLE":
            role = Buffer.from(BLACKLIST_MANAGER_ROLE);
            roleAccountPda = getBlacklistManagerRolePda(
                config.program.programId,
                user
            );
            break;
        case "OPERATOR_ROLE":
            role = Buffer.from(OPERATOR_ROLE);
            roleAccountPda = getOperatorRolePda(config.program.programId, user);
            break;
        default:
            throw new Error("Invalid role");
    }

    console.log("Managing role...");

    const txSignature = await config.program.methods
        .manageRoles(role, user, hasRole)
        .accounts({
            defaultAdminRole: getDefaultAdminRolePda(
                config.program.programId,
                config.provider.wallet.publicKey
            ),
            role: roleAccountPda,
            signer: config.provider.wallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

    console.log("Successfully managed role");
    console.log("Transaction signature: ", txSignature);
}

manageRole();
