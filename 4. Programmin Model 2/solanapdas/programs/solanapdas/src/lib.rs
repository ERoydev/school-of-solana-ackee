use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("BoMswARpoApys34RFKzBpxtxNDkm5KPvwDPjNs2LWkMF");

#[program]
pub mod solanapdas {

    use super::*;

    pub fn create(ctx: Context<Create>, name: String) -> ProgramResult {
        let bank = &mut ctx.accounts.bank;
        bank.owner = ctx.accounts.user.key();
        bank.name = name;
        bank.balance = 0;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> ProgramResult {
        let tx = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(), // Your wallet's public key (from MetaMask) of the user that is interacting with this instruciton
            &ctx.accounts.bank.key(), // The PDA address derived using your public key
            amount
        ); // I construct my transaction

        // After i have constructed my transaction i need to invoke it 
        anchor_lang::solana_program::program::invoke(
            &tx,
            &[
                // Specify all accounts that are used in this transaction
                ctx.accounts.user.to_account_info(),
                ctx.accounts.bank.to_account_info() // Converts my high level Account<'info, T> into a lower-level AccountInfo<'info> raw Solana account type.
            ],
        )?;
        
        (&mut ctx.accounts.bank).balance += amount; // This only updates my program custom state ( It does not acually move SOL from one account to another)

        Ok({})
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> ProgramResult {
        let bank = &mut ctx.accounts.bank;
        let user = &mut ctx.accounts.user;

        if bank.owner != user.key() {
            return Err(ProgramError::IncorrectProgramId);
        }
        
        // All of the accounts should store rent
        // So i need to make sure i have enough rent in my PDA so it will not get destroyed
        let rent = Rent::get()?.minimum_balance(bank.to_account_info().data_len()); 
        // That gets the minimum amount of rent for this account that i need to store in this PDA account to not be destroyed after the withdraw

        // First check if bank account have enough lamports to make this withdraw
        if **bank.to_account_info().lamports.borrow() - rent < amount {
            return Err(ProgramError::InsufficientFunds)
        }

        **bank.to_account_info().try_borrow_mut_lamports()? -= amount;
        **user.to_account_info().try_borrow_mut_lamports()? += amount;

        Ok({})
    }

}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(
        init,
        payer=user,
        space = 5000,
        seeds = [b"bank_account", user.key().as_ref()],
        bump,
    )]
    pub bank: Account<'info, Bank>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct Bank {
    pub owner: Pubkey, // To be able to check if my PDF own this account
    pub name: String,
    pub balance: u64,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub bank: Account<'info, Bank>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub bank: Account<'info, Bank>,
    pub system_program: Program<'info, System>
}

#[error_code]
pub enum CustomError {
    #[msg("You are not authorized to update this profile.")]
    Unauthorized,
}