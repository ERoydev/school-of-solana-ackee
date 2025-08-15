use anchor_lang::prelude::*;

declare_id!("2znSHte1kU3zLepo5UFNPxmm6fkABVRtBBMkNFLpf6MY");

#[program]
pub mod solana_errors_lecture_5 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, count: u8) -> Result<()> {
        let data = &mut ctx.accounts.data;

        data.authority = ctx.accounts.user.key();
        require!(count <= 10, MyError::InvalidCount);

        data.counter = math_function(count).unwrap();

        msg!("Data counter = {}", data.counter);
        msg!("Data pubkey = {}", data.key());
        msg!("User pubkey = {}", ctx.accounts.user.key());

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    user: Signer<'info>,
    
    #[account(
        init, // The PDA sign itself, but since this is not i need to provide this as the second SIGNER in my tests
        space = 8 + 32 + 1, // 8 = Account discriminator(fixed prefix), 32 -> Pubkey 32 bytes, 1 -> counter fields is u8
        payer = user,
        seeds = [b"data"],
        bump
    )]
    pub data: Account<'info, MyData>,
    pub system_program: Program<'info, System>,
}

/*
Solana accounts are just raw byte buffers.
So without a discriminator, there's no built-in way to know whether a given account is:

    A MyData account?

    A Bank account?

    A token account?

    Garbage?

Anchor solves this by writing a discriminator at the beginning of every account it initializes.

*/

#[account]
pub struct MyData {
    authority: Pubkey,
    counter: u8,
}


#[error_code]
pub enum MyError {
    #[msg("Invalid count value")]
    InvalidCount
}

// SINCE THIS IS RUST WAY INSTEAD OF USING TYPES, PREVENTS BUGS

fn math_function(count: u8) -> Option<u8> {
    10u8.checked_sub(count)
}

// BECAUSE I NEED TO TEST I CAN USE UNIT TESTS LIKE THAT

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math_function() {
        assert_eq!(math_function(2), Some(8));
        assert_eq!(math_function(11), None);
    }
}