use anchor_lang::prelude::*;

declare_id!("FTsmRN3PadXd6ksCqwUP6MvvoxE8veNHXy3rJP4Hbk1N");

#[program]
pub mod anchor_close_acoount_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
