use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

use crate::{state::Pool, ESCROW_A_SEED, ESCROW_B_SEED, LIQUIDITY_POOL_SEEDS, POOL_LP_MINT_ACCOUNT_SEED, POOL_SWAP_FEE};

/*
1. So now user own both tokens(Token A, Token B)
    - He deposits an initial amount of each into the pool's token accounts.
    - This initial deposit becomes the starting liquidity of the pool
    - Set the initial price ratio between Token A and Token B

2. The program creates the pool PDA & token accounts(ATA).
    - Transfers those tokens from the user -> pool token accounts.
    - Mints LP tokens to the user representing 100% ownership of the pool.

*/
pub fn _initialize_liquidity_pool(ctx: Context<InitializeLiquidityPool>,) -> Result<()> {
    // to ensure i get the same PDA for a token pair, regardless of the order user provides the accounts
    // Sort mint addresses before using them in the seeds.
    let pool = &mut ctx.accounts.pool;
    pool.token_a_mint = ctx.accounts.token_a_mint.key();
    pool.token_b_mint = ctx.accounts.token_b_mint.key();

    pool.escrow_token_a_account = ctx.accounts.escrow_token_a_account.key();
    pool.escrow_token_b_account = ctx.accounts.escrow_token_b_account.key();

    pool.lp_mint = ctx.accounts.lp_mint.key();
    pool.bump = ctx.bumps.pool;
    pool.last_update = Clock::get()?.unix_timestamp; // Gets the current Solana clock sysvar.
    pool.fee_bps = POOL_SWAP_FEE;

    pool.reserve_a = 0;
    pool.reserve_b = 0;

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeLiquidityPool<'info> {
    #[account(mut)]
    pub creator: Signer<'info>, // The guy who will own the LP mint and initializes the pool
    #[account(
        init,
        payer = creator,
        space = 8 + Pool::INIT_SPACE,
        // if i include the creator in the seeds that means multiple users can create Pool with those tokens
        seeds = [LIQUIDITY_POOL_SEEDS.as_bytes(), token_a_mint.key().as_ref(), token_b_mint.key().as_ref()], 
        bump
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        init,
        payer = creator,
        mint::decimals = 9,
        mint::authority = pool,
        mint::token_program = token_program,
        seeds = [POOL_LP_MINT_ACCOUNT_SEED.as_bytes(), pool.key().as_ref()], 
        bump
    )] // This `Pool` will have only one `Mint Account` for LP tokens and on `add_liquidity()` this account will be used again
    pub lp_mint: Account<'info, Mint>,

    // These two are owned by the pool and holds the `deposited` tokens ( ESCROW ACCOUNTS )
    #[account(
        init,
        payer = creator,
        token::mint = token_a_mint,
        token::authority = pool,
        seeds = [
            ESCROW_A_SEED.as_bytes(),
            pool.key().as_ref()
        ],
        bump
    )]
    pub escrow_token_a_account: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = creator,
        token::mint = token_b_mint,
        token::authority = pool,
        seeds = [
            ESCROW_B_SEED.as_bytes(),
            pool.key().as_ref()
        ],
        bump
    )]
    pub escrow_token_b_account: Account<'info, TokenAccount>,

    // // These tell my program which token types are being exchanged(USDC, SOL)
    // // They are already initialized and user must provide these mint accounts already initialized on Solana.
    pub token_a_mint: Account<'info, Mint>,
    pub token_b_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    // i need to use it because i use `init_if_needed` constraint
    // Anchor needs the Associated Token Program to create the associated token account if it doesn't exist.
    pub associated_token_program: Program<'info, AssociatedToken>
}
