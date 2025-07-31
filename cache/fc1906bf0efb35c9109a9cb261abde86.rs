// Cached result for function: sub_100002cd8
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: fc1906bf0efb35c9109a9cb261abde86.rs

pub struct Initialize;

impl Initialize {
    pub const ACCOUNTS: [AccountMeta; 2] = [
        AccountMeta::new_writable(crate::ID), // Program ID (writable for data modification)
        AccountMeta::new_readonly(crate::ID), // System Program (read-only)
    ];
}

pub fn process_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let program_account = next_account_info(accounts.iter_mut())?;
    let system_program = next_account_info(accounts.iter_mut())?;

    if program_account.key != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    if !program_account.is_writable {
        return Err(ProgramError::InvalidAccountData); // Or a more specific error
    }

    if system_program.key != &system_program::ID {
        return Err(ProgramError::IncorrectProgramId); // System program ID mismatch
    }

    // This instruction does not expect any instruction data.
    if !instruction_data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }

    // The program account's data is initialized to all zeros by default when created.
    // This function effectively acts as a no-op for initialization,
    // as the `Initialize` instruction in the IDL has no arguments and
    // the program account is simply checked for writability and program ID.
    // No actual data is written or modified here beyond the initial account creation.

    Ok(())
}