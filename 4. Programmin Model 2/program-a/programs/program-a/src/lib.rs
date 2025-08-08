use anchor_lang::prelude::*;
use program_b::program::ProgramB;

declare_id!("4XZ7uQyeUMprrAX4qv3otAMzHMSkk76dajMgTStmo6t5");

#[program]
pub mod program_a {
    use anchor_lang::{system_program::{transfer, Transfer}};
    

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from program A");
        let from_pubkey = ctx.accounts.pda_account.to_account_info();
        let to_pubkey = ctx.accounts.signer.to_account_info();
        let program_id = ctx.accounts.system_program.to_account_info();

        let signer_address = ctx.accounts.signer.key();

        // We need PDA account to sign that CPI, because we need to authorize the PDA ACCOUNT
        // Because we want to transfer lamports from the PDA account to the `signer`
        let bump_seed = ctx.bumps.pda_account;
        let signer_seeds: &[&[&[u8]]] = &[&[b"ackee",signer_address.as_ref(),&[bump_seed]]];

        let transfer_cpi = CpiContext::new(
            program_id,
            Transfer {
                from: from_pubkey.clone(),
                to: to_pubkey,
            }
        )
        .with_signer(signer_seeds);

        transfer(transfer_cpi, 1_000_000)?;

        // So i just make a CPI to a programB to invoke the instruction from it
        let cpi_context = CpiContext::new_with_signer( // new_with_signer() allows us to specify the signer seeds in order to sing the CPI for the PDA account
            ctx.accounts.program_b.to_account_info(),
            program_b::cpi::accounts::Initialize{ pda_account: from_pubkey }, 
            signer_seeds
        );

        // If i use just ::new() the PDA won't be marked as a signer

        program_b::cpi::initialize(cpi_context)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // AccountInfo -> Does not specify any checks
    // Account -> anchor checks that the owner is my program, data can be serialized into structure and so on
    /// CHECK: This PDA is derived deterministically using seeds [b"ackee", signer.key()] and verified by Anchor's seeds constraint. Safe to use as AccountInfo since we only need it for signing CPI calls, not reading/writing structured data.
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
