use crate::state::user::User;
use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};

pub fn create_user(program_id: &Pubkey, accounts: &[AccountInfo], data: User) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let target_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let data_len = borsh::to_vec(&data)?.len();
    let required_lamports = Rent::get()?.minimum_balance(data_len);

    let (pda, bump) = Pubkey::find_program_address(&[User::SEED_PREFIX.as_bytes(), payer.key.as_ref()], program_id);

    if &pda != target_account.key {
        return Err(ProgramError::InvalidArgument);
    }

    invoke_signed(&solana_system_interface::instruction::create_account(payer.key, target_account.key, required_lamports, data_len as u64, program_id), &[payer.clone(), target_account.clone(), system_program.clone()], &[&[User::SEED_PREFIX.as_bytes(), payer.key.as_ref(), &[bump]]])?;

    data.serialize(&mut &mut target_account.data.borrow_mut()[..])?;
    Ok(())
}
