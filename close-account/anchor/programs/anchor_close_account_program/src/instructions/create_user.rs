use anchor_lang::prelude::*;

use crate::state::user_state::{UserDetails, UserState};

#[derive(Accounts)]
pub struct CreateUserContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = UserState::INIT_SPACE,
        seeds = [
            UserState::SEED_PREFIX.as_bytes(),
            payer.key().as_ref()
            ],
        bump

    )]
    pub target_account: Account<'info, UserState>,
    pub system_program: Program<'info, System>,
}

pub fn create_user(ctx: Context<CreateUserContext>, user_details: UserDetails) -> Result<()> {
    *ctx.accounts.target_account = UserState {
        bump: ctx.bumps.target_account,
        payer_key: ctx.accounts.payer.key(),
        name: user_details.name,
        id: user_details.id,
    };
    Ok(())
}
