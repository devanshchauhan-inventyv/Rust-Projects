use std::str::FromStr;

use counter_program::CounterInstruction;
use solana_client::{rpc_client::RpcClient, rpc_config::CommitmentConfig};
use solana_instruction::{AccountMeta, Instruction};
use solana_sdk::{
    pubkey::{Pubkey, PubkeyError},
    signature::Keypair,
    signer::Signer,
    transaction::Transaction,
};
use solana_sdk_ids::system_program;

#[tokio::main]
async fn main() {
    let program_id = Pubkey::from_str("B4dPFynYm8uvJGNBi8LdWKcx8yBa1sroRKribcMXmekH")
        .expect("Invalid program ID");

    let rpc_url = String::from("http://localhost:8899");
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let payer = Keypair::new();

    println!("Requesting airdrop");
    let airdrop_signature = client
        .request_airdrop(&payer.pubkey(), 1_000_000_000)
        .expect("Failed to reuqest airdrop");

    loop {
        if client
            .confirm_transaction(&airdrop_signature)
            .unwrap_or(false)
        {
            break;
        }
        println!("Waiting for airdrop confirmation...");
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    println!("Intializing counter...");
    let counter_keypair = Keypair::new();
    let initial_value = 100u64;

    let initialization_data =
        borsh::to_vec(&CounterInstruction::InitializeCounter { initial_value })
            .expect("Failed to serialize intruction");

    let initialize_instruction = Instruction::new_with_bytes(
        program_id,
        &initialization_data,
        vec![
            AccountMeta::new(counter_keypair.pubkey(), true),
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
    );

    let mut transaction =
        Transaction::new_with_payer(&[initialize_instruction], Some(&payer.pubkey()));

    let blockhash = client
        .get_latest_blockhash()
        .expect("Failed to get blockchain");
    transaction.sign(&[&payer, &counter_keypair], blockhash);

    match client.send_and_confirm_transaction(&transaction) {
        Ok(sig) => {
            println!("Counter initialized!");
            println!("Transaction: {}", sig);
            println!("Counter address: {}", counter_keypair.pubkey());
        }
        Err(err) => {
            eprintln!("Failed to initialize counter: {}", err);
            return;
        }
    }

    println!("Incrementing counter...");
    let increment_data = borsh::to_vec(&CounterInstruction::IncrementCounter)
        .expect("Failed to serialize instruction");

    let increment_instruction = Instruction::new_with_bytes(
        program_id,
        &increment_data,
        vec![
            AccountMeta::new(counter_keypair.pubkey(), true),
        ],
    );

    let increment_transaction = Transaction::new_signed_with_payer(
        &[increment_instruction],
        Some(&payer.pubkey()),
        &[&payer, &counter_keypair],
        client.get_latest_blockhash().unwrap(),
    );

    match client.send_and_confirm_transaction(&increment_transaction) {
        Ok(sig) => {
            println!("Counter incremented");
            println!("Transaction: {}", sig);
        }
        Err(err) => {
            eprintln!("Failed to increment counter: {}", err);
            return;
        }
    }
}
