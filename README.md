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
