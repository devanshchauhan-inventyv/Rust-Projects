use anchor_lang::prelude::*;

use crate::state::user_state::UserState;

#[derive(Accounts)]
pub struct CloseUserContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [
            UserState::SEED_PREFIX.as_bytes(),
            payer.key().as_ref()
        ],
        bump = target_account.bump,
        close = payer
    )]
    pub target_account: Account<'info, UserState>,
}

pub fn close_user(_ctx: Context<CloseUserContext>) -> Result<()> {
    Ok(())
}
