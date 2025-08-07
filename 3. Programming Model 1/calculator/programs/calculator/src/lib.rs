use anchor_lang::prelude::*;

declare_id!("GocTGUf6mfZk7kqYjUyhqaGdcS5PWuJh2BnzDebwk7ih");

const MAX_STRING_LENGTH: usize = 100;

#[program]
pub mod calculator {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;

    // Context will help me access the accounts that the user is supplying while calling this function
    pub fn create(ctx: Context<Create>, init_message: String) -> Result<()> {
        require!(
            init_message.len() < MAX_STRING_LENGTH,
            CustomError::MessageTooLong
        );

        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;

        Ok({})
    }

    pub fn add(ctx: Context<Addition>, x: i64, y: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = x + y;

        Ok({})
    } 

    pub fn subtraction(ctx: Context<Subtraction>, x: i64, y: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = x - y;


        Ok({})
    }
}

#[error_code]
pub enum CustomError {
    MessageTooLong,
}

// Defines which accounts are required for the `create` instruction
#[derive(Accounts)]
pub struct Create<'info> {
    #[account(
        init, // specify that if this account does not exists it will be initialized here
        payer = signer, // payer for the initialization
        space = 8 + 8 + 4 + MAX_STRING_LENGTH, // Amount of space that this `data account` is going to take
    )]
    pub calculator: Account<'info, Calculator>,
    #[account(mut)] // Mut because singer will have decreased his balance when paying fees for account creation
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>, // Empty accounts are owned by (System Program), so system program must be defined in order to initialize(create) an account
    // When initializing new account you need to specify `system_program`
}

#[derive(Accounts)]
pub struct Subtraction<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Addition<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

// This is a Data Account stored on-chain in a Solana account
#[account]
pub struct Calculator {
    greeting: String,
    result: i64,
}