# Programming Model 1
## Solana Handbook

- Introduction to Solana by Ackee Blockchain
[Solana Handbook](https://ackeeblockchain.com/solana-handbook.pdf)

## Command cheatsheet

### Solana CLI commands

- #### Get current config

    ```bash
    solana config get
    ```

- #### Set CLI config url to localhost cluster

    ```bash
    solana config set --url localhost # useful for local development
    solana config set -u l # shorter option, l stands for localhost
    ```
    ```bash
    solana config set --url devnet # useful for devnet testing
    solana config set -u d # shorter option, d stands for devnet
    ```
    More at [Clusters and Public RPC Endpoints](https://solana.com/docs/core/clusters)


- #### Create CLI Keypair
    ```bash
    solana-keygen new -o test.json
    ```
- #### Airdrop
    > As you may guess, Airdrop will only work on Devnet, Testnet or Localhost. No you cannot airdrop SOL on Mainnet!!
    ```bash
    solana airdrop 5
    ```
    > You can also optionally specify the destination address of the airdrop
    ```bash
    solana airdrop 5 <YOUR_PUBKEY>
    ```
    > You can also use the [Solana Faucet](https://faucet.solana.com/) to get some SOL.

- #### Get PubKey from Keypair
    ```bash
    solana-keygen pubkey ~/my-solana-wallet/my-keypair.json
    ```
- #### Run Solana test validator
    > In **most cases (99%)** you **DO NOT NEED TO** start the local validator by yourself. **Down below** you can find the **Anchor commands** which will handle everything for you.
    ```bash
    solana-test-validator
    ```
- #### Get logs from the Solana validator
    ```bash
    solana logs
    ```

### Anchor commands
- #### Initialize new project
    ```bash
    anchor init <your_project_name>
    ```
- #### Build the project
    ```bash
    anchor build
    ```
- #### Test the project (preferred)
    ```bash
    anchor test
    ```
- #### Test the project (less preferred)
    In separate window, call:
    ```bash
    solana-test-validator
    ```
    Within the anchor project directory
    - Build the project
        ```bash
        anchor build
        ```
    - Run Tests without starting the local validator (as you started it manually in the step above)
        ```bash
        anchor test --skip-local-validator
        ```
