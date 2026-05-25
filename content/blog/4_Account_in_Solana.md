+++
title = "Account in solana"
date = 2026-02-12

[extra]
cover_image = "/covers/forth.jpeg"
cover_sentence = " "
tags = ["Solana", "Account", "Note"]
draft = false
+++



## 0x01

In Solidity and Ethereum, SSTORE2 or SSTORE3 can store data in another smart contract, but in Solana, if we are the original deployer and the Solana program is not marked as immutable, we can update the bytecode at any time.

```rust
{
    key: 5JaQgKk2ggYYNHyyAyNSBCNPdeytNPAFhFoJHy1gwiNj
    value: {
        data: ...
    }
}
```

Solana's model is similar to Ethereum, a key-value store.(be up to 10MB)

![](/covers/4/1.png)



In Solana(Anchor), all storage(account data), is treared as a struct when we try to read or write data, we should try to interpret these data. In Anchor, it will derserialize and serialize account data into structs.So we need to initialize the Solana account before we can use it.

address of initialized accounts in Solana depond on the program that owns the storage account, basic_storage (which is akin to the address of the deploying contract) and the seeds (which is akin to create2’s “salt”)

note: in Anchor, it silently converts rust snake case to typescript camel case.

## 0x02 

In solana it has "rent", nowadays all accounts are required to be paid than 2 years of rent. 

And accounts with zero data are not free because Solana should create index and store metadata about them.

We can use "solana rent `<number of bytes>` to calculate.

```rust
minimum_balance: (account_size + 128) * 3,480 lamports/byte - year * 2 years
```

[a funny post about solana rent](https://www.reddit.com/r/solana/comments/qwin1h/my_sol_balance_in_the_wallet_is_decreasing/?rdt=33377)

## 0x03

UncheckedAccount in Anchor

the type to tell Anchor do not check the account when this account being read, and this program doesn't have `Context`

**"If a malicious user crafts an account the program did not create and then passes it to the Solana program, and the Solana program blindly trusts the data in the account, critical errors may occur."**

for example:

a bank contract struct:

```rust

pub struct BankAccount {
  pub owner: Pubkey,
  pub balance: u64,
}

...

pub fn withdraw(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  amount: u64,
) -> -> ProgramResult {
    let bank_account = &accounts[0];

    // directly deserialize the data
    let mut data = bank_account.try_borrow_mut_data()?;
    let bank_data: &mut BankAccount =
        unsafe { &mut *(data.as_mut_ptr() as *mut BankAccount) };

    // trust the balance
    if bank_data.balance < amount {
        return Err(ProgramError::InsufficientFunds);
    }

    bank_data.balance -= amount;

    Ok(())
}
```

and we can create a fake account in another trade and write:

```rust
  owner = fake pubkey
  balance = 1_000_000_000
```
and send this account to withdraw:

```rust
  
  withdraw(
    fake_bank_account,
    amount = 100_000
  )

```

the check will pass

but in Anchor context:

```rust

#[derive(Accounts)]

  pub struct Withdraw<'info> {
    #[account(mut)]
    pub bank_account: Account<'info, BankAccount>,
  }

```

in Anchor it will check `account owner` and account data(8-byte),and will check account size:

```rust
  
  data.len() == 8 + size_of::<BankAccount>()
```

so in Anchor: `Account<T>` mandatory account soverrignty and structure verification.

## 0x04

Reentrancy attack in Solana:

In Solana, reentrancy is not about repeatedly entering a function, but rather about "the utilization of the observability of intermediate states across the program."

CPI logic reentrancy
Cross-Program Invocation (CPI) is the inter-program invocation mechanism in Solana. It's used for system instruction calls, SPL token transfers, custom program execution, and even event emission, making it a core part of writing functional programs in Solana. Its permission model and invocation capabilities differ significantly from EVM.

A simplified flowchat:

User
    -> call program A
                      -> CPI program B
                                       -> CPI program A //reentrancy attack

If program A updates status before CPI and continue depend on the status after CPI, it will be probably exploited.

```rust

  pub fn withdraw(ctx: Context<Withdraw>, amount: u64) {
    
    require!(vault.balance >= amount);

    token::transfer(...)?; //CPI transfer

    vault.balance -= amount;
  }

```

Read-only reentrancy
When CPI callback. it does not modify account informaition, but it reads "intermediate" data.
In Solana, we can not use reentrancy locks to defend read-only reentrancy beacause we can not stop others to read your account. 

```rust

pub fn price(ctx: Context<Price>) -> u64 {
  ctx.accoounts.pool.virtual_price
}

```
A simplified flowchat:

program A: update_price()
          -> CPI program B // B read A pool.virtual_price(intermediate data)

## 0x05

PDA(Program Derived Address)

PDA is also an account created by address from the program and the seeds from the `init` transaction.

`PDA = (seeds + Program ID) derive`

seeds: static(eg."hello_world") or dynamic("user's public key")
bump: a number between 0 and 255 to ensure that the gernerated PDA isn't on the Ed25519(so can't find the private key and sign it)

