+++
title = "Token in Solana"
date = 2025-11-28
+++


## SPL Token

Solana Program Library Token(SPL Token) is Solana's standard for tokens, like ERC-20 and ERC-721.

### How Solana's token architecture differs from Ethereum

#### Ethereum

On Ethereum, each token is its own smart contract.

For example, every ERC-20 token deploys its own contract containing:

- Token logic (transfer, approve, mint, burn)
- Token state (balances, allowance, total supply)

This design is flexible, but it also means:

- Different tokens may behave differently
- Bugs or malicious logic can exist in individual token contracts

On Solana, *** Token Logic $\neq$ Token Data ***

- All tokens share the same Token Program
- Token logic lives in the SPL Token Program
- Token data lives in separate on-chain accounts

instead, a token is defined by data accounts, not custom program code.

### The three key accounts that make SPL tokens work


1. Token Program

Token program  contains all token logic, it owns all token-related state accounts, and it also defines and enforces: 

- transfer rules

- minting and destruction

- authorization and freezing

- access control


2. Mint Account

Each SPL token has exactly one mint account, it stores global token metadata:

- decimals
- supply 
- mint_authority
- freeze_authority

*** The mint address is the token address on Solana ***

Mint accounyt does not store user balances, they only define token rules.


3. Token Accounts / Associated Token Accounts (ATA)

Token accounts stores user balances, each token account contains:

- mint
- owner
- amount
- delegate
- state

`user_wallet_address + token_mint_address => associated_token_account_address`  

Why ATA?

Regular Token accounts often faces problems: a user can create multiple accounts for the same token, it is difficult for external parties to know which acocunt to transfer to. ATA solves this problem: each(user wallet, mint) corresponds to a unique ATA, the address is a deterministic PDA, all applications on the network can derive the same address.(ATA just like Ethereum `mapping(address => unit256) balanceOf`) 

### Why Solana uses one program for all tokens

Solana uses the same program (SPL Token Program) for all tokens because it adopts an architecture design of "stateless program, state in account". The specific state of the token (such as balance, minting authority, etc.) is placed in a separate account, while the common token logic is centralized in a highly auditable and reusable program. This reduces the cost of repeatedly deploying contracts, lowers security risks, and makes it easier to perform parallel execution at runtime, thereby improving overall performance and throughput.

### How Solana tracks user token balances

Ethereum:

- balance is stored internally within the token contract
- query using `balanceOf(user)`

Solana:

- each user's balance is stored in an independent ATA
- ATA address is derived from (wallet_address, mint_address)
- directly read account data

| Aspect             | Ethereum          | Solana         |
| ------------------ | ----------------- | -------------- |
| Balance Storage    | Contract mapping  | User ATA       |
| Who pays storage   | Contract deployer | User           |
| Lookup             | `balanceOf`       | Derive ATA     |
| Parallel execution | Limited           | Fully parallel |


This design allows Solana to maintain high through even under high concurrency.

``` 
PDA signing:
When our program wants to mint tokens, it uses CpiContext::new_with_signer and provides the exact PDA seeds (e.g. "token_mint" + bump).
During CPI, the Solana runtime re-derives the PDA using the currently executing program’s ID and the provided seeds.
If the derived address matches the account being used and the account is owned by the program, the runtime temporarily marks that PDA as a signer for this CPI call.
```