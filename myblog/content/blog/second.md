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

transfer rules

minting and destruction

authorization and freezing

access control


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

### Why Solana uses one program for all tokens

### How Solana tracks user token balances