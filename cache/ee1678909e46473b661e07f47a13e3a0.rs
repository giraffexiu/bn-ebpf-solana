// Cached result for function: sub_100000538
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: ee1678909e46473b661e07f47a13e3a0.rs

pub struct Initialize {
    pub authority: Pubkey,
    pub config: Pubkey,
    pub system_program: Pubkey,
}

pub fn initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    authority_bump: u8,
) -> ProgramResult {
    let authority_account = &accounts[0];
    let config_account = &accounts[1];
    let system_program_account = &accounts[2];

    // Check if authority is a signer
    if !authority_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Check if config account is writable and not initialized
    if !config_account.is_writable || config_account.data_len() != 0 {
        return Err(ProgramError::InvalidArgument);
    }

    // Check system program
    if system_program_account.key != &system_program::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Calculate PDA for config account
    let (config_pda, config_bump) = Pubkey::find_program_address(&[b"config"], program_id);
    if config_account.key != &config_pda {
        return Err(ProgramError::InvalidArgument); // Or a custom error for PDA mismatch
    }

    // Create config account via CPI to system program
    let create_account_ix = solana_program::system_instruction::create_account(
        authority_account.key,
        config_account.key,
        Rent::get()?.minimum_balance(Config::LEN),
        Config::LEN as u64,
        program_id,
    );

    solana_program::program::invoke(
        &create_account_ix,
        &[
            authority_account.clone(),
            config_account.clone(),
            system_program_account.clone(),
        ],
    )?;

    // Deserialize config account
    let mut config_data = Config::try_from_slice(&config_account.data.borrow())?;
    config_data.authority = *authority_account.key;
    config_data.bump = config_bump;
    config_data.authority_bump = authority_bump; // Store the authority bump for future use

    // Serialize config data back to account
    config_data.serialize(&mut &mut config_account.data.borrow_mut()[..])?;

    Ok(())
}

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_program,
    sysvar::Sysvar,
};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Config {
    pub authority: Pubkey,
    pub bump: u8,
    pub authority_bump: u8,
}

impl Config {
    pub const LEN: usize = 32 + 1 + 1; // Pubkey + u8 + u8
}