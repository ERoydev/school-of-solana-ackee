use anchor_lang::{prelude::*, system_program::{create_account, CreateAccount}};
use anchor_spl::token_2022::{spl_token_2022::{extension::ExtensionType, pod::PodMint}, Token2022};


pub fn _initialize(ctx: Context<InitializeContext>) -> Result<()> {
    let system_program = &ctx.accounts.system_program;
    let creator = &ctx.accounts.creator;
    let mint = &ctx.accounts.mint;

    let token_program = &ctx.accounts.token_program;

    // I need to calculate the space first
    let space = ExtensionType::try_calculate_account_len::<PodMint>(&[ExtensionType::TransferFeeConfig])?;
    let lamports = Rent::get()?.minimum_balance(space);

    let create_account_ctx = CpiContext::new(
      system_program.to_account_info(),
      CreateAccount{
        from: creator.to_account_info(),
        to: mint.to_account_info()
      }
    );

    create_account(
        create_account_ctx,
        lamports,
        space as u64,
        &token_program.key()
    )?;

    let transfer_fee_init_ctx = CpiContext::new(
        token_program.to_account_info(),
    )

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeContext<'info> {
    #[account(mut)]
    pub creator: Signer<'info>, // Signer who will onw the mint
    #[account(mut)]
    pub mint: Signer<'info>, // The mint itself
    pub token_program: Program<'info, Token2022>, // Of course i need the token_program to create Token account
    pub system_program: Program<'info, System>
}
