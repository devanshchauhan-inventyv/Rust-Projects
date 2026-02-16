#![allow(unexpected_cfgs)]
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use solana_system_interface::instruction::create_account;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Your program logic goes here
    let instruction_data = CounterInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction_data {
        CounterInstruction::InitializeCounter { initial_value } => {
            process_initialize_counter(program_id, accounts, initial_value)?;
        }
        CounterInstruction::IncrementCounter => {
            process_increment_counter(program_id, accounts)?;
        }
    };

    Ok(())
}

fn process_initialize_counter(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    initial_value: u64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let payer_account = next_account_info(accounts_iter)?;
    let counter_account = next_account_info(accounts_iter)?;
    let system_account = next_account_info(accounts_iter)?;

    let account_space = 8;

    let rent = Rent::get()?;
    let required_lamports = rent.minimum_balance(account_space);

    invoke(
        &create_account(
            payer_account.key,
            counter_account.key,
            required_lamports,
            account_space as u64,
            program_id,
        ),
        &[
            payer_account.clone(),
            counter_account.clone(),
            system_account.clone(),
        ],
    )?;

    let counter_data = CounterAccount {
        count: initial_value,
    };

    let mut account_data = &mut counter_account.data.borrow_mut()[..];

    counter_data.serialize(&mut account_data)?;

    msg!("Counter initialized with value: {}", initial_value);

    Ok(())
}

fn process_increment_counter(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let counter_account = next_account_info(accounts_iter)?;

    if counter_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    let mut account_data = counter_account.data.borrow_mut();

    let mut counter_data = CounterAccount::try_from_slice(&account_data)?;

    counter_data.count = counter_data
        .count
        .checked_add(1)
        .ok_or(ProgramError::InvalidAccountData)?;

    counter_data.serialize(&mut &mut account_data[..])?;

    msg!("Counter incremented to {}", counter_data.count);

    Ok(())
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CounterAccount {
    pub count: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum CounterInstruction {
    InitializeCounter { initial_value: u64 },
    IncrementCounter,
}

#[cfg(test)]
mod test {
    use borsh::BorshDeserialize;
    use litesvm::LiteSVM;
    use solana_instruction::{AccountMeta, Instruction};
    use solana_sdk::{
        message::Message, signature::Keypair, signer::Signer,
        transaction::Transaction,
    };
    use solana_sdk_ids::system_program;

    use crate::{CounterAccount, CounterInstruction};

    #[test]
    fn test_counter_program() {
        let mut svm = LiteSVM::new();

        let payer = Keypair::new();

        svm.airdrop(&payer.pubkey(), 1_000_000_000)
            .expect("Failed to airdrop");

        let program_keypair = Keypair::new();
        let program_id = program_keypair.pubkey();

        svm.add_program_from_file(program_id, "../target/deploy/counter_program.so")
            .expect("Failed to add program");

        let counter_keypair = Keypair::new();
        let initial_value = 42;

        println!("Testing counter initialization...");

        let init_instruction_data =
            borsh::to_vec(&CounterInstruction::InitializeCounter { initial_value })
                .expect("Failed to serialize instruction data");

        let initialize_instruction = Instruction::new_with_bytes(
            program_id,
            &init_instruction_data,
            vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(counter_keypair.pubkey(), true),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
        );

        let message = Message::new(&[initialize_instruction], Some(&payer.pubkey()));

        let transaction =
            Transaction::new(&[&payer, &counter_keypair], message, svm.latest_blockhash());

        let result = svm.send_transaction(transaction);
        println!("Initialize transaction result: {:?}", result);
        assert!(result.is_ok(), "Initialize transaction should succeed");

        let logs = result.unwrap().logs;
        println!("Logs: {:?}", logs);

        let account = svm
            .get_account(&counter_keypair.pubkey())
            .expect("Failed to get counter account");

        println!(
            "Pub key of counter account while initializing : {}",
            counter_keypair.pubkey()
        );

        let counter: CounterAccount = CounterAccount::try_from_slice(&account.data)
            .expect("Failed to deserialize counter data");

        assert_eq!(counter.count, 42);
        println!(
            "Counter initialized successfully with value: {}",
            counter.count
        );

        let increment_intruction_data = borsh::to_vec(&CounterInstruction::IncrementCounter)
            .expect("Failed to serilaize increment instruction data");

        let increment_intruction = Instruction::new_with_bytes(
            program_id,
            &increment_intruction_data,
            vec![AccountMeta::new(counter_keypair.pubkey(), true)],
        );

        let increment_message = Message::new(&[increment_intruction], Some(&payer.pubkey()));

        let increment_transaction = Transaction::new(
            &[&payer, &counter_keypair],
            increment_message,
            svm.latest_blockhash(),
        );

        let increment_result = svm.send_transaction(increment_transaction);
        println!("Increment transaction result: {:?}", increment_result);
        assert!(increment_result.is_ok(), "Incremented Counter Successfully");

        let logs = increment_result.unwrap().logs;
        println!("Logs: {:?}", logs);

        let updated_account = svm
            .get_account(&counter_keypair.pubkey())
            .expect("Failed to get counter account after increment");

        println!(
            "Pub key of counter account while incrementing : {}",
            counter_keypair.pubkey()
        );

        let incremented_counter = CounterAccount::try_from_slice(&updated_account.data)
            .expect("Failed to deserialize counter data for increment testing");
        println!("Counter value: {:#?}", incremented_counter);
        assert_eq!(counter.count + 1, incremented_counter.count);
        println!(
            "Counter incremented successfully to value: {}",
            incremented_counter.count
        );
    }
}
