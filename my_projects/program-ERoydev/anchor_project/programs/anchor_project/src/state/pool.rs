use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Pool {
    // Mint Accounts containing information of the SPL token
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    // Token Account that contains the token from the specific `Mint Account`
    pub escrow_token_a_account: Pubkey,
    pub escrow_token_b_account: Pubkey,
    // The LP_mint of the liquidity pool (The providers keep this as `shares` for their staked tokens)
    pub lp_mint: Pubkey,
    pub total_lp_supply: u64,

    pub fee_bps: f64, // Represents the pool fees basis points, example: 0.30% fee
    pub bump: u8,
    // Holds amount of token currently held in the pool
    pub reserve_a: u64,
    pub reserve_b: u64,
    pub last_update: i64
}
