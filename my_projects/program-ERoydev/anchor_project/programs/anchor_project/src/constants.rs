/*
Pool swap fee = 0.3%
    - User swaps 100 Token A → 0.3 Token A goes into the pool as a fee
    - The remaining 99.7 Token A is used in the swap to calculate how much Token B the user receives.
    - That 0.3 Token A stays in the pool, which increases the value of LP tokens.
    - The pool now has slightly more Token A → all LP holders benefit

    - Anyone who holds LP tokens can later redeem them for a share of the larger pool, including the accumulated fees.
*/

use anchor_lang::prelude::*;

pub const POOL_SWAP_FEE: f64 = 0.3; // The fee goes into the pool
pub const POOL_LP_MINT_ACCOUNT_SEED: &str = "lp-mint";
pub const LP_PROVIDER_SEED: &str = "lp-provider";

pub const ESCROW_A_SEED: &str = "escrow-a";
pub const ESCROW_B_SEED: &str = "escrow-b";


#[constant]
pub const LIQUIDITY_POOL_SEEDS: &str = "liquidity_pool";

#[constant]
pub const TOKEN_METADATA_SEED: &str = "token_metadata";