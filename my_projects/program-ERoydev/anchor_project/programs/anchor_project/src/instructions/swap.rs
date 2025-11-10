use anchor_lang::prelude::*;

pub fn _swap(ctx: Context<SwapTokens>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct SwapTokens<'info> {
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>
}

impl<'info> SwapTokens<'info> {
    pub fn get_amount_token() -> u64 {
        todo!()
        // constant product formula => x * y = k
        // X and Y are Token in reserves.
        // fee need to be implemented
        // Token_B_out = y - (k / x + Token_A_in * (1 - fee))
    }
}
