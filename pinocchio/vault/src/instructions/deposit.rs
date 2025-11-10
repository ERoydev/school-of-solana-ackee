use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::find_program_address, ProgramResult};
use pinocchio_system::instructions::Transfer;


// The input structure
// It will contain all the required accounts that are needed for this instruction
pub struct Deposit<'a> {
    pub accounts: DepositAccounts<'a>,
    pub instruction_data: DepositInstructionData,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for Deposit<'a> {
    type Error = ProgramError;

    // And here i execute the bellow validation and parsing logic
    fn try_from((accounts, data): (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let accounts = DepositAccounts::try_from(accounts)?;
        let instruction_data = DepositInstructionData::try_from(data)?;


        Ok(Self { accounts: accounts, instruction_data: instruction_data })
    }
}

impl<'a> Deposit<'a> {
    // I have to define a Discriminator, so i can safely and correctly distinguish between different instructions
    // We have to get the Discriminator manually here
    pub const DISCRIMINATOR: &'a u8 = &0;

    // Executor of this instruction, and holder of all the business logic of this instruction
    pub fn process(&mut self) -> ProgramResult {
        // I send from owner to vault some lamports
        Transfer {
            from: self.accounts.owner,
            to: self.accounts.vault,
            lamports: self.instruction_data.amount
        }
        .invoke()
    }
}

// Definitions of all deposit accounts
pub struct DepositAccounts<'a> {
    pub owner: &'a AccountInfo,
    pub vault: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for DepositAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [owner, vault, _] = accounts else {
            return Err(ProgramError::InvalidAccountData);
        };

        // Validate that the owner is `singer` because in this example he wants to send to vault
        if !owner.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Check if vault is system_program -> check if its owned by our pinocchio systemID
        if !vault.is_owned_by(&pinocchio_system::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        // Check if vault is empty
        if vault.lamports() != 0 {
            return Err(ProgramError::InvalidAccountData);
        }

        // Check if a proper vault account has been provided to our instruction

        let (vault_key, _) = find_program_address(&[b"vault", owner.key()], &crate::ID);
        if vault.key() != &vault_key {
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(Self { owner: owner, vault: vault })
    }   

    
}

pub struct DepositInstructionData {
    pub amount: u64,
}

// Implement parsing and validation of the `DepositInstructionData` by implementing `TryFrom` trait
impl<'a> TryFrom<&'a [u8]> for DepositInstructionData {
    type Error = ProgramError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        // Validate
        if data.len() != size_of::<u64>() {
            return Err(ProgramError::InvalidInstructionData);
        }

        // Parse
        let amount = u64::from_le_bytes(data.try_into().unwrap());

        // Additional validation
        if amount == 0 {
            return Err(ProgramError::InvalidInstructionData);
        }

        // If everything is fine, return the initialized struct
        Ok(Self { amount })
    }
}


// So first since DepositAccounts is a reference it should have a lifetime parameter
// And thats why i give a lifetime to the pub accounts so the referenced values will live long enough to be stored in accounts

/*  

When you use references in a struct (like &AccountInfo), 
Rust needs to know how long those references are valid. 
You do this by adding a lifetime parameter to the struct:

    pub struct DepositAccounts<'a> {
        pub owner: &'a AccountInfo,
        pub vault: &'a AccountInfo,
    }

Then, any struct that contains DepositAccounts (like Deposit) 
must also have the same lifetime parameter 
if it stores DepositAccounts by value:

    pub struct Deposit<'a> {
        pub accounts: DepositAccounts<'a>,
        pub instruction_data: DepositInstructionData,
    }


Note:
    The 'a in pub struct Deposit<'a> is just a placeholder for a lifetime. 
    It lets you specify how long the references inside DepositAccounts<'a> must be valid.

*/