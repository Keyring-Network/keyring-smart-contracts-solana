# Description
This solana program is adaption of [solidity smart contracts](https://github.com/Keyring-Network/smart-contracts) and is base on Anchor framework.

# Installation
Install solana and anchor cli from the [official website](https://www.anchor-lang.com/docs/installation).
Ensure to install solana using the official documentation. The homebrew installation is not recommended and can cause an issue relating to `cargo-build-sbf` missing.

# Tests

## Setup
You will need a solana keypair to run tests.

```shell
solana-keygen new
```

## Run tests
```shell
./run-tests.sh
```

# Deployment
```shell
# Make sure that you have chosen correct network and your solana keypair is up-to-date.
anchor deploy

# Wait for a bit for the deployment to be recognized and run migration to initialize the account
anchor migrate
```

# Upgrade authority
Typically, the keypair that is deploying the contract is upgrade authority which can update the program. 
Each structure in program has version byte which can be used in subsequent upgrade to provide backward compatibility and/or migration.

# Deployment to Mainnet

## Prerequisites
```bash
# Build the verifiable program
anchor build --verifiable

# Verify the program matches what's deployed
solana program dump <PROGRAM_ID> deployed-program.so
sha256sum deployed-program.so target/verifiable/smart_contract_solana.so
```
Create a deployment keypair at `./deploy-keypair.json`
Fund the deployment account (needs ~8 SOL)

## Initial Deployment

Deploy the program
```bash
anchor deploy
```
Initialize the program state (only needed for first deployment)
```bash
anchor migrate
```

## Upgrading Existing Program
```bash
# Build verifiable version
anchor build --verifiable
```

## Error Handling

If deployment fails with buffer account error, recover the buffer
```bash
solana-keygen recover -o buffer-keypair.json --force
```
Enter the seed phrase from the error message.

Close failed buffer to reclaim rent:
```bash
solana program close $(solana program show --buffers --keypair ./deploy-keypair.json | awk 'NR==3 {print $1}') \
  --keypair ./deploy-keypair.json \
  --recipient $(solana-keygen pubkey ./deploy-keypair.json)
```

Deploy upgrade with longer timeout and finalized commitment
```bash
solana program deploy \
  --program-id <PROGRAM_ID> \
  --keypair ./deploy-keypair.json \
  target/verifiable/smart_contract_solana.so \
  --commitment finalized
```

## Gotchas
- Deployment requires ~8 SOL for rent and transaction fees
- Failed deployments create buffer accounts that need to be closed to reclaim rent
- Use `--commitment finalized` to avoid timeout errors on mainnet
- Always build with `--verifiable` flag for mainnet deployments
- Keep deployment keypair secure as it has upgrade authority
- Program state is preserved across upgrades
