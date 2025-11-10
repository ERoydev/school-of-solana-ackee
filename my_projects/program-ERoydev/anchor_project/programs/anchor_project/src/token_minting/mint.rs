use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, Mint, MintTo, Token, TokenAccount};

pub fn _mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
    // Since the mint authority is a PDA, my program PDA should sign that operation
    let mint_account = ctx.accounts.mint.key();
    let seeds = &[
        b"mint_authority",
        ctx.accounts.signer.key.as_ref(),
        mint_account.as_ref(),
        &[ctx.bumps.mint_authority],
    ];

    let signer_seeds = &[&seeds[..]];
    let cpi_ctx = ctx.accounts.into_mint_to_context().with_signer(signer_seeds);

    mint_to(
        cpi_ctx,
        amount
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    pub signer: Signer<'info>, 

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub to: Account<'info, TokenAccount>, // Destination ATA of the owner

    /// CHECK: This is the mint authority and must match the mint's authority
    #[account(
        seeds = [b"mint_authority", signer.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub mint_authority: UncheckedAccount<'info>, // I put my program as the authority of this mint.

    pub token_program: Program<'info, Token>,
}

impl<'info> MintTokens<'info> {
    pub fn into_mint_to_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.mint.to_account_info(),
                to: self.to.to_account_info(),
                authority: self.mint_authority.to_account_info(),
            },
        )
    }
}