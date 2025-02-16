// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.

const anchor = require("@coral-xyz/anchor");
import { KeyringNetwork } from "../target/types/keyring_network";
import {
  Program,
  setProvider,
  AnchorProvider,
  BN,
  workspace,
  web3,
} from "@coral-xyz/anchor";

module.exports = async function (provider) {
    console.log("Starting deployment...");
    
    // Configure client to use the provider.
    anchor.setProvider(provider);
    console.log("Provider configured");
    
    const program = workspace.KeyringNetwork as Program<KeyringNetwork>;
    console.log("Program ID:", program.programId.toString());
    console.log("Program loaded");

    const [programStatePubkey] = await web3.PublicKey.findProgramAddressSync(
        [Buffer.from("keyring_program"), Buffer.from("global_state")],
        program.programId
    );
    console.log("PDA derived:", programStatePubkey.toString());

    try {
        console.log("Sending initialize transaction...");
        const txHash = await program.methods
        .initialize()
        .accounts({})
        .rpc({ commitment: "finalized" });
        console.log("Initialize tx sent. Hash: ", txHash);
        
        console.log("Fetching program state...");
        const programStateAccount = await program.account.programState.fetch(programStatePubkey);
        console.log("Program state is: ", programStateAccount);
        console.log("Admin is:", programStateAccount.admin.toString());
    } catch (e) {
        console.error("Error details:", e);
        throw e;
    }
};
