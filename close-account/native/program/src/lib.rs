pub mod instructions;
pub mod processor;
pub mod state;
use solana_program::entrypoint;

use crate::processor::process_instruction;

entrypoint!(process_instruction);
