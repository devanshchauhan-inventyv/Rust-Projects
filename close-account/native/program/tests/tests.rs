use close_account::state::user::User;
use litesvm::LiteSVM;
use solana_instruction::{AccountMeta, Instruction};
use solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction};
use solana_system_interface::program;

fn check_closing_account() {
    let mut svm = LiteSVM::new();

    let program_id = Pubkey::new_unique();
    let payer = Keypair::new();
    let target_account = Keypair::new();

    svm.airdrop(&payer.pubkey(), LAMPORTS_PER_SOL * 5).unwrap();

    svm.add_program_from_file(program_id, "../../../../target/deploy/close_account.so").unwrap();

    let create_user_data = User { name: "Devansh".to_string(), id: 77 };

    let instruction = Instruction::new_with_borsh(program_id, &create_user_data, vec![AccountMeta::new(target_account.pubkey(), true), AccountMeta::new(payer.pubkey(), true), AccountMeta::new_readonly(program::ID, false)]);

    let transaction = Transaction::new_signed_with_payer(&[instruction], Some(&payer.pubkey()), &[&payer, &target_account], svm.latest_blockhash());
    assert!(svm.send_transaction(transaction).is_ok());
}
