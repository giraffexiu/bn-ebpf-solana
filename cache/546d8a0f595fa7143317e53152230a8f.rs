// Cached result for function: sub_1000018d8
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: 546d8a0f595fa7143317e53152230a8f.rs

pub struct Initialize;

impl Initialize {
    pub const ACCOUNTS_LEN: usize = 3;
    pub const DATA_LEN: usize = 8; // Discriminator for Initialize instruction

    pub fn instruction(
        program_id: Pubkey,
        payer: Pubkey,
        new_account: Pubkey,
        system_program: Pubkey,
    ) -> Instruction {
        let accounts = vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(new_account, false),
            AccountMeta::new_readonly(system_program, false),
        ];
        let data = solana_program::hash::hashv(&[b"global:initialize"]).to_bytes()[..8].to_vec();
        Instruction {
            program_id,
            accounts,
            data,
        }
    }
}

pub fn process_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Account 0: [signer, writable] payer
    // Account 1: [writable] new_account
    // Account 2: [] system_program

    let payer_account = &accounts[0];
    let new_account = &accounts[1];
    let system_program_account = &accounts[2];

    if instruction_data.len() != Initialize::DATA_LEN {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Check discriminator (first 8 bytes of instruction data)
    let expected_discriminator = solana_program::hash::hashv(&[b"global:initialize"]).to_bytes()[..8];
    if instruction_data[0..8] != expected_discriminator {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Check account properties
    if !payer_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if !payer_account.is_writable {
        return Err(ProgramError::InvalidAccountData); // Or specific error for non-writable
    }
    if !new_account.is_writable {
        return Err(ProgramError::InvalidAccountData); // Or specific error for non-writable
    }
    if system_program_account.key != &solana_program::system_program::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Calculate rent exemption for the new account
    let space = 8; // Size of the new account data (discriminator)
    let rent_exemption_lamports = Rent::get()?.minimum_balance(space);

    // Create the new account via CPI to System Program
    let create_account_ix = solana_program::system_instruction::create_account(
        payer_account.key,
        new_account.key,
        rent_exemption_lamports,
        space as u64,
        program_id,
    );

    let account_infos = [
        payer_account.clone(),
        new_account.clone(),
        system_program_account.clone(),
    ];

    solana_program::program::invoke(
        &create_account_ix,
        &account_infos,
    )?;

    // Write the discriminator to the new account's data
    new_account.data.borrow_mut()[0..8].copy_from_slice(&expected_discriminator);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use solana_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, instruction::Instruction,
        program_error::ProgramError, pubkey::Pubkey, rent::Rent, system_program,
    };
    use std::{cell::RefCell, rc::Rc};

    fn create_account_info(
        key: &Pubkey,
        is_signer: bool,
        is_writable: bool,
        lamports: u64,
        data: