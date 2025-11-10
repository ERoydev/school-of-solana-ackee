use anchor_lang::prelude::*;

declare_id!("7caxL1ckrUPM1bPydPDf4aoqyNv49dhNa8G1ZnKFPg37");

pub mod instructions;
pub mod constants;
pub mod errors;
pub mod state;
pub mod events;
pub mod token_minting;

pub use constants::*;
pub use errors::*;
pub use instructions::*;
pub use state::*;
pub use token_minting::*;

// LP = Liquidity Provider

#[program]
pub mod anchor_project {
    use super::*;

    // Creates a new pool PDA, initialize token accounts, LP token mint and fees
    pub fn initialize_liquidity_pool(ctx: Context<InitializeLiquidityPool>) -> Result<()> {
        _initialize_liquidity_pool(ctx)
    }

    // Deposit tokens, mint LP tokens
    pub fn add_liquidity(ctx: Context<AddLiquidity>, amount_a: u64, amount_b: u64) -> Result<()> {
        _add_liquidity(ctx, amount_a, amount_b)
    }

    // Burn LP tokens, withdraw proportional share
    pub fn remove_liquidity(ctx: Context<RemoveLiquidity>) -> Result<()> {
        _remove_liquidity(ctx)
    }

    // Swap Token A <-> Token B using constant-product-formula
    pub fn swap(ctx: Context<SwapTokens>) -> Result<()> {
        _swap(ctx)
    }

    // Handle fee distribution to LPs
    pub fn collect_fees(ctx: Context<CollectFees>) -> Result<()> {
        _collect_fees(ctx)
    }

    // Token Minting instructions
    pub fn initialize_mint_account(ctx: Context<Initialize>, name: String, symbol: String, uri: String, supply: u64) -> Result<()> {
        _initialize(ctx, name, symbol, uri, supply)
    }

    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        _mint_tokens(ctx, amount)
    }
}
