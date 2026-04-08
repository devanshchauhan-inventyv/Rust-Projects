use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize,Debug)]
pub struct User {
    pub name: String,
    pub id: u8,
}

impl User {
    pub const SEED_PREFIX: &'static str = "USER";
}
