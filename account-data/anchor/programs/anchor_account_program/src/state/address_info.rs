use anchor_lang::prelude::*;

use crate::constants::ANCHOR_DISCRIMINATOR_SIZE;

#[derive(Accounts)]
pub struct CreateAddressInfo<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init,payer=payer,space = ANCHOR_DISCRIMINATOR_SIZE + AddressInfo::INIT_SPACE)]
    pub address_info: Account<'info, AddressInfo>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct AddressInfo {
    #[max_len(60)]
    pub name: String,
    pub house_number: u8,
    #[max_len(60)]
    pub street: String,
    #[max_len(60)]
    pub city: String,
}

impl AddressInfo {
    pub fn new(name: String, house_number: u8, street: String, city: String) -> Self {
        AddressInfo { name, house_number, street, city }
    }
}
