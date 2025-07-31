// Cached result for function: sub_1000004c0
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: e819c959196591cb5329ecdf667d5e55.rs

pub fn entry(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let discriminator = u64::from_le_bytes(instruction_data[0..8].try_into().unwrap());

    match discriminator {
        0x6207062400000000 => { // Anchor discriminator for `initialize`
            let ix_data = &instruction_data[8..];
            let ix_args: Initialize = AnchorDeserialize::deserialize(&mut ix_data.as_ref())?;
            initialize(program_id, accounts, ix_args.value)
        }
        0x6207062400000001 => { // Anchor discriminator for `update`
            let ix_data = &instruction_data[8..];
            let ix_args: Update = AnchorDeserialize::deserialize(&mut ix_data.as_ref())?;
            update(program_id, accounts, ix_args.value)
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

pub fn initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    value: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let account_0 = next_account_info(account_info_iter)?; // Counter account
    let account_1 = next_account_info(account_info_iter)?; // Payer account
    let account_2 = next_account_info(account_info_iter)?; // System program

    // Check if the counter account is writable and a signer
    if !account_0.is_writable || !account_0.is_signer {
        return Err(ProgramError::InvalidArgument);
    }

    // Check if the payer account is writable and a signer
    if !account_1.is_writable || !account_1.is_signer {
        return Err(ProgramError::InvalidArgument);
    }

    // Check if the system program is the correct program ID
    if account_2.key != &solana_program::system_program::id() {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Calculate rent exemption and allocate space for the counter account
    let rent = Rent::get()?;
    let space = 8 + 8; // 8 bytes for discriminator, 8 bytes for u64 value
    let lamports = rent.minimum_balance(space);

    // Invoke the system program to create the account
    invoke(
        &system_instruction::create_account(
            account_1.key,
            account_0.key,
            lamports,
            space as u64,
            program_id,
        ),
        &[account_1.clone(), account_0.clone(), account_2.clone()],
    )?;

    // Write the initial value to the counter account
    let mut account_data = account_0.data.borrow_mut();
    account_data[8..16].copy_from_slice(&value.to_le_bytes());

    Ok(())
}

pub fn update(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    value: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let account_0 = next_account_info(account_info_iter)?; // Counter account
    let account_1 = next_account_info(account_info_iter)?; // Payer account (signer)

    // Check if the counter account is writable and owned by the program
    if !account_0.is_writable || account_0.owner != program_id {
        return Err(ProgramError::