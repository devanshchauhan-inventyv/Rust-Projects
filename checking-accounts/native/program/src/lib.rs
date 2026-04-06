use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

//This program is a simple example of how to create a new account and change an existing account's data.
//It does not do anything useful, but it serves as a starting point for learning how to write Solana programs and validate it in native rust coding.
fn process_instruction(program_id: &Pubkey, accounts: &[AccountInfo], _instruction_data: &[u8]) -> ProgramResult {
    if solana_system_interface::program::check_id(program_id) {
        return Err(ProgramError::IncorrectProgramId);
    }

    if accounts.len() < 4 {
        msg!("This instuction requires 4 accounts:");
        msg!("  payer,account_to_create,account_to_chnage,system_program");
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let account_to_create = next_account_info(accounts_iter)?;
    let account_to_change = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    if account_to_create.lamports() != 0 {
        msg!("The program expected the account_to_create to not yet be initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    if account_to_change.lamports() == 0 {
        msg!("The program ewxpected account_to_change to be already initialized");
        return Err(ProgramError::UninitializedAccount);
    }

    if !payer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !account_to_change.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    if !account_to_create.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if account_to_change.owner != program_id {
        msg!("Account to change does not have the correct program id.");
        return Err(ProgramError::IncorrectProgramId);
    }

    if system_program.key != &solana_system_interface::program::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    Ok(())
}
