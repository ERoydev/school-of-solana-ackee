
# SPL Escrow Example (Ackee Season 5)

This project demonstrates how to work with SPL Tokens in Solana using Anchor, including Associated Token Accounts (ATA), Token Accounts, and Mint Accounts.

---

## SPL Tokens Overview

### Associated Token Account (ATA)

- **Definition:** An ATA is an account automatically derived from a user’s wallet address and a Mint Account address.
- **Purpose:** Allows each wallet and token mint combination to have a unique token account.
- **Benefits:** Programs can derive the correct ATA address without user input, enabling direct token transfers.
- **Ownership:** Always owned by a user (wallet).
- **Use Case:** When users should hold and control their own tokens.
- **Examples:** User balances, wallet integrations, direct transfers between users.

**Anchor Account Constraints for ATA:**
```rust
#[account(
    mut,
    associated_token::mint = a_to_b_mint,
    associated_token::authority = side_a
)]
pub side_a_send_token_account_ata: Account<'info, TokenAccount>, // Alice's Token Account for token X
```
- `associated_token::mint = a_to_b_mint`: Ensures the account is the ATA for the given mint.
- `associated_token::authority = side_a`: Ensures the account is owned by the correct user.

---

### Token Account

- **Definition:** Created by the Token Program, holds tokens from a specific mint.
- **Ownership:** Can be owned by a user, a PDA, or any authority.
- **Authority:** The Token Program manages the account structure, but any account can be set as authority to control token transfers.
- **Use Case:** When your program needs to control tokens (escrow, custody, automated logic).
- **Examples:** Escrow accounts, vaults, programmatic token management.

**Anchor Account Constraints for Token Account:**
```rust
#[account(
    init,
    payer = side_a,
    token::mint = a_to_b_mint,
    token::authority = escrow,
    seeds = [escrow.key().as_ref()],
    bump
)]
pub escrow_token_account: Account<'info, TokenAccount>,
```
- `token::mint = a_to_b_mint`: Specifies which mint this account holds.
- `token::authority = escrow`: Sets the authority to the escrow PDA.

---

### Mint Account
// ...existing code...