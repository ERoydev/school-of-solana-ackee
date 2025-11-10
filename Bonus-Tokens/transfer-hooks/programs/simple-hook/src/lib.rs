use std::cell::RefMut;

use anchor_lang::{
    prelude::*,
    system_program::{create_account, CreateAccount},
};
use anchor_spl::{
    associated_token::AssociatedToken, token_2022::spl_token_2022::{extension::{BaseStateWithExtensionsMut, PodStateWithExtensionsMut, transfer_hook::TransferHookAccount}, pod::PodAccount}, token_interface::{Mint, TokenAccount, TokenInterface}
};
use spl_transfer_hook_interface::instruction::{ExecuteInstruction, TransferHookInstruction};
use spl_discriminator::SplDiscriminate; // This trait is required for ExecuteInstruction::SPL_DISCRIMINATOR_SLICE to work

declare_id!("GLJosyGfzpEH1YRpek8rDnSBK8NG8vD3V7At3LwZCp43");

// transfer hook instruction can only be called during a transfer, 
// otherwise someone could just call the transfer hook instruction directly and mess up the business logic
fn assert_is_transferring(ctx: &Context<TransferHook>) -> Result<()> {
    let source_token_info = ctx.accounts.source_token.to_account_info();
    let mut account_data_ref: RefMut<&mut [u8]> = source_token_info.try_borrow_mut_data()?;
    let mut account = PodStateWithExtensionsMut::<PodAccount>::unpack(*account_data_ref)?;
    let account_extension = account.get_extension_mut::<TransferHookAccount>()?;

    // All above is for a purpose to access this flag of the account_extension to check if we transfer currently or not
    if !bool::from(account_extension.transferring) {
        return Err(ProgramError::Custom(202).into());
    } else {
        msg!("We are currently transferring!")
    }

    Ok(())
}

#[error_code]
pub enum MyError {
    #[msg("The amount is too big")]
    AmountTooBig,
}

/*
Good Blog: https://www.quicknode.com/guides/solana-development/spl-tokens/token-2022/transfer-hooks -> Explained better than Sol docs

So i have created using CLI the Mint Token Account and Two Token Accounts to invoke a transfer between them.
- I have minted some tokens to the sender Token Account
- And when i execute `spl-token transfer <from> <amount> <to>` it should invoke my transfer_hook logic.

    spl-token create-token --transfer-hook <this_program_id(transfer_hook program)> --program-2022 => This command creates a Mint Account using this program with extension attached `transferHook`
    spl-token create-account <token_address> --owner ./sender.json
    spl-token create-account <token_address> --owner ./receiver.json

    Invoke:
        spl-token transfer 9cSXtFxpe64QyArPdEZPJLQLvRYrJ1GZDoNaGGhVW2gv 2 <DESTINATION_WALLET_ADDRESS> --from <SOURCE_ATA> --fund-recipient

        spl-token transfer 9cSXtFxpe64QyArPdEZPJLQLvRYrJ1GZDoNaGGhVW2gv 2 29JYrehHUWpyyf8CVi8Lii3fwYhSvDDxmAjSBVWqvgTv --from 7v3HcGrxjvxzC68CtR8Gqb727BE6Kcqmw7oy5yBhVNrd --allow-unfunded-recipient
*/

#[program]
pub mod simple_hook {
    use super::*;

    // #[instruction(discriminator=[220, 57, 220, 152, 126, 125, 97, 168])] // I can put the discriminator here and remove the `fallback` function
    #[instruction(discriminator = ExecuteInstruction::SPL_DISCRIMINATOR_SLICE)] // This is the same as hardcoding it but instead i take it from the lib
    pub fn transfer_hook(ctx: Context<TransferHook>, amount: u64) -> Result<()> {
        msg!("Hook called!");

        // Check if we currently are transferring
        // This is a guard, to ensure this function is not called without transferring and can only execute while it is transferring.
        assert_is_transferring(&ctx)?;

        msg!("The discriminator is: {:?}", ExecuteInstruction::SPL_DISCRIMINATOR_SLICE);


        Ok(())
    }

    // fallback instruction handler as workaround to anchor instruction discriminator check
    // Basically the transfer hook invokes and `Execute` fn, but instead we have `transfer_hook` fn, so we have to set it to correct fn discriminator to handle that different names
    // Instead u can hardcode or derive the discriminator with a macro on the `transfer_hook` as described in the comments
    // pub fn fallback<'info>(
    //     program_id: &Pubkey,
    //     accounts: &'info [AccountInfo<'info>],
    //     data: &[u8],
    // ) -> Result<()> {
    //     let instruction = TransferHookInstruction::unpack(data)
    //         .map_err(|_e| anchor_lang::prelude::ProgramError::Custom(0))?;

    //     // match instruction discriminator to transfer hook interface execute instruction
    //     // token2022 program CPIs this instruction on token transfer
    //     match instruction {
    //         TransferHookInstruction::Execute { amount } => {
    //             let amount_bytes = amount.to_le_bytes();

    //             // invoke custom transfer hook instruction on our program
    //             __private::__global::transfer_hook(program_id, accounts, &amount_bytes)
    //         }
    //         _ => return Err(ProgramError::InvalidInstructionData.into()),
    //     }
    // }
}

// #[derive(Accounts)]
// pub struct TransferHook<'info> {
//     pub signer: Signer<'info>,
// }

// Order of accounts matters for this struct.
// The first 4 accounts are the accounts required for token transfer (source, mint, destination, transfer_authority(owner))
// Remaining accounts are the extra accounts required from the ExtraAccountMetaList account
// These accounts are provided via CPI to this program from the token2022 program
#[derive(Accounts)]
pub struct TransferHook<'info> {
    #[account(
        token::mint = mint, 
        token::authority = transfer_authority,
    )]
    pub source_token: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        token::mint = mint,
    )]
    pub destination_token: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: source token account owner, can be SystemAccount or PDA owned by another program
    pub transfer_authority: UncheckedAccount<'info>,

    // If my program needs more accounts, then i will need to add that.
    // /// CHECK: ExtraAccountMetaList Account,
    // #[account(
    //     seeds = [b"extra-account-metas", mint.key().as_ref()], 
    //     bump
    // )]
    // pub extra_account_meta_list: UncheckedAccount<'info>,
}
