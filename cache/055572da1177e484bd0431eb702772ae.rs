// Cached result for function: sub_100001978
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: 055572da1177e484bd0431eb702772ae.rs

pub struct Initialize;

impl Initialize {
    pub const ACCOUNTS: [(&'static str, bool); 2] = [("authority", true), ("state", true)];
    pub const IDENT: [u8; 8] = [175, 179, 114, 237, 143, 206, 112, 235];

    pub fn instruction(
        authority: Pubkey,
        state: Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: crate::ID,
            accounts: vec![
                AccountMeta::new(authority, true),
                AccountMeta::new(state, true),
            ],
            data: Initialize::IDENT.to_vec(),
        }
    }
}

pub fn process_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let authority_account = next_account_info(account_info_iter)?;
    let state_account = next_account_info(account_info_iter)?;

    if authority_account.key != &program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    if !authority_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut state_data = state_account.try_borrow_mut_data()?;
    let mut state = State::try_from_slice(&state_data)?;

    state.authority = *authority_account.key;
    state.is_initialized = true;

    state.serialize(&mut &mut state_data[..])?;

    Ok(())
}