use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::{events::TokenInitialized, TokenMetadata};
use crate::{TOKEN_METADATA_SEED};

pub fn _initialize(
    ctx: Context<Initialize>,
    name: String,
    symbol: String,
    uri: String,
    supply: u64,
) -> Result<()> {
    let token_metadata = &mut ctx.accounts.token_metadata;
    
    token_metadata.mint = ctx.accounts.mint.key();
    token_metadata.name = name.clone();
    token_metadata.symbol = symbol.clone();
    token_metadata.uri = uri;
    token_metadata.decimals = 9;
    token_metadata.creator = ctx.accounts.payer.key();
    token_metadata.created_at = Clock::get()?.unix_timestamp;
    token_metadata.supply = supply;

    emit!(TokenInitialized {
        mint: token_metadata.mint,
        name,
        symbol,
        creator: token_metadata.creator,
        timestamp: token_metadata.created_at,
    });
    
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        mint::decimals = 9,
        mint::authority = mint_authority,
        mint::freeze_authority = mint_authority,
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = payer,
        space = 8 + TokenMetadata::INIT_SPACE,
        seeds = [TOKEN_METADATA_SEED.as_bytes(), payer.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub token_metadata: Account<'info, TokenMetadata>,

    /// CHECK: PDA only derive and passed, used as authority, no account creation or initialize needed
    #[account(
        seeds = [b"mint_authority", payer.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub mint_authority: UncheckedAccount<'info>, // I put my program as the authority of this mint.

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}
