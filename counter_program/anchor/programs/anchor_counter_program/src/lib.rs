use anchor_lang::prelude::*;

declare_id!("Etsk5VxJ2LfMDWiQRPc6fwQEPTGtoXrstRZA6EQ5DBuy");

#[program]
pub mod anchor_counter_program {
    use super::*;

    pub fn initialize_counter(ctx: Context<Counter>, initial_data: u64) -> Result<()> {
        let counter_program = &mut ctx.accounts.counter;
        counter_program.data = initial_data;

        Ok(())
    }

    pub fn increment_counter(ctx: Context<IncrementCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.data = counter.data.checked_add(1).ok_or(ProgramError::ArithmeticOverflow)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Counter<'info> {
    #[account(init,payer=signer,space= 8+8)]
    pub counter: Account<'info, MyCounter>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct IncrementCounter<'info> {
    #[account(mut)]
    counter: Account<'info, MyCounter>,
}

#[account]
pub struct MyCounter {
    pub data: u64,
}
