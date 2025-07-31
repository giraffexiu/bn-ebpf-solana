// Cached result for function: sub_100001758
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: adfb0d6d0894d49a7bf304c0d495c74a.rs

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction_discriminant = instruction_data[0];
    match instruction_discriminant {
        0 => {
            let mut instruction_data_slice = &instruction_data[8..];
            let amount = u64::from_le_bytes(instruction_data_slice[0..8].try_into().unwrap());
            instruction_data_slice = &instruction_data_slice[8..];
            let _padding = instruction_data_slice[0]; // Padding byte
            let _ = instruction_data_slice[1]; // Another padding byte
            let _ = instruction_data_slice[2]; // Another padding byte
            let _ = instruction_data_slice[3]; // Another padding byte

            let account_info_iter = &mut accounts.iter();
            let source_account = next_account_info(account_info_iter)?;
            let destination_account = next_account_info(account_info_iter)?;
            let system_program = next_account_info(account_info_iter)?;

            // Check if source_account is a signer
            if !source_account.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            }

            // Check if source_account is writable
            if !source_account.is_writable {
                return Err(ProgramError::InvalidAccountData);
            }

            // Check if destination_account is writable
            if !destination_account.is_writable {
                return Err(ProgramError::InvalidAccountData);
            }

            // Check if system_program is the correct program ID
            if system_program.key != &system_program::id() {
                return Err(ProgramError::IncorrectProgramId);
            }

            // Create the instruction for transferring lamports
            let transfer_instruction = system_instruction::transfer(
                source_account.key,
                destination_account.key,
                amount,
            );

            // Invoke the system program to transfer lamports
            invoke(
                &transfer_instruction,
                &[
                    source_account.clone(),
                    destination_account.clone(),
                    system_program.clone(),
                ],
            )?;
            Ok(())
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

// Helper function to get the next account info from an iterator
fn next_account_info<'a, 'b>(
    iter: &mut std::slice::Iter<'a, AccountInfo<'b>>,
) -> Result<&'a AccountInfo<'b>, ProgramError> {
    iter.next().ok_or(ProgramError::NotEnoughAccountKeys)
}

// Inline the `invoke` function from `solana_program::program`
#[inline(always)]
pub fn invoke(instruction: &Instruction, account_infos: &[AccountInfo]) -> ProgramResult {
    let instruction_data = instruction.data.as_slice();
    let instruction_accounts = instruction.accounts.as_slice();

    let mut account_metas = Vec::with_capacity(instruction_accounts.len());
    for account_meta in instruction_accounts {
        account_metas.push(SolAccountMeta {
            pubkey: account_meta.pubkey,
            is_writable: account_meta.is_writable,
            is_signer: account_meta.is_signer,
        });
    }

    let mut account_infos_c = Vec::with_capacity(account_infos.len());
    for account_info in account_infos {
        account_infos_c.push(SolAccountInfo {
            key: account_info.key,
            lamports: &mut account_info.lamports.borrow_mut(),
            data_len: account_info.data_len(),
            data: &mut account_info.data.borrow_mut(),
            owner: account_info.owner,
            rent_epoch: account_info.rent_epoch,
            is_signer: account_info.is_signer,
            is_writable: account_info.is_writable,
            executable: account_info.executable,
        });
    }