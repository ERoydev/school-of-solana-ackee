use anchor_lang::prelude::*;
use program_b::program::ProgramB;

declare_id!("4XZ7uQyeUMprrAX4qv3otAMzHMSkk76dajMgTStmo6t5");

#[program]
pub mod program_a {
    use anchor_lang::solana_program::{instruction::Instruction, program::invoke_signed};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from program A");

        // We need PDA account to sign that CPI, because we need to authorize the PDA ACCOUNT
        // Because we want to transfer lamports from the PDA account to the `signer`
        let program_id = ctx.accounts.system_program.to_account_info();
        let account_metas = ctx.accounts.to_account_metas(Some(true));

        let instruction = Instruction {
            program_id: program_id.key(),
            accounts: account_metas,
            data: []
        }

        invoke_signed(instruction, account_infos, signers_seeds);


        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // AccountInfo -> Does not specify any checks
    // Account -> anchor checks that the owner is my program, data can be serialized into structure and so on
    #[account(
        mut,
        seeds = [b"ackee", signer.key().as_ref()],
        // bump = 5, i can specify constant value like 5 but this have possibility to land on the curve
        bump // this will do the loop bump thing to find a bump that does not land on the curve
        
    )]
    pub pda_account: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>, 
    // Since i want to make CPI to program_b i should also include that into the inputs
    pub program_b: Program<'info, ProgramB> // So i have added this program_b to the Cargo.toml to use `ProgramB` in this context as type

}
