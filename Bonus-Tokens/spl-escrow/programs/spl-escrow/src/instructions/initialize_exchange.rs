
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, transfer, Transfer};

use crate::Escrow;

pub fn _initialize_exchange(ctx: Context<InitializeExchange>, a_to_b_amount: u64, b_to_a_amount: u64, side_b: Pubkey) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;

    escrow.side_a = ctx.accounts.side_a.key();
    escrow.side_b = side_b;

    escrow.a_to_b_amount = a_to_b_amount;
    escrow.b_to_a_amount = b_to_a_amount;

    escrow.a_to_b_mint = ctx.accounts.a_to_b_mint.key();
    escrow.b_to_a_mint = ctx.accounts.b_to_a_mint.key();

    escrow.bump = ctx.bumps.escrow;
    escrow.escrow_token_bump = ctx.bumps.escrow_token_account;

    let token_program = &ctx.accounts.token_program;

    let side_a_send_token_account_ata = &mut ctx.accounts.side_a_send_token_account_ata;
    let escrow_token_account = &mut ctx.accounts.escrow_token_account;
    let side_a = &ctx.accounts.side_a;

    // Now i need to transfer from `Alice ATA` to `Escrow Token Account`
    let transfer_cpi = CpiContext::new(
        token_program.to_account_info(), // CpiContext specify which program we are going to call (SPL Token program)
        Transfer { // Specify Account structure
            from: side_a_send_token_account_ata.to_account_info(), // Alice
            to: escrow_token_account.to_account_info(), // Escrow hold by program
            authority: side_a.to_account_info(), // Alice is authority of this token account so she needs to sign this
        }
    );

    // 
    transfer(transfer_cpi, a_to_b_amount)?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(a_to_b_amount: u64, b_to_a_amount: u64, side_b: Pubkey)]
pub struct InitializeExchange<'info> {
    #[account(mut)]
    pub side_a: Signer<'info>,
    #[account(
        init,
        payer = side_a,
        space = 8 + Escrow::LEN,
        seeds = [
            // The idea is to specify more variables as seed to create more unique PDA
            side_a.key().as_ref(),
            side_b.key().as_ref(),
            a_to_b_mint.key().as_ref(),
            b_to_a_mint.key().as_ref(),
            a_to_b_amount.to_le_bytes().as_ref(),
            b_to_a_amount.to_le_bytes().as_ref(),
        ],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mut, // modify the balance of the token account
        // Two important constraints in order to tell the Anchor this is an `ATA`, its address derived from owner Pubkey and Mint Pubkey
        associated_token::mint = a_to_b_mint,
        associated_token::authority = side_a
        // I am telling the Anchor to use those two Public keys and check if addresses are equal
    )]
    pub side_a_send_token_account_ata: Account<'info, TokenAccount>, // Alice Token Account for token X

    #[account(
        init,
        payer = side_a,
        token::mint = a_to_b_mint, // Which `Mint Account` this `Token Account` is going to hold
        token::authority = escrow, // Specify authority of this mint means the `escrow` Data Account (PDA) is going to be authority to transfer this token
        seeds = [
            escrow.key().as_ref()
        ],
        bump
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,

    // These accounts tell my program which token types are being exchanged(USDC, SOL, etc.)
    // They are used to validate and operate on the correct tokens for transfers and account creation.
    // They are already initialized and user must provide these mint accounts already initialized on Solana.
    // My Program does not create new mints it only references existing ones to ensure the escrow logic works with the correct tokens.
    pub a_to_b_mint: Account<'info, Mint>,
    pub b_to_a_mint: Account<'info, Mint>,

    // I need to use token_program, because we are going to transfer tokens from Token accounts
    // And in order to use `transfer` function which is from SPL-Token program i need to specify this Token program
    // Only the owner (which is token_program) of a token account can authorize transfers
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
