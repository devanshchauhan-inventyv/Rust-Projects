use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    program::invoke,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use solana_system_interface::instruction::create_account;

use crate::state::address_info::AddressInfo;

pub fn create_address_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    address_info: AddressInfo,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let address_info_account = next_account_info(accounts_iter)?;
    let payer_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let account_len = borsh::to_vec(&address_info)?.len();
    let lamports_required = Rent::get()?.minimum_balance(account_len);

    invoke(
        &create_account(
            payer_account.key,
            address_info_account.key,
            lamports_required,
            account_len as u64,
            program_id,
        ),
        &[
            payer_account.clone(),
            address_info_account.clone(),
            system_program.clone(),
        ],
    )?;

    address_info.serialize(&mut &mut address_info_account.data.borrow_mut()[..])?;
    Ok(())
}
