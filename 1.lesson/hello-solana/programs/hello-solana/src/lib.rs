use anchor_lang::prelude::*;

declare_id!("2zYm73T1ZXELcPpJA1ftKVExLona5RnLbakDYwfGTz6J");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, hello: String) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        
        let data_account = &mut ctx.accounts.data_account;
        
        data_account.hello = hello;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)] // Since signer pays rent his balance will be decreased that is why i need to be mutable
    pub signer: Signer<'info>, // payer
    #[account(
        init,
        payer = signer, // specify payer of the rent for this account
        space = 200, // i allocate 200 bytes for this account
    )]
    pub data_account: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,


}

#[account]
pub struct DataAccount {
    pub hello: String,

}
