use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, next_account_infos, AccountInfo},
    borsh::get_instance_packed_len,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use std::convert::TryFrom;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Conversation {
    pub conversation_name: String,
    pub createdTime: String,
    pub messages: Vec<Message>,
    pub members: Vec<User>,
    pub admin: User,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Message {
    pub messages_id: String,
    pub text: String,
    pub time: String,
    pub seen_by: Vec<User>,
    pub sender_address: String,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct User {
    pub user_address: String,
    pub user_name: String,
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
    if instruction_data[0] == 0 {
        return create_converstion(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );
    } else if instruction_data[0] == 1 {
        return add_member(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );
    } else if instruction_data[0] == 2 {
        return send_message(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );
    } else if instruction_data[0] == 3 {
        return left_user(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );
    } else {
        return Err(ProgramError::InvalidInstructionData);
    }
}

fn create_converstion(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let pda_account = next_account_info(accounts_iter)?;
    let size = 1024 * 1024;
    let mut data = vec![0u8; size];
    let mut input_data = Conversation::try_from_slice(&instruction_data)?;

    input_data.messages = Vec::<Message>::new();

    let data_length = DataLength {
        length: u32::try_from(get_instance_packed_len(&input_data)?).unwrap(),
    };

    data_length.serialize(&mut &mut pda_account.data.borrow_mut()[..4])?;
    input_data.serialize(&mut &mut pda_account.data.borrow_mut()[4..])?;
    pda_account.data.borrow_mut()[..size].copy_from_slice(&data);
    Ok(())
}

fn add_member(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let admin = next_account_info(accounts_iter)?;
    let mut input_data = User::try_from_slice(&instruction_data)?;
    let data_length = DataLength::try_from_slice(&admin.data.borrow()[..4])?;
    let length = usize::try_from(data_length.length + u32::try_from(4).unwrap()).unwrap();
    let mut stored_data = Conversation::try_from_slice(&admin.data.borrow()[4..length])?;
    stored_data.members.push(input_data);
    let data_length = DataLength {
        length: u32::try_from(get_instance_packed_len(&stored_data)?).unwrap(),
    };
    data_length.serialize(&mut &mut admin.data.borrow_mut()[..4])?;
    stored_data.serialize(&mut &mut admin.data.borrow_mut()[4..])?;
    
    Ok(())
}

fn send_message(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let admin = next_account_info(accounts_iter)?;
    let mut input_data = Message::try_from_slice(&instruction_data)?;
    let data_length = DataLength::try_from_slice(&admin.data.borrow()[..4])?;
    let length = usize::try_from(data_length.length + u32::try_from(4).unwrap()).unwrap();
    let mut stored_data = Conversation::try_from_slice(&admin.data.borrow()[4..length])?;
    stored_data.messages.push(input_data);
    let data_length = DataLength {
        length: u32::try_from(get_instance_packed_len(&stored_data)?).unwrap(),
    };
    data_length.serialize(&mut &mut admin.data.borrow_mut()[..4])?;
    stored_data.serialize(&mut &mut admin.data.borrow_mut()[4..])?;
    Ok(())
}

fn left_user(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let pda_account = next_account_info(accounts_iter)?;
    let target_user = next_account_info(accounts_iter)?;
    let data_length = DataLength::try_from_slice(&pda_account.data.borrow()[..4])?;
    let length = usize::try_from(data_length.length + u32::try_from(4).unwrap()).unwrap();
    let mut stored_data = Conversation::try_from_slice(&pda_account.data.borrow()[4..length])?;
    let mut j = 0;
    for (i, element) in stored_data.members.iter().enumerate() {
        if element.user_address == target_user.key.to_string() {
            j = i;
        }
    }
    stored_data.members.remove(j);
    let data_length = DataLength {
        length: u32::try_from(get_instance_packed_len(&stored_data)?).unwrap(),
    };
    data_length.serialize(&mut &mut pda_account.data.borrow_mut()[..4])?;
    stored_data.serialize(&mut &mut pda_account.data.borrow_mut()[4..])?;
    Ok(())
}
