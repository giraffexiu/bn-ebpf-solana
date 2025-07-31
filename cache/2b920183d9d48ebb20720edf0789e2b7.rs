// Cached result for function: sub_100002340
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: 2b920183d9d48ebb20720edf0789e2b7.rs

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = MyInstruction::try_from_slice(instruction_data)?;

    match instruction {
        MyInstruction::Initialize {
            authority,
            vault_bump,
            mint_bump,
        } => initialize(
            program_id,
            accounts,
            authority,
            vault_bump,
            mint_bump,
        ),
        MyInstruction::Deposit { amount } => deposit(program_id, accounts, amount),
        MyInstruction::Withdraw { amount } => withdraw(program_id, accounts, amount),
        MyInstruction::CloseAccount => close_account(program_id, accounts),
    }
}

pub fn initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    authority: Pubkey,
    vault_bump: u8,
    mint_bump: u8,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let initializer_account = next_account_info(account_info_iter)?;
    let vault_account = next_account_info(account_info_iter)?;
    let vault_authority_account = next_account_info(account_info_iter)?;
    let mint_account = next_account_info(account_info_iter)?;
    let system_program_account = next_account_info(account_info_iter)?;
    let token_program_account = next_account_info(account_info_iter)?;
    let rent_sysvar_account = next_account_info(account_info_iter)?;

    if !initializer_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if !initializer_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }
    if !vault_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }
    if !mint_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    let (expected_vault_authority_key, expected_vault_authority_bump) =
        Pubkey::find_program_address(&[b"vault_authority"], program_id);
    if expected_vault_authority_key != *vault_authority_account.key {
        return Err(ProgramError::InvalidSeeds);
    }
    if expected_vault_authority_bump != vault_bump {
        return Err(ProgramError::InvalidSeeds);
    }

    let (expected_mint_key, expected_mint_bump) =
        Pubkey::find_program_address(&[b"mint"], program_id);
    if expected_mint_key != *mint_account.key {
        return Err(ProgramError::InvalidSeeds);
    }
    if expected_mint_bump != mint_bump {
        return Err(ProgramError::InvalidSeeds);
    }

    // Create the mint account
    invoke_signed(
        &spl_token::instruction::initialize_mint(
            token_program_account.key,
            mint_account.key,
            vault_authority_account.key,
            Some(vault_authority_account.key),
            0,
        )?,
        &[
            mint_account.clone(),
            rent_sysvar_account.clone(),
            token_program_account.clone(),
        ],
        &[&[b"mint", &[mint_bump]]],
    )?;

    // Create the token account for the vault
    invoke_signed(
        &spl_token::instruction::initialize_account(
            token_program_account.key,
            vault_account.key,
            mint_account.key,
            vault_authority_account.key,
        )?,
        &[
            vault_account.clone(),
            mint_account.clone(),
            vault_authority_account.clone(),
            rent_sysvar_account.clone(),