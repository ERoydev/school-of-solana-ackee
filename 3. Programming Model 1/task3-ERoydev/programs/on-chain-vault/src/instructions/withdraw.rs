//-------------------------------------------------------------------------------
///
/// TASK: Implement the withdraw functionality for the on-chain vault
/// 
/// Requirements:
/// - Verify that the vault is not locked
/// - Verify that the vault has enough balance to withdraw
/// - Transfer lamports from vault to vault authority
/// - Emit a withdraw event after successful transfer
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::state::Vault;
use crate::errors::VaultError;
use crate::events::WithdrawEvent;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault_authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", vault_authority.key().as_ref()], // I am telling anchor to derive the vault PDA using those seeds and
        bump,
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

pub fn _withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let user = &ctx.accounts.vault_authority;
    let vault = &ctx.accounts.vault;

    require!(
       !vault.locked,
       VaultError::VaultLocked
    );
    require!(
        vault.to_account_info().lamports() >= amount,
        VaultError::InsufficientBalance
    );

    **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.vault_authority.to_account_info().try_borrow_mut_lamports()? += amount;

    emit!(WithdrawEvent {
        amount: amount,
        vault_authority: user.key(),
        vault: vault.key()
    });

    Ok(())
}