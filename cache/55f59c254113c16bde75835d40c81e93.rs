// Cached result for function: sub_100002db8
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: 55f59c254113c16bde75835d40c81e93.rs

pub struct Initialize;

impl Initialize {
    pub const ACCOUNTS: usize = 1;
    pub const DATA: usize = 0;
}

pub fn process_initialize(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let pda_account = next_account_info(account_info_iter)?;

    if pda_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    if pda_account.data_len() != 8 {
        return Err(ProgramError::InvalidAccountData);
    }

    let mut pda_data = pda_account.try_borrow_mut_data()?;
    let value = u64::from_le_bytes(instruction_data.try_into().map_err(|_| ProgramError::InvalidInstructionData)?);
    pda_data[0..8].copy_from_slice(&value.to_le_bytes());

    Ok(())
}