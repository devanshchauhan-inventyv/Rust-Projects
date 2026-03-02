use account_data::state::address_info::AddressInfo;
use borsh::BorshDeserialize;
use litesvm::LiteSVM;
use solana_instruction::{AccountMeta, Instruction};
use solana_sdk::{
    message::Message, native_token::LAMPORTS_PER_SOL, signature::Keypair, signer::Signer,
    transaction::Transaction,
};

#[test]
fn test_account_data() {
    let mut svm = LiteSVM::new();

    let payer = Keypair::new();
    let address_info_account = Keypair::new();
    let program_id = Keypair::new();

    svm.airdrop(&payer.pubkey(), LAMPORTS_PER_SOL * 10).unwrap();

    svm.add_program_from_file(program_id.pubkey(), "../../../target/deploy/account_data.so")
        .unwrap();

    let instruction_data = borsh::to_vec(&AddressInfo {
        name: "Devansh Chauhan".to_string(),
        house_number: 7,
        street: "Sitaram-baug".to_string(),
        city: "Surendranagar".to_string(),
    })
    .unwrap();

    let initialize_instruction = Instruction::new_with_bytes(
        program_id.pubkey(),
        &instruction_data,
        vec![
            AccountMeta::new(address_info_account.pubkey(), true),
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(solana_system_interface::program::ID, false),
        ],
    );

    let message = Message::new(&[initialize_instruction], Some(&payer.pubkey()));

    let transaction = Transaction::new(
        &[&address_info_account, &payer],
        message,
        svm.latest_blockhash(),
    );

    let txn_result = svm.send_transaction(transaction);

    assert!(
        txn_result.is_ok(),
        "Address Account successfully initialized"
    );

    let address_info_account_data = svm
        .get_account(&address_info_account.pubkey())
        .unwrap()
        .data;

    let serialized_address_data = AddressInfo::try_from_slice(&address_info_account_data).unwrap();

    assert_eq!(serialized_address_data.city, "Surendranagar");
    assert_eq!(serialized_address_data.name, "Devansh Chauhan");
    assert_eq!(serialized_address_data.house_number, 7);
    assert_eq!(serialized_address_data.street, "Sitaram-baug");
}
