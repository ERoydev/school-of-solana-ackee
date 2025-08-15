
use anchor_lang::prelude::*;
use anchor_spl::token::{close_account, Token, TokenAccount, CloseAccount, Transfer, transfer};

use crate::Escrow;


pub fn _cancel_exchange(ctx: Context<CancelExchange>) -> Result<()> {
    let token_program = &ctx.accounts.token_program;
    let escrow = &ctx.accounts.escrow;
    let side_a_send_token_account = &ctx.accounts.side_a_send_token_account;
    let side_a = &ctx.accounts.side_a;

    // First since i cancel the exchange i need to return the deposited money from `escrow_token_account` back to alice's ATA
    // The amount that was initially send from Alice
    transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.escrow_token_account.to_account_info(),
                to: side_a_send_token_account.to_account_info(),
                authority: escrow.to_account_info(),
            },
            &[&[
                escrow.side_a.key().as_ref(),
                escrow.side_b.key().as_ref(),
                escrow.a_to_b_mint.key().as_ref(),
                escrow.b_to_a_mint.key().as_ref(),
                escrow.a_to_b_amount.to_le_bytes().as_ref(),
                escrow.b_to_a_amount.to_le_bytes().as_ref(),
                &[ctx.accounts.escrow.bump]
            ]]
        ),
        escrow.a_to_b_amount
    )?;

    close_account(
        CpiContext::new_with_signer(
        token_program.to_account_info(),
        CloseAccount { 
            account: ctx.accounts.escrow_token_account.to_account_info(),
            destination: ctx.accounts.side_a.to_account_info(),
            authority: escrow.to_account_info(),
        },
        &[&[
            escrow.side_a.key().as_ref(),
            escrow.side_b.key().as_ref(),
            escrow.a_to_b_mint.key().as_ref(),
            escrow.b_to_a_mint.key().as_ref(),
            escrow.a_to_b_amount.to_le_bytes().as_ref(),
            escrow.b_to_a_amount.to_le_bytes().as_ref(),
            &[ctx.accounts.escrow.bump]
        ]]
    ))?;


    Ok(())
}


#[derive(Accounts)]
pub struct CancelExchange<'info> {
    #[account(mut)]
    pub side_a: Signer<'info>,
    #[account(
        mut,
        close = side_a,
        seeds = [
            side_a.key().as_ref(),
            escrow.side_b.key().as_ref(),
            escrow.a_to_b_mint.key().as_ref(),
            escrow.b_to_a_mint.key().as_ref(),
            escrow.a_to_b_amount.to_le_bytes().as_ref(),
            escrow.b_to_a_amount.to_le_bytes().as_ref(),
        ],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mut,
        close = side_a,
        token::mint = escrow.a_to_b_mint,
        token::authority = escrow,
        seeds = [
            escrow.key().as_ref(),
        ],
        bump = escrow.escrow_token_bump
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::authority = side_a,
        associated_token::mint = escrow.a_to_b_mint,
    )]
    pub side_a_send_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}