```rust
#[derive(Accounts)]
pub struct InitializePDA<'info> { 
  #[account(init,
            payer = size_of::<PDA>() + 8,
            seeds = [],
            bump)]

  pub pda: Account<'info, PDA>
}
```
usage: Deterministic Address Generation: Generates a unique and predictable account address for each user.
       
       Cross-Program Permissions: Allows one program to sign and operate accounts on behalf of another program.
       
       Data Association: Organizes related data together, such as user A's position data in project B.
       
       Eliminates Transaction Signatures: Users only need to authorize the program once; subsequent operations can be completed by the program via PDA.

Keypair Account

no seeds and bump


## 0x06

Owner and Authority

In solana, only programs can write data to accounts, and more specifically, a porgram can only write to accounts that it owns.
account -> owner is a program

But program do not spontaneously modify accounts, they must be invoked via an instruction sent by a wallet.
wallet -> authority
Most program will only accept write instructions if the transaction is signed by a privileged wallet, known as the authority.

*** Only the owner of an account can modify the data in it *** 

···
An authority is an address from which a program will accept instructions if it sees a valid signature. An authority cannot modify an account directly. It needs to work through a program that owns the account it is trying to modify.
···

When we start to learn solana and we use `solana account <address>`, we will notice something surprising:

The owner is 11111111111111111111111111111111

That address corresponds to the System Program.

This is intentional.

Only the owner of an account can modify its data.
If wallets owned themselves, users could arbitrarily change balances.

Instead:

You sign a transaction

The System Program verifies your signature

The System Program updates the balance on your behalf

## 0x07

`#[derive(Accounts)]` and account types

In solana Anchor, `#[derive(Accounts)]`is an attribute-like macro applied to a rust struct. This struct defines all accounts that an instruction may access during execution.

Solana is parallel transaction execution so it is fast, if both Alice and Bob specify the same storage account, solana will infer a write conflict, and usually choose the one with higher priority fee, the other transacation will fail. This conflict detection is only possible because accounts are decalred up front.

Each field represents an account the program intends to access during execution.(even though it may not always use all of them)

The most commonly used account types are:

*Account*
*UncheckedAccount / AccountInfo*
*Signer*
*Program*

```rust

use anchor_lang::prelude::*;

  #[derive(Accounts)]
  pub struct Initialize<'info> {

    #[account(init,
              payer = signer,
              space = size_of::<Mystorage>() + 8,
              seeds = [],
              bump)]
    pub my_storage: Account<'info, MyStorage>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
  }

 #[derive(Accounts)]
 pub struct ReadBalance<'info> {
    #[account(mut)]
    pub unchecked_program: UncheckedAccount<'info>, ///when we read an account balance
 }
```
### Account<T>

Account<T> will verify the account is owned by the current program. If ownership does not match, the instruction fails immediately. This pervents accidentally reading or writing data that the program did not create.

Account<T> will check:

1.the account is owned by the current program
2.whether the data can be deserialized.

### uncheckedAccount / AccountInfo:

1.it does not check the owner or the data format
2.it is suitable for: only read lamports/CPI transit account addresses


*** Attackers can forge account data**

### Signer<'info>

It will check whether the account participated in transaction signing, it does not check the owner or the data.

Commonly used for:

1.User authentication
2.Access control
3.Multi-signature logic

### Program<'info, T>

it represents an executabkle account, can be used makeing CPI to call it


## 0x08

In Solana, off-chain client can read a storage account directly.
Any off-chain client — and even another on-chain program — can read account data directly, as long as:

the account public key is provided in the transaction, and

the program knows how to interpret the raw bytes.

In Anchor we use two programs:

```rust
data_holder
  └── PDA (Storage { x: u64 })  //owns and initialize a PDA storing data

data_reader
  └── reads PDA data via UncheckedAccount  //reads the PDA data on-chain

```

### Accounts data stores in Anchor
an Anchor account is laid out as raw bytes:

```rust
|    8 bytes    |          N bytes         |
|---------------|--------------------------|
| Discriminator | Serialized struct fields |

```

