use anchor_lang::prelude::*;

declare_id!("2ucHaJAaxnBymvJ6BycLDd7JC6vbGGQHnyZfUkEYETFa");

pub mod instructions;
pub use instructions::*;

pub mod state;
pub use state::*;

#[program]
pub mod spl_escrow {
    use super::*;

    pub fn initialize_exchange(ctx: Context<InitializeExchange>, a_to_b_amount: u64, b_to_a_amount: u64, side_b: Pubkey) -> Result<()> {
        _initialize_exchange(ctx, a_to_b_amount, b_to_a_amount, side_b)
    }

    pub fn finalize_exchange(ctx: Context<FinalizeExchange>) -> Result<()> {
        _finalize_exchange(ctx)
    }

    pub fn cancel_exchange(ctx: Context<CancelExchange>) -> Result<()> {
        _cancel_exchange(ctx)
    }
}
