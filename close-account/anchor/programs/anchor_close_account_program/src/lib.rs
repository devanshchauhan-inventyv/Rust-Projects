use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;

use instructions::*;
use state::*;

declare_id!("FJduhFdXgZK29J6riGCsyMMy4CXKCjioXugMgdwyjJ6g");

#[program]
pub mod anchor_close_account_program {
    use super::*;

    pub fn create_user(ctx: Context<CreateUserContext>, user_details: UserDetails) -> Result<()> {
        instructions::create_user::create_user(ctx, user_details)
    }

    pub fn close_user(ctx: Context<CloseUserContext>) -> Result<()> {
        instructions::close_user::close_user(ctx)
    }
}
