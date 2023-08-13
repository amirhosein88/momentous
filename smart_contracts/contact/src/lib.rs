use borsh::{BorshDeserialize, BorshSerialize};
use std::convert::TryFrom;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
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
pub struct Contact {
    pub user_name: String,
    pub last_name: String,
    pub public_key: String,
    pub base_pubkey: String,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct ContactList {
  pub index: Vec<Contact>,
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
    let methodNo=instruction_data[0];
    let data=&instruction_data[1..instruction_data.len()];
    if methodNo == 0{ // Append User
        return append_user(
            accounts,
            data,
        );
    }
    else if methodNo == 1{ // Delete User
        return delete_user(
            accounts,
            data,
        );
    }
    else if methodNo == 2{ // Clear All Data
        return clear(
            accounts,
            data,
        );
    }

    else {
        return Err(ProgramError::InvalidInstructionData);
    }
}


// fn getPDA(accounts: &[AccountInfo]) -> dyn Any {
//     let accounts_iter = &mut accounts.iter();
//     let pda_account = next_account_info(accounts_iter)?;
//     pda_account;
// }

fn append_user(
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult{
    let accounts_iter = &mut accounts.iter();
    let pda_account = next_account_info(accounts_iter)?;

    let mut input_data = Contact::try_from_slice(&instruction_data)?;
    let offset: usize = 4;
    let mut stored_data;

    let data_length_of_stored_data = DataLength::try_from_slice(&pda_account.data.borrow()[..offset])?;

    if data_length_of_stored_data.length > 0{
        let flag = usize::try_from(data_length_of_stored_data.length+u32::try_from(offset).unwrap()).unwrap();
        stored_data = ContactList::try_from_slice(&pda_account.data.borrow()[offset..flag])?;
    }else{
        stored_data = ContactList{
            index:Vec::new(),
        };
    }

    stored_data.index.push(input_data);
    let data_length = DataLength {
        length: u32::try_from(get_instance_packed_len(&stored_data)?).unwrap(),
    };
    data_length.serialize(&mut &mut pda_account.data.borrow_mut()[..4])?;
    stored_data.serialize(&mut &mut pda_account.data.borrow_mut()[4..])?;
     

    
    Ok(())
}
fn delete_user(
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult{
    let accounts_iter = &mut accounts.iter();
    let pda_account = next_account_info(accounts_iter)?;

    let mut input_data = Contact::try_from_slice(&instruction_data)?;
    let offset: usize = 4;
    let mut stored_data;
    let data_length_of_stored_data = DataLength::try_from_slice(&pda_account.data.borrow()[..offset])?;  
    let flag = usize::try_from(data_length_of_stored_data.length+u32::try_from(offset).unwrap()).unwrap();
    stored_data = ContactList::try_from_slice(&pda_account.data.borrow()[offset..flag])?;
    let mut j = 0;
    for ( i, element) in stored_data.index.iter().enumerate(){
        if element.user_name==input_data.user_name{
            if i>0{
                j=i;
            }
        }
    }
    stored_data.index.remove(j);
    let data_length = DataLength {
        length: u32::try_from(get_instance_packed_len(&stored_data)?).unwrap(),
    };
    data_length.serialize(&mut &mut pda_account.data.borrow_mut()[..4])?;
    stored_data.serialize(&mut &mut pda_account.data.borrow_mut()[4..])?;
    Ok(())
}

fn clear(
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult{
    let accounts_iter = &mut accounts.iter();
    let pda_account = next_account_info(accounts_iter)?;
    let offset: usize = 4;
    let mut stored_data;
    stored_data = ContactList{
        index:Vec::new(),
    };

    let data_length = DataLength {
        length: u32::try_from(get_instance_packed_len(&stored_data)?).unwrap(),
    };
    data_length.serialize(&mut &mut pda_account.data.borrow_mut()[..4])?;
    stored_data.serialize(&mut &mut pda_account.data.borrow_mut()[4..])?;
    Ok(())
}

