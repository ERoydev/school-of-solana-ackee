use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, TokenAccount, TransferChecked},
    token_2022::Token2022,
};

pub fn _transfer(ctx: Context<TransferContext>, amount: u64) -> Result<()> {
    let sender = &mut ctx.accounts.sender;
    let mint = &ctx.accounts.mint;
    let sender_ata = &mut ctx.accounts.sender_ata;

    let token_program = &ctx.accounts.token_program;
    let recipient_ata = &ctx.accounts.recipient_ata;

    let transfer_ctx = CpiContext::new(
        token_program.to_account_info(),
        TransferChecked {
            from: sender.to_account_info(),
            mint: mint.to_account_info(),
            to: recipient_ata.to_account_info(),
            authority: sender_ata.to_account_info(),
        },
    );

    transfer_checked(transfer_ctx, amount, 9);

    Ok(())
}

#[derive(Accounts)]
pub struct TransferContext<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    /// CHECK: Mint of the token to transfer
    pub mint: UncheckedAccount<'info>,
    /// CHECK: Recipient of the minted tokens
    pub recipient: UncheckedAccount<'info>,
    #[account(
    init_if_needed,
    payer = sender,
    associated_token::mint = mint,
    associated_token::authority = recipient,
    associated_token::token_program = token_program
  )]
    pub recipient_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = sender,
    associated_token::token_program = token_program
  )]
    pub sender_ata: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token2022>, // Of course i need the token_program to create Token account
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
