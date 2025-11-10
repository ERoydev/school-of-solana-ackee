# Project Description

**Deployed Frontend URL:** [TODO: Link to your deployed frontend]

**Solana Program ID:** [TODO: Your deployed program's public key]

## Project Overview

### Description
This project is a AMM. The goal is to implement a basic decentralized Automated Market Maker (AMM) on Solana.
The dApp will allow users to swap between two tokens using a constant-product formula (x * y = k), add or remove liquidity from the pool, and track liquidity provider shares and fees.

### Key Features
- **Liquidity Pool Creation**: Initialize a pool with two tokens, set initial liquidity and fee parameters.
  
- **Token Swaps**: Users can swap Token A for Token B, swap price is calculated using the formula, fees are applied on each swap to reward liquidity providers.
  
- **Add / Remove Liquidity**: Users can deposit both tokens into the pool to receive LP tokens, LP tokens represents a user's share of the pool, users can burn LP tokens to withdraw proportional liquidity.
  
- **Pool State Management**: Track pool balances, total LP tokens, and accumulated fees, use PDA accounts to manage pool state and LP token accounts securely.
  
### How to Use the dApp
1. **Connect Wallet** - Connect your Solana wallet.
2. **Main Action 1:** [Step-by-step instructions]
3. **Main Action 2:** [Step-by-step instructions]
4. ...

## Program Architecture
[TODO: Describe your Solana program's architecture. Explain the main instructions, account structures, and data flow.]

### PDA Usage
[TODO: Explain how you implemented Program Derived Addresses (PDAs) in your project. What seeds do you use and why?]

**PDAs Used:**
- PDA 1: [Purpose and description]
- PDA 2: [Purpose and description]

### Program Instructions
[TODO: List and describe all the instructions in your Solana program]

**Instructions Implemented:**
- Instruction 1: [Description of what it does]
- Instruction 2: [Description of what it does]
- ...

### Account Structure
[TODO: Describe your main account structures and their purposes]

```rust
// Example account structure (replace with your actual structs)
#[account]
pub struct YourAccountName {
    // Describe each field
}
```

## Testing

### Test Coverage
[TODO: Describe your testing approach and what scenarios you covered]

**Happy Path Tests:**
- Test 1: [Description]
- Test 2: [Description]
- ...

**Unhappy Path Tests:**
- Test 1: [Description of error scenario]
- Test 2: [Description of error scenario]
- ...

### Running Tests
```bash
# Commands to run your tests
anchor test
```

### Additional Notes for Evaluators

[TODO: Add any specific notes or context that would help evaluators understand your project better]