#![no_std]

use pinocchio::{account_info::AccountInfo, entrypoint, nostd_panic_handler, program_error::ProgramError, pubkey::Pubkey, ProgramResult};

pub mod instructions;
pub use instructions::*;

// 22222222222222222...
pub const ID: Pubkey = [
    50, 50, 50, 50, 50, 50, 50, 50,
    50, 50, 50, 50, 50, 50, 50, 50,
    50, 50, 50, 50, 50, 50, 50, 50,
    50, 50, 50, 50, 50, 50, 50, 50
];

// I have to announce my entrypoint with this macro
entrypoint!(process_instruction);

// I need to specify panic handler, because i use the `no_std` attribute
nostd_panic_handler!();


fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // So here i basically create a dispatcher based on the `DISCRIMINATOR` for each instruction to decide which one is called
    match instruction_data.split_first() {
        // So it will apply all the checks and then execute the instruction via `process()`
        Some((Deposit::DISCRIMINATOR, data)) => Deposit::try_from((accounts, data))?.process(),
        Some((Withdraw::DISCRIMINATOR, _)) => Withdraw::try_from((accounts, instruction_data))?.process(),
        _ => Err(ProgramError::InvalidInstructionData)
    }
}