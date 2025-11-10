use anchor_lang::prelude::*;

pub fn _remove_liquidity(ctx: Context<RemoveLiquidity>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct RemoveLiquidity<'info> {
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>
}

impl<'info> RemoveLiquidity<'info> {
    pub fn get_amount_lp_tokens_to_burn() -> u64 {
        // Burn amount = (LP burned / total_lp_supply) * reserve_a (token that gets removed)
        todo!();
    }
}
