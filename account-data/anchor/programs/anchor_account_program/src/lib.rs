use anchor_lang::prelude::*;

pub mod constants;
pub mod instructions;
pub mod state;

declare_id!("5caWRBTBmP1m5dokghPBeRsaBDUXmR1CBegzdoT9Jw42");

#[program]
pub mod account_data_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
