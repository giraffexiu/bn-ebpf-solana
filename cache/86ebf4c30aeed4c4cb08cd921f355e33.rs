// Cached result for function: sub_1000018e8
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: 86ebf4c30aeed4c4cb08cd921f355e33.rs

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

            let config_account = next_account_info(accounts_iter)?;
            let signer_account = next_account_info(accounts_iter)?;

            let (config_pda, config_bump) =
                Pubkey::find_program_address(&[b"config"], program_id);

            if config_pda != *config_account.key {
                return Err(ProgramError::InvalidSeeds);
            }

            if !signer_account.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            }

            let config_data = Config {
                signer: *signer_account.key,
                bump: config_bump,
            };

            config_data.serialize(&mut &mut config_account.data.borrow_mut()[..])?;

            Ok(())
        }
        MyInstruction::Update { new_signer } => {
            let accounts_iter = &mut accounts.iter();
            let mut remaining_accounts = accounts_iter.clone();

            let config_account = next_account_info(accounts_iter)?;
            let signer_account = next_account_info(accounts_iter)?;

            let (config_pda, _) = Pubkey::find_program_address(&[b"config"], program_id);

            if config_pda != *config_account.key {
                return Err(ProgramError::InvalidSeeds);
            }

            if !signer_account.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            }

            let mut config_data = Config::try_from_slice(&config_account.data.borrow())?;

            if config_data.signer != *signer_account.key {
                return Err(ProgramError::MissingRequiredSignature); // Or a custom error like Unauthorized
            }

            config_data.signer = new_signer;
            config_data.serialize(&mut &mut config_account.data.borrow_mut()[..])?;

            Ok(())
        }
        MyInstruction::Close => {
            let accounts_iter = &mut accounts.iter();
            let mut remaining_accounts = accounts_iter.clone();

            let config_account = next_account_info(accounts_iter)?;
            let signer_account = next_account_info(accounts_iter)?;
            let destination_account = next_account_info(accounts_iter)?;

            let (config_pda, _) = Pubkey::find_program_address(&[b"config"], program_id);

            if config_pda != *config_account.key {
                return Err(ProgramError::InvalidSeeds);
            }

            if !signer_account.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            }

            let config_data = Config::try_from_slice(&config_account.data.borrow())?;

            if config_data.signer != *signer_account.key {
                return Err(ProgramError::MissingRequiredSignature); // Or a custom error like Unauthorized
            }

            let lamports_to_transfer = config_account.lamports();
            **destination_account.lamports.borrow_mut() += lamports_to_transfer;
            **config_account.lamports.borrow_mut() = 0;

            Ok(())
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum MyInstruction {
    Initialize,
    Update {