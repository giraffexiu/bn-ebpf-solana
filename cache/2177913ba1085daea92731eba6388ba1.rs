// Cached result for function: sub_100001860
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: 2177913ba1085daea92731eba6388ba1.rs

pub struct Initialize;

impl Initialize {
    pub const ACCOUNTS_LEN: usize = 3;
    pub const DATA_LEN: usize = 8;
}

pub fn process_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let _payer_account = next_account_info(accounts_iter)?;
    let _state_account = next_account_info(accounts_iter)?;
    let _system_program_account = next_account_info(accounts_iter)?;

    let _amount = u64::from_le_bytes(instruction_data[8..16].try_into().unwrap());

    Ok(())
}