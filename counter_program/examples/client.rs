use std::str::FromStr;

use solana_sdk::pubkey::{Pubkey, PubkeyError};

#[tokio::main]
async fn main() {
    let program_id = Pubkey::from_str("BVg18GRinZ3Jja1Kg9BpRjBGeTB3JEcsVRPz61CQD2ZK")
        .expect("Invalid program ID");

    
}
