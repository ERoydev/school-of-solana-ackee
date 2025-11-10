# Here i write the Issues i have found:
-  But this repo have folder from ackee [Common Issues](https://github.com/ERoydev/school-of-solana-ackee/tree/main/Common%20Issues)

# SPL TOKENS ISSUES
### 1. Issue i have encountered when i have exact same code as in the video, but mine is not compiling:
#### When i try to `anchor build` to compile my code i receive errors like that:
```rs
   error[E0599]: no associated item named `DISCRIMINATOR` found for struct `anchor_spl::token::TokenAccount` in the current scope
  --> programs/spl-escrow/src/instructions/initialize_exchange.rs:44:10
   |
44 | #[derive(Accounts)]
   |          ^^^^^^^^ associated item not found in `anchor_spl::token::TokenAccount`
   |
   = note: this error originates in the derive macro `Accounts` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0599]: no function or associated item named `insert_types` found for struct `anchor_spl::token::TokenAccount` in the current scope
  --> programs/spl-escrow/src/instructions/initialize_exchange.rs:44:10
   |
44 | #[derive(Accounts)]
   |          ^^^^^^^^ function or associated item not found in `anchor_spl::token::TokenAccount`
   |
   = note: this error originates in the derive macro `Accounts` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0599]: no function or associated item named `create_type` found for struct `anchor_spl::token::Mint` in the current scope
  --> programs/spl-escrow/src/instructions/initialize_exchange.rs:44:10
   |
44 | #[derive(Accounts)]
   |          ^^^^^^^^ function or associated item not found in `anchor_spl::token::Mint`
   |
   = note: this error originates in the derive macro `Accounts` (in Nightly builds, run with -Z macro-backtrace for more info)
```

#### The fix is that in `Cargo.toml` i set this:
```toml
idl-build = ["anchor-lang/idl-build", "anchor-spl/idl-build"]
```
- That tells the Cargo to enable idl-build feature for both `anchor-lang` and `anchor-spl`.
- The #[derive(Accounts)] macro in Anchor needs certain metadata to generate the IDL (including how to serialize/deserialize accounts like TokenAccount).
- Without the idl-build feature enabled for both crates, the macro can’t “see” the trait impls (Discriminator, insert_types, create_type) for SPL account types.
- Adding idl-build puts the crates into a build mode that includes all the IDL code paths, which also brings in those trait implementations.
- Anchor macro was running with half the code hidden behind a disabled feature flag, so the types didn’t “exist” in the macro’s eyes.


# LP Mint PDA Storage Before Initialization
## Problem:
In Anchor, when creating a pool, the LP token mint account doesn’t exist yet at the point where we want to store its address in the Pool struct. Directly assigning ctx.accounts.lp_mint.key() will fail because the account isn’t initialized.

## Solution:
Compute the LP mint’s Program Derived Address (PDA) ahead of time using the pool key and a seed, and store it in the pool struct. Anchor will initialize the actual account later, but the Pool account already holds the correct mint address.

```rs
let (lp_mint_pda, _bump) = Pubkey::find_program_address(
    &[POOL_LP_MINT_ACCOUNT_SEED.as_bytes(), pool.key().as_ref()],
    ctx.program_id,    
);
pool.lp_mint = lp_mint_pda;
```
