use anchor_lang::prelude::*;

declare_id!("XKZvP1Kttr2AUHDjR6SjXVfLR6n93ze3i4TtJWakrzM");

#[program]
pub mod program_b {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub pda_account: Signer<'info>,
}
