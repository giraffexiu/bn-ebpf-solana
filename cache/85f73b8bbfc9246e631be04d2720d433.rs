// Cached result for function: sub_1000000e8
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: 85f73b8bbfc9246e631be04d2720d433.rs

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let discriminator = u64::from_le_bytes(instruction_data[0..8].try_into().unwrap());
    match discriminator {
        0x6a09020967f1390e => {
            let ix_data = &instruction_data[8..];
            let (amount, remaining_ix_data) = ix_data.split_at(8);
            let amount = u64::from_le_bytes(amount.try_into().unwrap());
            let (authority_bump, _) = remaining_ix_data.split_at(1);
            let authority_bump = authority_bump[0];

            let account_info_iter = &mut accounts.iter();
            let mut_token_account = next_account_info(account_info_iter)?;
            let token_program = next_account_info(account_info_iter)?;
            let authority = next_account_info(account_info_iter)?;

            let authority_seeds: &[&[u8]] = &[
                b"authority",
                &[authority_bump],
            ];
            let signer_seeds = &[&authority_seeds[..]];

            let cpi_accounts = spl_token::instruction::transfer_checked_cpi_accounts(
                mut_token_account.clone(),
                mut_token_account.clone(), // This seems like a bug in the original code, should be destination
                authority.clone(),
                mut_token_account.clone(), // This seems like a bug in the original code, should be mint
            );
            let cpi_instruction = spl_token::instruction::transfer_checked(
                token_program.key,
                cpi_accounts,
                amount,
                9, // Assuming 9 decimal places based on typical SPL token usage
            )?;

            invoke_signed(
                &cpi_instruction,
                &[
                    mut_token_account.clone(),
                    mut_token_account.clone(), // This seems like a bug in the original code, should be destination
                    authority.clone(),
                    mut_token_account.clone(), // This seems like a bug in the original code, should be mint
                    token_program.clone(),
                ],
                signer_seeds,
            )
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

pub fn invoke_signed<'a>(
    instruction: &Instruction,
    account_infos: &[AccountInfo<'a>],
    signers_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let instruction_bytes = instruction.data.as_slice();
    let instruction_data_len = instruction_bytes.len() as u64;

    let mut metas: Vec<SolAccountMeta> = Vec::with_capacity(instruction.accounts.len());
    for account_meta in instruction.accounts.iter() {
        metas.push(SolAccountMeta {
            pubkey: account_meta.pubkey,
            is_writable: account_meta.is_writable,
            is_signer: account_meta.is_signer,
        });
    }

    let mut cpi_instruction = SolInstruction {
        program_id: instruction.program_id,
        accounts: &mut FixedAccountMeta { metas: metas.as_mut_ptr() as *mut _ },
        account_len: metas.len() as u64,
        data: instruction_bytes.as_ptr() as *mut u8,
        data_len: instruction_data_len,
    };

    let mut account_info_array: Vec<SolAccountInfo> = Vec::with_capacity(account_infos.len());
    for acc_info in account_infos.iter() {
        account_info_array.push(SolAccountInfo {