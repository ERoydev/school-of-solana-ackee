use anchor_lang::prelude::*;

declare_id!("GocTGUf6mfZk7kqYjUyhqaGdcS5PWuJh2BnzDebwk7ih");

#[program]
pub mod calculator {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;

    // Context will help me access the accounts that the user is supplying while calling this function
    pub fn create(ctx: Context<Create>, init_message: String) -> ProgramResult {
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


#[derive(Accounts)]
pub struct Create<'info> {
    #[account(
        init, // specify that if this account does not exists it will be initialized here
        payer=signer, // payer for the initialization
        space= 256, // Amount of space that my calculator is going to take
    )]
    pub calculator: Account<'info, Calculator>,

    #[account(mut)]
    pub signer: Signer<'info>, 
    pub system_program: Program<'info, System>,
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

#[account]
pub struct Calculator {
    greeting: String,
    result: i64,
}