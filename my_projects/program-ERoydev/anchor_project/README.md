
# Anchor AMM Program

This Solana program, built with Anchor, implements an Automated Market Maker (AMM) for token swaps and liquidity management.

## Features

- **Initialize Liquidity Pool:** Set up pool PDA, token accounts, LP token mint, and fees.
- **Add Liquidity:** Deposit tokens and mint LP tokens.
- **Remove Liquidity:** Burn LP tokens to withdraw a proportional share.
- **Swap Tokens:** Swap between Token A and Token B using a constant-product formula (AMM).
- **Collect Fees:** Distribute fees to liquidity providers.

## Structure

- Organized into `constants`, `errors`, `instructions`, and `state` modules.
- Main entrypoint: `anchor_project` program module.

## Usage

Deploy with Anchor, then interact using Anchor client scripts or Solana CLI.

---

