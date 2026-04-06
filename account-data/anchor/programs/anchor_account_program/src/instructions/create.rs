use anchor_lang::prelude::*;

use crate::{AddressInfo, CreateAddressInfo};

pub fn create_address_info(ctx: Context<CreateAddressInfo>, name: String, street: String, house_number: u8, city: String) -> Result<()> {
    *ctx.accounts.address_info = AddressInfo::new(name, house_number, street, city);
    Ok(())
}
