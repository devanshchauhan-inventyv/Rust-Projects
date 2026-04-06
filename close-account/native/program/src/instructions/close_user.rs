use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
};

pub fn close_user(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let target_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let system_account = next_account_info(accounts_iter)?;

    if !payer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if target_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    if system_account.key != &solana_system_interface::program::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    let account_span = 0usize;

    let lamports = target_account.lamports();
    **target_account.lamports.borrow_mut() -= lamports;
    **payer.lamports.borrow_mut() += lamports;

    target_account.resize(account_span)?;

    target_account.assign(system_account.key);

    Ok(())
}
