// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.

const anchor = require("@coral-xyz/anchor");
import { SmartContractSolana } from "../target/types/smart_contract_solana";
import {
  Program,
  setProvider,
  AnchorProvider,
  BN,
  workspace,
  web3,
} from "@coral-xyz/anchor";

module.exports = async function (provider) {
  // Configure client to use the provider.
  anchor.setProvider(provider);

  // Add your deploy script here.
  const program =
      workspace.SmartContractSolana as Program<SmartContractSolana>;

  const [programStatePubkey] = await web3.PublicKey.findProgramAddressSync(
      [Buffer.from("keyring_program"), Buffer.from("global_state")],
      program.programId
  );

  const txHash = await program.methods
      .initialize()
      .accounts({})
      .rpc({ commitment: "finalized" });

  console.log("Initialize tx sent. Hash: ", txHash);

  const programStateAccount = await program.account.programState.fetch(programStatePubkey);
  console.log("Program state is: ", programStateAccount);
  console.log("Admin is:", programStateAccount.admin.toString());
};
