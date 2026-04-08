use borsh::BorshDeserialize;
use close_account::{processor::MyInstruction, state::user::User};
use solana_client::{rpc_client, rpc_config::CommitmentConfig};
use solana_instruction::{AccountMeta, Instruction};
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::Keypair, signer::Signer,
    transaction::Transaction,
};

#[tokio::main]
async fn main() {
    let program_id = Pubkey::from_str_const("5nBx3Vwv9X6Qwa8Nriuz6JfmsjuK3SQKj1FMWSyjy2yf");

    let rpc_client = rpc_client::RpcClient::new_with_commitment(
        "http://localhost:8899",
        CommitmentConfig::confirmed(),
    );

    let payer = Keypair::new();
    let (target_account, _) = Pubkey::find_program_address(
        &[User::SEED_PREFIX.as_bytes(), payer.pubkey().as_array()],
        &program_id,
    );

    let airdrop_res = rpc_client
        .request_airdrop(&payer.pubkey(), LAMPORTS_PER_SOL * 5)
        .unwrap();

    rpc_client
        .confirm_transaction_with_spinner(
            &airdrop_res,
            &rpc_client.get_latest_blockhash().unwrap(),
            CommitmentConfig::confirmed(),
        )
        .unwrap();
    println!("Airdrop to payer successfull !");

    let instruction = Instruction::new_with_borsh(
        program_id,
        &MyInstruction::CreateUser(User {
            name: "Devansh Chauhan-Client".to_string(),
            id: 7,
        }),
        vec![
            AccountMeta::new(target_account, false),
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(solana_system_interface::program::ID, false),
        ],
    );

    let create_txn = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        rpc_client.get_latest_blockhash().unwrap(),
    );

    let create_txn_res = rpc_client.send_and_confirm_transaction_with_spinner(&create_txn);

    assert!(create_txn_res.is_ok(), "{:?}", create_txn_res);

    let created_data =
        User::try_from_slice(&rpc_client.get_account_data(&target_account).unwrap()).unwrap();

    println!("Created user with this details {:#?}", created_data);

    let close_instruction_data = MyInstruction::CloseUser;

    let close_instruction = Instruction::new_with_borsh(
        program_id,
        &close_instruction_data,
        vec![
            AccountMeta::new(target_account, false),
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(solana_system_interface::program::ID, false),
        ],
    );

    let close_txn = Transaction::new_signed_with_payer(
        &[close_instruction],
        Some(&payer.pubkey()),
        &[&payer],
        rpc_client.get_latest_blockhash().unwrap(),
    );

    assert!(
        rpc_client
            .send_and_confirm_transaction_with_spinner(&close_txn)
            .is_ok()
    );
    println!("Successfully closed the user account");

    let closed_account_data = rpc_client.get_account(&target_account);
    println!("Closed Client Account : {:#?}", closed_account_data);
}
