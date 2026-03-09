use std::str::FromStr;

use account_data::state::address_info::AddressInfo;
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::CommitmentConfig};
use solana_instruction::{AccountMeta, Instruction};
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::Keypair, signer::Signer,
    transaction::Transaction,
};

#[tokio::main]
async fn main() {
    let program_id = Pubkey::from_str("7hWfAm2pVgXYjY4RPk3yNQiaJAfQXcHrw2Yen7QiFgnG")
        .expect("Invalid Program Id");

    let rpc_url = String::from("http://localhost:8899");
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let payer = Keypair::new();

    println!("Requesting Airdrop");
    match client
        .request_airdrop(&payer.pubkey(), LAMPORTS_PER_SOL)
        .await
    {
        Ok(airdrop_sig) => {
            let recent_blockhash = client.get_latest_blockhash().await.unwrap();
            match client
                .confirm_transaction_with_spinner(
                    &airdrop_sig,
                    &recent_blockhash,
                    CommitmentConfig::confirmed(),
                )
                .await
            {
                Ok(_) => println!("Airdrop Succesfull!"),
                Err(e) => {
                    eprintln!("Failed to confirm airdrop transaction, Error => {e}");
                    return;
                }
            }
        }
        Err(err) => {
            eprintln!("Failed to airdrop for payer, Error -> {err}");
            return;
        }
    }

    let address_acc_kp = Keypair::new();

    let address_info = AddressInfo {
        name: "Devansh Chauhan".to_string(),
        house_number: 7,
        street: "Sitram Baug".to_string(),
        city: "Surendranagar".to_string(),
    };

    // let address_data = borsh::to_vec()

    let address_init_instuction = Instruction::new_with_borsh(
        program_id,
        &address_info,
        vec![
            AccountMeta::new(address_acc_kp.pubkey(), true),
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(solana_system_interface::program::ID, false),
        ],
    );

    let address_init_txn = Transaction::new_signed_with_payer(
        &[address_init_instuction],
        Some(&payer.pubkey()),
        &[address_acc_kp, payer],
        client
            .get_latest_blockhash()
            .await
            .expect("Failed to get latest blockhash"),
    );

    match client.send_and_confirm_transaction(&address_init_txn).await {
        Ok(sig) => {
            println!("Address Account initilaized with data : {address_info:?}");
            println!("Transaction Signature : {sig}");
        }
        Err(err) => {
            eprintln!("Failed to initialize address account, Error -> {err}");
            return;
        }
    };
}
