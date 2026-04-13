use anchor_lang::prelude::*;

#[derive(AnchorDeserialize,AnchorSerialize, Debug)]
pub struct UserDetails {
    pub name: String,
    pub id: u8,
}

#[account]
#[derive(InitSpace)]
pub struct UserState {
    pub bump: u8,
    pub payer_key: Pubkey,
    #[max_len(50)]
    pub name: String,
    pub id: u8,
}

impl UserState {
    pub const SEED_PREFIX: &'static str = "USER";
}
