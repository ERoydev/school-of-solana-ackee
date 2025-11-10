use anchor_lang::prelude::*;

use anchor_spl::{associated_token::AssociatedToken, token::{transfer, Mint, MintTo, Token, TokenAccount, Transfer, mint_to}};

use crate::{LpProvider, Pool, LIQUIDITY_POOL_SEEDS, LP_PROVIDER_SEED};


pub fn _add_liquidity(ctx: Context<AddLiquidity>, amount_a: u64, amount_b: u64) -> Result<()> {

    let pool = &mut ctx.accounts.pool;
    let provider = &ctx.accounts.provider;
    
    // User ATA accounts that holds the tokens
    let user_send_token_a_account_ata = &mut ctx.accounts.user_send_token_a_account_ata;
    let user_send_token_b_account_ata = &ctx.accounts.user_send_token_b_account_ata;
    
    let escrow_token_a_account = &ctx.accounts.escrow_token_a_account;
    let escrow_token_b_account = &ctx.accounts.escrow_token_b_account;
    
    let token_program = &ctx.accounts.token_program;
    
     // 1. Transfer from the creator `ATA` to `Pool token account for tokenA`
    let transfer_token_a_cpi = CpiContext::new(
        token_program.to_account_info(),
        Transfer {
            from: user_send_token_a_account_ata.to_account_info(),
            to: escrow_token_a_account.to_account_info(),
            authority: provider.to_account_info()
        }
    );

    transfer(
        transfer_token_a_cpi,
        amount_a
    )?;

    // 2. Transfer from the creator `ATA` to `Pool token account for tokenB`
    let transfer_token_b_cpi = CpiContext::new(
        token_program.to_account_info(),
        Transfer {
            from: user_send_token_b_account_ata.to_account_info(),
            to: escrow_token_b_account.to_account_info(),
            authority: provider.to_account_info()
        }
    );

    transfer(
        transfer_token_b_cpi,
        amount_b
    )?;    

    let lp_to_mint = if pool.total_lp_supply == 0 {
        AddLiquidity::get_amount_initial_lp_tokens_to_mint(amount_a, amount_b)
    } else {
        AddLiquidity::get_amount_lp_tokens_to_mint(
            amount_a,
            amount_b, 
            pool.reserve_a, 
            pool.reserve_b, 
            pool.total_lp_supply,
        )
    };

    let lp_provider = &mut ctx.accounts.lp_provider;
    lp_provider.pool = pool.key();
    lp_provider.user = provider.key();
    lp_provider.token_a_provided = amount_a;
    lp_provider.token_b_provided = amount_b;
    lp_provider.bump = ctx.bumps.lp_provider;

    let lp_mint = &ctx.accounts.lp_mint;

    mint_to(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            MintTo {
                mint: lp_mint.to_account_info(),
                to: ctx.accounts.lp_user_receive_ata.to_account_info(),
                authority: pool.to_account_info()
            }, 
            // Since `Pool` is an PDA i should sign with seeds this transfer 
              &[&[
                LIQUIDITY_POOL_SEEDS.as_bytes(),
                pool.token_a_mint.key().as_ref(), 
                pool.token_b_mint.key().as_ref(),
                &[pool.bump]
            ]]
        ), 
    lp_to_mint)?;

    // Update State
    pool.reserve_a += amount_a;
    pool.reserve_b += amount_b;
    pool.last_update = Clock::get()?.unix_timestamp;

    lp_provider.lp_tokens_owned = lp_to_mint;
    lp_provider.last_update = Clock::get()?.unix_timestamp;

    Ok(())
}

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(mut)]
    pub provider: Signer<'info>,

    #[account(mut)]
    pub pool: Account<'info, Pool>,

    #[account(
        mut,
        // Two important constraints in order to tell the Anchor this is an `ATA`, its address derived from owner Pubkey and Mint Pubkey
        associated_token::mint = token_a_mint,
        associated_token::authority = provider
    )]
    pub user_send_token_a_account_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_b_mint,
        associated_token::authority = provider
    )]
    pub user_send_token_b_account_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub escrow_token_a_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub escrow_token_b_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = provider,
        space = 8 + LpProvider::INIT_SPACE,
        seeds = [LP_PROVIDER_SEED.as_bytes(), provider.key().as_ref(), pool.key().as_ref()],
        bump
    )]
    pub lp_provider: Account<'info, LpProvider>,

    #[account(
        init_if_needed,
        payer = provider,
        associated_token::mint = lp_mint,
        associated_token::authority = provider
    )]
    pub lp_user_receive_ata: Account<'info, TokenAccount>,


    #[account(mut)]
    pub lp_mint: Account<'info, Mint>,

    // These tell my program which token types are being exchanged(USDC, SOL)
    // They are already initialized and user must provide these mint accounts already initialized on Solana.
    pub token_a_mint: Account<'info, Mint>,
    pub token_b_mint: Account<'info, Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

impl<'info> AddLiquidity<'info> {
    pub fn get_amount_lp_tokens_to_mint(
        deposit_token_a: u64,
        deposit_token_b: u64,
        reserve_token_a: u64,
        reserve_token_b: u64,
        total_lp_supply: u64,
    ) -> u64 {
        /*
            Formula: 
            LP minted = min(
                deposit_token_a * total_lp_supply / reserve_token_a,
                deposit_token_b * total_lp_supply / reserve_token_b
            )
         */
        if reserve_token_a == 0 || reserve_token_b == 0 || total_lp_supply == 0 {
            return 0;
        }
        let share_a = deposit_token_a as u128 * total_lp_supply as u128 / reserve_token_a as u128;
        let share_b = deposit_token_b as u128 * total_lp_supply as u128 / reserve_token_b as u128;
        std::cmp::min(share_a, share_b) as u64
    }

    // Used when Pool `Lp token supply` is 0 meaning only for the first liquidity provider 
    pub fn get_amount_initial_lp_tokens_to_mint(deposit_a: u64, deposit_b: u64) -> u64 {
        // LP minted = sqrt(token_a_qty * token_b_qty)
        let total = deposit_a as u128 * deposit_b as u128; // prevent overflow when multiplying
        let amount_lp_to_mint = (total as f64).sqrt() as u64;
        amount_lp_to_mint
    }
}


