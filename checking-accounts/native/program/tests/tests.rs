use litesvm::LiteSVM;
use solana_instruction::{AccountMeta, Instruction};
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::Keypair, signer::Signer,
    transaction::Transaction,
};
use solana_system_interface::instruction::create_account;

#[test]
fn test_checking_accounts() {
    let mut svm = LiteSVM::new();

    let payer = Keypair::new();
    let account_to_change = Keypair::new();
    let account_to_create = Keypair::new();

    svm.airdrop(&payer.pubkey(), LAMPORTS_PER_SOL * 10).unwrap();

    let program_id = Pubkey::new_unique();

    svm.add_program_from_file(
        program_id,
        "../../../target/deploy/checking_accounts_native.so",
    )
    .unwrap();

    let ix = create_account(
        &payer.pubkey(),
        &account_to_change.pubkey(),
        LAMPORTS_PER_SOL,
        0,
        &program_id,
    );

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &account_to_change],
        svm.latest_blockhash(),
    );

    assert!(svm.send_transaction(tx).is_ok());

    let main_create_ix = Instruction::new_with_borsh(
        program_id,
        &vec![0],
        vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(account_to_create.pubkey(), true),
            AccountMeta::new(account_to_change.pubkey(), true),
            AccountMeta::new_readonly(solana_system_interface::program::ID, false),
        ],
    );

    let main_create_tx = Transaction::new_signed_with_payer(
        &[main_create_ix],
        Some(&payer.pubkey()),
        &[&payer, &account_to_change, &account_to_create],
        svm.latest_blockhash(),
    );

    assert!(svm.send_transaction(main_create_tx).is_ok());
}
