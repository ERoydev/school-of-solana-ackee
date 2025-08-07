//-------------------------------------------------------------------------------
///
/// TASK: Implement the toggle lock functionality for the on-chain vault
/// 
/// Requirements:
/// - Toggle the locked state of the vault (locked becomes unlocked, unlocked becomes locked)
/// - Only the vault authority should be able to toggle the lock
/// - Emit a toggle lock event after successful state change
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use crate::errors::VaultError;
use crate::state::Vault;
use crate::events::ToggleLockEvent;

#[derive(Accounts)]
pub struct ToggleLock<'info> {
    #[account(mut)]
    pub vault_authority: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, Vault>,
}

pub fn _toggle_lock(ctx: Context<ToggleLock>) -> Result<()> {
    let user = &mut ctx.accounts.vault_authority;
    let vault = &mut ctx.accounts.vault;

    require!(
        user.key() == vault.vault_authority.key(),
        VaultError::Overflow
    );

    vault.locked = !vault.locked;

    emit!(ToggleLockEvent {
        vault: vault.key(),
        vault_authority: user.key(),
        locked: vault.locked,
    });

    Ok(())
}