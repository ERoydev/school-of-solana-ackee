use anchor_lang::prelude::*;

declare_id!("C8jwoVgQsRTpVxt8ZcSDZMTU9kJ8oz3PUysxTTuUcryy");

mod instructions;
use instructions::*;

#[program]
pub mod token_example {
    use super::*;

    pub fn initialize(ctx: Context<InitializeContext>) -> Result<()> {
        _initialize(ctx)
    }
}