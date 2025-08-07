//-------------------------------------------------------------------------------
///
/// TASK: Implement the deposit functionality for the on-chain vault
/// 
/// Requirements:
/// - Verify that the user has enough balance to deposit
/// - Verify that the vault is not locked
/// - Transfer lamports from user to vault using CPI (Cross-Program Invocation)
/// - Emit a deposit event after successful transfer
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::state::Vault;
use crate::errors::VaultError;
use crate::events::DepositEvent;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

pub fn _deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let user = &mut ctx.accounts.user;

    require!(user.lamports() >= amount, VaultError::InsufficientBalance);
    require!(!vault.locked, VaultError::VaultLocked);

    // Look this through the documentation for CPI there are 3 types of syntax to transfer
    /*
        - The system program is the only program allowed to transfer lamports between accounts.
        - CPI is required because you are invoking another program (system program) to perform the transfer.
     */

    let from_pubkey = user.to_account_info();
    let to_pubkey = vault.to_account_info();
    let program_id = ctx.accounts.system_program.to_account_info();

    // When i use this syntax for CPI the signer is the user that is creating this transfer instruction
    let cpi_context = CpiContext::new(
        program_id,
        Transfer {
            from: from_pubkey,
            to: to_pubkey,
        },
    );
    transfer(cpi_context, amount)?;

    // Emit event after successful transfer
    emit!(DepositEvent {
        amount: amount,
        user: user.key(),
        vault: vault.key()
    });
    Ok(())
}