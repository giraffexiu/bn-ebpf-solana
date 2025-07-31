// Cached result for function: sub_100002f08
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: 755b21276884d0110a535f2dbfe974e3.rs

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = MyInstruction::try_from_slice(instruction_data)?;

    match instruction {
        MyInstruction::Initialize => {
            let account_iter = &mut accounts.iter();
            let mut remaining_accounts = account_iter.clone();

            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(account_iter)?;
            let _ = next_account_info(