the first 8 bytes: account discriminator
the rest: the struct serialized field-by-field

for example:

```rust
#[account]
pub struct Storage {
  x: u64,
}
 
```
the total size is `8(discriminator) + 8(u64) = 16 bytes`

### Reading another program's account

```rust

let mut data_slice: &[u8] = &data_account.data.borrow();

let data_struct: Storage = AccountDeserialize::try_deserialize(&mut data_slice)?;

```

Key observations

UncheckedAccount<'info> is required (beacuse the current program does not own this account)


Anchor only:

_checks the discriminator_

_checks that there are enough bytes_

It does **NOT**:

_validate field names_

_validate semantic meaning_

_validate business correctness_


### NOT check in Anchor

1. field name not match(will pass)

```rust

pub struct Storage {
  y: u64,
}
```
Anchor only cares about byte position, not field name

2. field type mismatch(will pass)

```rust

pub struct Storage {
  y: u32,
}

```

`u32` only reads the first 4 bytes(it may lead to silent data corruption.)

3. read more data than exists(not pass)

```rust

pub struct Storage {
  y: u64,
  z: u64,
}

```
the account only has 16 bytes and this has 24(8+ 16)bytes.


## 0x09

CPI
Cross Program Invocation: a program calls the public function of other program in Solana.

### The simplest CPI example: System Program

when transfer SOL via the system program:

```rust

pub fn send_sol(ctx: Context<SendSol>, amount: u64) -> Result<()> {
  
  let cpi_context = CpiContext::new(
    ctx.accounts.system_program,to_account_info(),
    system_program::Transfer (
      from: ctx.accounts.signer.to_account_info(),
      to: ctx.accounts.recipient.to_account_info(),
    )
  );

  system_program::transfer(cpi_context, amount)?;
  Ok(())
}

```

There is already a full CPI call.

### Use cases of CPI in Solana/Anchor

1. sol transfer

just like code mentioned above.

Agreement on transaction fees

Reward distribution

PDA transfers funds to users(`invoke_signed`) 

What we should be aware of:

PDA tranfers must use `invoke_signed`, and verify that the from account is indeed the account you expected.

2. DeFi/ dex transaction

```rust

raydium::swap(...)
orca::swap(...)
phoenix::place_order(...)

```
it usually uses in Aggregated trading, Arbitrage, Route matching.

3. Lending/ Flash Loan/ Oracle

``` rust

lending::borrow(...)
lending::repay(...)


pyth::get_price(...)
switchboard::read_feed(...)

```
## 0x10

invoke_signed

As mentioned earlier,  `invoke_signed` is required when the PDA involves transactions, because the PDA does not have a private key and can not be acted as a signer. `invoke_signed` is precisely in this context that it allows the PDA  to execute contract calls. Specifically, it allows Solana to verify the PDA's identity based on the PDA account's `seeds` and `bump`.

Only seeds cause "permission hijacking":

seeds are the only "privilege input provided by the program that can be abused by the program" in invoke_signed.program_id(publickey) and bump, none of them have the ability to express permissions.

although invoke_signed has three elements:

```rust

invoke_signed (
  program_id,
  seeds,
  bump
)

```

`bump`: a bump has no semantic meaning, and does not represent user, permissions, ownership and identity, it just a mathematical padding bit, a correction value to prevent the PDA from falling on the ed25519 curve.

`program_id`: the currently executing program and can not be forged by the user or the CPI caller. It is automatically determined by the runtime.

`seeds` was decided:

"What does this PDA "stand for"

"Which logical domain does this PDA belong to"

"Whether this PDA is allowed to be signed under the current semantics"

"commonly put" in seeds:

1. domain separator:

```rust

b"vault"
b"config"
b"authority"

```

purpose:

Differentiating PDA namespace

Anti-collision

Protect against misuse

2. PublicKey

```rust

user.key().as_ref()
market.key().as_ref()
mint.key().as_ref()

```

purpose:

means "to whom"

indicates "which object to associate"

3. number / index


```rust

&index.to_le_bytes()

```

purpose:

multiple instances

position id

order id

4. hash

```rust

hash(data).to_bytes()

```

purpose:

Compress complex information

Hide original structure


5. The key of the PDA itself (hierarchical PDA)

```rust

vault.key().as_ref()
```

purpose:

Derive child object from "parent object"

Ref: [rareskills](https://rareskills.io/solana-tutorial)
     [blog](https://blog.asymmetric.re/invocation-security-navigating-vulnerabilities-in-solana-cpis/#how-cpi-works-in-solana)
