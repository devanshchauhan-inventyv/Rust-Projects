use close_account::{processor::MyInstruction, state::user::User};
use litesvm::LiteSVM;
use solana_instruction::{AccountMeta, Instruction};
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::Keypair, signer::Signer,
    transaction::Transaction,
};
use solana_system_interface::program;

#[test]
fn check_closing_account() {
    let mut svm = LiteSVM::new();

    let program_id = Pubkey::new_unique();
    let payer = Keypair::new();
    // find pda derived account fro target account
    let target_account = solana_sdk::pubkey::Pubkey::find_program_address(
        &[User::SEED_PREFIX.as_bytes(), payer.pubkey().as_ref()],
        &program_id,
    )
    .0;

    svm.airdrop(&payer.pubkey(), LAMPORTS_PER_SOL * 5).unwrap();

    svm.add_program_from_file(program_id, "../../../target/deploy/close_account.so")
        .unwrap();

    let create_user_data = MyInstruction::CreateUser(User {
        name: "Devansh".to_string(),
        id: 77,
    });

    let instruction = Instruction::new_with_borsh(
        program_id,
        &create_user_data,
        vec![
            AccountMeta::new(target_account, false),
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(program::ID, false),
        ],
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );
    println!("Transaction : {:?}", transaction);
    let txn_result = svm.send_transaction(transaction.clone());
    println!("Transaction Result : {:?}", txn_result);
    assert!(txn_result.is_ok());

    let closed_target_account = svm.get_account(&target_account);
    println!("Newly created Target Account : {:?}", closed_target_account);

    let data = MyInstruction::CloseUser;

    let close_instruction = Instruction::new_with_borsh(
        program_id,
        &data,
        vec![
            AccountMeta::new(target_account, false),
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(program::ID, false),
        ],
    );

    let close_transaction = Transaction::new_signed_with_payer(
        &[close_instruction],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );
    assert!(svm.send_transaction(close_transaction).is_ok());

    let closed_target_account = svm.get_account(&target_account);
    println!("Closed Target Account : {:?}", closed_target_account); // should get a None value
}
