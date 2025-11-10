use anchor_lang::prelude::*;

declare_id!("C8sHNustasu4BJ6nM1StfBBvsBsfc3bsrZYsMihiYLzE");

#[program]
pub mod transfer_hooks {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
