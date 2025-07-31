// Cached result for function: sub_1000004d0
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: a58ae725d94d6ed4f1af7a2601ad291e.rs

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = MyInstruction::try_from_slice(instruction_data)?;

    match instruction {
        MyInstruction::Initialize => {
            let account_info_iter = &mut accounts.iter();
            let mut account_info_iter_mut = accounts.iter_mut();

            let funder_account_info = next_account_info(account_info_iter)?;
            let config_account_info = next_account_info(account_info_iter)?;
            let system_program_account_info = next_account_info(account_info_iter)?;

            let (config_pda_key, config_pda_bump) =
                Pubkey::find_program_address(&[b"config"], program_id);

            if config_pda_key != *config_account_info.key {
                return Err(ProgramError::InvalidSeeds);
            }

            let config_account_data = &mut config_account_info.data.borrow_mut();
            if config_account_data[0] != 0 {
                return Err(ProgramError::AccountAlreadyInitialized);
            }

            let space = 8; // Size of u64
            let lamports = Rent::get()?.minimum_balance(space);

            invoke_signed(
                &system_instruction::create_account(
                    funder_account_info.key,
                    config_account_info.key,
                    lamports,
                    space as u64,
                    program_id,
                ),
                &[
                    funder_account_info.clone(),
                    config_account_info.clone(),
                    system_program_account_info.clone(),
                ],
                &[&[b"config", &[config_pda_bump]]],
            )?;

            config_account_data[0] = 1; // Mark as initialized
            Ok(())
        }
        MyInstruction::SetData { value } => {
            let account_info_iter = &mut accounts.iter();
            let config_account_info = next_account_info(account_info_iter)?;

            let (config_pda_key, _config_pda_bump) =
                Pubkey::find_program_address(&[b"config"], program_id);

            if config_pda_key != *config_account_info.key {
                return Err(ProgramError::InvalidSeeds);
            }

            let mut config_account_data = config_account_info.data.borrow_mut();
            if config_account_data[0] == 0 {
                return Err(ProgramError::UninitializedAccount);
            }

            // Write the u64 value starting from offset 1 (after the initialized byte)
            config_account_data[1..9].copy_from_slice(&value.to_le_bytes());
            Ok(())
        }
    }
}