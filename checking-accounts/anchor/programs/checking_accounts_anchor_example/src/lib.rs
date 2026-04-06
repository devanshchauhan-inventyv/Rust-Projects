use anchor_lang::prelude::*;

declare_id!("Fm1wSbTj7WDRuipT5WyQfohVcJcnwBchFxG5ZYC547BB");

#[program]
pub mod checking_accounts_anchor {
    use super::*;

    pub fn check_accounts(_ctx: Context<CheckingAccounts>) -> Result<()> {
        msg!("All the constraints have been checked and verified successfully");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CheckingAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: We are just checking if it is uninitialized (lamports == 0)
    #[account(
        signer,
        constraint = accounts_to_create.lamports() == 0 @ CustomError::AlreadyInitialized
    )]
    pub accounts_to_create: AccountInfo<'info>,
    /// CHECK: We are just checking if it is already initialized (lamports > 0)
    #[account(
        mut,
        owner = *__program_id @ CustomError::IncorrectOwner,
        constraint = account_to_change.lamports() > 0 @ CustomError::Uninitialized
    )]
    pub account_to_change: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum CustomError {
    #[msg("Account already initialized")]
    AlreadyInitialized,
    #[msg("Account not initialized")]
    Uninitialized,
    #[msg("Incorrect Owner")]
    IncorrectOwner,
}
