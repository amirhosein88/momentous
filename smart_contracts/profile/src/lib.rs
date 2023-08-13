use borsh::{BorshDeserialize, BorshSerialize};
use std::convert::TryFrom;
use std::string::ToString;
use solana_program::{
    account_info::{next_account_info, AccountInfo, next_account_infos},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
    borsh::get_instance_packed_len
};

#[derive(BorshDeserialize,BorshSerialize,Debug)]
pub struct Profile {
    pub userName: String,
    pub createdTime: String,
    pub conversation_list: Vec<Conversation>,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Conversation {
    pub conversation_id: String,
    pub conversation_name: String,
}


#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct DataLength {
  pub length: u32,
}

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    if instruction_data[0] == 0{
        return create_profile(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );
    }
    else if instruction_data[0] == 1{
        return add_conversation(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        )
    }
    else if instruction_data[0] == 2{
        return delete_profile(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        )
    }
    else {
        return Err(ProgramError::InvalidInstructionData);
    }
}

fn create_profile(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult{
    let accounts_iter = &mut accounts.iter();
    let pda_account = next_account_info(accounts_iter)?;
    let mut input_data = Profile::try_from_slice(&instruction_data)?;
    let data_length = DataLength {
        length: u32::try_from(get_instance_packed_len(&input_data)?).unwrap(),
    };
    data_length.serialize(&mut &mut pda_account.data.borrow_mut()[..4])?;
    input_data.serialize(&mut &mut pda_account.data.borrow_mut()[4..])?;
    Ok(())
}

fn add_conversation(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult{
    let accounts_iter = &mut accounts.iter();
    for pda_account in next_account_infos(accounts_iter,accounts_iter.len())?{
        let mut input_data = Conversation::try_from_slice(&instruction_data)?;
        let data_length = DataLength::try_from_slice(&pda_account.data.borrow()[..4])?;
        let length = usize::try_from(data_length.length + u32::try_from(4).unwrap()).unwrap();
        let mut stored_data = Profile::try_from_slice(&pda_account.data.borrow()[4..length])?;
        stored_data.conversation_list.push(input_data);
        let data_length = DataLength {
            length: u32::try_from(get_instance_packed_len(&stored_data)?).unwrap(),
        };
        data_length.serialize(&mut &mut pda_account.data.borrow_mut()[..4])?;
        stored_data.serialize(&mut &mut pda_account.data.borrow_mut()[4..])?;
    }
    Ok(())
}

fn delete_profile(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult{
    let accounts_iter = &mut accounts.iter();
    let pda_account = next_account_info(accounts_iter)?;
    let mut data_length = DataLength{
        length:0
    };
    data_length.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    Ok(())
}
