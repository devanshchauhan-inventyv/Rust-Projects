use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::{instructions::{close_user::close_user, create_user::create_user}, state::user::User};

#[derive(BorshDeserialize, BorshSerialize)]
enum MyInstruction {
    CreateUser(User),
    CloseUser,
}
pub fn process_instruction(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let instruction = MyInstruction::try_from_slice(instruction_data)?;

    match instruction {
        MyInstruction::CreateUser(data) => create_user(program_id, accounts, data)?,
        MyInstruction::CloseUser => close_user(program_id, accounts)?,
    }

    Ok(())
}
