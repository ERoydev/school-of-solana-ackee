use anchor_lang::prelude::*;

#[error_code]
pub enum LiquidityPoolError {
    #[msg("Invalid pool")]
    InvalidPool,
    #[msg("Lp tokens to mint should be greater than zero")]
    InvalidMint
}
