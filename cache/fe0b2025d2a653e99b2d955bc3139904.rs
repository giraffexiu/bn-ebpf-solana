// Cached result for function: sub_1000004a0
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: fe0b2025d2a653e99b2d955bc3139904.rs

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = MyInstruction::try_from_slice(instruction_data)?;

    match instruction {
        MyInstruction::Initialize => {
            let account_info_iter = &mut accounts.iter();
            let admin_account = next_account_info(account_info_iter)?;
            let config_account = next_account_info(account_info_iter)?;
            let system_program = next_account_info(account_info_iter)?;

            // Check if config_account is already initialized
            if config_account.data_len() != 0 {
                return Err(ProgramError::AccountAlreadyInitialized);
            }

            // Check if admin_account is a signer
            if !admin_account.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            }

            // Create account for config_account
            invoke_signed(
                &system_instruction::create_account(
                    admin_account.key,
                    config_account.key,
                    Rent::get()?.minimum_balance(CONFIG_ACCOUNT_SIZE),
                    CONFIG_ACCOUNT_SIZE as u64,
                    program_id,
                ),
                &[
                    admin_account.clone(),
                    config_account.clone(),
                    system_program.clone(),
                ],
                &[],
            )?;

            // Deserialize config_account data
            let mut config_data = Config::try_from_slice(&config_account.data.borrow())?;
            config_data.admin = *admin_account.key;
            config_data.is_initialized = true;
            config_data.serialize(&mut &mut config_account.data.borrow_mut()[..])?;
        }
        MyInstruction::UpdateAdmin { new_admin } => {
            let account_info_iter = &mut accounts.iter();
            let admin_account = next_account_info(account_info_iter)?;
            let config_account = next_account_info(account_info_iter)?;

            // Check if admin_account is a signer
            if !admin_account.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            }

            // Deserialize config_account data
            let mut config_data = Config::try_from_slice(&config_account.data.borrow())?;

            // Check if the current admin matches the signer
            if config_data.admin != *admin_account.key {
                return Err(ProgramError::MissingRequiredSignature); // Or a custom error like Unauthorized
            }

            // Update admin
            config_data.admin = new_admin;
            config_data.serialize(&mut &mut config_account.data.borrow_mut()[..])?;
        }
        MyInstruction::Deposit { amount } => {
            let account_info_iter = &mut accounts.iter();
            let user_account = next_account_info(account_info_iter)?;
            let config_account = next_account_info(account_info_iter)?;
            let system_program = next_account_info(account_info_iter)?;

            // Check if user_account is a signer
            if !user_account.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            }

            // Transfer lamports from user to config account
            invoke(
                &system_instruction::transfer(user_account.key, config_account.key, amount),
                &[user_account.clone(), config_account.clone(), system_program.clone()],
            )?;
        }
        MyInstruction::Withdraw { amount } => {
            let account_info_iter = &mut accounts.iter();
            let admin_account = next_account_info(account_info_iter)?;
            let config_account = next_account_info(account_info_iter)?;

            // Check if admin_account is a signer
            if !admin_account.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            }

            // Deserialize config_account data
            let config