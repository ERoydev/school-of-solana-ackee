use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct TokenMetadata {
    pub mint: Pubkey,
    #[max_len(20)]
    pub name: String,
    #[max_len(3)]
    pub symbol: String,
    pub supply: u64,
    pub decimals: u8, // Default will be 9 for now
    #[max_len(255)]
    pub uri: String,
    pub creator: Pubkey,
    pub created_at: i64,
}

// Example usage in your instruction handler:
// let metadata = TokenMetadata {
//     mint: mint.key(),
//     name: "USD Coin".to_string(),
//     symbol: "USDC".to_string(),
// };