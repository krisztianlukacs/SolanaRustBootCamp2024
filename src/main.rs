// This is a Solna BootCamp Exam project. Krisztian Lukacs, Group 5
// 2024.09.07

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    msg,
};
use borsh::{BorshDeserialize, BorshSerialize};
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct State {
    pub key: String,
    pub value: String, // store values as JSON strings
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct History {
    pub key: String,
    pub history: Vec<(u64, String)>, // timestamp and the value
}

// Entry point for the Solana program
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    // Deserialize the input instruction data
    let instruction: Instruction = Instruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    // Perform the requested action
    match instruction {
        Instruction::SetValue { key, value } => set_value(account, key, value)?,
        Instruction::GetValue { key } => get_value(account, key)?,
        Instruction::GetHistory { key } => get_history(account, key)?,
    }

    Ok(())
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Instruction {
    SetValue { key: String, value: String },
    GetValue { key: String },
    GetHistory { key: String },
}

// Function to set or update a value in the key-value store
fn set_value(account: &AccountInfo, key: String, value: String) -> ProgramResult {
    let mut state = State::try_from_slice(&account.data.borrow())?;
    let mut history = History::try_from_slice(&account.data.borrow())?;

    // Update the current value
    state.key = key.clone();
    state.value = value.clone();

    // Record the previous value in history with a timestamp (using the slot as a timestamp)
    let current_slot = solana_program::clock::Clock::get()?.slot;
    history.history.push((current_slot, value));

    // Serialize and store the new state and history
    state.serialize(&mut *account.data.borrow_mut())?;
    history.serialize(&mut *account.data.borrow_mut())?;

    msg!("Set value for key: {}, new value: {}", key, value);

    Ok(())
}

// Function to get the current value for a key
fn get_value(account: &AccountInfo, key: String) -> ProgramResult {
    let state = State::try_from_slice(&account.data.borrow())?;

    if state.key == key {
        msg!("Current value for key {}: {}", key, state.value);
    } else {
        msg!("Key not found");
    }

    Ok(())
}

// Function to get the history of a key
fn get_history(account: &AccountInfo, key: String) -> ProgramResult {
    let history = History::try_from_slice(&account.data.borrow())?;

    if history.key == key {
        msg!("History for key {}: {:?}", key, history.history);
    } else {
        msg!("Key not found");
    }

    Ok(())
}
