use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::{instructions::create::create_address_account, state::address_info::AddressInfo};

pub fn process_instruction(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    if let Ok(address) = AddressInfo::try_from_slice(instruction_data) {
        return create_address_account(program_id, accounts, address);
    }
    Ok(())
}
