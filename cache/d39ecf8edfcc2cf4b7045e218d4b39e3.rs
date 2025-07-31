// Cached result for function: sub_100001858
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: d39ecf8edfcc2cf4b7045e218d4b39e3.rs

pub struct Initialize {
    pub authority: Pubkey,
    pub token_mint: Pubkey,
    pub token_account: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub rent: Pubkey,
}

pub fn initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    authority_bump: u8,
) -> ProgramResult {
    let authority_account = &accounts[0];
    let token_mint_account = &accounts[1];
    let token_account_account = &accounts[2];
    let system_program_account = &accounts[3];
    let token_program_account = &accounts[4];
    let rent_sysvar_account = &accounts[5];

    let seeds = &[b"authority", &[authority_bump]];
    let signer_seeds = &[&seeds[..]];

    // Create the token account
    invoke_signed(
        &spl_token::instruction::initialize_account(
            &token_program_account.key,
            &token_account_account.key,
            &token_mint_account.key,
            &authority_account.key,
        )?,
        &[
            token_account_account.clone(),
            token_mint_account.clone(),
            authority_account.clone(),
            rent_sysvar_account.clone(),
            token_program_account.clone(),
        ],
        signer_seeds,
    )?;

    Ok(())
}

#[inline(always)]
pub fn invoke_signed(
    instruction: &Instruction,
    account_infos: &[AccountInfo],
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    solana_program::program::invoke_signed(instruction, account_infos, signer_seeds)
}

#[inline(always)]
pub fn invoke(instruction: &Instruction, account_infos: &[AccountInfo]) -> ProgramResult {
    solana_program::program::invoke(instruction, account_infos)
}

#[inline(always)]
pub fn check_program_account(
    program_id: &Pubkey,
    account_info: &AccountInfo,
) -> ProgramResult {
    if account_info.key != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}

#[inline(always)]
pub fn check_signer(account_info: &AccountInfo) -> ProgramResult {
    if !account_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    Ok(())
}

#[inline(always)]
pub fn check_writable(account_info: &AccountInfo) -> ProgramResult {
    if !account_info.is_writable {
        return Err(ProgramError::InvalidAccountData); // Or a more specific error
    }
    Ok(())
}

#[inline(always)]
pub fn check_owner(account_info: &AccountInfo, expected_owner: &Pubkey) -> ProgramResult {
    if account_info.owner != expected_owner {
        return Err(ProgramError::IllegalOwner);
    }
    Ok(())
}