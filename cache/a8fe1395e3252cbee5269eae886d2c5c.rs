// Cached result for function: sub_100000fd8
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: a8fe1395e3252cbee5269eae886d2c5c.rs

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = MyInstruction::try_from_slice(instruction_data)?;

    match instruction {
        MyInstruction::Initialize => {
            let accounts_iter = &mut accounts.iter();
            let mut remaining_accounts = accounts_iter.clone();

            let _ = next_account_info(accounts_iter)?; // system_program
            let _ = next_account_info(accounts_iter)?; // rent_sysvar
            let _ = next_account_info(accounts_iter)?; // clock_sysvar

            let config_account = next_account_info(accounts_iter)?;
            let signer_account = next_account_info(accounts_iter)?;

            let config_data = &mut config_account.try_borrow_mut_data()?;
            let mut config_state = Config::try_from_slice(config_data)?;

            if config_state.is_initialized {
                return Err(ProgramError::AccountAlreadyInitialized);
            }

            if !signer_account.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            }

            config_state.is_initialized = true;
            config_state.admin = *signer_account.key;
            config_state.serialize(&mut &mut config_data[..])?;

            Ok(())
        }
        MyInstruction::Deposit { amount } => {
            let accounts_iter = &mut accounts.iter();
            let mut remaining_accounts = accounts_iter.clone();

            let config_account = next_account_info(accounts_iter)?;
            let user_account = next_account_info(accounts_iter)?;
            let vault_account = next_account_info(accounts_iter)?;
            let token_program = next_account_info(accounts_iter)?;

            let config_data = &mut config_account.try_borrow_mut_data()?;
            let config_state = Config::try_from_slice(config_data)?;

            if !config_state.is_initialized {
                return Err(ProgramError::UninitializedAccount);
            }

            let cpi_accounts = Transfer {
                from: user_account.clone(),
                to: vault_account.clone(),
                authority: user_account.clone(),
            };
            let cpi_context = CpiContext::new(token_program.clone(), cpi_accounts);
            token::transfer(cpi_context, amount)?;

            Ok(())
        }
        MyInstruction::Withdraw { amount } => {
            let accounts_iter = &mut accounts.iter();
            let mut remaining_accounts = accounts_iter.clone();

            let config_account = next_account_info(accounts_iter)?;
            let user_account = next_account_info(accounts_iter)?;
            let vault_account = next_account_info(accounts_iter)?;
            let token_program = next_account_info(accounts_iter)?;

            let config_data = &mut config_account.try_borrow_mut_data()?;
            let config_state = Config::try_from_slice(config_data)?;

            if !config_state.is_initialized {
                return Err(ProgramError::UninitializedAccount);
            }

            if config_state.admin != *user_account.key {
                return Err(ProgramError::MissingRequiredSignature); // Or a custom error like Unauthorized
            }

            let cpi_accounts = Transfer {
                from: vault_account.clone(),
                to: user_account.clone(),
                authority: vault_account.clone(),
            };
            let seeds = &[
                b"vault",
                &config_account.key.to_bytes(),
                &[config_state.vault_bump],
            ];
            let signer_seeds = &[&seeds[..]];
            let cpi_context = CpiContext::new_with_signer(token_program.clone(), cpi_accounts, signer_seeds);
            token::transfer(cpi_context, amount)?;

            Ok(())