use solana_client::rpc_config::CommitmentConfig;
use solana_instruction::{AccountMeta, Instruction};
use solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction};

#[tokio::main]
async fn main() {
    let program_id = Pubkey::from_str_const("2Ar45LzRBSN6o1bQmoxTXkdP9oEMM4MZeSUcD8mbmhBt");

    let client = solana_client::rpc_client::RpcClient::new_with_commitment("http://localhost:8899", CommitmentConfig::confirmed());

    let payer = Keypair::new();

    let airdrop_sig = client.request_airdrop(&payer.pubkey(), LAMPORTS_PER_SOL * 2).expect("Failed to get airdrop fro payer");

    client.poll_for_signature_confirmation(&airdrop_sig, 1).expect("Failed to confirm airdrop");

    let account_to_change = Keypair::new();
    let account_to_create = Keypair::new();

    let instruction = solana_system_interface::instruction::create_account(&payer.pubkey(), &account_to_change.pubkey(), LAMPORTS_PER_SOL, 0, &program_id);

    let tx = Transaction::new_signed_with_payer(&[instruction], Some(&payer.pubkey()), &[&payer, &account_to_change], client.get_latest_blockhash().unwrap());

    match client.send_and_confirm_transaction_with_spinner(&tx) {
        Ok(sig) => {
            println!("Transaction for creating initial account_to_change sucessfull");
            println!("Transaction Sig : {sig:?}");
        }
        Err(e) => {
            eprintln!("Transaction to create account_to_chnage failed:");
            eprintln!("{e:?}");
            return;
        }
    };

    let main_ix = Instruction::new_with_borsh(program_id, &vec![0], vec![AccountMeta::new(payer.pubkey(), true), AccountMeta::new(account_to_create.pubkey(), true), AccountMeta::new(account_to_change.pubkey(), true), AccountMeta::new_readonly(solana_system_interface::program::ID, false)]);

    let main_tx = Transaction::new_signed_with_payer(&[main_ix], Some(&payer.pubkey()), &[&payer, &account_to_change, &account_to_create], client.get_latest_blockhash().unwrap());

    match client.send_and_confirm_transaction_with_spinner(&main_tx) {
        Ok(sig) => {
            println!("Transaction for creating new account_to_create sucessfull");
            println!("Transaction Sig : {sig:?}");
        }
        Err(e) => {
            eprintln!("Transaction to create account_to_create failed:");
            eprintln!("{e:?}");
            return;
        }
    };
}
