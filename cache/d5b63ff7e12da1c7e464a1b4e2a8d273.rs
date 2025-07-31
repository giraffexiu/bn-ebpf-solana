// Cached result for function: sub_100002c70
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: d5b63ff7e12da1c7e464a1b4e2a8d273.rs

pub struct Initialize;

impl Initialize {
    pub const ACCOUNTS: usize = 2;
    pub const DATA_LEN: usize = 8;

    pub fn instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: &[u8],
    ) -> Result<(), ProgramError> {
        let account_info_iter = &mut accounts.iter();
        let _payer_account = next_account_info(account_info_iter)?;
        let _counter_account = next_account_info(account_info_iter)?;

        // No specific instruction data parsing needed for Initialize based on the decompilation,
        // as the data is just the discriminator.
        // The actual initialization logic happens within the CPI to the system program.

        Ok(())
    }
}

pub fn entry(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    if instruction_data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let instruction_discriminator = u64::from_le_bytes(instruction_data[0..8].try_into().unwrap());

    match instruction_discriminator {
        0x1f0b5d84a7703362 => { // Initialize instruction discriminator
            Initialize::instruction(program_id, accounts, instruction_data)
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}