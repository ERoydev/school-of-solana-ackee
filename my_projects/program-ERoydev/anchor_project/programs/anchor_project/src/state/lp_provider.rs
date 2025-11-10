use anchor_lang::prelude::*;


/*
When user add liquidity and he sends his tokens to the pool:
- He receives LP tokens this is the Data Account for it

*/

#[account]
#[derive(InitSpace)]
pub struct LpProvider {
    pub pool: Pubkey,            // Reference to the pool
    pub user: Pubkey,            // Reference to the user who provided
    pub token_a_provided: u64,   // Amount of Token A provided
    pub token_b_provided: u64,   // Amount of Token B provided
    pub lp_tokens_owned: u64,    // Amount of LP tokens currently owned by the user
    pub last_update: i64,        // Timestamp of last update
    pub bump: u8,
}