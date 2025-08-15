
use anchor_lang::prelude::*;

use anchor_spl::token::{close_account, transfer, CloseAccount, Token, TokenAccount, Transfer};

use crate::Escrow;

pub fn _finalize_exchange(ctx: Context<FinalizeExchange>) -> Result<()> {
    let escrow = &ctx.accounts.escrow;

    let token_program = &ctx.accounts.token_program;

    let side_b_send_token_account_ata = &ctx.accounts.side_b_send_token_account_ata;
    let side_b = &ctx.accounts.side_b;

    // Now i need to transfer from `Bob ATA` to `Alice's Token Account`
    let transfer_cpi = CpiContext::new(
        token_program.to_account_info(), // CpiContext specify which program we are going to call (SPL Token program)
        Transfer { // Specify Account structure
            from: side_b_send_token_account_ata.to_account_info(), // Bob
            to: ctx.accounts.side_a_receive_token_account_ata.to_account_info(), // To Alice token account
            authority: side_b.to_account_info(), // Bob is authority of this token account so he needs to sign this
        }
    );

    transfer(transfer_cpi, escrow.b_to_a_amount)?;

    // Now i need to perform transfer from `Escrow Token Account` which holds Alice's deposit into `Bob ATA` to finish the exchange
    let escrow_token_program = &ctx.accounts.escrow_token_account;
    let side_b_receive_token_account_ata = &ctx.accounts.side_b_receive_token_account_ata;


    transfer(
    CpiContext::new_with_signer(
        token_program.to_account_info(),
        Transfer {
                from: escrow_token_program.to_account_info(),
                to: side_b_receive_token_account_ata.to_account_info(),
                authority: ctx.accounts.escrow.to_account_info(),
            },
            // So now `Escrow` Account should sign this as authority of this `escrow_token_program` authority to make this transfer
            // Since its PDA i should use the `seeds` to sign this ONLY in my program
            &[&[
                escrow.side_a.key().as_ref(),
                escrow.side_b.key().as_ref(),
                escrow.a_to_b_mint.key().as_ref(),
                escrow.b_to_a_mint.key().as_ref(),
                escrow.a_to_b_amount.to_le_bytes().as_ref(),
                escrow.b_to_a_amount.to_le_bytes().as_ref(),
                &[ctx.accounts.escrow.bump]
                // ProgramID is automatically included in the seeds
            ]],
        ),
        escrow.a_to_b_amount,
    )?;

    // So since my program is not a owner of the `escrow_token_account` i should invoke it with a instruction
    // `escrow` account is the authority so he should sign this using the PDA seeds in order to be closed and rent returned to Alice.
    close_account(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            CloseAccount {
                account: ctx.accounts.escrow_token_account.to_account_info(),
                destination: ctx.accounts.side_a.to_account_info(),
                authority: escrow.to_account_info()
            },
            &[&[
                escrow.side_a.key().as_ref(),
                escrow.side_b.key().as_ref(),
                escrow.a_to_b_mint.key().as_ref(),
                escrow.b_to_a_mint.key().as_ref(),
                escrow.a_to_b_amount.to_le_bytes().as_ref(),
                escrow.b_to_a_amount.to_le_bytes().as_ref(),
                &[ctx.accounts.escrow.bump]
                // ProgramID is automatically included in the seeds
            ]],
        )
    )?;

    Ok(())
}


#[derive(Accounts)]
pub struct FinalizeExchange<'info> {
    pub side_b: Signer<'info>, // Wont pay any rent so he is not `mutable` -> Bob
    #[account(
        mut,
        close = side_a, // Close this account and return the remaining lamports(rent) to `side_a`
        // I can close it like that because this account is owned by our program
        seeds = [
            side_a.key().as_ref(),
            side_b.key().as_ref(),
            escrow.a_to_b_mint.key().as_ref(),
            escrow.b_to_a_mint.key().as_ref(),
            escrow.a_to_b_amount.to_le_bytes().as_ref(),
            escrow.b_to_a_amount.to_le_bytes().as_ref(),
        ],
        bump=escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,
    /// CHECK: this has to be here as close will return rent, and it is also
    /// part of the seed which will ensure that we have correct account( because,
    /// wrong side_a or wrong escrow will result in PDA mismatch)
    #[account(mut)] // it receives the rent back (transfer)
    pub side_a: UncheckedAccount<'info>,

    #[account(
        mut,
        associated_token::authority = escrow.side_a,
        associated_token::mint = escrow.b_to_a_mint,
    )]
    pub side_a_receive_token_account_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::authority = side_b,
        associated_token::mint = escrow.a_to_b_mint,
    )]
    pub side_b_receive_token_account_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::authority = side_b,
        associated_token::mint = escrow.b_to_a_mint,
    )]
    pub side_b_send_token_account_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        close = side_a, // the token account is not owned by my program
        token::mint = escrow.a_to_b_mint,
        token::authority = escrow,
        seeds = [
            escrow.key().as_ref()
        ],
        bump = escrow.escrow_token_bump
    )]
    pub escrow_token_account: Account<'info, TokenAccount>, // This is the account that holds the tokens and should be closed too
    pub token_program: Program<'info, Token>,
}