use anchor_lang::prelude::*;

pub mod constants;
pub mod instructions;
pub mod state;
//This is below re-export is added to remove a error for program macro code generation ,
//as its confuses the path when using sub-module,so need to re-export at crate root.
pub use crate::state::*;
declare_id!("3pEVYJLrLesGxAHj86Nk9iWoneK7FuMwqQ6BvpzkdvZb");

#[program]
pub mod account_data_anchor {
    use super::*;

    pub fn create_address_acc(ctx: Context<CreateAddressInfo>, name: String, house_number: u8, street: String, city: String) -> Result<()> {
        instructions::create_address_info(ctx, name, street, house_number, city)
    }
}
