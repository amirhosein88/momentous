use solana_program::pubkey::Pubkey;
use std::str::FromStr;

fn main() {
    let program_id = Pubkey::from_str("3BGuH21t5dCVH8JvoFxsxhE3k3vKyPeEERL6oLRHSgKy").unwrap();

    let (pda, bump_seed) = Pubkey::find_program_address(&[b"test"], &program_id);
    println!("pda: {}, bump: {}", pda, bump_seed);
}


use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};
use solana_sdk::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::create_account_with_seed,
    sysvar::{rent::Rent, Sysvar},
    transaction::Transaction,
};

// Define your program's entry point
#[solana_program::entrypoint]
pub fn create_pda(
    program_id: &Pubkey,
    accounts: &mut [AccountInfo],
    seed: String,
    space: u64,
    lamports: u64,
) -> ProgramResult {
    // Extract account information from the provided accounts slice
    let account_info_iter = &mut accounts.iter();
    let payer = next_account_info(account_info_iter)?;

    // Create a PDA using the provided seed
    let pda = Pubkey::create_with_seed(payer.key, &seed, program_id)?;

    // Get the minimum balance required for rent exemption
    let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;
    let rent_exempt = rent.minimum_balance(space as usize);

    // Check if the payer has enough lamports for the operation
    if payer.lamports() < lamports + rent_exempt {
        return Err(ProgramError::InsufficientFunds);
    }
//
    // Create an account with the provided seed and parameters
    let create_account_instruction = create_account_with_seed(
        payer.key,
        &pda,
        payer.key,
        &seed,
        lamports,
        space,
        program_id,
    );
//
//     // Build and sign the transaction
//     let mut transaction = Transaction::new_with_payer(&[create_account_instruction], Some(payer.key));
//     transaction.sign(&[payer], recent_blockhash);
//     let signature = solana_sdk::blockhash::





__________________________________________________________________


// use solana_program::{
//     program_pack::{IsInitialized, Pack, Sealed},
//     pubkey::Pubkey,
//     sysvar::{rent::Rent, Sysvar},
// };
// use solana_sdk::{
//     account_info::{next_account_info, AccountInfo},
//     entrypoint::ProgramResult,
//     program_error::ProgramError,
//     program_pack::Pack,
//     pubkey::Pubkey,
//     rent::Rent,
//     system_instruction::create_account_with_seed,
//     sysvar::{rent::Rent, Sysvar},
//     transaction::Transaction,
// };
//
// #[solana_program::entrypoint]
// pub fn create_pda(
//     program_id: &Pubkey,
//     accounts: &mut [AccountInfo],
//     seed: String,
//     space: u64,
//     lamports: u64,
// ) -> ProgramResult {
//     let account_info_iter = &mut accounts.iter();
//     let payer = next_account_info(account_info_iter)?;
//
//     let pda = Pubkey::create_with_seed(payer.key, &seed, program_id)?;
//
//     let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;
//     let rent_exempt = rent.minimum_balance(space as usize);
//
//     if payer.lamports() < lamports + rent_exempt {
//         return Err(ProgramError::InsufficientFunds);
//     }
//
//     let create_account_instruction = create_account_with_seed(
//         payer.key,
//         &pda,
//         payer.key,
//         &seed,
//         lamports,
//         space,
//         program_id,
//     );
//
//     let mut transaction = Transaction::new_with_payer(&[create_account_instruction], Some(payer.key));
//     let recent_blockhash = solana_sdk::blockhash::Blockhash::default(); // Replace with a valid blockhash
//     transaction.sign(&[payer], recent_blockhash);
//     let signature = solana_sdk::blockhash::Blockhash::default(); // Replace with a valid blockhash
//
//     solana_sdk::processor::Processor::process(&[create_account_instruction], &mut accounts, &signature)?;
//
//     Ok(())
// }

// Add other necessary dependencies and entry point function
// ...

