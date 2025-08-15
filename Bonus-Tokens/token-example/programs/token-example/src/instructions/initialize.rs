use anchor_lang::{
    prelude::*,
    system_program::{create_account, CreateAccount},
};
use anchor_spl::{
    token::{initialize_mint2, InitializeMint2},
    token_2022::{
        spl_token_2022::{extension::ExtensionType, pod::PodMint},
        Token2022,
    },
    token_interface::{transfer_fee_initialize, TransferFeeInitialize},
};

/*
Summary:
- This function creates a new token mint,
  adds a transfer fee extension, and initializes the mint with the creator as the authority.
  It’s a typical pattern for setting up advanced Solana tokens with custom features.
*/

pub fn _initialize(ctx: Context<InitializeContext>, fee_bps: u16, max_fee: u64) -> Result<()> {
    // Create the `Account Setup` -> Gets reference to the system program, creator, mint and token program accounts from the context
    let system_program = &ctx.accounts.system_program;
    let creator = &mut ctx.accounts.creator;
    let mint = &ctx.accounts.mint;

    let token_program = &ctx.accounts.token_program;

    // Calculate Space and Rent, Calculates the required account size for a mint with the TransferFeeConfig extension.
    let space =
        ExtensionType::try_calculate_account_len::<PodMint>(&[ExtensionType::TransferFeeConfig])?;
    // Determines the minimum lamports needed for rent exemption.
    let lamports = Rent::get()?.minimum_balance(space);

    // Create mint Account
    let create_account_ctx = CpiContext::new(
        system_program.to_account_info(),
        CreateAccount {
            from: creator.to_account_info(),
            to: mint.to_account_info(),
        },
    );

    // Uses the system program to `create the mint account` with the calculated space and lamports.
    create_account(
        create_account_ctx,
        lamports,
        space as u64,
        &token_program.key(),
    )?;

    // Initialize Transfer Fee Extension
    // Prepares a CPI (Cross-Program Invocation) context for initializing the transfer fee extension.
    let transfer_fee_init_ctx = CpiContext::new(
        token_program.to_account_info(),
        TransferFeeInitialize {
            token_program_id: token_program.to_account_info(),
            mint: mint.to_account_info(),
        },
    );

    // Calls transfer_fee_initialize to set up transfer fee parameters (fee_bps and max_fee).
    transfer_fee_initialize(
        transfer_fee_init_ctx,
        None,
        Some(&creator.key()),
        fee_bps,
        max_fee,
    )?;

    // Initialize Mint
    // Prepares a CPI context for mint initialization.
    let initialize_mint_ctx = CpiContext::new(
        token_program.to_account_info(),
        InitializeMint2 {
            mint: mint.to_account_info(),
        },
    );

    // Calls initialize_mint2 to set the mint’s decimals (9), mint authority (creator), and optionally a freeze authority.
    initialize_mint2(initialize_mint_ctx, 9, &creator.key(), None)?;

    Ok(())
}

// Accounts Struct
// Defines the accounts required for this instruction: the creator, mint, token program, and system program.

#[derive(Accounts)]
pub struct InitializeContext<'info> {
    #[account(mut)]
    pub creator: Signer<'info>, // Signer who will onw the mint
    #[account(mut)]
    pub mint: Signer<'info>, // The mint itself
    pub token_program: Program<'info, Token2022>, // Of course i need the token_program to create Token account
    pub system_program: Program<'info, System>,
}
